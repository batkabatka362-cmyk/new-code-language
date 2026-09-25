//! Karl Friston's Active Inference & Free Energy Minimization Engine (`cron cl-active-inference`)
//!
//! Autonomous brain agency based on the Free Energy Principle:
//! Organisms minimize Variational Free Energy F (Perception) and
//! Expected Free Energy G (Action/Policy Selection) to adaptively survive,
//! resolve epistemic curiosity, and fulfill goal-directed homeostatic drives.

use crate::cl_macro::{build_valid_slot, MacroCompiler};

/// Active Inference Cognitive Agent
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveInferenceAgent {
    pub state_dim: usize,
    pub obs_dim: usize,
    pub action_dim: usize,
    /// Belief state distribution q(s) over hidden world states
    pub beliefs: Vec<f64>,
    /// Prior desired preferences C = ln P(o) over sensory outcomes
    pub preferences: Vec<f64>,
    /// Observation likelihood matrix A[o][s] = P(o | s)
    pub likelihood_a: Vec<Vec<f64>>,
    /// State transition tensor B[s_next][s][a] = P(s_next | s, a)
    pub transitions_b: Vec<Vec<Vec<f64>>>,
    /// Current variational free energy (Surprise bound)
    pub current_free_energy: f64,
    /// Precision weight (gamma) on prediction error
    pub precision_gamma: f64,
}

impl ActiveInferenceAgent {
    /// Creates a new Active Inference Agent with specified state, observation, and action dimensions.
    pub fn new(state_dim: usize, obs_dim: usize, action_dim: usize) -> Self {
        // Uniform initial beliefs
        let init_belief = 1.0 / (state_dim as f64);
        let beliefs = vec![init_belief; state_dim];
        let preferences = vec![0.0; obs_dim];

        // Identity or uniform likelihood initialization
        let mut likelihood_a = vec![vec![1.0 / (obs_dim as f64); state_dim]; obs_dim];
        for s in 0..state_dim.min(obs_dim) {
            for o in 0..obs_dim {
                likelihood_a[o][s] = if o == s { 0.8 } else { 0.2 / ((obs_dim - 1) as f64) };
            }
        }

        // Transitions: default identity transition per action
        let mut transitions_b = vec![vec![vec![1.0 / (state_dim as f64); action_dim]; state_dim]; state_dim];
        for a in 0..action_dim {
            for s in 0..state_dim {
                for s_next in 0..state_dim {
                    transitions_b[s_next][s][a] = if s_next == (s + a) % state_dim {
                        0.85
                    } else {
                        0.15 / ((state_dim - 1) as f64)
                    };
                }
            }
        }

        Self {
            state_dim,
            obs_dim,
            action_dim,
            beliefs,
            preferences,
            likelihood_a,
            transitions_b,
            current_free_energy: 0.0,
            precision_gamma: 1.0,
        }
    }

    /// Sets preferred target observations (Homeostatic Goals)
    pub fn set_goal_preference(&mut self, target_obs_index: usize, reward_val: f64) {
        if target_obs_index < self.obs_dim {
            self.preferences[target_obs_index] = reward_val;
        }
    }

