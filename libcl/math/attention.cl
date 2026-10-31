// ============================================================================
// CRON STANDARD MATHEMATICAL CALCULUS: ATTENTION
// libcl/math/attention.cl
// True Zero-Word Formulation: A(Q, K, V, s) = softmax((Q @ K) * s) @ V
// ============================================================================

s = 0.125
ε = 1e-6

// 1. Scaled Query-Key Product: (Q @ K) * s
Ω(Q, K, s) = (Q @ K) * s

// 2. Scaled Dot-Product Attention: Softmax(Ω(Q, K, s)) @ V
A(Q, K, V, s) = softmax((Q @ K) * s) @ V

// Compatibility Aliases
scale = s
eps = ε
qk_score(Q, K) = (Q @ K) * scale
attention(Q, K, V) = softmax((Q @ K) * scale) @ V
linear_out(A, Wo) = Wo @ A
mha_proj(x, Wq, Wk, Wv, Wo) = (attention(Wq @ x, Wk @ x, Wv @ x)) -> linear_out(Wo)
