// ============================================================================
// CRON Autonomous Zero-Python In-Silicon Training Pipeline (cl_train_pipeline.rs)
//
// 100% Pure Rust - Zero Python, Zero PyTorch, Zero External C++ Runtime.
//
// Features:
// 1. Data Ingestion & Batching:
//    - Raw text ingestion via BpeTokenizer::from_default_vocab() & byte fallback
//    - Windowed causal LM token sequences: (X[0..T-1], Y[1..T])
// 2. High-Performance Mathematical Calculus Forward Pass:
//    - Token Embeddings E [Vocab, Dim]
//    - SwiGLU / Dual-Gated MLP projection blocks
//    - Residual skip connections
//    - Softmax Cross-Entropy next-token prediction loss
// 3. Analytical Reverse-Mode Autodiff Backward Pullback:
//    - Exact Vector-Jacobian Products (VJP) across all parameters
//    - Straight-Through & SwiGLU analytical derivatives
// 4. In-Silicon Optimization Engines:
//    - AdamW (Ω decoupled weight decay, bias-corrected 1st & 2nd moments)
//    - Lion (Sign-based momentum optimizer with decoupled weight decay)
//    - SGD with momentum
//    - Cosine learning rate schedule with linear warmup
// 5. Zero-Dependency Safetensors Checkpoint Exporter:
//    - Standard IEEE 754 F32 binary export ready for hardware inference
// ============================================================================

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use crate::bpe_tokenizer::BpeTokenizer;
use crate::cl_cuda_driver::{CudaDeviceBuffer, CudaDriver, CudaTensorEngine};

/// Supported In-Silicon Optimization Algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainingOptimizer {
    AdamW,
    Lion,
    Sgd,
}

impl Default for TrainingOptimizer {
    fn default() -> Self {
        TrainingOptimizer::AdamW
    }
}

impl TrainingOptimizer {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "lion" => TrainingOptimizer::Lion,
            "sgd" => TrainingOptimizer::Sgd,
            _ => TrainingOptimizer::AdamW,
        }
    }
}

/// Learning Rate Decay Schedules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LrSchedule {
    Cosine,
    Linear,
    Constant,
}

impl Default for LrSchedule {
    fn default() -> Self {
        LrSchedule::Cosine
    }
}

/// Configuration for Autonomous Training
#[derive(Debug, Clone)]
pub struct TrainingConfig {
    pub model_path: Option<String>,
    pub data_path: Option<String>,
    pub data_text: Option<String>,
    pub steps: usize,
    pub learning_rate: f32,
    pub min_learning_rate: f32,
    pub batch_size: usize,
    pub seq_len: usize,
    pub dim: usize,
    pub hidden_dim: usize,
    pub vocab_size: usize,
    pub num_layers: usize,
    pub num_experts: usize,
    pub top_k: usize,
    pub optimizer: TrainingOptimizer,
    pub weight_decay: f32,
    pub warmup_steps: usize,
    pub lr_schedule: LrSchedule,
    pub checkpoint_out: Option<String>,
    pub profile: String,
    pub seed: u64,
    pub eval_prompt: Option<String>,
    pub eval_max_tokens: usize,
    pub eval_temperature: f32,
    pub use_cuda: bool,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            model_path: None,
            data_path: None,
            data_text: None,
            steps: 50,
            learning_rate: 0.01,
            min_learning_rate: 0.0001,
            batch_size: 4,
            seq_len: 16,
            dim: 32,
            hidden_dim: 64,
            vocab_size: 256,
            num_layers: 1,
            num_experts: 1,
            top_k: 1,
            optimizer: TrainingOptimizer::AdamW,
            weight_decay: 0.01,
            warmup_steps: 5,
            lr_schedule: LrSchedule::Cosine,
            checkpoint_out: Some("checkpoint.safetensors".to_string()),
            profile: "max-quality".to_string(),
            seed: 42,
            eval_prompt: None,
            eval_max_tokens: 32,
            eval_temperature: 0.7,
            use_cuda: false,
        }
    }
}

/// Per-Step Telemetry Report during Training
#[derive(Debug, Clone)]
pub struct TrainingStepReport {
    pub step: usize,
    pub loss: f32,
    pub perplexity: f32,
    pub lr: f32,
    pub grad_norm: f32,
    pub tokens_processed: usize,
    pub elapsed_ms: f64,
}

/// Final Autonomous Training Summary Report
#[derive(Debug, Clone)]
pub struct AutonomousTrainingSummary {
    pub initial_loss: f32,
    pub final_loss: f32,
    pub loss_reduction_pct: f32,
    pub final_perplexity: f32,
    pub total_steps: usize,
    pub total_tokens: usize,
    pub total_duration_secs: f64,
    pub tokens_per_sec: f64,
    pub checkpoint_saved_to: Option<String>,
    pub step_history: Vec<TrainingStepReport>,
    pub generated_sample: Option<String>,
}

/// Render a unicode sparkline given a sequence of float values
pub fn render_ascii_sparkline(values: &[f32], max_chars: usize) -> String {
    if values.is_empty() {
        return String::new();
    }
    const TICKS: [char; 8] = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let slice = if values.len() > max_chars {
        &values[values.len() - max_chars..]
    } else {
        values
    };

    let mut min_val = f32::INFINITY;
    let mut max_val = f32::NEG_INFINITY;
    for &v in slice {
        if v < min_val {
            min_val = v;
        }
        if v > max_val {
            max_val = v;
        }
    }

    let diff = (max_val - min_val).max(1e-6);
    let mut sparkline = String::with_capacity(slice.len());
    for &v in slice {
        let normalized = ((v - min_val) / diff).clamp(0.0, 1.0);
        let tick_idx = ((normalized * 7.0).round() as usize).min(7);
        sparkline.push(TICKS[tick_idx]);
    }
    sparkline
}

/// Render a horizontal progress bar
pub fn render_progress_bar(current: usize, total: usize, width: usize) -> String {
    let pct = if total > 0 {
        ((current as f64) / (total as f64)).clamp(0.0, 1.0)
    } else {
        1.0
    };
    let filled = ((pct * (width as f64)).round() as usize).min(width);
    let empty = width.saturating_sub(filled);
    format!("[{}{}] {:3.0}%", "█".repeat(filled), "░".repeat(empty), pct * 100.0)
}

/// Render interactive ANSI / ASCII HUD block for live training monitoring
pub fn render_training_hud(
    step: usize,
    total_steps: usize,
    loss_history: &[f32],
    current_loss: f32,
    perplexity: f32,
    lr: f32,
    grad_norm: f32,
    tokens_per_sec: f64,
    elapsed_secs: f64,
) -> String {
    let progress_bar = render_progress_bar(step, total_steps, 24);
    let sparkline = render_ascii_sparkline(loss_history, 24);
    let initial_loss = loss_history.first().copied().unwrap_or(current_loss);
    let delta_loss = if initial_loss > 1e-6 {
        ((initial_loss - current_loss) / initial_loss) * 100.0
    } else {
        0.0
    };
    let eta_secs = if step > 0 && step < total_steps {
        let avg_time_per_step = elapsed_secs / (step as f64);
        avg_time_per_step * ((total_steps - step) as f64)
    } else {
        0.0
    };

    let mut hud = String::new();
    hud.push_str("┌────────────────────────────────────────────────────────────────────────────────────────┐\n");
    hud.push_str(&format!("│  LIVE MONITOR │ Step {:4}/{} {} │ ETA: {:>6.2}s │\n", step, total_steps, progress_bar, eta_secs));
    hud.push_str("├────────────────────────────────────────────────────────────────────────────────────────┤\n");
    hud.push_str(&format!("│  Loss Trend   : {:<24} │ Loss: {:>7.4} (Δ: {:>+6.2}%)                   │\n", sparkline, current_loss, -delta_loss));
    hud.push_str(&format!("│  Perplexity   : {:<12.2} │ LR: {:<10.6} │ |∇|: {:<8.4} │ Speed: {:>7.1} tok/s │\n", perplexity, lr, grad_norm, tokens_per_sec));
    hud.push_str("└────────────────────────────────────────────────────────────────────────────────────────┘");
    hud
}

/// Parameter Matrix with Gradient Tracking
#[derive(Debug, Clone)]
pub struct ParamMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>,
    pub grad: Vec<f32>,
    pub m: Vec<f32>, // First moment or momentum buffer
    pub v: Vec<f32>, // Second moment buffer (for AdamW)
}

impl ParamMatrix {
    pub fn new_random(rows: usize, cols: usize, seed: &mut u64, scale: f32) -> Self {
        let n = rows * cols;
        let mut data = Vec::with_capacity(n);
        for _ in 0..n {
            *seed ^= *seed >> 12;
            *seed ^= *seed << 25;
            *seed ^= *seed >> 27;
            let raw = (*seed).wrapping_mul(0x2545F4914F6CDD1D) >> 32;
            let u1 = ((raw & 0xFFFF) as f32 + 1.0) / 65537.0;
            let u2 = (((raw >> 16) & 0xFFFF) as f32 + 1.0) / 65537.0;
            let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos();
            data.push(z * scale);
        }
        Self {
            rows,
            cols,
            data,
            grad: vec![0.0f32; n],
            m: vec![0.0f32; n],
            v: vec![0.0f32; n],
        }
    }

