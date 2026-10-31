// ============================================================================
// CRON STANDARD MATHEMATICAL CALCULUS: OPTIMIZER (AdamW)
// libcl/math/adam.cl
// True Zero-Word Formulation: θ' = θ - η * (m / (√(v) + ε) + λ * θ)
// ============================================================================

β1 = 0.9
β2 = 0.999
ε = 1e-8
η = 0.001
λ = 0.01

// 1. First Moment EMA: m' = β1 * m + (1 - β1) * g
m_(m, g) = β1 * m + (1.0 - β1) * g

// 2. Second Moment EMA: v' = β2 * v + (1 - β2) * (g^2)
v_(v, g) = β2 * v + (1.0 - β2) * (g^2)

// 3. AdamW Parameter Step
θ_(w, m, v) = w - η * (m / (√(v) + ε) + λ * w)

// Compatibility Aliases
beta1 = β1
beta2 = β2
eps = ε
lr = η
wd = λ
adam_m(m, g) = m_(m, g)
adam_v(v, g) = v_(v, g)
adam_step(w, m, v) = θ_(w, m, v)
