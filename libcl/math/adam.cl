// ============================================================================
// CRON STANDARD MATHEMATICAL LIBRARY: OPTIMIZER (AdamW)
// libcl/math/adam.cl
// First/Second Moment Estimator & Weight Decay Step in Zero Verbal Keywords
// ============================================================================

beta1 = 0.9
beta2 = 0.999
eps = 1e-8
lr = 0.001
wd = 0.01

// 1. First Moment Exponential Moving Average (Mean)
adam_m(m: f32, g: f32) = beta1 * m + (1.0 - beta1) * g

// 2. Second Moment Exponential Moving Average (Variance)
adam_v(v: f32, g: f32) = beta2 * v + (1.0 - beta2) * (g^2)

// 3. AdamW Parameter Step: w - lr * (m / (sqrt(v) + eps) + wd * w)
adam_step(w: f32, m: f32, v: f32) =
    w - lr * (m / (sqrt(v) + eps) + wd * w)

