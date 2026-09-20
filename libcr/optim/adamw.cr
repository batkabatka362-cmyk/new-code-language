// ============================================================================
// CRON Standard Library: AdamW Optimizer with Decoupled Weight Decay
// Module: cron.optim.adamw
// Target: 256-Core 4D-Torus Photonic Neuromorphic Silicon & SIMD Accelerators
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

.MODULE cron.optim.adamw

// Configuration hyperparameters for AdamW
struct AdamWConfig {
    lr: f32,
    beta1: f32,
    beta2: f32,
    eps: f32,
    weight_decay: f32
}

// Per-parameter moment state accumulators (First moment m, Second moment v)
struct AdamWState {
    m: vec8f,
    v: vec8f,
    step: i32
}

// Scalar AdamW state for individual scalar weights
struct ScalarAdamWState {
    m: f32,
    v: f32,
    step: i32
}

// Initialize default AdamWConfig
def create_adamw_config(lr: f32, weight_decay: f32) -> AdamWConfig {
    return AdamWConfig {
        lr: lr,
        beta1: 0.9,
        beta2: 0.999,
        eps: 0.00000001,
        weight_decay: weight_decay
    }
}

// Initialize zeroed SIMD AdamW state
def create_adamw_state() -> AdamWState {
    let zero = simd_splat(0.0)
    return AdamWState {
        m: zero,
        v: zero,
        step: 0
    }
}

// Initialize zeroed Scalar AdamW state
def create_scalar_adamw_state() -> ScalarAdamWState {
    return ScalarAdamWState {
        m: 0.0,
        v: 0.0,
        step: 0
    }
}

// Cosine Annealing Learning Rate Schedule: lr_t = min_lr + 0.5 * (max_lr - min_lr) * (1 + cos(pi * t / T_max))
def cosine_annealing_lr(current_step: i32, total_steps: i32, max_lr: f32, min_lr: f32) -> f32 {
    if current_step >= total_steps {
        return min_lr
    }
    let ratio: f32 = (current_step as f32) / (total_steps as f32)
    // Approximate cosine wave: 1 - 2 * ratio^2
    let cos_approx: f32 = 1.0 - (2.0 * ratio * ratio)
    let lr_decay: f32 = min_lr + (0.5 * (max_lr - min_lr) * (1.0 + cos_approx))
    return lr_decay
}

// Vectorized SIMD AdamW Parameter Update Step:
//   m_t = beta1 * m_{t-1} + (1 - beta1) * g_t
//   v_t = beta2 * v_{t-1} + (1 - beta2) * g_t^2
//   param_t = param_{t-1} - lr * (m_hat / (sqrt(v_hat) + eps) + weight_decay * param_{t-1})
def adamw_step_simd(param: vec8f, grad_val: vec8f, state: AdamWState, cfg: AdamWConfig) -> vec8f {
    let next_step: i32 = state.step + 1

    let b1_vec: vec8f = simd_splat(cfg.beta1)
    let one_minus_b1: vec8f = simd_splat(1.0 - cfg.beta1)
    let b2_vec: vec8f = simd_splat(cfg.beta2)
    let one_minus_b2: vec8f = simd_splat(1.0 - cfg.beta2)
    let eps_vec: vec8f = simd_splat(cfg.eps)
    let lr_vec: vec8f = simd_splat(cfg.lr)
    let wd_vec: vec8f = simd_splat(cfg.weight_decay)

    // 1. Update biased first moment estimate: m_t = beta1 * m + (1 - beta1) * g
    let m_new: vec8f = (state.m * b1_vec) + (grad_val * one_minus_b1)

    // 2. Update biased second raw moment estimate: v_t = beta2 * v + (1 - beta2) * g^2
    let grad_sq: vec8f = grad_val * grad_val
    let v_new: vec8f = (state.v * b2_vec) + (grad_sq * one_minus_b2)

    // 3. Decoupled weight decay: param = param - (lr * weight_decay * param)
    let decayed_param: vec8f = param - (param * wd_vec * lr_vec)

    // 4. Direction update: m_hat / (sqrt(v_hat) + eps)
    let denom: vec8f = v_new + eps_vec
    let step_dir: vec8f = m_new / denom
    let update: vec8f = step_dir * lr_vec

    let final_param: vec8f = decayed_param - update
    return final_param
}

// Scalar AdamW Parameter Update Step
def adamw_step_scalar(param: f32, grad_val: f32, state: ScalarAdamWState, cfg: AdamWConfig) -> f32 {
    let next_step: i32 = state.step + 1

    // 1. Update first moment
    let m_new: f32 = (cfg.beta1 * state.m) + ((1.0 - cfg.beta1) * grad_val)

    // 2. Update second moment
    let v_new: f32 = (cfg.beta2 * state.v) + ((1.0 - cfg.beta2) * grad_val * grad_val)

    // 3. Decoupled weight decay
    let decayed: f32 = param - (cfg.lr * cfg.weight_decay * param)

    // 4. Update parameter
    let denom: f32 = v_new + cfg.eps
    let update: f32 = (m_new / denom) * cfg.lr

    return decayed - update
}
