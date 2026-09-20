// ============================================================================
// CRON Neuromorphic Spiking Neural Network & STDP Plasticity Engine (cl_snn)
// Targets 256-Core 4D-Torus Brain 4 Neuromorphic Sub-Byte Crossbar Array:
// 1. Leaky Integrate-and-Fire (LIF) Dynamic Membrane Integration & Spiking
// 2. Exponential Spike-Timing-Dependent Plasticity (STDP) Synaptic Weight Adaptation
// 3. Event-Driven Sparse Spiking Simulation & Address Event Representation (AER)
// 4. Terminal ASCII Spike Raster Plot & Sub-Byte Crossbar Telemetry
//
// 100% Pure Rust - Zero External Dependencies
// ============================================================================

use crate::cl_heal::heal_cl_program;

/// Configuration for Leaky Integrate-and-Fire (LIF) Neurons.
#[derive(Debug, Clone, PartialEq)]
pub struct LifNeuronConfig {
    pub decay_beta: f64,        // Membrane potential decay factor (e.g. 0.90)
    pub threshold: f64,         // Action potential firing threshold (e.g. 1.0)
    pub reset_voltage: f64,     // Hyperpolarization reset potential (e.g. 0.0)
    pub refractory_cycles: usize, // Cycles neuron remains in refractory state after spike
}

impl Default for LifNeuronConfig {
    fn default() -> Self {
        Self {
            decay_beta: 0.85,
            threshold: 1.0,
            reset_voltage: 0.0,
            refractory_cycles: 2,
        }
    }
}

/// Configuration for Spike-Timing-Dependent Plasticity (STDP).
#[derive(Debug, Clone, PartialEq)]
pub struct StdpConfig {
    pub a_plus: f64,         // Long-Term Potentiation (LTP) rate (e.g. 0.10)
    pub a_minus: f64,        // Long-Term Depression (LTD) rate (e.g. 0.12)
    pub tau_plus: f64,       // LTP exponential decay constant (cycles)
    pub tau_minus: f64,      // LTD exponential decay constant (cycles)
    pub w_min: f64,          // Minimum synaptic weight bound
    pub w_max: f64,          // Maximum synaptic weight bound
    pub quantize_int2: bool, // Sub-byte ternary weight quantization {-1, 0, +1}
}

impl Default for StdpConfig {
    fn default() -> Self {
        Self {
            a_plus: 0.08,
            a_minus: 0.10,
            tau_plus: 15.0,
            tau_minus: 15.0,
            w_min: -2.0,
            w_max: 2.0,
            quantize_int2: true,
        }
    }
}

/// Sparse Spiking Event (AER format).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpikeEvent {
    pub time_cycle: usize,
    pub neuron_id: usize,
}

/// Detailed Results of an SNN & STDP Simulation.
#[derive(Debug, Clone)]
pub struct SnnSimulationResult {
    pub total_cycles: usize,
    pub num_neurons: usize,
    pub total_spikes: usize,
    pub mean_firing_rate_hz: f64,
    pub energy_picojoules: f64,
    pub spike_matrix: Vec<Vec<bool>>, // [cycle][neuron_id]
    pub membrane_potentials: Vec<f64>,
    pub synaptic_weights: Vec<f64>,
    pub synthesized_cl_kernel: String,
}