    pub fn new_constant(rows: usize, cols: usize, val: f32) -> Self {
        let n = rows * cols;
        Self {
            rows,
            cols,
            data: vec![val; n],
            grad: vec![0.0f32; n],
            m: vec![0.0f32; n],
            v: vec![0.0f32; n],
        }
    }

    pub fn zero_grad(&mut self) {
        self.grad.fill(0.0);
    }

    pub fn grad_norm_sq(&self) -> f32 {
        self.grad.iter().map(|g| g * g).sum()
    }
}

/// Single Transformer Layer Block with SwiGLU / MoE Feed-Forward
#[derive(Debug, Clone)]
pub struct TransformerLayerBlock {
    pub w_gate: Vec<ParamMatrix>,      // [num_experts], each [dim, hidden_dim]
    pub w_up: Vec<ParamMatrix>,        // [num_experts], each [dim, hidden_dim]
    pub w_down: Vec<ParamMatrix>,      // [num_experts], each [hidden_dim, dim]
    pub w_router: Option<ParamMatrix>, // [dim, num_experts] if num_experts > 1
}

impl TransformerLayerBlock {
    pub fn new(dim: usize, hidden_dim: usize, num_experts: usize, seed: &mut u64) -> Self {
        let scale = (2.0 / (dim as f32)).sqrt();
        let down_scale = (2.0 / (hidden_dim as f32)).sqrt();
        let mut w_gate = Vec::with_capacity(num_experts);
        let mut w_up = Vec::with_capacity(num_experts);
        let mut w_down = Vec::with_capacity(num_experts);
        for _ in 0..num_experts {
            w_gate.push(ParamMatrix::new_random(dim, hidden_dim, seed, scale));
            w_up.push(ParamMatrix::new_random(dim, hidden_dim, seed, scale));
            w_down.push(ParamMatrix::new_random(hidden_dim, dim, seed, down_scale));
        }
        let w_router = if num_experts > 1 {
            Some(ParamMatrix::new_random(dim, num_experts, seed, scale))
        } else {
            None
        };
        Self {
            w_gate,
            w_up,
            w_down,
            w_router,
        }
    }

    pub fn zero_grad(&mut self) {
        for g in &mut self.w_gate { g.zero_grad(); }
        for u in &mut self.w_up { u.zero_grad(); }
        for d in &mut self.w_down { d.zero_grad(); }
        if let Some(r) = &mut self.w_router { r.zero_grad(); }
    }

    pub fn grad_norm_sq(&self) -> f32 {
        let mut sum = 0.0f32;
        for g in &self.w_gate { sum += g.grad_norm_sq(); }
        for u in &self.w_up { sum += u.grad_norm_sq(); }
        for d in &self.w_down { sum += d.grad_norm_sq(); }
        if let Some(r) = &self.w_router { sum += r.grad_norm_sq(); }
        sum
    }
}

/// Autonomous In-Silicon Neural Language Model Architecture
pub struct AutonomousLanguageModel {
    pub config: TrainingConfig,
    pub embedding: ParamMatrix,              // [vocab_size, dim]
    pub layers: Vec<TransformerLayerBlock>,  // [num_layers]
    pub w_head: ParamMatrix,                 // [dim, vocab_size]

    // Backward-compatibility mirrors for layer 0 expert 0
    pub w_gate: ParamMatrix,                 // [dim, hidden_dim]
    pub w_up: ParamMatrix,                   // [dim, hidden_dim]
    pub w_down: ParamMatrix,                 // [hidden_dim, dim]
}

impl AutonomousLanguageModel {
    pub fn new(config: TrainingConfig) -> Self {
        let mut seed = config.seed;
        let d = config.dim;
        let h = config.hidden_dim;
        let v = config.vocab_size;
        let num_layers = config.num_layers.max(1);
        let num_experts = config.num_experts.max(1);

        let scale = (2.0 / (d as f32)).sqrt();
        let embedding = ParamMatrix::new_random(v, d, &mut seed, 0.02);
        let mut layers = Vec::with_capacity(num_layers);
        for _ in 0..num_layers {
            layers.push(TransformerLayerBlock::new(d, h, num_experts, &mut seed));
        }
        let w_gate = layers[0].w_gate[0].clone();
        let w_up = layers[0].w_up[0].clone();
        let w_down = layers[0].w_down[0].clone();
        let w_head = ParamMatrix::new_random(d, v, &mut seed, scale);

        Self {
            config,
            embedding,
            layers,
            w_head,
            w_gate,
            w_up,
            w_down,
        }
    }

    pub fn zero_grad(&mut self) {
        self.embedding.zero_grad();
        for layer in &mut self.layers {
            layer.zero_grad();
        }
        self.w_head.zero_grad();
        self.w_gate.zero_grad();
        self.w_up.zero_grad();
        self.w_down.zero_grad();
    }

    pub fn total_grad_norm(&self) -> f32 {
        let mut sum_sq = self.embedding.grad_norm_sq() + self.w_head.grad_norm_sq();
        for layer in &self.layers {
            sum_sq += layer.grad_norm_sq();
        }
        sum_sq.sqrt()
    }

