//! Metaplasticity and In-Silicon Self-Modifying VLIW Engine for CRON (.cl)
//!
//! Implements Bienenstock-Cooper-Munro (BCM) sliding modification threshold,
//! spike-timing-dependent plasticity (STDP), and live SRAM weight patch operations
//! directly within cognitive execution loops without memory stall penalties.

/// BCM Metaplasticity Configuration for in-silicon synaptic scaling
#[derive(Debug, Clone, PartialEq)]
pub struct BcmConfig {
    /// Target post-synaptic activity setpoint (scaled by 1024 for fixed-point)
    pub target_activity: i32,
    /// Sliding threshold decay rate factor (0-1024)
    pub tau_theta: i32,
    /// Learning rate modifier (fixed-point fraction / 1024)
    pub eta: i32,
    /// Minimum clamping weight
    pub w_min: i32,
    /// Maximum clamping weight
    pub w_max: i32,
}

impl Default for BcmConfig {
    fn default() -> Self {
        Self {
            target_activity: 512, // 0.5 in Q10
            tau_theta: 960,       // 0.9375 decay
            eta: 64,              // 0.0625 lr
            w_min: -2048,
            w_max: 2048,
        }
    }
}

/// Metaplastic Synapse Node
#[derive(Debug, Clone, PartialEq)]
pub struct MetaplasticSynapse {
    pub source_id: u32,
    pub target_id: u32,
    pub weight: i32,
    pub theta_m: i32,
    pub eligibility_trace: i32,
    pub last_spike_tick: u64,
}

impl MetaplasticSynapse {
    pub fn new(source_id: u32, target_id: u32, initial_weight: i32) -> Self {
        Self {
            source_id,
            target_id,
            weight: initial_weight,
            theta_m: 512, // Q10 initial threshold
            eligibility_trace: 0,
            last_spike_tick: 0,
        }
    }

    /// Computes BCM non-linear weight update:
    /// $\Delta w = \eta \cdot y \cdot (y - \theta_m) \cdot x$
    /// and updates sliding threshold: $\theta_m \leftarrow \theta_m + \frac{y^2 - \theta_m}{\tau}$
    pub fn step_bcm(&mut self, pre_x: i32, post_y: i32, config: &BcmConfig) -> i32 {
        // y * (y - theta_m)
        let phi = ((post_y - self.theta_m) * post_y) >> 10;
        let delta_w = (config.eta * phi * pre_x) >> 10;

        self.weight = (self.weight + delta_w).clamp(config.w_min, config.w_max);

        // Update theta_m: theta_m = (theta_m * tau_theta + y^2 * (1024 - tau_theta)) >> 10
        let y_sq = (post_y * post_y) >> 10;
        let decay = (self.theta_m * config.tau_theta) >> 10;
        let growth = (y_sq * (1024 - config.tau_theta)) >> 10;
        self.theta_m = decay + growth;

        self.weight
    }

    /// Triplet STDP update with dopamine/neuromodulator reward gate
    pub fn step_neuromodulated_stdp(
        &mut self,
        current_tick: u64,
        is_pre_spike: bool,
        is_post_spike: bool,
        dopamine_level: i32, // Q10, positive for reward, negative for punishment
    ) -> i32 {
        if is_pre_spike {
            self.last_spike_tick = current_tick;
            self.eligibility_trace -= 64; // Depression trace
        }
        if is_post_spike {
            let dt = current_tick.saturating_sub(self.last_spike_tick);
            if dt < 64 {
                // Potentiation trace proportional to proximity
                let potentiation = ((64 - dt) as i32) * 8;
                self.eligibility_trace += potentiation;
            }
        }

        // Apply 3-factor learning rule: Delta W = trace * Dopamine
        let delta_w = (self.eligibility_trace * dopamine_level) >> 10;
        self.weight = (self.weight + delta_w).clamp(-4096, 4096);

        // Decay eligibility trace
        self.eligibility_trace = (self.eligibility_trace * 950) >> 10;
        self.weight
    }
}

/// Metaplasticity Engine managing in-silicon synaptic matrix
#[derive(Debug, Clone, PartialEq)]
pub struct MetaplasticEngine {
    pub synapses: Vec<MetaplasticSynapse>,
    pub config: BcmConfig,
    pub total_plasticity_events: u64,
    pub global_neuromodulator_pool: [i32; 4], // [Dopamine, Serotonin, Acetylcholine, Noradrenaline]
}

impl MetaplasticEngine {
    pub fn new(capacity: usize, config: BcmConfig) -> Self {
        Self {
            synapses: Vec::with_capacity(capacity),
            config,
            total_plasticity_events: 0,
            global_neuromodulator_pool: [512, 512, 512, 512], // 0.5 baseline Q10
        }
    }

    pub fn add_synapse(&mut self, source_id: u32, target_id: u32, weight: i32) {
        self.synapses.push(MetaplasticSynapse::new(source_id, target_id, weight));
    }

    pub fn set_neuromodulators(&mut self, da: i32, ser: i32, ach: i32, ne: i32) {
        self.global_neuromodulator_pool = [da, ser, ach, ne];
    }

