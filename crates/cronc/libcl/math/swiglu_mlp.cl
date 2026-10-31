// ============================================================================
// CRON STANDARD MATHEMATICAL CALCULUS: SWIGLU MLP
// libcl/math/swiglu_mlp.cl
// True Zero-Word Formulation: S(X, W1, W2, W3) = ((X @ W1 * σ(X @ W1)) * (X @ W2)) @ W3
// ============================================================================

// 1. True Zero-Word SwiGLU Gated Feed-Forward Network
S(X[B, D], W1[D, H], W2[D, H], W3[H, D]) = fused_swiglu(X, W1, W2, W3)

// 2. Pure Mathematical Formulation
S_pure(X[B, D], W1[D, H], W2[D, H], W3[H, D]) = ((X @ W1 * σ(X @ W1)) * (X @ W2)) @ W3

// 3. Compatibility Aliases
swiglu(X, W_gate, W_up, W_down) = fused_swiglu(X, W_gate, W_up, W_down)
swiglu_fused(X, W_gate, W_up, W_down) = fused_swiglu(X, W_gate, W_up, W_down)