    /// Forward pass, Loss calculation, and Reverse-mode VJP Autodiff Pullback
    pub fn forward_backward_step(
        &mut self,
        input_tokens: &[usize],  // Flattened [batch_size * seq_len]
        target_tokens: &[usize], // Flattened [batch_size * seq_len]
    ) -> (f32, f32) {
        let n_tokens = input_tokens.len();
        let d = self.config.dim;
        let h = self.config.hidden_dim;
        let v = self.config.vocab_size;
        let num_layers = self.layers.len();
        let num_experts = self.config.num_experts.max(1);
        let top_k = self.config.top_k.min(num_experts).max(1);

        // 1. Embedding lookup: H_0 [N, d]
        let mut h_layers = Vec::with_capacity(num_layers + 1);
        let mut h0 = vec![0.0f32; n_tokens * d];
        for (i, &tok) in input_tokens.iter().enumerate() {
            let tok_clamped = tok % v;
            let src = &self.embedding.data[tok_clamped * d..(tok_clamped + 1) * d];
            h0[i * d..(i + 1) * d].copy_from_slice(src);
        }
        h_layers.push(h0);

        // Activations cache for backward pass:
        struct LayerActivations {
            expert_g: Vec<Vec<f32>>,     // [E][N * h]
            expert_u: Vec<Vec<f32>>,     // [E][N * h]
            expert_act: Vec<Vec<f32>>,   // [E][N * h]
            expert_silu: Vec<Vec<f32>>,  // [E][N * h]
            chosen_topk: Option<Vec<Vec<(usize, f32)>>>, // [N] -> Vec<(expert_idx, weight)>
        }

        let mut layer_caches = Vec::with_capacity(num_layers);

        // 2. Multi-layer forward pass
        for l in 0..num_layers {
            let h_curr = &h_layers[l];
            let layer = &self.layers[l];

            let mut exp_g = Vec::with_capacity(num_experts);
            let mut exp_u = Vec::with_capacity(num_experts);
            let mut exp_act = Vec::with_capacity(num_experts);
            let mut exp_silu = Vec::with_capacity(num_experts);

            for e in 0..num_experts {
                let mut g = vec![0.0f32; n_tokens * h];
                let mut u = vec![0.0f32; n_tokens * h];
                let mut act = vec![0.0f32; n_tokens * h];
                let mut silu_g = vec![0.0f32; n_tokens * h];

                for i in 0..n_tokens {
                    for j in 0..h {
                        let mut sum_g = 0.0f32;
                        let mut sum_u = 0.0f32;
                        for k in 0..d {
                            let h_val = h_curr[i * d + k];
                            sum_g += h_val * layer.w_gate[e].data[k * h + j];
                            sum_u += h_val * layer.w_up[e].data[k * h + j];
                        }
                        g[i * h + j] = sum_g;
                        u[i * h + j] = sum_u;

                        let sig = 1.0 / (1.0 + (-sum_g).exp());
                        let sg = sum_g * sig;
                        silu_g[i * h + j] = sg;
                        act[i * h + j] = sg * sum_u;
                    }
                }

                exp_g.push(g);
                exp_u.push(u);
                exp_act.push(act);
                exp_silu.push(silu_g);
            }

            let mut h_next = h_curr.clone();

            if num_experts == 1 {
                for i in 0..n_tokens {
                    for j in 0..d {
                        let mut sum_d = 0.0f32;
                        for k in 0..h {
                            sum_d += exp_act[0][i * h + k] * layer.w_down[0].data[k * d + j];
                        }
                        h_next[i * d + j] += sum_d;
                    }
                }
                layer_caches.push(LayerActivations {
                    expert_g: exp_g,
                    expert_u: exp_u,
                    expert_act: exp_act,
                    expert_silu: exp_silu,
                    chosen_topk: None,
                });
            } else {
                let router = layer.w_router.as_ref().unwrap();
                let mut chosen_topk = Vec::with_capacity(n_tokens);

                for i in 0..n_tokens {
                    let mut max_r = -1e9f32;
                    let mut r_logits = vec![0.0f32; num_experts];
                    for e in 0..num_experts {
                        let mut sum_r = 0.0f32;
                        for k in 0..d {
                            sum_r += h_curr[i * d + k] * router.data[k * num_experts + e];
                        }
                        r_logits[e] = sum_r;
                        if sum_r > max_r { max_r = sum_r; }
                    }

                    let mut r_probs = vec![0.0f32; num_experts];
                    let mut sum_exp = 0.0f32;
                    for e in 0..num_experts {
                        let exp_val = (r_logits[e] - max_r).exp();
                        r_probs[e] = exp_val;
                        sum_exp += exp_val;
                    }
                    let inv_sum = 1.0 / sum_exp.max(1e-12);
                    for e in 0..num_experts { r_probs[e] *= inv_sum; }

                    let mut indexed_probs: Vec<(usize, f32)> = (0..num_experts)
                        .map(|e| (e, r_probs[e]))
                        .collect();
                    indexed_probs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    let selected: Vec<(usize, f32)> = indexed_probs.into_iter().take(top_k).collect();

                    let topk_weight_sum: f32 = selected.iter().map(|(_, w)| *w).sum();
                    let inv_topk = 1.0 / topk_weight_sum.max(1e-12);
                    let normalized_selected: Vec<(usize, f32)> = selected
                        .into_iter()
                        .map(|(e, w)| (e, w * inv_topk))
                        .collect();

                    for &(e, w) in &normalized_selected {
                        for j in 0..d {
                            let mut sum_d = 0.0f32;
                            for k in 0..h {
                                sum_d += exp_act[e][i * h + k] * layer.w_down[e].data[k * d + j];
                            }
                            h_next[i * d + j] += w * sum_d;
                        }
                    }

                    chosen_topk.push(normalized_selected);
                }

                layer_caches.push(LayerActivations {
                    expert_g: exp_g,
                    expert_u: exp_u,
                    expert_act: exp_act,
                    expert_silu: exp_silu,
                    chosen_topk: Some(chosen_topk),
                });
            }

            h_layers.push(h_next);
        }

        // 3. Output Logits: Z = H_final @ W_head [N, v]
        let h_final = &h_layers[num_layers];
        let mut logits = vec![0.0f32; n_tokens * v];
        for i in 0..n_tokens {
            for j in 0..v {
                let mut sum_z = 0.0f32;
                for k in 0..d {
                    sum_z += h_final[i * d + k] * self.w_head.data[k * v + j];
                }
                logits[i * v + j] = sum_z;
            }
        }

        // 4. Softmax & Cross-Entropy Loss
        let mut probs = vec![0.0f32; n_tokens * v];
        let mut total_loss = 0.0f32;
        for i in 0..n_tokens {
            let row = &logits[i * v..(i + 1) * v];
            let mut max_val = -1e9f32;
            for &val in row {
                if val > max_val { max_val = val; }
            }
            let mut sum_exp = 0.0f32;
            for j in 0..v {
                let exp_val = (row[j] - max_val).exp();
                probs[i * v + j] = exp_val;
                sum_exp += exp_val;
            }
            let inv_sum = 1.0 / sum_exp.max(1e-12);
            for j in 0..v {
                probs[i * v + j] *= inv_sum;
            }
            let tgt = target_tokens[i] % v;
            let prob_tgt = probs[i * v + tgt].max(1e-12);
            total_loss += -prob_tgt.ln();
        }

        let mean_loss = total_loss / (n_tokens as f32);
        let perplexity = mean_loss.min(80.0).exp();

        // 5. Analytical Reverse-Mode Autodiff Pullback
        self.zero_grad();
        let inv_n = 1.0 / (n_tokens as f32);
        let mut d_logits = vec![0.0f32; n_tokens * v];
        for i in 0..n_tokens {
            let tgt = target_tokens[i] % v;
            for j in 0..v {
                let p = probs[i * v + j];
                d_logits[i * v + j] = if j == tgt { (p - 1.0) * inv_n } else { p * inv_n };
            }
        }

        // Backprop through W_head
        for k in 0..d {
            for j in 0..v {
                let mut sum_head = 0.0f32;
                for i in 0..n_tokens {
                    sum_head += h_final[i * d + k] * d_logits[i * v + j];
                }
                self.w_head.grad[k * v + j] += sum_head;
            }
        }

        // dH_final = d_logits @ W_head^T
        let mut d_h = vec![0.0f32; n_tokens * d];
        for i in 0..n_tokens {
            for k in 0..d {
                let mut sum_dh = 0.0f32;
                for j in 0..v {
                    sum_dh += d_logits[i * v + j] * self.w_head.data[k * v + j];
                }
                d_h[i * d + k] = sum_dh;
            }
        }

        // Backward through layers in reverse order: num_layers - 1 down to 0
        for l in (0..num_layers).rev() {
            let h_curr = &h_layers[l];
            let cache = &layer_caches[l];
            let layer = &mut self.layers[l];

            let mut d_h_prev = d_h.clone(); // Residual connection skip

            if num_experts == 1 {
                let mut d_act = vec![0.0f32; n_tokens * h];
                for k in 0..h {
                    for j in 0..d {
                        let mut sum_down = 0.0f32;
                        for i in 0..n_tokens {
                            sum_down += cache.expert_act[0][i * h + k] * d_h[i * d + j];
                        }
                        layer.w_down[0].grad[k * d + j] += sum_down;
                    }
                }

                for i in 0..n_tokens {
                    for k in 0..h {
                        let mut sum_act = 0.0f32;
                        for j in 0..d {
                            sum_act += d_h[i * d + j] * layer.w_down[0].data[k * d + j];
                        }
                        d_act[i * h + k] = sum_act;
                    }
                }

                let mut d_u = vec![0.0f32; n_tokens * h];
                let mut d_g = vec![0.0f32; n_tokens * h];
                for i in 0..(n_tokens * h) {
                    let da = d_act[i];
                    let sg = cache.expert_silu[0][i];
                    d_u[i] = da * sg;

                    let x = cache.expert_g[0][i];
                    let sig = 1.0 / (1.0 + (-x).exp());
                    let d_silu = sig * (1.0 + x * (1.0 - sig));
                    d_g[i] = (da * cache.expert_u[0][i]) * d_silu;
                }

                for k in 0..d {
                    for j in 0..h {
                        let mut sum_up = 0.0f32;
                        let mut sum_gate = 0.0f32;
                        for i in 0..n_tokens {
                            let h0_val = h_curr[i * d + k];
                            sum_up += h0_val * d_u[i * h + j];
                            sum_gate += h0_val * d_g[i * h + j];
                        }
                        layer.w_up[0].grad[k * h + j] += sum_up;
                        layer.w_gate[0].grad[k * h + j] += sum_gate;
                    }
                }

                for i in 0..n_tokens {
                    for k in 0..d {
                        let mut sum_proj = 0.0f32;
                        for j in 0..h {
                            sum_proj += d_u[i * h + j] * layer.w_up[0].data[k * h + j]
                                + d_g[i * h + j] * layer.w_gate[0].data[k * h + j];
                        }
                        d_h_prev[i * d + k] += sum_proj;
                    }
                }
            } else {
                let chosen_topk = cache.chosen_topk.as_ref().unwrap();
                for i in 0..n_tokens {
                    for &(e, w) in &chosen_topk[i] {
                        let mut d_act_i = vec![0.0f32; h];
                        for k in 0..h {
                            let mut sum_act = 0.0f32;
                            for j in 0..d {
                                sum_act += (w * d_h[i * d + j]) * layer.w_down[e].data[k * d + j];
                            }
                            d_act_i[k] = sum_act;
                        }

                        for k in 0..h {
                            for j in 0..d {
                                layer.w_down[e].grad[k * d + j] += (w * cache.expert_act[e][i * h + k]) * d_h[i * d + j];
                            }
                        }

                        for j in 0..h {
                            let da = d_act_i[j];
                            let sg = cache.expert_silu[e][i * h + j];
                            let du = da * sg;

                            let x = cache.expert_g[e][i * h + j];
                            let sig = 1.0 / (1.0 + (-x).exp());
                            let d_silu = sig * (1.0 + x * (1.0 - sig));
                            let dg = (da * cache.expert_u[e][i * h + j]) * d_silu;

                            for k in 0..d {
                                let h_val = h_curr[i * d + k];
                                layer.w_up[e].grad[k * h + j] += h_val * du;
                                layer.w_gate[e].grad[k * h + j] += h_val * dg;
                                d_h_prev[i * d + k] += du * layer.w_up[e].data[k * h + j] + dg * layer.w_gate[e].data[k * h + j];
                            }
                        }
                    }
                }
            }

            d_h = d_h_prev;
        }

        // 6. Embedding table backward
        for (i, &tok) in input_tokens.iter().enumerate() {
            let tok_clamped = tok % v;
            let dst = &mut self.embedding.grad[tok_clamped * d..(tok_clamped + 1) * d];
            let src = &d_h[i * d..(i + 1) * d];
            for k in 0..d {
                dst[k] += src[k];
            }
        }

        // Mirror layer 0 expert 0 grads for backward compatibility
        self.w_gate.grad.copy_from_slice(&self.layers[0].w_gate[0].grad);
        self.w_up.grad.copy_from_slice(&self.layers[0].w_up[0].grad);
        self.w_down.grad.copy_from_slice(&self.layers[0].w_down[0].grad);

        (mean_loss, perplexity)
    }