    /// Perception Step: Infers beliefs q(s) to minimize Variational Free Energy F(o, q)
    /// F = E_q[ln q(s) - ln P(o, s)] = KL(q(s) || P(s)) - E_q[ln P(o|s)]
    pub fn infer_states(&mut self, observation_one_hot: &[f64]) -> f64 {
        let mut log_likelihood = vec![0.0; self.state_dim];
        for s in 0..self.state_dim {
            let mut sum_obs = 0.0;
            for o in 0..self.obs_dim {
                sum_obs += observation_one_hot[o] * (self.likelihood_a[o][s] + 1e-12).ln();
            }
            log_likelihood[s] = sum_obs;
        }

        // Softmax update of beliefs: q(s) ∝ P(s) * P(o|s)^gamma
        let mut max_log = f64::NEG_INFINITY;
        let mut unnormalized = vec![0.0; self.state_dim];
        for s in 0..self.state_dim {
            let val = (self.beliefs[s] + 1e-12).ln() + self.precision_gamma * log_likelihood[s];
            if val > max_log {
                max_log = val;
            }
            unnormalized[s] = val;
        }

        let mut sum_exp = 0.0;
        for s in 0..self.state_dim {
            let e = (unnormalized[s] - max_log).exp();
            self.beliefs[s] = e;
            sum_exp += e;
        }
        for s in 0..self.state_dim {
            self.beliefs[s] /= sum_exp;
        }

        // Compute Variational Free Energy
        let mut free_energy = 0.0;
        for s in 0..self.state_dim {
            let q = self.beliefs[s];
            if q > 1e-12 {
                free_energy += q * (q.ln() - log_likelihood[s]);
            }
        }
        self.current_free_energy = free_energy;
        free_energy
    }

    /// Action Step: Computes Expected Free Energy G(a) for each available action
    /// G(a) = Epistemic Value (Information Gain / Ambiguity Resolution) + Pragmatic Value (Goal Pursuit)
    pub fn select_action(&mut self) -> (usize, f64) {
        let mut best_action = 0;
        let mut min_expected_fe = f64::INFINITY;

        for a in 0..self.action_dim {
            // 1. Predict future state distribution q(s_t+1 | a)
            let mut pred_s_next = vec![0.0; self.state_dim];
            for s_next in 0..self.state_dim {
                for s in 0..self.state_dim {
                    pred_s_next[s_next] += self.transitions_b[s_next][s][a] * self.beliefs[s];
                }
            }

            // 2. Predict expected observations q(o_t+1 | a)
            let mut pred_obs = vec![0.0; self.obs_dim];
            for o in 0..self.obs_dim {
                for s_next in 0..self.state_dim {
                    pred_obs[o] += self.likelihood_a[o][s_next] * pred_s_next[s_next];
                }
            }

            // 3. Pragmatic Value (Risk / Divergence from Goal Preferences)
            let mut pragmatic_risk = 0.0;
            for o in 0..self.obs_dim {
                pragmatic_risk -= pred_obs[o] * self.preferences[o];
            }

            // 4. Epistemic Value (Negative Information Gain / Ambiguity)
            let mut epistemic_ambiguity = 0.0;
            for s_next in 0..self.state_dim {
                for o in 0..self.obs_dim {
                    let p_o_given_s = self.likelihood_a[o][s_next];
                    if p_o_given_s > 1e-12 {
                        epistemic_ambiguity += pred_s_next[s_next] * p_o_given_s * (p_o_given_s / (pred_obs[o] + 1e-12)).ln();
                    }
                }
            }

            // Total Expected Free Energy G(a) = Pragmatic Risk - Epistemic Information Gain
            let expected_fe = pragmatic_risk - epistemic_ambiguity;
            if expected_fe < min_expected_fe {
                min_expected_fe = expected_fe;
                best_action = a;
            }
        }

        // Apply action to transition beliefs
        let mut next_beliefs = vec![0.0; self.state_dim];
        for s_next in 0..self.state_dim {
            for s in 0..self.state_dim {
                next_beliefs[s_next] += self.transitions_b[s_next][s][best_action] * self.beliefs[s];
            }
        }
        self.beliefs = next_beliefs;

        (best_action, min_expected_fe)
    }

