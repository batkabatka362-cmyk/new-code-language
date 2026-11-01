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
use std::time::Instant;

use crate::bpe_tokenizer::BpeTokenizer;

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
    pub optimizer: TrainingOptimizer,
    pub weight_decay: f32,
    pub warmup_steps: usize,
    pub lr_schedule: LrSchedule,
    pub checkpoint_out: Option<String>,
    pub profile: String,
    pub seed: u64,
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
            optimizer: TrainingOptimizer::AdamW,
            weight_decay: 0.01,
            warmup_steps: 5,
            lr_schedule: LrSchedule::Cosine,
            checkpoint_out: Some("checkpoint.safetensors".to_string()),
            profile: "max-quality".to_string(),
            seed: 42,
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

    pub fn zero_grad(&mut self) {
        self.grad.fill(0.0);
    }

    pub fn grad_norm_sq(&self) -> f32 {
        self.grad.iter().map(|g| g * g).sum()
    }
}

/// Autonomous In-Silicon Neural Language Model Architecture
pub struct AutonomousLanguageModel {
    pub config: TrainingConfig,
    pub embedding: ParamMatrix, // [vocab_size, dim]
    pub w_gate: ParamMatrix,    // [dim, hidden_dim]
    pub w_up: ParamMatrix,      // [dim, hidden_dim]
    pub w_down: ParamMatrix,    // [hidden_dim, dim]
    pub w_head: ParamMatrix,    // [dim, vocab_size]
}

impl AutonomousLanguageModel {
    pub fn new(config: TrainingConfig) -> Self {
        let mut seed = config.seed;
        let d = config.dim;
        let h = config.hidden_dim;
        let v = config.vocab_size;

        let scale = (2.0 / (d as f32)).sqrt();
        let embedding = ParamMatrix::new_random(v, d, &mut seed, 0.02);
        let w_gate = ParamMatrix::new_random(d, h, &mut seed, scale);
        let w_up = ParamMatrix::new_random(d, h, &mut seed, scale);
        let w_down = ParamMatrix::new_random(h, d, &mut seed, (2.0 / (h as f32)).sqrt());
        let w_head = ParamMatrix::new_random(d, v, &mut seed, scale);

        Self {
            config,
            embedding,
            w_gate,
            w_up,
            w_down,
            w_head,
        }
    }

    pub fn zero_grad(&mut self) {
        self.embedding.zero_grad();
        self.w_gate.zero_grad();
        self.w_up.zero_grad();
        self.w_down.zero_grad();
        self.w_head.zero_grad();
    }