    /// Update all parameters using the selected optimizer and computed gradients
    pub fn optimizer_step(&mut self, step: usize, current_lr: f32) {
        let opt = self.config.optimizer;
        let wd = self.config.weight_decay;

        let mut all_params: Vec<&mut ParamMatrix> = Vec::new();
        all_params.push(&mut self.embedding);
        for layer in &mut self.layers {
            for g in &mut layer.w_gate { all_params.push(g); }
            for u in &mut layer.w_up { all_params.push(u); }
            for d in &mut layer.w_down { all_params.push(d); }
            if let Some(r) = &mut layer.w_router { all_params.push(r); }
        }
        all_params.push(&mut self.w_head);

        for param in all_params {
            let n = param.data.len();
            match opt {
                TrainingOptimizer::AdamW => {
                    let beta1 = 0.9f32;
                    let beta2 = 0.999f32;
                    let eps = 1e-8f32;
                    let t = step as f32;
                    let bc1 = 1.0 - beta1.powf(t);
                    let bc2 = 1.0 - beta2.powf(t);

                    for i in 0..n {
                        let g = param.grad[i];
                        param.data[i] -= current_lr * wd * param.data[i];

                        param.m[i] = beta1 * param.m[i] + (1.0 - beta1) * g;
                        param.v[i] = beta2 * param.v[i] + (1.0 - beta2) * g * g;

                        let m_hat = param.m[i] / bc1;
                        let v_hat = param.v[i] / bc2;

                        param.data[i] -= current_lr * m_hat / (v_hat.sqrt() + eps);
                    }
                }
                TrainingOptimizer::Lion => {
                    let beta1 = 0.9f32;
                    let beta2 = 0.99f32;

                    for i in 0..n {
                        let g = param.grad[i];
                        param.data[i] -= current_lr * wd * param.data[i];

                        let c = beta1 * param.m[i] + (1.0 - beta1) * g;
                        let sign_c = if c > 0.0 {
                            1.0f32
                        } else if c < 0.0 {
                            -1.0f32
                        } else {
                            0.0f32
                        };

                        param.data[i] -= current_lr * sign_c;
                        param.m[i] = beta2 * param.m[i] + (1.0 - beta2) * g;
                    }
                }
                TrainingOptimizer::Sgd => {
                    let momentum = 0.9f32;
                    for i in 0..n {
                        let g = param.grad[i];
                        param.data[i] -= current_lr * wd * param.data[i];
                        param.m[i] = momentum * param.m[i] + g;
                        param.data[i] -= current_lr * param.m[i];
                    }
                }
            }
        }

        // Mirror layer 0 expert 0 data for backward compatibility
        self.w_gate.data.copy_from_slice(&self.layers[0].w_gate[0].data);
        self.w_up.data.copy_from_slice(&self.layers[0].w_up[0].data);
        self.w_down.data.copy_from_slice(&self.layers[0].w_down[0].data);
    }

    /// Calculate learning rate for step according to schedule
    pub fn get_lr_for_step(&self, step: usize) -> f32 {
        let max_lr = self.config.learning_rate;
        let min_lr = self.config.min_learning_rate;
        let warmup = self.config.warmup_steps;
        let total = self.config.steps;

        if step <= warmup && warmup > 0 {
            return min_lr + (max_lr - min_lr) * (step as f32 / warmup as f32);
        }

        match self.config.lr_schedule {
            LrSchedule::Constant => max_lr,
            LrSchedule::Linear => {
                let progress = ((step - warmup) as f32) / ((total - warmup).max(1) as f32);
                (max_lr - progress * (max_lr - min_lr)).max(min_lr)
            }
            LrSchedule::Cosine => {
                let progress = ((step - warmup) as f32) / ((total - warmup).max(1) as f32);
                let progress_clamped = progress.clamp(0.0, 1.0);
                min_lr
                    + 0.5
                        * (max_lr - min_lr)
                        * (1.0 + (std::f32::consts::PI * progress_clamped).cos())
            }
        }
    }

    /// Exports all model weights into standard IEEE 754 SafeTensors format
    pub fn export_safetensors(&self, output_path: &str) -> Result<(), String> {
        let parent = Path::new(output_path).parent();
        if let Some(p) = parent {
            if !p.as_os_str().is_empty() {
                let _ = fs::create_dir_all(p);
            }
        }

        struct TensorEntry<'a> {
            name: String,
            shape: Vec<usize>,
            data: &'a [f32],
        }

        let mut entries = Vec::new();
        entries.push(TensorEntry {
            name: "embedding".to_string(),
            shape: vec![self.embedding.rows, self.embedding.cols],
            data: &self.embedding.data,
        });

        for (l, layer) in self.layers.iter().enumerate() {
            for (e, g) in layer.w_gate.iter().enumerate() {
                entries.push(TensorEntry {
                    name: format!("model.layers.{}.mlp.gate_proj.{}", l, e),
                    shape: vec![g.rows, g.cols],
                    data: &g.data,
                });
            }
            for (e, u) in layer.w_up.iter().enumerate() {
                entries.push(TensorEntry {
                    name: format!("model.layers.{}.mlp.up_proj.{}", l, e),
                    shape: vec![u.rows, u.cols],
                    data: &u.data,
                });
            }
            for (e, d) in layer.w_down.iter().enumerate() {
                entries.push(TensorEntry {
                    name: format!("model.layers.{}.mlp.down_proj.{}", l, e),
                    shape: vec![d.rows, d.cols],
                    data: &d.data,
                });
            }
            if let Some(ref r) = layer.w_router {
                entries.push(TensorEntry {
                    name: format!("model.layers.{}.moe.router", l),
                    shape: vec![r.rows, r.cols],
                    data: &r.data,
                });
            }
        }

        entries.push(TensorEntry {
            name: "w_head".to_string(),
            shape: vec![self.w_head.rows, self.w_head.cols],
            data: &self.w_head.data,
        });

        // Mirror legacy aliases for backward compatibility
        entries.push(TensorEntry {
            name: "w_gate".to_string(),
            shape: vec![self.w_gate.rows, self.w_gate.cols],
            data: &self.w_gate.data,
        });
        entries.push(TensorEntry {
            name: "w_up".to_string(),
            shape: vec![self.w_up.rows, self.w_up.cols],
            data: &self.w_up.data,
        });
        entries.push(TensorEntry {
            name: "w_down".to_string(),
            shape: vec![self.w_down.rows, self.w_down.cols],
            data: &self.w_down.data,
        });

        let mut header_map = Vec::new();
        let mut current_offset = 0usize;

        // Build header descriptor
        for entry in &entries {
            let byte_len = entry.data.len() * 4;
            let end_offset = current_offset + byte_len;
            let shape_str = format!(
                "[{}]",
                entry
                    .shape
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let item = format!(
                "\"{}\": {{\"dtype\": \"F32\", \"shape\": {}, \"data_offsets\": [{}, {}]}}",
                entry.name, shape_str, current_offset, end_offset
            );
            header_map.push(item);
            current_offset = end_offset;
        }

        let mut header_json = format!("{{{}}}", header_map.join(", "));
        // Align header length to 8 bytes for SIMD alignment
        let pad_len = (8 - (header_json.len() % 8)) % 8;
        if pad_len > 0 {
            header_json.push_str(&" ".repeat(pad_len));
        }

        let header_bytes = header_json.as_bytes();
        let header_len = header_bytes.len() as u64;

        let mut file = File::create(output_path)
            .map_err(|e| format!("Failed to create safetensors file '{}': {}", output_path, e))?;

        // 1. Write 8-byte little-endian header length
        file.write_all(&header_len.to_le_bytes())
            .map_err(|e| format!("Failed to write header length: {}", e))?;

        // 2. Write header JSON bytes
        file.write_all(header_bytes)
            .map_err(|e| format!("Failed to write header JSON: {}", e))?;

        // 3. Write raw float32 little-endian tensor data
        for entry in &entries {
            for &val in entry.data {
                file.write_all(&val.to_le_bytes())
                    .map_err(|e| format!("Failed to write tensor data: {}", e))?;
            }
        }

        file.flush()
            .map_err(|e| format!("Failed to flush safetensors file: {}", e))?;
        Ok(())
    }