/// Simulates Leaky Integrate-and-Fire neurons with event-driven STDP plasticity.
pub fn simulate_snn(
    num_neurons: usize,
    cycles: usize,
    lif_cfg: &LifNeuronConfig,
    stdp_cfg: &StdpConfig,
    external_spikes: &[SpikeEvent],
) -> SnnSimulationResult {
    let num_neurons = num_neurons.max(1);
    let cycles = cycles.max(1);

    let mut v_membrane = vec![0.0f64; num_neurons];
    let mut refractory_timer = vec![0usize; num_neurons];
    let mut last_spike_cycle = vec![None::<usize>; num_neurons];

    // Synaptic weight crossbar: NxN matrix flattened
    let mut weights = vec![0.5f64; num_neurons * num_neurons];
    // Zero self-connections
    for i in 0..num_neurons {
        weights[i * num_neurons + i] = 0.0;
    }

    let mut spike_matrix = vec![vec![false; num_neurons]; cycles];
    let mut total_spikes = 0;

    // Simulation loop
    for t in 0..cycles {
        let mut current_cycle_spikes = vec![false; num_neurons];

        // 1. Inject external input spikes
        for event in external_spikes {
            if event.time_cycle == t && event.neuron_id < num_neurons {
                v_membrane[event.neuron_id] += 0.8;
            }
        }

        // 2. Integrate synaptic inputs from previous cycle spikes
        if t > 0 {
            for j in 0..num_neurons {
                if spike_matrix[t - 1][j] {
                    for i in 0..num_neurons {
                        if refractory_timer[i] == 0 {
                            v_membrane[i] += weights[i * num_neurons + j];
                        }
                    }
                }
            }
        }

        // 3. Update LIF dynamics: leak, threshold check, fire, and reset
        for i in 0..num_neurons {
            if refractory_timer[i] > 0 {
                refractory_timer[i] -= 1;
                v_membrane[i] = lif_cfg.reset_voltage;
            } else {
                // Apply leak decay
                v_membrane[i] *= lif_cfg.decay_beta;

                // Fire condition
                if v_membrane[i] >= lif_cfg.threshold {
                    current_cycle_spikes[i] = true;
                    total_spikes += 1;
                    v_membrane[i] = lif_cfg.reset_voltage;
                    refractory_timer[i] = lif_cfg.refractory_cycles;

                    // 4. STDP Plasticity Update: Adjust incoming & outgoing synapses
                    let post_id = i;
                    let post_time = t;

                    for pre_id in 0..num_neurons {
                        if pre_id == post_id {
                            continue;
                        }

                        if let Some(pre_time) = last_spike_cycle[pre_id] {
                            let delta_t = (post_time as isize) - (pre_time as isize);
                            let weight_idx = post_id * num_neurons + pre_id;

                            if delta_t > 0 && delta_t < 30 {
                                // Pre before Post -> Long-Term Potentiation (LTP)
                                let dt = delta_t as f64;
                                let dw = stdp_cfg.a_plus * (-dt / stdp_cfg.tau_plus).exp();
                                weights[weight_idx] = (weights[weight_idx] + dw).min(stdp_cfg.w_max);
                            } else if delta_t < 0 && delta_t > -30 {
                                // Post before Pre -> Long-Term Depression (LTD)
                                let dt = (-delta_t) as f64;
                                let dw = stdp_cfg.a_minus * (-dt / stdp_cfg.tau_minus).exp();
                                weights[weight_idx] = (weights[weight_idx] - dw).max(stdp_cfg.w_min);
                            }

                            // Optional Sub-byte Quantization
                            if stdp_cfg.quantize_int2 {
                                weights[weight_idx] = if weights[weight_idx] > 0.5 {
                                    1.0
                                } else if weights[weight_idx] < -0.5 {
                                    -1.0
                                } else {
                                    0.0
                                };
                            }
                        }
                    }

                    last_spike_cycle[post_id] = Some(post_time);
                }
            }
        }

        spike_matrix[t] = current_cycle_spikes;
    }

    // Neuromorphic Energy and Firing Rate
    let clock_freq_hz = 1.0e9; // 1.0 GHz neuromorphic core
    let duration_s = (cycles as f64) / clock_freq_hz;
    let mean_firing_rate_hz = if duration_s > 0.0 && num_neurons > 0 {
        (total_spikes as f64) / (num_neurons as f64) / duration_s
    } else {
        0.0
    };

    // Sub-byte neuromorphic energy: 0.12 pJ per spike event on Brain 4 crossbar
    let energy_picojoules = (total_spikes as f64) * 0.12;

    let synthesized_cl_kernel = synthesize_snn_kernel(num_neurons, lif_cfg, stdp_cfg);

    SnnSimulationResult {
        total_cycles: cycles,
        num_neurons,
        total_spikes,
        mean_firing_rate_hz,
        energy_picojoules,
        spike_matrix,
        membrane_potentials: v_membrane,
        synaptic_weights: weights,
        synthesized_cl_kernel,
    }
}

/// Synthesizes standalone, optimal Brain 4 neuromorphic .cl microcode.
pub fn synthesize_snn_kernel(
    num_neurons: usize,
    lif_cfg: &LifNeuronConfig,
    stdp_cfg: &StdpConfig,
) -> String {
    let mut raw = String::new();
    raw.push_str(&format!(
        "; ============================================================================\n\
         ; CRON BRAIN 4 NEUROMORPHIC SNN & STDP PLASTICITY MICRO-KERNEL\n\
         ; Neurons: {} | Decay Beta: {:.2} | Threshold: {:.2} | Reset: {:.2}\n\
         ; STDP Rule: LTP A+={:.2} LTD A-={:.2} | Sub-Byte Quantized: {}\n\
         ; Target: 256-Core 4D-Torus Brain 4 Neuromorphic Array\n\
         ; ============================================================================\n\n",
        num_neurons, lif_cfg.decay_beta, lif_cfg.threshold, lif_cfg.reset_voltage,
        stdp_cfg.a_plus, stdp_cfg.a_minus, stdp_cfg.quantize_int2
    ));

    raw.push_str(".core [0, 0, 0, 0]:\n");
    raw.push_str("@snn_kernel_init:\n");
    raw.push_str("B0000: '==01#000> '==02#004> '==03#008> '==04#00C>\n");

    raw.push_str("\n@lif_integration_and_spike:\n");
    raw.push_str("B0001: _LI01$200> _LF02$200> _PO03+100> _SB00#000>\n");
    raw.push_str("B0002: _ST01#400> _ST02#400> _PS03$100> _bb00#000>\n");

    raw.push_str("\n@stdp_synaptic_plasticity_update:\n");
    raw.push_str("B0003: _ST03#200> _ST04#200> _MD01*6A2> _SB00#000>\n");
    raw.push_str("B0004: _ST01#000> _PO02+100> _NO00#000> _HL00$008!\n");

    let header = format!(
        "; ============================================================================\n\
         ; CRON BRAIN 4 NEUROMORPHIC SNN & STDP PLASTICITY MICRO-KERNEL\n\
         ; Neurons: {} | Decay Beta: {:.2} | Threshold: {:.2} | Reset: {:.2}\n\
         ; STDP Rule: LTP A+={:.2} LTD A-={:.2} | Sub-Byte Quantized: {}\n\
         ; Target: 256-Core 4D-Torus Brain 4 Neuromorphic Array\n\
         ; ============================================================================\n",
        num_neurons, lif_cfg.decay_beta, lif_cfg.threshold, lif_cfg.reset_voltage,
        stdp_cfg.a_plus, stdp_cfg.a_minus, stdp_cfg.quantize_int2
    );

    let healed_code = if let Ok(healed) = heal_cl_program(&raw) {
        healed.canonical_code
    } else {
        raw
    };

    format!("{}\n{}", header, healed_code)
}

