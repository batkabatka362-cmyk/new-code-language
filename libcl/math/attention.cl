// ============================================================================
// CRON STANDARD MATHEMATICAL LIBRARY: ATTENTION
// libcl/math/attention.cl
// Scaled Dot-Product & Flash-Attention Calculus in Zero Verbal Keywords
// ============================================================================

scale = 0.125
eps = 1e-6

// 1. Scaled Query-Key Matrix Product
qk_score(Q: f16, K: f16) = (Q @ K) * scale

// 2. Scaled Dot-Product Attention: Softmax(Q @ K * scale) @ V
attention(Q: f16, K: f16, V: f16) = softmax((Q @ K) * scale) @ V

// 3. Multi-Head Projection with Bias
mha_proj(x: f16, Wq: f16, Wk: f16, Wv: f16, Wo: f16) =
    (attention(Wq @ x, Wk @ x, Wv @ x)) -> linear_out(Wo)

linear_out(A: f16, Wo: f16) = Wo @ A