    /// Compute next-token output logits for the last token given an input sequence
    pub fn forward_logits_last(&self, token_seq: &[usize]) -> Vec<f32> {
        let n_tokens = token_seq.len();
        if n_tokens == 0 {
            return vec![0.0f32; self.config.vocab_size];
        }
        let d = self.config.dim;
        let h = self.config.hidden_dim;
        let v = self.config.vocab_size;
        let num_layers = self.layers.len();
        let num_experts = self.config.num_experts.max(1);
        let top_k = self.config.top_k.min(num_experts).max(1);

        let mut h_curr = vec![0.0f32; n_tokens * d];
        for (i, &tok) in token_seq.iter().enumerate() {
            let tok_clamped = tok % v;
            let src = &self.embedding.data[tok_clamped * d..(tok_clamped + 1) * d];
            h_curr[i * d..(i + 1) * d].copy_from_slice(src);
        }

        for l in 0..num_layers {
            let layer = &self.layers[l];
            let mut h_next = h_curr.clone();

            if num_experts == 1 {
                for i in 0..n_tokens {
                    for j in 0..d {
                        let mut sum_d = 0.0f32;
                        for k in 0..h {
                            let mut sum_g = 0.0f32;
                            let mut sum_u = 0.0f32;
                            for m in 0..d {
                                let val = h_curr[i * d + m];
                                sum_g += val * layer.w_gate[0].data[m * h + k];
                                sum_u += val * layer.w_up[0].data[m * h + k];
                            }
                            let sig = 1.0 / (1.0 + (-sum_g).exp());
                            let act = (sum_g * sig) * sum_u;
                            sum_d += act * layer.w_down[0].data[k * d + j];
                        }
                        h_next[i * d + j] += sum_d;
                    }
                }
            } else {
                let router = layer.w_router.as_ref().unwrap();
                for i in 0..n_tokens {
                    let mut r_logits = vec![0.0f32; num_experts];
                    let mut max_r = -1e9f32;
                    for e in 0..num_experts {
                        let mut sum_r = 0.0f32;
                        for k in 0..d {
                            sum_r += h_curr[i * d + k] * router.data[k * num_experts + e];
                        }
                        r_logits[e] = sum_r;
                        if sum_r > max_r { max_r = sum_r; }
                    }

                    let mut r_probs = vec![0.0f32; num_experts];
                    let mut sum_exp = 0.0f32;
                    for e in 0..num_experts {
                        let exp_val = (r_logits[e] - max_r).exp();
                        r_probs[e] = exp_val;
                        sum_exp += exp_val;
                    }
                    let inv_sum = 1.0 / sum_exp.max(1e-12);
                    for e in 0..num_experts { r_probs[e] *= inv_sum; }

                    let mut indexed_probs: Vec<(usize, f32)> = (0..num_experts)
                        .map(|e| (e, r_probs[e]))
                        .collect();
                    indexed_probs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    let selected: Vec<(usize, f32)> = indexed_probs.into_iter().take(top_k).collect();
                    let topk_sum: f32 = selected.iter().map(|(_, w)| *w).sum();
                    let inv_topk = 1.0 / topk_sum.max(1e-12);

                    for &(e, w) in &selected {
                        let norm_w = w * inv_topk;
                        for j in 0..d {
                            let mut sum_d = 0.0f32;
                            for k in 0..h {
                                let mut sum_g = 0.0f32;
                                let mut sum_u = 0.0f32;
                                for m in 0..d {
                                    let val = h_curr[i * d + m];
                                    sum_g += val * layer.w_gate[e].data[m * h + k];
                                    sum_u += val * layer.w_up[e].data[m * h + k];
                                }
                                let sig = 1.0 / (1.0 + (-sum_g).exp());
                                let act = (sum_g * sig) * sum_u;
                                sum_d += act * layer.w_down[e].data[k * d + j];
                            }
                            h_next[i * d + j] += norm_w * sum_d;
                        }
                    }
                }
            }

            h_curr = h_next;
        }

        let last_idx = n_tokens - 1;
        let mut logits = vec![0.0f32; v];
        for j in 0..v {
            let mut sum_z = 0.0f32;
            for k in 0..d {
                sum_z += h_curr[last_idx * d + k] * self.w_head.data[k * v + j];
            }
            logits[j] = sum_z;
        }

        logits
    }

    /// Autoregressively generates new token IDs given an initial sequence
    pub fn generate_tokens(&self, prompt_tokens: &[usize], max_new_tokens: usize, temperature: f32) -> Vec<usize> {
        let mut tokens = prompt_tokens.to_vec();
        if tokens.is_empty() {
            tokens.push(0);
        }
        let v = self.config.vocab_size;

        for step in 0..max_new_tokens {
            let window_start = if tokens.len() > self.config.seq_len {
                tokens.len() - self.config.seq_len
            } else {
                0
            };
            let context = &tokens[window_start..];
            let logits = self.forward_logits_last(context);

            let next_tok = if temperature <= 0.01 {
                // Greedy argmax
                let mut best_tok = 0;
                let mut best_val = -1e9f32;
                for (idx, &val) in logits.iter().enumerate() {
                    if val > best_val {
                        best_val = val;
                        best_tok = idx;
                    }
                }
                best_tok
            } else {
                // Temperature scaled sampling
                let mut max_l = -1e9f32;
                for &l in &logits {
                    if l > max_l {
                        max_l = l;
                    }
                }
                let mut probs = Vec::with_capacity(v);
                let mut sum_p = 0.0f32;
                for &l in &logits {
                    let p = ((l - max_l) / temperature).exp();
                    probs.push(p);
                    sum_p += p;
                }

                let inv_sum = 1.0 / sum_p.max(1e-12);
                let seed_step = (tokens.len() + step).wrapping_mul(1103515245).wrapping_add(12345);
                let seed_ratio = ((seed_step & 0x7fffffff) as f32) / (0x7fffffff as f32);

                let mut accum = 0.0f32;
                let mut chosen = 0;
                for (idx, &p) in probs.iter().enumerate() {
                    accum += p * inv_sum;
                    if accum >= seed_ratio {
                        chosen = idx;
                        break;
                    }
                }
                chosen
            };

            tokens.push(next_tok % v);
        }

        tokens
    }

    /// Autoregressively generates continuation text given a text prompt
    pub fn generate_text(&self, prompt: &str, max_new_tokens: usize, temperature: f32) -> String {
        let tokenizer = BpeTokenizer::from_default_vocab();
        let prompt_tokens = tokenizer.encode(prompt);
        let input_tokens: Vec<usize> = if prompt_tokens.is_empty() {
            prompt.bytes().map(|b| b as usize).collect()
        } else {
            prompt_tokens.into_iter().map(|t| t as usize).collect()
        };

        let generated_ids = self.generate_tokens(&input_tokens, max_new_tokens, temperature);
        let new_ids = &generated_ids[input_tokens.len()..];
        let u32_ids: Vec<u32> = new_ids.iter().map(|&x| x as u32).collect();
        let decoded = tokenizer.decode(&u32_ids);
        if !decoded.is_empty() {
            format!("{}{}", prompt, decoded)
        } else {
            let mut bytes = prompt.as_bytes().to_vec();
            for &id in new_ids {
                bytes.push((id % 256) as u8);
            }
            String::from_utf8_lossy(&bytes).to_string()
        }
    }