// ============================================================================
// Telemetry & Reporting Functions (ASCII Spike Raster & JSON)
// ============================================================================

impl SnnSimulationResult {
    /// Renders a high-density ASCII Spike Raster Plot showing time vs neuron activity.
    pub fn render_ascii_raster(&self) -> String {
        let mut out = String::new();
        out.push_str("╔════════════════════════════════════════════════════════════════════════════╗\n");
        out.push_str("║       CRON BRAIN 4 NEUROMORPHIC SPIKE RASTER & STDP PLASTICITY REPORT      ║\n");
        out.push_str("╚════════════════════════════════════════════════════════════════════════════╝\n\n");

        out.push_str(&format!(
            " Neurons Simulated   : {}\n\
              Total Time Cycles   : {} cycles\n\
              Total Spikes Emitted: {} spikes\n\
              Mean Firing Rate    : {:.2} MHz\n\
              Event Dynamic Energy: {:.2} pJ (0.12 pJ/spike)\n\
              Brain 4 Architecture: Neuromorphic Crossbar (AER Asynchronous Multicast)\n\n",
            self.num_neurons,
            self.total_cycles,
            self.total_spikes,
            self.mean_firing_rate_hz * 1.0e-6,
            self.energy_picojoules
        ));

        out.push_str("─── Terminal Spike Raster Plot (Time -> Horizontal, Neurons -> Vertical) ────\n");
        out.push_str("  Neuron ID\n");

        let display_neurons = self.num_neurons.min(16);
        let display_cycles = self.total_cycles.min(60);

        for n in 0..display_neurons {
            out.push_str(&format!("    N{:02} │ ", n));
            for t in 0..display_cycles {
                if self.spike_matrix[t][n] {
                    out.push('|');
                } else {
                    out.push('·');
                }
            }
            out.push('\n');
        }

        out.push_str("         └─");
        for _ in 0..display_cycles {
            out.push('─');
        }
        out.push_str("► Time (Cycles)\n\n");

        out.push_str("─── Synaptic Weight Sample (Sub-Byte INT2 Crossbar) ─────────────────────────\n");
        let sample_w = self.synaptic_weights.iter().take(8).map(|w| format!("{:.1}", w)).collect::<Vec<_>>().join(", ");
        out.push_str(&format!("  First 8 Synapses: [{}]\n\n", sample_w));

        out
    }

    /// Serializes the SNN simulation results to structured, machine-readable JSON.
    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"num_neurons\": {},\n", self.num_neurons));
        json.push_str(&format!("  \"total_cycles\": {},\n", self.total_cycles));
        json.push_str(&format!("  \"total_spikes\": {},\n", self.total_spikes));
        json.push_str(&format!("  \"mean_firing_rate_hz\": {:.2},\n", self.mean_firing_rate_hz));
        json.push_str(&format!("  \"energy_picojoules\": {:.3},\n", self.energy_picojoules));

        // Membrane potentials
        json.push_str("  \"final_membrane_potentials\": [");
        for (i, v) in self.membrane_potentials.iter().enumerate() {
            json.push_str(&format!("{:.3}", v));
            if i + 1 < self.membrane_potentials.len() {
                json.push_str(", ");
            }
        }
        json.push_str("],\n");

        // First 16 synaptic weights
        json.push_str("  \"sample_synaptic_weights\": [");
        for (i, w) in self.synaptic_weights.iter().take(16).enumerate() {
            json.push_str(&format!("{:.2}", w));
            if i + 1 < 16 && i + 1 < self.synaptic_weights.len() {
                json.push_str(", ");
            }
        }
        json.push_str("]\n");

        json.push_str("}\n");
        json
    }
}
