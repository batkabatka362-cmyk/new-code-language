// ============================================================================
// CRON STANDARD MATHEMATICAL LIBRARY: BITNET B1.58 TERNARY LAYER
// libcl/math/bitnet_layer.cl
// 1-bit / 1.58-bit Ternary {-1, 0, +1} Weights Zero-Multiply Linear Layer
// ============================================================================

// 1. BitLinear projection with RMSNorm and activation scaling:
bitnet_layer(X: f32[B, D], W_ternary: f32[D, M], gamma: f32) =
    bitlinear(rmsnorm(X), W_ternary, gamma)

// 2. Unnormalized raw BitLinear:
bitnet_raw(X: f32[B, D], W_ternary: f32[D, M], gamma: f32) =
    bitlinear(X, W_ternary, gamma)