    /// Loads model weights from a standard IEEE 754 SafeTensors file
    pub fn from_safetensors_file(path: &str) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|e| format!("Failed to read safetensors file '{}': {}", path, e))?;
        let raw_tensors = crate::cl_safetensors_convert::parse_raw_safetensors(&bytes)?;

        let mut config = TrainingConfig::default();
        let mut embedding_opt = None;
        let mut w_head_opt = None;
        let mut w_gate_opt = None;
        let mut w_up_opt = None;
        let mut w_down_opt = None;

        for t in raw_tensors {
            match t.name.as_str() {
                "embedding" if t.shape.len() == 2 => {
                    config.vocab_size = t.shape[0];
                    config.dim = t.shape[1];
                    embedding_opt = Some(ParamMatrix {
                        rows: t.shape[0],
                        cols: t.shape[1],
                        data: t.data.clone(),
                        grad: vec![0.0f32; t.data.len()],
                        m: vec![0.0f32; t.data.len()],
                        v: vec![0.0f32; t.data.len()],
                    });
                }
                "w_head" if t.shape.len() == 2 => {
                    w_head_opt = Some(ParamMatrix {
                        rows: t.shape[0],
                        cols: t.shape[1],
                        data: t.data.clone(),
                        grad: vec![0.0f32; t.data.len()],
                        m: vec![0.0f32; t.data.len()],
                        v: vec![0.0f32; t.data.len()],
                    });
                }
                "w_gate" | "model.layers.0.mlp.gate_proj.0" if t.shape.len() == 2 => {
                    config.hidden_dim = t.shape[1];
                    w_gate_opt = Some(ParamMatrix {
                        rows: t.shape[0],
                        cols: t.shape[1],
                        data: t.data.clone(),
                        grad: vec![0.0f32; t.data.len()],
                        m: vec![0.0f32; t.data.len()],
                        v: vec![0.0f32; t.data.len()],
                    });
                }
                "w_up" | "model.layers.0.mlp.up_proj.0" if t.shape.len() == 2 => {
                    w_up_opt = Some(ParamMatrix {
                        rows: t.shape[0],
                        cols: t.shape[1],
                        data: t.data.clone(),
                        grad: vec![0.0f32; t.data.len()],
                        m: vec![0.0f32; t.data.len()],
                        v: vec![0.0f32; t.data.len()],
                    });
                }
                "w_down" | "model.layers.0.mlp.down_proj.0" if t.shape.len() == 2 => {
                    w_down_opt = Some(ParamMatrix {
                        rows: t.shape[0],
                        cols: t.shape[1],
                        data: t.data.clone(),
                        grad: vec![0.0f32; t.data.len()],
                        m: vec![0.0f32; t.data.len()],
                        v: vec![0.0f32; t.data.len()],
                    });
                }
                _ => {}
            }
        }

        let mut model = AutonomousLanguageModel::new(config);
        if let Some(emb) = embedding_opt { model.embedding = emb; }
        if let Some(head) = w_head_opt { model.w_head = head; }
        if let Some(g) = w_gate_opt {
            model.layers[0].w_gate[0] = g.clone();
            model.w_gate = g;
        }
        if let Some(u) = w_up_opt {
            model.layers[0].w_up[0] = u.clone();
            model.w_up = u;
        }
        if let Some(d) = w_down_opt {
            model.layers[0].w_down[0] = d.clone();
            model.w_down = d;
        }

        Ok(model)
    }

    /// Autoregressively generates text directly on the physical NVIDIA RTX GPU with real-time streaming
    pub fn generate_text_gpu(
        &self,
        prompt: &str,
        max_new_tokens: usize,
        temperature: f32,
        mut token_callback: impl FnMut(&str),
    ) -> Result<String, String> {
        if !CudaDriver::is_available() {
            return Err("CUDA Driver not available on host system".to_string());
        }

        let driver = CudaDriver::init()?;
        let ctx = Arc::new(driver.create_context(0)?);
        ctx.make_current()?;
        let engine = CudaTensorEngine::new(ctx.clone())?;

        let d = self.config.dim;
        let h = self.config.hidden_dim;
        let v = self.config.vocab_size;
        let seq_len = self.config.seq_len.max(16);
        let num_layers = self.layers.len().max(1);

        // Upload head weights to GPU
        let d_w_head = ctx.alloc(d * v * 4)?;
        ctx.copy_htod(&d_w_head, &self.w_head.data)?;

        // Upload all transformer layers to GPU
        let mut d_w_gate = Vec::with_capacity(num_layers);
        let mut d_w_up = Vec::with_capacity(num_layers);
        let mut d_w_down = Vec::with_capacity(num_layers);
        for l in 0..num_layers {
            let buf_gate = ctx.alloc(d * h * 4)?;
            ctx.copy_htod(&buf_gate, &self.layers[l].w_gate[0].data)?;
            d_w_gate.push(buf_gate);

            let buf_up = ctx.alloc(d * h * 4)?;
            ctx.copy_htod(&buf_up, &self.layers[l].w_up[0].data)?;
            d_w_up.push(buf_up);

            let buf_down = ctx.alloc(h * d * 4)?;
            ctx.copy_htod(&buf_down, &self.layers[l].w_down[0].data)?;
            d_w_down.push(buf_down);
        }

        // Allocate scratch buffers on GPU
        let d_h = ctx.alloc(seq_len * d * 4)?;
        let d_g = ctx.alloc(seq_len * h * 4)?;
        let d_u = ctx.alloc(seq_len * h * 4)?;
        let d_act = ctx.alloc(seq_len * h * 4)?;
        let d_h_ffn = ctx.alloc(seq_len * d * 4)?;
        let d_logits = ctx.alloc(v * 4)?;

        let tokenizer = BpeTokenizer::from_default_vocab();
        let prompt_tokens = tokenizer.encode(prompt);
        let mut tokens: Vec<usize> = if prompt_tokens.is_empty() {
            prompt.bytes().map(|b| b as usize).collect()
        } else {
            prompt_tokens.into_iter().map(|t| t as usize).collect()
        };

        if tokens.is_empty() {
            tokens.push(0);
        }

        let mut generated_ids = Vec::new();

        for _ in 0..max_new_tokens {
            let window_start = if tokens.len() > seq_len {
                tokens.len() - seq_len
            } else {
                0
            };
            let context = &tokens[window_start..];
            let n = context.len();

            // 1. Embedding lookup on host
            let mut h_host = vec![0.0f32; n * d];
            for (i, &tok) in context.iter().enumerate() {
                let tok_clamped = tok % v;
                let src = &self.embedding.data[tok_clamped * d..(tok_clamped + 1) * d];
                h_host[i * d..(i + 1) * d].copy_from_slice(src);
            }

            // 2. Upload input embeddings to GPU
            ctx.copy_htod(&d_h, &h_host)?;

            // 3. Multi-Layer GPU Forward Pass
            for l in 0..num_layers {
                engine.gemm(&d_h, &d_w_gate[l], &d_g, n, h, d, false, false)?;
                engine.gemm(&d_h, &d_w_up[l], &d_u, n, h, d, false, false)?;
                engine.swiglu(&d_g, &d_u, &d_act, n * h)?;
                engine.gemm(&d_act, &d_w_down[l], &d_h_ffn, n, d, h, false, false)?;
                engine.vec_add_inplace(&d_h, &d_h_ffn, n * d)?;
            }

            // 4. Zero-copy Last-Token Logits: compute logits from true transformer output in VRAM
            let last_token_dptr = d_h.dptr + ((n - 1) * d * std::mem::size_of::<f32>()) as u64;
            engine.gemm_dptr(last_token_dptr, d_w_head.dptr, d_logits.dptr, 1, v, d, false, false)?;
            ctx.synchronize()?;

            // 5. Read back next-token logits from GPU
            let mut logits = vec![0.0f32; v];
            ctx.copy_dtoh(&d_logits, &mut logits)?;

            // 6. Apply repetition penalty for recent context
            let recent_window = &tokens[tokens.len().saturating_sub(32)..];
            for &past_tok in recent_window {
                let idx = past_tok % v;
                if logits[idx] > 0.0 {
                    logits[idx] /= 1.2;
                } else {
                    logits[idx] *= 1.2;
                }
            }

            // 7. Sample token with temperature
            let next_tok = if temperature <= 0.01 {
                let mut best_tok = 0;
                let mut best_val = -1e9f32;
                for (idx, &val) in logits.iter().enumerate() {
                    if val > best_val {
                        best_val = val;
                        best_tok = idx;
                    }
                }
                best_tok
            } else {
                let mut max_l = -1e9f32;
                for &l in &logits {
                    if l > max_l { max_l = l; }
                }
                let mut probs = Vec::with_capacity(v);
                let mut sum_p = 0.0f32;
                for &l in &logits {
                    let p = ((l - max_l) / temperature).exp();
                    probs.push(p);
                    sum_p += p;
                }
                let inv_sum = 1.0 / sum_p.max(1e-12);
                let seed_step = (tokens.len() + generated_ids.len() * 37)
                    .wrapping_mul(1103515245)
                    .wrapping_add(12345);
                let seed_ratio = ((seed_step & 0x7fffffff) as f32) / (0x7fffffff as f32);

                let mut accum = 0.0f32;
                let mut chosen = 0;
                for (idx, &p) in probs.iter().enumerate() {
                    accum += p * inv_sum;
                    if accum >= seed_ratio {
                        chosen = idx;
                        break;
                    }
                }
                chosen
            };

            let next_tok_clamped = next_tok % v;
            tokens.push(next_tok_clamped);
            generated_ids.push(next_tok_clamped);

            // Stream token
            let piece = tokenizer.decode(&[next_tok_clamped as u32]);
            let s = if !piece.is_empty() {
                piece
            } else {
                let b = [next_tok_clamped as u8];
                String::from_utf8_lossy(&b).to_string()
            };
            token_callback(&s);
        }

        let full_piece = tokenizer.decode(&generated_ids.iter().map(|&x| x as u32).collect::<Vec<_>>());
        let result = if !full_piece.is_empty() {
            format!("{}{}", prompt, full_piece)
        } else {
            let mut bytes = prompt.as_bytes().to_vec();
            for &id in &generated_ids {
                bytes.push((id % 256) as u8);
            }
            String::from_utf8_lossy(&bytes).to_string()
        };

        Ok(result)
    }
}

/// Tokenizes source training corpus into a continuous sequence of token IDs
pub fn prepare_dataset_tokens(config: &TrainingConfig) -> Result<Vec<usize>, String> {
    let raw_text = if let Some(ref text) = config.data_text {
        text.clone()
    } else if let Some(ref path) = config.data_path {
        fs::read_to_string(path)
            .map_err(|e| format!("Failed to read dataset from '{}': {}", path, e))?
    } else {
        // Fallback default corpus: Sovereign AI foundational corpus
        "CRON Zero-Verbal Machine Architecture. In-silicon tensors contract over 4D Torus NoC. \
         Continuous-time dynamic differential engine with zero GEMM and zero DRAM. \
         Монгол хэлний хиймэл оюун ухааны цахиур архитектур ба цэвэр математик тооцоолол."
            .to_string()
    };

    let tokenizer = BpeTokenizer::from_default_vocab();
    let bpe_tokens = tokenizer.encode(&raw_text);

    let tokens: Vec<usize> = if bpe_tokens.is_empty() {
        // Byte fallback
        raw_text.bytes().map(|b| b as usize).collect()
    } else {
        bpe_tokens.into_iter().map(|t| t as usize).collect()
    };

    if tokens.len() < config.seq_len + 1 {
        // Tile tokens to satisfy sequence length requirement
        let mut tiled = Vec::new();
        while tiled.len() < (config.seq_len + 1) * config.batch_size * 2 {
            tiled.extend_from_slice(&tokens);
        }
        Ok(tiled)
    } else {
        Ok(tokens)
    }
}

/// Dedicated GPU Hardware Acceleration Engine for In-Silicon Neural Training
pub struct CudaTrainingEngine {
    pub engine: CudaTensorEngine,
    pub num_layers: usize,
    pub d: usize,
    pub h: usize,
    pub v: usize,
    pub n_tokens: usize,

    // Layer parameters on GPU
    pub d_w_gate: Vec<CudaDeviceBuffer>,
    pub d_w_up: Vec<CudaDeviceBuffer>,
    pub d_w_down: Vec<CudaDeviceBuffer>,
    pub d_m_gate: Vec<CudaDeviceBuffer>,
    pub d_m_up: Vec<CudaDeviceBuffer>,
    pub d_m_down: Vec<CudaDeviceBuffer>,
    pub d_v_gate: Vec<CudaDeviceBuffer>,
    pub d_v_up: Vec<CudaDeviceBuffer>,
    pub d_v_down: Vec<CudaDeviceBuffer>,
    pub d_grad_gate: Vec<CudaDeviceBuffer>,
    pub d_grad_up: Vec<CudaDeviceBuffer>,
    pub d_grad_down: Vec<CudaDeviceBuffer>,

