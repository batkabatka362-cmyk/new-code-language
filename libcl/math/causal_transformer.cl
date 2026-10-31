// ============================================================================
// CRON STANDARD MATHEMATICAL CALCULUS: CAUSAL TRANSFORMER ATTENTION
// libcl/math/causal_transformer.cl
// True Zero-Word Formulation: A(X, Wq, Wk, Wv, Wo, s) = (causal_attn(X @ Wq, X @ Wk, X @ Wv, s)) @ Wo
// ============================================================================

// 1. True Zero-Word Causal Masked Multi-Head Attention
A(X[T, D], Wq[D, D], Wk[D, D], Wv[D, D], Wo[D, D], s) =
    (causal_attn(X @ Wq, X @ Wk, X @ Wv, s)) @ Wo

// 2. Rotary Augmented Causal Attention
A_rope(X[T, D], Wq[D, D], Wk[D, D], Wv[D, D], Wo[D, D], s) =
    (causal_attn(rope(X @ Wq, 10000.0), rope(X @ Wk, 10000.0), X @ Wv, s)) @ Wo

// 3. Compatibility Aliases
causal_mha(X, W_q, W_k, W_v, W_o, scale) = A(X, W_q, W_k, W_v, W_o, scale)
causal_rope_mha(X, W_q, W_k, W_v, W_o, scale) = A_rope(X, W_q, W_k, W_v, W_o, scale)