    pub fn total_grad_norm(&self) -> f32 {
        let sum_sq = self.embedding.grad_norm_sq()
            + self.w_gate.grad_norm_sq()
            + self.w_up.grad_norm_sq()
            + self.w_down.grad_norm_sq()
            + self.w_head.grad_norm_sq();
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

        // 1. Embedding lookup: H0 [N, d]
        let mut h0 = vec![0.0f32; n_tokens * d];
        for (i, &tok) in input_tokens.iter().enumerate() {
            let tok_clamped = tok % v;
            let src = &self.embedding.data[tok_clamped * d..(tok_clamped + 1) * d];
            h0[i * d..(i + 1) * d].copy_from_slice(src);
        }

        // 2. SwiGLU forward:
        // Gate: G = H0 @ W_gate [N, h]
        // Up:   U = H0 @ W_up   [N, h]
        let mut g = vec![0.0f32; n_tokens * h];
        let mut u = vec![0.0f32; n_tokens * h];
        for i in 0..n_tokens {
            for j in 0..h {
                let mut sum_g = 0.0f32;
                let mut sum_u = 0.0f32;
                for k in 0..d {
                    let h_val = h0[i * d + k];
                    sum_g += h_val * self.w_gate.data[k * h + j];
                    sum_u += h_val * self.w_up.data[k * h + j];
                }
                g[i * h + j] = sum_g;
                u[i * h + j] = sum_u;
            }
        }

        // SiLU(G) and SwiGLU activation A = SiLU(G) * U
        let mut silu_g = vec![0.0f32; n_tokens * h];
        let mut act = vec![0.0f32; n_tokens * h];
        for i in 0..(n_tokens * h) {
            let x = g[i];
            let sig = 1.0 / (1.0 + (-x).exp());
            let sg = x * sig;
            silu_g[i] = sg;
            act[i] = sg * u[i];
        }

        // 3. Down projection + Residual skip: H1 = A @ W_down + H0 [N, d]
        let mut h1 = vec![0.0f32; n_tokens * d];
        for i in 0..n_tokens {
            for j in 0..d {
                let mut sum_d = 0.0f32;
                for k in 0..h {
                    sum_d += act[i * h + k] * self.w_down.data[k * d + j];
                }
                h1[i * d + j] = sum_d + h0[i * d + j];
            }
        }

        // 4. Output Logits: Z = H1 @ W_head [N, v]
        let mut logits = vec![0.0f32; n_tokens * v];
        for i in 0..n_tokens {
            for j in 0..v {
                let mut sum_z = 0.0f32;
                for k in 0..d {
                    sum_z += h1[i * d + k] * self.w_head.data[k * v + j];
                }
                logits[i * v + j] = sum_z;
            }
        }

        // 5. Cross-Entropy Loss & Softmax probabilities
        let mut probs = vec![0.0f32; n_tokens * v];
        let mut total_loss = 0.0f32;

        for i in 0..n_tokens {
            let row = &logits[i * v..(i + 1) * v];
            let mut max_val = -1e9f32;
            for &val in row {
                if val > max_val {
                    max_val = val;
                }
            }

            let mut sum_exp = 0.0f32;
            for j in 0..v {
                let exp_val = (row[j] - max_val).exp();
                probs[i * v + j] = exp_val;
                sum_exp += exp_val;
            }

            let inv_sum = 1.0 / (sum_exp.max(1e-12));
            for j in 0..v {
                probs[i * v + j] *= inv_sum;
            }

            let tgt = target_tokens[i] % v;
            let prob_tgt = probs[i * v + tgt].max(1e-12);
            total_loss += -prob_tgt.ln();
        }

        let mean_loss = total_loss / (n_tokens as f32);
        let perplexity = mean_loss.exp();

        // 6. ANALYTICAL BACKWARD PASS (VJP Pullback)
        self.zero_grad();
        let scale_grad = 1.0 / (n_tokens as f32);

        // dLogits = (probs - 1_target) / N
        let mut d_logits = vec![0.0f32; n_tokens * v];
        for i in 0..n_tokens {
            let tgt = target_tokens[i] % v;
            for j in 0..v {
                let p = probs[i * v + j];
                let indicator = if j == tgt { 1.0f32 } else { 0.0f32 };
                d_logits[i * v + j] = (p - indicator) * scale_grad;
            }
        }

        // dW_head = H1^T @ dLogits [d, v]
        // dH1 = dLogits @ W_head^T [N, d]
        let mut d_h1 = vec![0.0f32; n_tokens * d];
        for k in 0..d {
            for j in 0..v {
                let mut sum = 0.0f32;
                for i in 0..n_tokens {
                    sum += h1[i * d + k] * d_logits[i * v + j];
                }
                self.w_head.grad[k * v + j] += sum;
            }
        }

        for i in 0..n_tokens {
            for k in 0..d {
                let mut sum = 0.0f32;
                for j in 0..v {
                    sum += d_logits[i * v + j] * self.w_head.data[k * v + j];
                }
                d_h1[i * d + k] = sum;
            }
        }

        // Residual skip: dH0_res = dH1
        // Down projection backward:
        // dAct = dH1 @ W_down^T [N, h]
        // dW_down = Act^T @ dH1 [h, d]
        let mut d_act = vec![0.0f32; n_tokens * h];
        for k in 0..h {
            for j in 0..d {
                let mut sum = 0.0f32;
                for i in 0..n_tokens {
                    sum += act[i * h + k] * d_h1[i * d + j];
                }
                self.w_down.grad[k * d + j] += sum;
            }
        }

        for i in 0..n_tokens {
            for k in 0..h {
                let mut sum = 0.0f32;
                for j in 0..d {
                    sum += d_h1[i * d + j] * self.w_down.data[k * d + j];
                }
                d_act[i * h + k] = sum;
            }
        }

        // SwiGLU elementwise backward:
        // Act = SiLU(G) * U
        // dU = dAct * SiLU(G)
        // dSiLU(G) = dAct * U
        // dG = dSiLU(G) * (sigma(G) * (1 + G * (1 - sigma(G))))
        let mut d_u = vec![0.0f32; n_tokens * h];
        let mut d_g = vec![0.0f32; n_tokens * h];
        for i in 0..(n_tokens * h) {
            let da = d_act[i];
            let sg = silu_g[i];
            d_u[i] = da * sg;

            let x = g[i];
            let sig = 1.0 / (1.0 + (-x).exp());
            let d_silu = sig * (1.0 + x * (1.0 - sig));
            d_g[i] = (da * u[i]) * d_silu;
        }

        // dW_up = H0^T @ dU [d, h]
        // dW_gate = H0^T @ dG [d, h]
        for k in 0..d {
            for j in 0..h {
                let mut sum_up = 0.0f32;
                let mut sum_gate = 0.0f32;
                for i in 0..n_tokens {
                    let h0_val = h0[i * d + k];
                    sum_up += h0_val * d_u[i * h + j];
                    sum_gate += h0_val * d_g[i * h + j];
                }
                self.w_up.grad[k * h + j] += sum_up;
                self.w_gate.grad[k * h + j] += sum_gate;
            }
        }

        // dH0_proj = dU @ W_up^T + dG @ W_gate^T
        // dH0 = dH0_res + dH0_proj
        let mut d_h0 = d_h1; // Start with residual connection gradient
        for i in 0..n_tokens {
            for k in 0..d {
                let mut sum_proj = 0.0f32;
                for j in 0..h {
                    sum_proj += d_u[i * h + j] * self.w_up.data[k * h + j]
                        + d_g[i * h + j] * self.w_gate.data[k * h + j];
                }
                d_h0[i * d + k] += sum_proj;
            }
        }

        // Embedding table backward:
        // dEmbedding[tok, :] += dH0[i, :]
        for (i, &tok) in input_tokens.iter().enumerate() {
            let tok_clamped = tok % v;
            let dst = &mut self.embedding.grad[tok_clamped * d..(tok_clamped + 1) * d];
            let src = &d_h0[i * d..(i + 1) * d];
            for k in 0..d {
                dst[k] += src[k];
            }
        }

        (mean_loss, perplexity)
    }

