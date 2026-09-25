//! Living Bio-Chemical Neuro-Modulation System
//!
//! Simulates 4 primary brain neurotransmitters (Dopamine, Serotonin, Norepinephrine, Acetylcholine)
//! and dynamically modulates synaptic plasticity rates, spiking thresholds, and NoC priority
//! across the 256 cores of the SAGI AGI supercomputer.

/// The 4 Primary Neuromodulatory Chemical Concentrations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NeuromodulatorState {
    /// Dopamine (DA): Reward signal, boosts synaptic reinforcement & learning rate [0.0..1.0]
    pub dopamine: f32,
    /// Serotonin (5-HT): Homeostatic stability, patience, suppresses noisy firing [0.0..1.0]
    pub serotonin: f32,
    /// Norepinephrine (NE): Arousal, surprise, elevates NoC routing priority [0.0..1.0]
    pub norepinephrine: f32,
    /// Acetylcholine (ACh): Attentional focus, sets encoding vs retrieval plasticity [0.0..1.0]
    pub acetylcholine: f32,
}

impl Default for NeuromodulatorState {
    fn default() -> Self {
        Self {
            dopamine: 0.5,
            serotonin: 0.7,
            norepinephrine: 0.3,
            acetylcholine: 0.6,
        }
    }
}

/// Dynamic Hardware Control Parameters derived from Chemical State
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DynamicCoreTuning {
    /// Effective STDP Learning Rate Multiplier (0.1x to 5.0x)
    pub stdp_learning_rate_scale: f32,
    /// Leaky Integrate-and-Fire (LIF) Spike Threshold Voltage
    pub lif_spike_threshold: f32,
    /// NoC 4D-Torus Packet Priority (0 = Background, 1 = Normal, 2 = High, 3 = Critical Emergency)
    pub noc_priority_level: u8,
    /// Memory Mode: True = Encode new memories, False = Recall existing memories
    pub is_encoding_dominant: bool,
}

/// Continuous Neuromodulatory Controller Engine
#[derive(Debug, Clone)]
pub struct NeuromodulationEngine {
    pub current_state: NeuromodulatorState,
    pub decay_rate: f32,
    pub step_counter: u64,
}

impl NeuromodulationEngine {
    pub fn new() -> Self {
        Self {
            current_state: NeuromodulatorState::default(),
            decay_rate: 0.05,
            step_counter: 0,
        }
    }

    /// Process cognitive event and modulate chemical concentrations
    pub fn trigger_event(&mut self, reward_delta: f32, surprise_salience: f32, task_complexity: f32) {
        self.step_counter += 1;

        // 1. Dopamine responds to Reward Prediction Error (RPE)
        self.current_state.dopamine = (self.current_state.dopamine + reward_delta * 0.4)
            .clamp(0.05, 1.0);

        // 2. Norepinephrine surges on unexpected surprise / novel stimuli
        self.current_state.norepinephrine = (self.current_state.norepinephrine + surprise_salience * 0.5)
            .clamp(0.05, 1.0);

        // 3. Acetylcholine increases with task complexity and attention demand
        self.current_state.acetylcholine = (self.current_state.acetylcholine + task_complexity * 0.3)
            .clamp(0.1, 1.0);

        // 4. Serotonin provides baseline stability and dampens runaway dopamine/NE
        if self.current_state.dopamine > 0.8 {
            self.current_state.serotonin = (self.current_state.serotonin + 0.1).clamp(0.0, 1.0);
        }
    }

    /// Homeostatic step: slowly decay chemicals back to resting equilibrium
    pub fn step_homeostasis(&mut self) {
        let baseline = NeuromodulatorState::default();
        self.current_state.dopamine += (baseline.dopamine - self.current_state.dopamine) * self.decay_rate;
        self.current_state.serotonin += (baseline.serotonin - self.current_state.serotonin) * self.decay_rate;
        self.current_state.norepinephrine += (baseline.norepinephrine - self.current_state.norepinephrine) * self.decay_rate;
        self.current_state.acetylcholine += (baseline.acetylcholine - self.current_state.acetylcholine) * self.decay_rate;
    }

    /// Derives hardware control tuning for the 256-Core processor
    pub fn derive_core_tuning(&self) -> DynamicCoreTuning {
        // High DA + High ACh = Maximum synaptic plasticity
        let stdp_rate = (self.current_state.dopamine * 2.0 + self.current_state.acetylcholine * 1.5)
            .clamp(0.1, 5.0);

        // High 5-HT increases firing threshold (reduces noisy hyperactivity)
        let spike_threshold = 1.0 + (self.current_state.serotonin * 0.5) - (self.current_state.norepinephrine * 0.3);

        // High NE triggers emergency high-priority routing
        let noc_priority = if self.current_state.norepinephrine > 0.75 {
            3 // Critical
        } else if self.current_state.norepinephrine > 0.5 {
            2 // High
        } else {
            1 // Normal
        };

        // ACh > 0.5 favors Encoding new sensory input over Retrospective Recall
        let is_encoding = self.current_state.acetylcholine >= 0.5;

        DynamicCoreTuning {
            stdp_learning_rate_scale: stdp_rate,
            lif_spike_threshold: spike_threshold.clamp(0.5, 2.5),
            noc_priority_level: noc_priority,
            is_encoding_dominant: is_encoding,
        }
    }

    /// Render human-readable chemical HUD
    pub fn render_ascii_hud(&self) -> String {
        let tuning = self.derive_core_tuning();
        format!(
            "+-------------------------------------------------------------------------+\n\
             | SAGI LIVING NEUROMODULATORY CHEMICAL DYNAMICS                           |\n\
             +-------------------------------------------------------------------------+\n\
             | Dopamine (DA - Reward/Plasticity):     [{:<20}] {:.2}        |\n\
             | Serotonin (5-HT - Stability/Patience): [{:<20}] {:.2}        |\n\
             | Norepinephrine (NE - Arousal/Surprise):[{:<20}] {:.2}        |\n\
             | Acetylcholine (ACh - Focus/Encoding):  [{:<20}] {:.2}        |\n\
             +-------------------------------------------------------------------------+\n\
             | Derived Hardware Tuning:                                                |\n\
             | • STDP Plasticity Multiplier:  {:.2}x                                     |\n\
             | • LIF Spiking Threshold:       {:.2} V                                    |\n\
             | • 4D-Torus NoC Packet Priority:Level {} ({})                           |\n\
             | • Dominant Cognitive Mode:     {}                                |\n\
             +-------------------------------------------------------------------------+\n",
            make_bar(self.current_state.dopamine),
            self.current_state.dopamine,
            make_bar(self.current_state.serotonin),
            self.current_state.serotonin,
            make_bar(self.current_state.norepinephrine),
            self.current_state.norepinephrine,
            make_bar(self.current_state.acetylcholine),
            self.current_state.acetylcholine,
            tuning.stdp_learning_rate_scale,
            tuning.lif_spike_threshold,
            tuning.noc_priority_level,
            match tuning.noc_priority_level { 3 => "CRITICAL", 2 => "HIGH", _ => "NORMAL" },
            if tuning.is_encoding_dominant { "NOVEL ENCODING" } else { "ASSOCIATIVE RECALL" },
        )
    }
}

fn make_bar(val: f32) -> String {
    let filled = ((val.clamp(0.0, 1.0) * 20.0).round() as usize).min(20);
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('#');
    }
    while bar.len() < 20 {
        bar.push('-');
    }
    bar
}