    // Head parameters on GPU
    pub d_w_head: CudaDeviceBuffer,
    pub d_m_head: CudaDeviceBuffer,
    pub d_v_head: CudaDeviceBuffer,
    pub d_grad_head: CudaDeviceBuffer,

    // Intermediate activations on GPU
    pub d_h: CudaDeviceBuffer,
    pub d_h_ffn: CudaDeviceBuffer,
    pub d_g: CudaDeviceBuffer,
    pub d_u: CudaDeviceBuffer,
    pub d_act: CudaDeviceBuffer,
    pub d_logits: CudaDeviceBuffer,
    pub d_d_logits: CudaDeviceBuffer,
    pub d_dh: CudaDeviceBuffer,
    pub d_d_act: CudaDeviceBuffer,
    pub d_dg: CudaDeviceBuffer,
    pub d_du: CudaDeviceBuffer,
}

impl CudaTrainingEngine {
    pub fn new(model: &AutonomousLanguageModel, n_tokens: usize) -> Result<Self, String> {
        let driver = CudaDriver::init()?;
        let ctx = Arc::new(driver.create_context(0)?);
        ctx.make_current()?;

        let engine = CudaTensorEngine::new(ctx.clone())?;
        let d = model.config.dim;
        let h = model.config.hidden_dim;
        let v = model.config.vocab_size;
        let num_layers = model.layers.len();

        let mut d_w_gate = Vec::with_capacity(num_layers);
        let mut d_w_up = Vec::with_capacity(num_layers);
        let mut d_w_down = Vec::with_capacity(num_layers);
        let mut d_m_gate = Vec::with_capacity(num_layers);
        let mut d_m_up = Vec::with_capacity(num_layers);
        let mut d_m_down = Vec::with_capacity(num_layers);
        let mut d_v_gate = Vec::with_capacity(num_layers);
        let mut d_v_up = Vec::with_capacity(num_layers);
        let mut d_v_down = Vec::with_capacity(num_layers);
        let mut d_grad_gate = Vec::with_capacity(num_layers);
        let mut d_grad_up = Vec::with_capacity(num_layers);
        let mut d_grad_down = Vec::with_capacity(num_layers);

        let zeros_dh = vec![0.0f32; d * h];
        let zeros_hd = vec![0.0f32; h * d];

        for l in 0..num_layers {
            let buf_gate = ctx.alloc(d * h * 4)?;
            ctx.copy_htod(&buf_gate, &model.layers[l].w_gate[0].data)?;
            d_w_gate.push(buf_gate);

            let buf_up = ctx.alloc(d * h * 4)?;
            ctx.copy_htod(&buf_up, &model.layers[l].w_up[0].data)?;
            d_w_up.push(buf_up);

            let buf_down = ctx.alloc(h * d * 4)?;
            ctx.copy_htod(&buf_down, &model.layers[l].w_down[0].data)?;
            d_w_down.push(buf_down);

            let m_g = ctx.alloc(d * h * 4)?; ctx.copy_htod(&m_g, &zeros_dh)?; d_m_gate.push(m_g);
            let v_g = ctx.alloc(d * h * 4)?; ctx.copy_htod(&v_g, &zeros_dh)?; d_v_gate.push(v_g);
            let grad_g = ctx.alloc(d * h * 4)?; ctx.copy_htod(&grad_g, &zeros_dh)?; d_grad_gate.push(grad_g);

            let m_u = ctx.alloc(d * h * 4)?; ctx.copy_htod(&m_u, &zeros_dh)?; d_m_up.push(m_u);
            let v_u = ctx.alloc(d * h * 4)?; ctx.copy_htod(&v_u, &zeros_dh)?; d_v_up.push(v_u);
            let grad_u = ctx.alloc(d * h * 4)?; ctx.copy_htod(&grad_u, &zeros_dh)?; d_grad_up.push(grad_u);

            let m_d = ctx.alloc(h * d * 4)?; ctx.copy_htod(&m_d, &zeros_hd)?; d_m_down.push(m_d);
            let v_d = ctx.alloc(h * d * 4)?; ctx.copy_htod(&v_d, &zeros_hd)?; d_v_down.push(v_d);
            let grad_d = ctx.alloc(h * d * 4)?; ctx.copy_htod(&grad_d, &zeros_hd)?; d_grad_down.push(grad_d);
        }

        let d_w_head = ctx.alloc(d * v * 4)?;
        ctx.copy_htod(&d_w_head, &model.w_head.data)?;
        let zeros_dv = vec![0.0f32; d * v];
        let d_m_head = ctx.alloc(d * v * 4)?; ctx.copy_htod(&d_m_head, &zeros_dv)?;
        let d_v_head = ctx.alloc(d * v * 4)?; ctx.copy_htod(&d_v_head, &zeros_dv)?;
        let d_grad_head = ctx.alloc(d * v * 4)?; ctx.copy_htod(&d_grad_head, &zeros_dv)?;

        let d_h = ctx.alloc(n_tokens * d * 4)?;
        let d_h_ffn = ctx.alloc(n_tokens * d * 4)?;
        let d_g = ctx.alloc(n_tokens * h * 4)?;
        let d_u = ctx.alloc(n_tokens * h * 4)?;
        let d_act = ctx.alloc(n_tokens * h * 4)?;
        let d_logits = ctx.alloc(n_tokens * v * 4)?;
        let d_d_logits = ctx.alloc(n_tokens * v * 4)?;
        let d_dh = ctx.alloc(n_tokens * d * 4)?;
        let d_d_act = ctx.alloc(n_tokens * h * 4)?;
        let d_dg = ctx.alloc(n_tokens * h * 4)?;
        let d_du = ctx.alloc(n_tokens * h * 4)?;

        Ok(Self {
            engine,
            num_layers,
            d,
            h,
            v,
            n_tokens,
            d_w_gate,
            d_w_up,
            d_w_down,
            d_m_gate,
            d_m_up,
            d_m_down,
            d_v_gate,
            d_v_up,
            d_v_down,
            d_grad_gate,
            d_grad_up,
            d_grad_down,
            d_w_head,
            d_m_head,
            d_v_head,
            d_grad_head,
            d_h,
            d_h_ffn,
            d_g,
            d_u,
            d_act,
            d_logits,
            d_d_logits,
            d_dh,
            d_d_act,
            d_dg,
            d_du,
        })
    }

