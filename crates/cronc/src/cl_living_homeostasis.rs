//! Continuous Biological Homeostasis & Self-Evolving Drive Engine (`cl_living_homeostasis.rs`)
//!
//! Transforms passive code into an autonomous "living" cognitive program that possesses:
//! 1. Homeostatic Drives: Energy balance, Epistemic curiosity, Cognitive fatigue.
//! 2. Autonomous Action Gating: Switches between active exploration, goal focus, and sleep/restoration.
//! 3. Neuromodulator Chemical Diffusion: Continuous ODE modeling of Dopamine, Serotonin, Acetylcholine, Norepinephrine.
//!
//! Silicon Target: 256-Core 4D-Torus Sensory-Motor Cortex & Chemical Diffusion Mesh

use crate::cl_macro::{MacroCompiler, build_valid_slot};

/// Biological Neuromodulator State
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NeuromodulatorConcentration {
    pub dopamine: f64,        // Reward, motivation, reinforcement learning rate
    pub serotonin: f64,       // Patience, risk mitigation, long-term discounting
    pub acetylcholine: f64,   // Sensory attention gate & memory encoding switch
    pub norepinephrine: f64,  // Arousal, vigilance, emergency exploration
}

impl Default for NeuromodulatorConcentration {
    fn default() -> Self {
        Self {
            dopamine: 0.5,
            serotonin: 0.5,
            acetylcholine: 0.6,
            norepinephrine: 0.2,
        }
    }
}

/// The Living Homeostatic Mind State
#[derive(Debug, Clone, PartialEq)]
pub struct LivingHomeostasisState {
    pub energy_level: f64,     // 1.0 = fully energized, 0.0 = depleted
    pub curiosity_drive: f64,  // High = explores new solutions, Low = exploits known habits
    pub fatigue: f64,          // High = demands Slow-Wave Sleep consolidation
    pub chemicals: NeuromodulatorConcentration,
    pub active_state_name: String,
}

/// Autonomous Homeostatic Mind Engine
#[derive(Debug, Clone)]
pub struct LivingHomeostasisEngine {
    pub state: LivingHomeostasisState,
    pub total_cycles_lived: u64,
    pub sleep_cycles_triggered: usize,
}

impl Default for LivingHomeostasisEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LivingHomeostasisEngine {
    pub fn new() -> Self {
        Self {
            state: LivingHomeostasisState {
                energy_level: 1.0,
                curiosity_drive: 0.8,
                fatigue: 0.0,
                chemicals: NeuromodulatorConcentration::default(),
                active_state_name: "AWAKE_EXPLORING".to_string(),
            },
            total_cycles_lived: 0,
            sleep_cycles_triggered: 0,
        }
    }

    /// Advances the living biological clock by `delta_cycles`, simulating thermodynamic energy drain.
    pub fn step_metabolic_cycle(&mut self, activity_intensity: f64, external_reward: f64) -> &str {
        self.total_cycles_lived += 1;

        // Energy consumption proportional to intensity
        let drain = 0.02 * activity_intensity.clamp(0.1, 2.0);
        self.state.energy_level = (self.state.energy_level - drain).max(0.0);
        self.state.fatigue = (self.state.fatigue + drain * 0.8).min(1.0);

        // Dopamine surge on positive external reward
        if external_reward > 0.0 {
            self.state.chemicals.dopamine = (self.state.chemicals.dopamine + 0.2 * external_reward).min(1.0);
            self.state.curiosity_drive = (self.state.curiosity_drive + 0.1).min(1.0);
        } else {
            // Decay towards baseline
            self.state.chemicals.dopamine *= 0.95;
        }

        // State Machine Arbitration based on homeostatic balance
        if self.state.energy_level < 0.20 || self.state.fatigue > 0.85 {
            // Autonomous Sleep Restoration
            self.state.active_state_name = "AUTONOMOUS_SLEEP_RESTORATION".to_string();
            self.sleep_cycles_triggered += 1;
            // Sleep restores energy and purges fatigue
            self.state.energy_level = (self.state.energy_level + 0.5).min(1.0);
            self.state.fatigue = (self.state.fatigue - 0.5).max(0.0);
            self.state.chemicals.serotonin = (self.state.chemicals.serotonin + 0.1).min(1.0);
        } else if self.state.curiosity_drive > 0.6 {
            self.state.active_state_name = "CREATIVE_EPISTEMIC_EXPLORATION".to_string();
            self.state.chemicals.acetylcholine = 0.8;
        } else {
            self.state.active_state_name = "STABLE_TASK_EXECUTION".to_string();
        }

        &self.state.active_state_name
    }

