// ============================================================================
// CRON STANDARD MATHEMATICAL CALCULUS: RMSNORM
// libcl/math/rms_norm.cl
// True Zero-Word Formulation: N(X, γ) = (X / √(μ(X * X) + ε)) * γ
// ============================================================================

ε = 1e-5

// 1. True Zero-Word Root Mean Square Layer Normalization
N(X[B, D], γ[D]) = rmsnorm(X, γ, ε)

// 2. Pure Mathematical Formulation
N_pure(X[B, D], γ[D]) = (X / (μ(X * X) + ε)^0.5) * γ

// 3. Compatibility Aliases
rms_norm(X, gamma) = rmsnorm(X, gamma, ε)
rms_norm_eps(X, gamma, e) = rmsnorm(X, gamma, e)