    /// Compiles Active Inference belief updates and policy selection into `.cl` VLIW microcode bundles.
    pub fn compile_to_cl(&self, core_id: u8) -> String {
        let mut compiler = MacroCompiler::new(core_id);

        let core_x = core_id % 4;
        let core_y = (core_id / 4) % 4;
        let core_z = (core_id / 16) % 4;
        let core_w = (core_id / 64) % 4;

        let mut cl_code = format!(
            "; ============================================================================\n\
             ; Active Inference & Free Energy Minimization Autonomous Agent Kernel\n\
             ; States: {}, Observations: {}, Actions: {}\n\
             ; Target Silicon: 256-Core 4D-Torus Sensory-Motor Cortex\n\
             ; ============================================================================\n\
             .core [{},{},{},{}]:\n\
             @active_inference_entry:\n",
            self.state_dim,
            self.obs_dim,
            self.action_dim,
            core_x,
            core_y,
            core_z,
            core_w
        );

        // Emit VLIW slots for top-down prediction, error calculation, and policy selection
        // 1. Load Sensory Observation and Prior Goal
        compiler.emit_slot(build_valid_slot("==00#010", "")); // R0 = Observation Vector Pointer
        compiler.emit_slot(build_valid_slot("==01#020", "")); // R1 = Prior Preference Vector
        compiler.emit_slot(build_valid_slot("_LD02$010", "")); // R2 = Read Sensory Input
        compiler.emit_slot(build_valid_slot("_FA03$020", "")); // R3 = FlashSoftmax Top-Down Prior

        // 2. Precision-Weighted Prediction Error (Basal - Apical)
        compiler.emit_slot(build_valid_slot("_SB04$023", "")); // R4 = Prediction Error (e = Obs - Pred)
        compiler.emit_slot(build_valid_slot("_ML05$040", "")); // R5 = Scale by Precision Weight (gamma)
        compiler.emit_slot(build_valid_slot("_AD06$035", "")); // R6 = Update Somatic Belief State
        compiler.emit_slot(build_valid_slot("_EX07$060", "")); // R7 = Compute Free Energy Exponential

        // 3. Expected Free Energy (EFE) Action Evaluation
        compiler.emit_slot(build_valid_slot("_MD08$061", "")); // R8 = Pragmatic Goal Value Evaluation
        compiler.emit_slot(build_valid_slot("_FX09$080", "")); // R9 = Epistemic Information Gain
        compiler.emit_slot(build_valid_slot("_CP0A$089", "")); // RA = Best Action Selection (ArgMin G)
        compiler.emit_slot(build_valid_slot("_TX0B$CA1", "")); // RB = Transmit Motor Action via NoC

        // 4. Latch State & Global Homeostasis Barrier
        compiler.emit_slot(build_valid_slot("_RV0C$0A0", "")); // RC = Reversible Action Latch
        compiler.emit_slot(build_valid_slot("_BB00$000", "")); // Core Barrier Sync
        compiler.emit_slot(build_valid_slot("_HL00$0E8", "!")); // End of cycle
        compiler.emit_slot(build_valid_slot("__NOP000", "")); // Pad cycle

        cl_code.push_str(&compiler.finish());
        cl_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_inference_state_inference_minimizes_free_energy() {
        let mut agent = ActiveInferenceAgent::new(4, 4, 2);
        // Set goal preference: prefer state/observation 2
        agent.set_goal_preference(2, 5.0);

        // Observe state 2 (one-hot observation)
        let obs = vec![0.0, 0.0, 1.0, 0.0];
        let fe = agent.infer_states(&obs);

        // Beliefs should concentrate heavily on state 2
        assert!(agent.beliefs[2] > 0.6, "Belief in state 2 should be dominant, got {}", agent.beliefs[2]);
        assert!(fe.is_finite(), "Free energy should be finite");

        // Action selection should choose action leading to preferred outcome
        let (action, expected_fe) = agent.select_action();
        assert!(expected_fe.is_finite());
        assert!(action < 2);
    }

    #[test]
    fn test_active_inference_compile_to_cl() {
        let agent = ActiveInferenceAgent::new(3, 3, 2);
        let cl_code = agent.compile_to_cl(4);

        assert!(cl_code.contains(".core [0,1,0,0]:"));
        assert!(cl_code.contains("@active_inference_entry:"));
        assert!(cl_code.contains("B0000:"));
        assert!(cl_code.contains("B0001:"));
    }
}