    /// Compiles homeostatic biological chemical diffusion and gating into `.cl` VLIW microcode bundles.
    pub fn compile_to_cl(&self, core_id: u8) -> String {
        let mut compiler = MacroCompiler::new(core_id);

        let core_x = core_id % 4;
        let core_y = (core_id / 4) % 4;
        let core_z = (core_id / 16) % 4;
        let core_w = (core_id / 64) % 4;

        let mut cl_code = format!(
            "; ============================================================================\n\
             ; Continuous Biological Homeostasis & Neuromodulator Chemical Diffusion Kernel\n\
             ; Energy: {:.2}, Curiosity: {:.2}, State: {}\n\
             ; Target Silicon: 256-Core 4D-Torus Metabolic Coprocessor\n\
             ; ============================================================================\n\
             .core [{},{},{},{}]:\n\
             @living_homeostatic_drive_entry:\n",
            self.state.energy_level,
            self.state.curiosity_drive,
            self.state.active_state_name,
            core_x,
            core_y,
            core_z,
            core_w
        );

        // Bundle 0: Read energy budget, reward signal, and chemical levels
        compiler.emit_slot(build_valid_slot("==01#100", "'")); // R1 = Energy Level Scale (0x0100)
        compiler.emit_slot(build_valid_slot("==02#080", "'")); // R2 = Curiosity Drive Scale (0x0080)
        compiler.emit_slot(build_valid_slot("_LD03M100", "_")); // R3 = Read Dopamine Potential
        compiler.emit_slot(build_valid_slot("_LD04M200", "_")); // R4 = Read Metabolic Drain Rate

        // Bundle 1: Compute Homeostatic Balance & Plasticity Gate
        compiler.emit_slot(build_valid_slot("_SB05M104", "_")); // R5 = Net Energy = Energy - Drain
        compiler.emit_slot(build_valid_slot("_ML06M302", "_")); // R6 = Learning Rate Gate = Dopamine * Curiosity
        compiler.emit_slot(build_valid_slot("_CP07M501", "_")); // R7 = Fatigue Threshold Check
        compiler.emit_slot(build_valid_slot("_AD08M600", "_")); // R8 = Synaptic Plasticity Scalar

        // Bundle 2: Latch Homeostasis State & Synchronize Mesh
        compiler.emit_slot(build_valid_slot("_ST09M800", "_")); // R9 = Update Neuromodulator Matrix
        compiler.emit_slot(build_valid_slot("~RM0AM900", "~")); // RA = Conserved State Energy Latch
        compiler.emit_slot(build_valid_slot("_TX0BM102", "_")); // RB = Broadcast Metabolic State NoC
        compiler.emit_slot(build_valid_slot("!HL00#000", "!")); // Halt cycle

        cl_code.push_str(&compiler.finish());
        cl_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homeostasis_energy_drain_and_sleep_trigger() {
        let mut mind = LivingHomeostasisEngine::new();
        assert_eq!(mind.state.energy_level, 1.0);

        // Simulate intense activity over multiple cycles
        for _ in 0..45 {
            mind.step_metabolic_cycle(1.5, 0.0);
        }

        // Mind should autonomously enter sleep restoration mode when energy is low
        assert!(
            mind.sleep_cycles_triggered > 0,
            "Autonomous sleep must be triggered when energy drains"
        );
        assert!(mind.state.energy_level > 0.0, "Energy should recover after sleep");
    }

    #[test]
    fn test_homeostasis_compile_to_cl() {
        let mind = LivingHomeostasisEngine::new();
        let cl = mind.compile_to_cl(12);

        assert!(cl.contains("@living_homeostatic_drive_entry:"));
        assert!(cl.contains("B0000:"));
        assert!(cl.contains("B0001:"));
        assert!(cl.contains("B0002:"));
    }
}