    /// Update all parameters using the selected optimizer and computed gradients
    pub fn optimizer_step(&mut self, step: usize, current_lr: f32) {
        let opt = self.config.optimizer;
        let wd = self.config.weight_decay;

        let params: [&mut ParamMatrix; 5] = [
            &mut self.embedding,
            &mut self.w_gate,
            &mut self.w_up,
            &mut self.w_down,
            &mut self.w_head,
        ];

        for param in params {
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
                        // Decoupled weight decay
                        param.data[i] -= current_lr * wd * param.data[i];

                        // First and second moment updates
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
                        // Decoupled weight decay
                        param.data[i] -= current_lr * wd * param.data[i];

                        // Lion update with sign direction
                        let c = beta1 * param.m[i] + (1.0 - beta1) * g;
                        let sign_c = if c > 0.0 {
                            1.0f32
                        } else if c < 0.0 {
                            -1.0f32
                        } else {
                            0.0f32
                        };

                        param.data[i] -= current_lr * sign_c;
                        // Momentum tracking
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
            name: &'a str,
            shape: Vec<usize>,
            data: &'a [f32],
        }

        let entries = vec![
            TensorEntry {
                name: "embedding",
                shape: vec![self.embedding.rows, self.embedding.cols],
                data: &self.embedding.data,
            },
            TensorEntry {
                name: "w_gate",
                shape: vec![self.w_gate.rows, self.w_gate.cols],
                data: &self.w_gate.data,
            },
            TensorEntry {
                name: "w_up",
                shape: vec![self.w_up.rows, self.w_up.cols],
                data: &self.w_up.data,
            },
            TensorEntry {
                name: "w_down",
                shape: vec![self.w_down.rows, self.w_down.cols],
                data: &self.w_down.data,
            },
            TensorEntry {
                name: "w_head",
                shape: vec![self.w_head.rows, self.w_head.cols],
                data: &self.w_head.data,
            },
        ];

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

        // Forward and backward gradient calculation
        let (loss, perplexity) = model.forward_backward_step(&input_batch, &target_batch);
        let grad_norm = model.total_grad_norm();

        // Parameter update
        model.optimizer_step(step, lr);

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
    })
}
