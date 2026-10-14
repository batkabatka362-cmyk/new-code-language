//! cl_topological_attention.rs
//! Native Rust implementation of 4D Topological Phase-Resonance Attention (TBT).
//! Replaces python/sovereign_toolkit/topological_attention.py with 100% zero-Python native Rust.
//!
//! Mathematical Specifications:
//! - Phase projections: theta_x, theta_z, R for each token and head
//! - Angular disparity: delta_theta_x = theta_x[i] - theta_x[j], delta_theta_z = theta_z[i] - theta_z[j]
//! - Depth disparity: delta_R = |R[i] - R[j]|
//! - Resonance coupling: A_ij = cos(delta_theta_x) * cos(delta_theta_z) * exp(-delta_R / T_depth)
//! - Constructive interference: relu(A_ij) + eps
//! - L1 Homeostatic Energy Conservation: sum_j A_ij == 1.0 (Exact Volume Invariant, Zero Softmax)

pub const DEFAULT_T_DEPTH: f64 = 5.0;

#[derive(Debug, Clone)]
pub struct PhaseResonanceConfig {
    pub d_model: usize,
    pub num_heads: usize,
    pub head_dim: usize,
    pub t_depth: f64,
}

impl Default for PhaseResonanceConfig {
    fn default() -> Self {
        Self {
            d_model: 64,
            num_heads: 4,
            head_dim: 16,
            t_depth: DEFAULT_T_DEPTH,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenPhase4D {
    pub theta_x: f64,
    pub theta_z: f64,
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct PhaseResonanceAttention {
    pub config: PhaseResonanceConfig,
    pub phase_weights: Vec<f64>, // (d_model x (num_heads * 3))
    pub v_weights: Vec<f64>,     // (d_model x d_model)
    pub out_weights: Vec<f64>,   // (d_model x d_model)
}

impl PhaseResonanceAttention {
    pub fn new(config: PhaseResonanceConfig) -> Self {
        let d = config.d_model;
        let h3 = config.num_heads * 3;
        let phase_weights = Self::init_weights(d * h3);
        let v_weights = Self::init_weights(d * d);
        let out_weights = Self::init_weights(d * d);

        Self {
            config,
            phase_weights,
            v_weights,
            out_weights,
        }
    }

    fn init_weights(size: usize) -> Vec<f64> {
        let mut w = Vec::with_capacity(size);
        for i in 0..size {
            // Deterministic pseudorandom initialization
            let val = (((i * 2654435761) % 1000) as f64 / 1000.0 - 0.5) * 0.1;
            w.push(val);
        }
        w
    }

    /// Extract 4D Phase coordinates (theta_x, theta_z, R) for a token embedding
    pub fn project_phase(&self, token_vec: &[f64], head_idx: usize) -> TokenPhase4D {
        let d = self.config.d_model;
        let base_idx = head_idx * 3;

        let mut tx = 0.0;
        let mut tz = 0.0;
        let mut r_raw = 0.0;

        for (i, &val) in token_vec.iter().enumerate().take(d) {
            let row = i * (self.config.num_heads * 3);
            tx += val * self.phase_weights[row + base_idx];
            tz += val * self.phase_weights[row + base_idx + 1];
            r_raw += val * self.phase_weights[row + base_idx + 2];
        }

        // Softplus for radius R >= 0
        let radius = (1.0 + r_raw.exp()).ln().max(0.0) * 10.0;

        TokenPhase4D {
            theta_x: tx,
            theta_z: tz,
            radius,
        }
    }

    /// Compute 4D Topological Phase-Resonance Attention matrix for N tokens
    /// Returns Attention Matrix [N x N] and Context Output [N x d_model]
    pub fn forward(&self, tokens: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let n = tokens.len();
        let d = self.config.d_model;
        let num_heads = self.config.num_heads;
        let head_dim = self.config.head_dim;

        // 1. Compute V projections: [N x num_heads x head_dim]
        let mut v_heads = vec![vec![vec![0.0; head_dim]; num_heads]; n];
        for (i, tok) in tokens.iter().enumerate() {
            for h in 0..num_heads {
                for hd in 0..head_dim {
                    let mut sum = 0.0;
                    for (k, &val) in tok.iter().enumerate().take(d) {
                        let w = self.v_weights[k * d + (h * head_dim + hd)];
                        sum += val * w;
                    }
                    v_heads[i][h][hd] = sum;
                }
            }
        }

        // 2. Compute 4D phase coordinates per token and head
        let mut phases = Vec::with_capacity(n);
        for tok in tokens {
            let mut head_phases = Vec::with_capacity(num_heads);
            for h in 0..num_heads {
                head_phases.push(self.project_phase(tok, h));
            }
            phases.push(head_phases);
        }

        // 3. Pairwise phase resonance attention matrix per head
        let mut head_outputs = vec![vec![vec![0.0; head_dim]; num_heads]; n];
        let mut avg_attn_matrix = vec![vec![0.0; n]; n];

        for h in 0..num_heads {
            let mut attn_matrix = vec![vec![0.0; n]; n];

            for i in 0..n {
                let p_i = &phases[i][h];
                let mut row_sum = 0.0;

                for j in 0..n {
                    let p_j = &phases[j][h];
                    let delta_tx = p_i.theta_x - p_j.theta_x;
                    let delta_tz = p_i.theta_z - p_j.theta_z;
                    let delta_r = (p_i.radius - p_j.radius).abs();

                    // Master formula: A_ij = cos(delta_theta_x) * cos(delta_theta_z) * exp(-delta_R / T_depth)
                    let phase_coupling = delta_tx.cos() * delta_tz.cos();
                    let depth_decay = (-delta_r / self.config.t_depth).exp();
                    let raw_coupling = phase_coupling * depth_decay;

                    // Constructive interference clipping (>= 0)
                    let a_pos = raw_coupling.max(0.0) + 1e-6;
                    attn_matrix[i][j] = a_pos;
                    row_sum += a_pos;
                }

                // L1 Homeostatic Energy Conservation: row_sum == 1.0000 (No Softmax needed)
                let inv_sum = 1.0 / row_sum.max(1e-12);
                for j in 0..n {
                    attn_matrix[i][j] *= inv_sum;
                    avg_attn_matrix[i][j] += attn_matrix[i][j] / (num_heads as f64);
                }

                // Value aggregation for token i, head h
                for hd in 0..head_dim {
                    let mut weighted_v = 0.0;
                    for j in 0..n {
                        weighted_v += attn_matrix[i][j] * v_heads[j][h][hd];
                    }
                    head_outputs[i][h][hd] = weighted_v;
                }
            }
        }

        // 4. Output projection out_proj: [N x d_model]
        let mut final_out = vec![vec![0.0; d]; n];
        for i in 0..n {
            let mut flat_heads = Vec::with_capacity(d);
            for h in 0..num_heads {
                flat_heads.extend_from_slice(&head_outputs[i][h]);
            }
            for col in 0..d {
                let mut sum = 0.0;
                for row in 0..d {
                    sum += flat_heads[row] * self.out_weights[row * d + col];
                }
                final_out[i][col] = sum;
            }
        }

        (avg_attn_matrix, final_out)
    }

    /// Fast 0-FLOP integer fixed-point resonance metric for pure silicon / ASIC
    pub fn fast_integer_resonance(theta_x1: u8, theta_z1: u8, r1: u8, theta_x2: u8, theta_z2: u8, r2: u8) -> i32 {
        let dtx = (theta_x1 as i32 - theta_x2 as i32).abs();
        let dtz = (theta_z1 as i32 - theta_z2 as i32).abs();
        let dr = (r1 as i32 - r2 as i32).abs();

        // 8-bit angular cosine approximation in integer [-128..127]
        let cos_x = 127 - (dtx * dtx) / 32;
        let cos_z = 127 - (dtz * dtz) / 32;
        let depth_factor = (100 - dr.min(100)) * 128 / 100;

        let coupling = (cos_x * cos_z) >> 7;
        let resonance = (coupling * depth_factor) >> 7;
        resonance.max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_resonance_attention_energy_conservation() {
        let config = PhaseResonanceConfig {
            d_model: 16,
            num_heads: 2,
            head_dim: 8,
            t_depth: 5.0,
        };
        let attn = PhaseResonanceAttention::new(config);

        let n = 4;
        let tokens: Vec<Vec<f64>> = (0..n)
            .map(|i| (0..16).map(|j| ((i * 16 + j) as f64 * 0.1).sin()).collect())
            .collect();

        let (attn_matrix, output) = attn.forward(&tokens);

        assert_eq!(attn_matrix.len(), n);
        assert_eq!(output.len(), n);

        // Every row in the attention matrix must sum to 1.0000 (L1 Volume invariant)
        for row in &attn_matrix {
            let row_sum: f64 = row.iter().sum();
            assert!((row_sum - 1.0).abs() < 1e-4, "Row sum was {}, expected 1.0", row_sum);
        }
    }

    #[test]
    fn test_fast_integer_resonance() {
        let r_ident = PhaseResonanceAttention::fast_integer_resonance(50, 50, 10, 50, 50, 10);
        assert!(r_ident > 100, "Identical phases should have maximum resonance");

        let r_diff = PhaseResonanceAttention::fast_integer_resonance(10, 10, 10, 90, 90, 80);
        assert!(r_diff < r_ident, "Different phases must have lower resonance");
    }
}
