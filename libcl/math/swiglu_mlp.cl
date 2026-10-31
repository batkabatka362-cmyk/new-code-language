// ============================================================================
// CRON STANDARD MATHEMATICAL LIBRARY: SWIGLU MLP
// libcl/math/swiglu_mlp.cl
// LLaMA / Mistral / DeepSeek SwiGLU Gated Feed-Forward Network
// ============================================================================

// 1. Direct Fused SwiGLU projection:
// (silu(X @ W_gate) * (X @ W_up)) @ W_down
swiglu(X: f32[B, D], W_gate: f32[D, H], W_up: f32[D, H], W_down: f32[H, D]) =
    (silu(X @ W_gate) * (X @ W_up)) @ W_down

// 2. Fused Linear SwiGLU kernel invocation:
swiglu_fused(X: f32[B, D], W_gate: f32[D, H], W_up: f32[D, H], W_down: f32[H, D]) =
    fused_swiglu(X, W_gate, W_up, W_down)