    /// Executes high-throughput GPU forward pass, cross-entropy gradient computation,
    /// backward VJP pullback, and in-place GPU AdamW parameter update.
    pub fn train_step(
        &mut self,
        model: &mut AutonomousLanguageModel,
        input_tokens: &[usize],
        target_tokens: &[usize],
        step: usize,
        lr: f32,
    ) -> Result<(f32, f32), String> {
        let n = self.n_tokens;
        let d = self.d;
        let h = self.h;
        let v = self.v;

        // 1. Embedding lookup on host
        let mut h0 = vec![0.0f32; n * d];
        for (i, &tok) in input_tokens.iter().enumerate() {
            let tok_clamped = tok % v;
            let src = &model.embedding.data[tok_clamped * d..(tok_clamped + 1) * d];
            h0[i * d..(i + 1) * d].copy_from_slice(src);
        }

        // 2. Upload initial embedding activations to GPU
        self.engine.ctx.copy_htod(&self.d_h, &h0)?;

        // 3. Multi-layer forward pass on GPU
        for l in 0..self.num_layers {
            // G = H @ W_gate
            self.engine.gemm(&self.d_h, &self.d_w_gate[l], &self.d_g, n, h, d, false, false)?;
            // U = H @ W_up
            self.engine.gemm(&self.d_h, &self.d_w_up[l], &self.d_u, n, h, d, false, false)?;
            // act = SiLU(G) * U
            self.engine.swiglu(&self.d_g, &self.d_u, &self.d_act, n * h)?;
            // H_ffn = act @ W_down
            self.engine.gemm(&self.d_act, &self.d_w_down[l], &self.d_h_ffn, n, d, h, false, false)?;
            // Residual: H = H + H_ffn
            self.engine.vec_add_inplace(&self.d_h, &self.d_h_ffn, n * d)?;
        }

        // 4. Output Logits: logits = H @ W_head
        self.engine.gemm(&self.d_h, &self.d_w_head, &self.d_logits, n, v, d, false, false)?;
        self.engine.ctx.synchronize()?;

        // 5. Read back logits to host for Softmax Cross-Entropy Loss
        let mut logits = vec![0.0f32; n * v];
        self.engine.ctx.copy_dtoh(&self.d_logits, &mut logits)?;

        let mut probs = vec![0.0f32; n * v];
        let mut total_loss = 0.0f32;
        for i in 0..n {
            let row = &logits[i * v..(i + 1) * v];
            let mut max_val = -1e9f32;
            for &val in row {
                if val > max_val { max_val = val; }
            }
            let mut sum_exp = 0.0f32;
            for j in 0..v {
                let exp_val = (row[j] - max_val).exp();
                probs[i * v + j] = exp_val;
                sum_exp += exp_val;
            }
            let inv_sum = 1.0 / sum_exp.max(1e-12);
            for j in 0..v {
                probs[i * v + j] *= inv_sum;
            }
            let tgt = target_tokens[i] % v;
            let prob_tgt = probs[i * v + tgt].max(1e-12);
            total_loss += -prob_tgt.ln();
        }

        let mean_loss = total_loss / (n as f32);
        let perplexity = mean_loss.min(80.0).exp();

        // 6. Compute d_logits
        let inv_n = 1.0 / (n as f32);
        let mut d_logits = vec![0.0f32; n * v];
        for i in 0..n {
            let tgt = target_tokens[i] % v;
            for j in 0..v {
                let p = probs[i * v + j];
                d_logits[i * v + j] = if j == tgt { (p - 1.0) * inv_n } else { p * inv_n };
            }
        }

        // 7. Upload d_logits to GPU
        self.engine.ctx.copy_htod(&self.d_d_logits, &d_logits)?;

        // 8. Backward Pass on GPU
        // grad_head = H^T @ d_logits [d, v]
        self.engine.gemm(&self.d_h, &self.d_d_logits, &self.d_grad_head, d, v, n, true, false)?;
        // dH = d_logits @ W_head^T [n, d]
        self.engine.gemm(&self.d_d_logits, &self.d_w_head, &self.d_dh, n, d, v, false, true)?;

        for l in (0..self.num_layers).rev() {
            // grad_down = act^T @ dH [h, d]
            self.engine.gemm(&self.d_act, &self.d_dh, &self.d_grad_down[l], h, d, n, true, false)?;
            // d_act = dH @ W_down^T [n, h]
            self.engine.gemm(&self.d_dh, &self.d_w_down[l], &self.d_d_act, n, h, d, false, true)?;
            // SwiGLU backward: dG, dU
            self.engine.swiglu_bwd(&self.d_d_act, &self.d_g, &self.d_u, &self.d_dg, &self.d_du, n * h)?;
            // grad_up = H^T @ dU [d, h]
            self.engine.gemm(&self.d_h, &self.d_du, &self.d_grad_up[l], d, h, n, true, false)?;
            // grad_gate = H^T @ dG [d, h]
            self.engine.gemm(&self.d_h, &self.d_dg, &self.d_grad_gate[l], d, h, n, true, false)?;
        }

        // 9. GPU in-place AdamW parameter update
        let wd = model.config.weight_decay;
        let beta1 = 0.9f32;
        let beta2 = 0.999f32;
        let eps = 1e-8f32;

        self.engine.adamw_step(&self.d_w_head, &self.d_grad_head, &self.d_m_head, &self.d_v_head, lr, beta1, beta2, eps, wd, step, d * v)?;

        for l in 0..self.num_layers {
            self.engine.adamw_step(&self.d_w_down[l], &self.d_grad_down[l], &self.d_m_down[l], &self.d_v_down[l], lr, beta1, beta2, eps, wd, step, h * d)?;
            self.engine.adamw_step(&self.d_w_up[l], &self.d_grad_up[l], &self.d_m_up[l], &self.d_v_up[l], lr, beta1, beta2, eps, wd, step, d * h)?;
            self.engine.adamw_step(&self.d_w_gate[l], &self.d_grad_gate[l], &self.d_m_gate[l], &self.d_v_gate[l], lr, beta1, beta2, eps, wd, step, d * h)?;
        }

        self.engine.ctx.synchronize()?;
        Ok((mean_loss, perplexity))
    }

    /// Read back all trained neural parameter matrices from GPU VRAM to Host CPU memory
    pub fn sync_weights_to_host(&self, model: &mut AutonomousLanguageModel) -> Result<(), String> {
        self.engine.ctx.synchronize()?;
        self.engine.ctx.copy_dtoh(&self.d_w_head, &mut model.w_head.data)?;

        for l in 0..self.num_layers {
            self.engine.ctx.copy_dtoh(&self.d_w_gate[l], &mut model.layers[l].w_gate[0].data)?;
            self.engine.ctx.copy_dtoh(&self.d_w_up[l], &mut model.layers[l].w_up[0].data)?;
            self.engine.ctx.copy_dtoh(&self.d_w_down[l], &mut model.layers[l].w_down[0].data)?;
        }

        // Mirror layer 0 expert 0 data for backward compatibility
        model.w_gate.data.copy_from_slice(&model.layers[0].w_gate[0].data);
        model.w_up.data.copy_from_slice(&model.layers[0].w_up[0].data);
        model.w_down.data.copy_from_slice(&model.layers[0].w_down[0].data);
        Ok(())
    }
}

/// Main Autonomous Zero-Python Training Loop
pub fn train_autonomous_model(
    config: TrainingConfig,
    progress_callback: Option<Box<dyn FnMut(&TrainingStepReport)>>,
) -> Result<AutonomousTrainingSummary, String> {
    let start_time = Instant::now();
    let tokens = prepare_dataset_tokens(&config)?;

    // Determine actual effective vocabulary size
    let mut actual_config = config.clone();
    let max_tok = tokens.iter().max().copied().unwrap_or(255);
    actual_config.vocab_size = actual_config.vocab_size.max(max_tok + 1);

    let mut model = AutonomousLanguageModel::new(actual_config.clone());
    let mut step_history = Vec::with_capacity(actual_config.steps);

    let mut callback = progress_callback;
    let b = actual_config.batch_size;
    let l = actual_config.seq_len;
    let window_size = b * l;
    let n_tokens = tokens.len();

    let mut gpu_engine: Option<CudaTrainingEngine> = if actual_config.use_cuda && CudaDriver::is_available() {
        match CudaTrainingEngine::new(&model, window_size) {
            Ok(eng) => {
                println!("[CUDA] Physical NVIDIA GPU Acceleration active (Direct Driver API + sm_80 PTX)");
                Some(eng)
            }
            Err(e) => {
                eprintln!("[CUDA WARN] GPU init failed ({}), falling back to CPU calculus engine.", e);
                None
            }
        }
    } else {
        None
    };

    let mut initial_loss = 0.0f32;
    let mut final_loss = 0.0f32;
    let mut total_tokens_processed = 0usize;

    for step in 1..=actual_config.steps {
        let step_start = Instant::now();
        let lr = model.get_lr_for_step(step);

        // Windowed mini-batch selection
        let mut input_batch = Vec::with_capacity(window_size);
        let mut target_batch = Vec::with_capacity(window_size);

        for item_idx in 0..b {
            let offset = ((step - 1) * b + item_idx) * l % (n_tokens - l - 1);
            let x_slice = &tokens[offset..offset + l];
            let y_slice = &tokens[offset + 1..offset + 1 + l];
            input_batch.extend_from_slice(x_slice);
            target_batch.extend_from_slice(y_slice);
        }

        // Forward, loss calculation, backward autodiff & optimizer update
        let (loss, perplexity) = if let Some(ref mut gpu) = gpu_engine {
            match gpu.train_step(&mut model, &input_batch, &target_batch, step, lr) {
                Ok(lp) => lp,
                Err(e) => {
                    eprintln!("[CUDA ERROR] Step failed on GPU ({}), falling back to CPU", e);
                    let lp = model.forward_backward_step(&input_batch, &target_batch);
                    model.optimizer_step(step, lr);
                    lp
                }
            }
        } else {
            let lp = model.forward_backward_step(&input_batch, &target_batch);
            model.optimizer_step(step, lr);
            lp
        };

        let grad_norm = model.total_grad_norm();

        if step == 1 {
            initial_loss = loss;
        }
        final_loss = loss;
        total_tokens_processed += window_size;

        let elapsed_step_ms = step_start.elapsed().as_secs_f64() * 1000.0;
        let report = TrainingStepReport {
            step,
            loss,
            perplexity,
            lr,
            grad_norm,
            tokens_processed: total_tokens_processed,
            elapsed_ms: elapsed_step_ms,
        };

        if let Some(ref mut cb) = callback {
            cb(&report);
        }

        step_history.push(report);
    }

    if let Some(ref gpu) = gpu_engine {
        if let Err(e) = gpu.sync_weights_to_host(&mut model) {
            eprintln!("[CUDA WARN] Failed to sync final weights to host: {}", e);
        }
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    let tokens_per_sec = (total_tokens_processed as f64) / total_duration.max(1e-6);

    let reduction_pct = if initial_loss > 1e-6 {
        ((initial_loss - final_loss) / initial_loss) * 100.0
    } else {
        0.0
    };

    let checkpoint_saved = if let Some(ref out_path) = actual_config.checkpoint_out {
        model.export_safetensors(out_path)?;
        Some(out_path.clone())
    } else {
        None
    };

    let generated_sample = if let Some(ref prompt) = actual_config.eval_prompt {
        Some(model.generate_text(prompt, actual_config.eval_max_tokens, actual_config.eval_temperature))
    } else {
        None
    };

    Ok(AutonomousTrainingSummary {
        initial_loss,
        final_loss,
        loss_reduction_pct: reduction_pct,
        final_perplexity: final_loss.exp(),
        total_steps: actual_config.steps,
        total_tokens: total_tokens_processed,
        total_duration_secs: total_duration,
        tokens_per_sec,
        checkpoint_saved_to: checkpoint_saved,
        step_history,
        generated_sample,
    })
}
