// ============================================================================
// CRON STANDARD MATHEMATICAL CALCULUS: BITNET B1.58 TERNARY LAYER
// libcl/math/bitnet_layer.cl
// True Zero-Word Formulation: B(X, W, γ) = bitlinear(rmsnorm(X), W, γ)
// ============================================================================

// 1. True Zero-Word BitLinear with RMSNorm
B(X[B, D], W[D, M], γ) = bitlinear(rmsnorm(X), W, γ)

// 2. Unnormalized raw BitLinear
B_raw(X[B, D], W[D, M], γ) = bitlinear(X, W, γ)

// 3. Compatibility Aliases
bitnet_layer(X, W_ternary, gamma) = B(X, W_ternary, gamma)
bitnet_raw(X, W_ternary, gamma) = B_raw(X, W_ternary, gamma)
