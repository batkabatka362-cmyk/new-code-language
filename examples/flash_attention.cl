// ============================================================================
// CRON Sovereign Multi-Head Attention & Zero-Word Tensor Calculus Example
// Level 1A Standard Mathematical Calculus
// ============================================================================

// 1. Define Query (Q), Key (K), and Value (V) Projection Matrices
Q = [[1.0, 0.0], [0.0, 1.0]]
K = [[1.0, 0.0], [0.0, 1.0]]
V = [[10.0, 20.0], [30.0, 40.0]]

// 2. Execute Zero-Word Scaled Dot-Product Attention:
//    Attn(Q, K, V) = softmax(Q @ K^T / sqrt(d)) @ V
//
// Q @ K^T = [[1.0, 0.0], [0.0, 1.0]]
// scaled by 1/sqrt(2) = 0.7071
// row-wise softmax:
//   row 0: [0.670, 0.330] -> @ V = [16.6, 26.6]
//   row 1: [0.330, 0.670] -> @ V = [23.4, 33.4]
attn_out = attention(Q, K, V)

// 3. Full Reduction Sum: 16.6 + 26.6 + 23.4 + 33.4 = 100.0
sum(attn_out)

