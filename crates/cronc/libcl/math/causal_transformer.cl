// ============================================================================
// CRON STANDARD MATHEMATICAL LIBRARY: CAUSAL TRANSFORMER ATTENTION
// libcl/math/causal_transformer.cl
// Autoregressive Masked Multi-Head Attention Block
// ============================================================================

// 1. Causal Self-Attention with projection matrices:
causal_mha(X: f32[T, D], W_q: f32[D, D], W_k: f32[D, D], W_v: f32[D, D], W_o: f32[D, D], scale: f32) =
    (causal_attn(X @ W_q, X @ W_k, X @ W_v, scale)) @ W_o

// 2. Rotary Position Embedding Augmented Causal Attention:
causal_rope_mha(X: f32[T, D], W_q: f32[D, D], W_k: f32[D, D], W_v: f32[D, D], W_o: f32[D, D], scale: f32) =
    (causal_attn(rope(X @ W_q, 10000.0), rope(X @ W_k, 10000.0), X @ W_v, scale)) @ W_o