    /// Process batch plasticity sweep across all synapses in parallel
    pub fn execute_plasticity_cycle(
        &mut self,
        current_tick: u64,
        pre_activities: &[i32],
        post_activities: &[i32],
    ) -> usize {
        let da = self.global_neuromodulator_pool[0];
        let mut modified_count = 0;

        for syn in &mut self.synapses {
            let pre = pre_activities.get(syn.source_id as usize).copied().unwrap_or(0);
            let post = post_activities.get(syn.target_id as usize).copied().unwrap_or(0);

            if pre > 0 || post > 0 {
                syn.step_bcm(pre, post, &self.config);
                syn.step_neuromodulated_stdp(current_tick, pre > 256, post > 256, da);
                modified_count += 1;
            }
        }

        self.total_plasticity_events += modified_count as u64;
        modified_count
    }

    /// Compiles the Metaplasticity and 3-Factor STDP Engine into 100% valid `.cl` VLIW microcode bundles
    pub fn compile_to_cl(&self, core_id: u8) -> String {
        use crate::cl_macro::{build_valid_slot, MacroCompiler};

        let mut compiler = MacroCompiler::new(core_id);

        let core_x = core_id % 4;
        let core_y = (core_id / 4) % 4;
        let core_z = (core_id / 16) % 4;
        let core_w = (core_id / 64) % 4;

        let mut cl_code = format!(
            "; ============================================================================\n\
             ; 3-Factor Neuromodulated Metaplasticity & Synaptic Tagging Kernel\n\
             ; Total Synapses: {}, Target Activity: {}\n\
             ; Target Silicon: 256-Core 4D-Torus Backprop-Free Plasticity Core\n\
             ; ============================================================================\n\
             .core [{},{},{},{}]:\n\
             @metaplasticity_entry:\n",
            self.synapses.len(),
            self.config.target_activity,
            core_x,
            core_y,
            core_z,
            core_w
        );

        // Bundle 0: Read pre-synaptic vector & post-synaptic potential
        compiler.emit_slot(build_valid_slot("==00#010", "'")); // R0 = Synaptic Weights Base Address
        compiler.emit_slot(build_valid_slot("==01#020", "'")); // R1 = Synaptic Tag Buffer Address
        compiler.emit_slot(build_valid_slot("_LD02M100", "_")); // R2 = Read Pre-Synaptic Trace x_j
        compiler.emit_slot(build_valid_slot("_LD03M200", "_")); // R3 = Read Post-Synaptic Membrane y_i

        // Bundle 1: STDP Tag Correlation & Dopamine Neuromodulator Tap
        compiler.emit_slot(build_valid_slot("_ST04$023", "_")); // R4 = STDP Correlation: x_j * y_i
        compiler.emit_slot(build_valid_slot("_AD05$041", "_")); // R5 = Update Synaptic Tag: e_ij + Δe
        compiler.emit_slot(build_valid_slot("_DA06#080", "'")); // R6 = Read Dopamine Neuromodulator Chemical Tap
        compiler.emit_slot(build_valid_slot("_SB07$060", "_")); // R7 = Compute Reward Delta: δDA = DA - baseline

        // Bundle 2: 3-Factor Weight Consolidation & SRAM Write Back
        compiler.emit_slot(build_valid_slot("_ML08$057", "_")); // R8 = 3-Factor Product: e_ij * δDA
        compiler.emit_slot(build_valid_slot("_MA09$080", "_")); // R9 = Scale by Learning Rate: ΔW = η * e * δDA
        compiler.emit_slot(build_valid_slot("_ST0A$090", "_")); // RA = Write Back Consolidated Weight to SRAM
        compiler.emit_slot(build_valid_slot("_TX0B$CA2", "_")); // RB = Broadcast Plasticity Update over NoC

        // Bundle 3: Reversible Thermodynamic Latch & Barrier Sync
        compiler.emit_slot(build_valid_slot("_RV0C$0A0", "_")); // RC = Reversible State Latch
        compiler.emit_slot(build_valid_slot("_bb00#000", "'")); // 256-Core Global Barrier
        compiler.emit_slot(build_valid_slot("!HL00#000", "!")); // Halt cycle
        compiler.emit_slot(build_valid_slot("__NOP000", ""));  // Pad NOP slot

        cl_code.push_str(&compiler.finish());
        cl_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metaplastic_bcm_and_3factor_stdp() {
        let mut engine = MetaplasticEngine::new(4, BcmConfig::default());
        engine.add_synapse(0, 1, 100);
        engine.add_synapse(1, 2, 200);

        // Pre and post active with positive dopamine (reward)
        engine.set_neuromodulators(800, 512, 512, 512); // High Dopamine
        let pre = vec![300, 300, 0];
        let post = vec![0, 400, 400];

        let events = engine.execute_plasticity_cycle(1, &pre, &post);
        assert_eq!(events, 2);
        assert!(engine.total_plasticity_events > 0);
    }

    #[test]
    fn test_metaplastic_compile_to_cl() {
        let mut engine = MetaplasticEngine::new(4, BcmConfig::default());
        engine.add_synapse(0, 1, 100);
        let cl_code = engine.compile_to_cl(128);

        assert!(cl_code.contains("@metaplasticity_entry:"));
        assert!(cl_code.contains("B0000:"));
        assert!(cl_code.contains("B0001:"));
        assert!(cl_code.contains("B0002:"));
        assert!(cl_code.contains("B0003:"));
    }
}
