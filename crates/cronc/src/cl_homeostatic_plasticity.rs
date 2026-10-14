//! cl_homeostatic_plasticity.rs
//! Native Rust implementation of Homeostatic Synaptic Regulation & Lifelong Plasticity for DPSTA.
//! Replaces python/sovereign_toolkit/continuous_plasticity.py with 100% zero-Python native Rust.
//!
//! Architectural Specifications:
//! - 94 x 94 Synaptic transition weight matrix
//! - Row-wise L1 Homeostatic Budget control (sum_j W[i, j] <= BUDGET_MAX = 1024)
//! - Proportional dynamic soft-decay: W - (W >> 3) (12.5% attenuation on saturation threshold)
//! - Rare-Token Protection: Ensures low-frequency transitions are preserved while dampening noise
//! - 255-Saturation hard ceiling avoidance (prevents catastrophic weight explosion)
//! - Shannon entropy and discrimination power metrics per discrete state

use crate::cl_matrix94::NUM_STATES;

pub const BUDGET_MAX: i32 = 1024;
pub const SATURATION_LIMIT: i32 = 255;
pub const DEFAULT_LEARNING_DELTA: i32 = 4;

#[derive(Debug, Clone)]
pub struct HomeostaticDPSTACore {
    pub budget: i32,
    pub weights: [[i32; NUM_STATES]; NUM_STATES],
    pub state_activity: [u64; NUM_STATES],
    pub total_updates: u64,
    pub saturation_incidents: u64,
}

impl Default for HomeostaticDPSTACore {
    fn default() -> Self {
        Self::new(BUDGET_MAX)
    }
}

impl HomeostaticDPSTACore {
    pub fn new(budget: i32) -> Self {
        Self {
            budget,
            weights: [[0; NUM_STATES]; NUM_STATES],
            state_activity: [0; NUM_STATES],
            total_updates: 0,
            saturation_incidents: 0,
        }
    }

    /// Learn state transition (src -> dst) under strict Homeostatic Synaptic Regulation
    pub fn learn_transition(&mut self, src: usize, dst: usize, delta: i32) {
        if src >= NUM_STATES || dst >= NUM_STATES {
            return;
        }

        self.total_updates += 1;
        self.state_activity[src] += 1;

        // 1. Calculate current row L1 weight budget
        let row_sum: i32 = self.weights[src].iter().sum();

        if row_sum + delta > self.budget {
            // Homeostatic Trigger: Soft-Decay attenuation on the entire row
            // W[src, j] = W[src, j] - (W[src, j] >> 3)  (12.5% reduction)
            for j in 0..NUM_STATES {
                let current_w = self.weights[src][j];
                if current_w > 0 {
                    let mut decay = current_w >> 3;
                    // Rare-Token Protection: single-count vital transitions retain at least weight 1
                    if decay == 0 && current_w > 1 {
                        decay = 1;
                    }
                    self.weights[src][j] = (current_w - decay).max(0);
                }
            }
        }

        // 2. Increment target weight with saturation bounding
        let new_val = self.weights[src][dst] + delta;
        if new_val >= SATURATION_LIMIT {
            self.saturation_incidents += 1;
            self.weights[src][dst] = SATURATION_LIMIT;
        } else {
            self.weights[src][dst] = new_val;
        }
    }

    /// Train a sequence of character states
    pub fn train_sequence(&mut self, text: &str) {
        let states: Vec<usize> = text
            .chars()
            .filter(|c| !c.is_control())
            .map(|c| {
                let b = c as u32;
                if (33..=126).contains(&b) {
                    (b - 33) as usize
                } else {
                    (b % (NUM_STATES as u32)) as usize
                }
            })
            .collect();

        for i in 0..states.len().saturating_sub(1) {
            self.learn_transition(states[i], states[i + 1], DEFAULT_LEARNING_DELTA);
        }
    }

    /// Predict most probable next state from current state
    pub fn predict_next(&self, src: usize) -> usize {
        if src >= NUM_STATES {
            return 0;
        }
        let row = &self.weights[src];
        let mut max_val = -1;
        let mut best_idx = 0;
        for (idx, &w) in row.iter().enumerate() {
            if w > max_val {
                max_val = w;
                best_idx = idx;
            }
        }
        best_idx
    }

    /// Measure Shannon Entropy (discrimination power) of state transitions from `src`
    pub fn get_row_entropy(&self, src: usize) -> f64 {
        if src >= NUM_STATES {
            return 0.0;
        }
        let row = &self.weights[src];
        let total: f64 = row.iter().map(|&w| w as f64).sum();
        if total <= 0.0 {
            return 0.0;
        }

        let mut entropy = 0.0;
        for &w in row {
            if w > 0 {
                let p = (w as f64) / total;
                entropy -= p * p.log2();
            }
        }
        entropy
    }

    /// Calculate percentage of weights currently saturated at SATURATION_LIMIT (255)
    pub fn get_saturation_ratio(&self) -> f64 {
        let mut saturated = 0;
        let total = NUM_STATES * NUM_STATES;
        for row in &self.weights {
            for &w in row {
                if w >= SATURATION_LIMIT {
                    saturated += 1;
                }
            }
        }
        ((saturated as f64) / (total as f64)) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homeostatic_budget_and_soft_decay() {
        let mut core = HomeostaticDPSTACore::new(100);

        // Learn transitions from state 10 repeatedly
        for dst in 0..30 {
            core.learn_transition(10, dst, 10);
        }

        let row_sum: i32 = core.weights[10].iter().sum();
        // Row sum must never wildly exceed budget + single delta
        assert!(row_sum <= 100 + 10, "Row sum {} exceeded budget threshold", row_sum);
        assert_eq!(core.get_saturation_ratio(), 0.0, "Zero saturation under homeostatic regulation");
    }

    #[test]
    fn test_sequence_training_and_prediction() {
        let mut core = HomeostaticDPSTACore::new(BUDGET_MAX);
        // Branching transitions from 'C': sometimes to 'R', sometimes to 'A'
        core.train_sequence("CRON_CAN_CRON_CAN!");

        let c_state = (b'C' - 33) as usize;
        let r_state = (b'R' - 33) as usize;

        // 'C' should predict next state
        let next = core.predict_next(c_state);
        assert!(next == r_state || next == (b'A' - 33) as usize);

        let entropy = core.get_row_entropy(c_state);
        assert!(entropy > 0.0, "Branching transitions from C must yield non-zero entropy");
    }
}
