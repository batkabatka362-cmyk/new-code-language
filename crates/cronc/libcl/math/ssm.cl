// ============================================================================
// CRON STANDARD MATHEMATICAL CALCULUS: SELECTIVE STATE-SPACE MODEL (Mamba)
// libcl/math/ssm.cl
// True Zero-Word Formulation: h' = A * h + B * x, y = C * h' + D * x
// ============================================================================

// 1. Recurrent State Update
H(h, x, A, B) = A * h + B * x

// 2. Output Projection
Y(h_next, x, C, D) = C * h_next + D * x

// 3. Fused Step: [h', y]
F(h, x, A, B, C, D) = [A * h + B * x, C * (A * h + B * x) + D * x]

// Compatibility Aliases
ssm_state(h, x, A, B) = H(h, x, A, B)
ssm_out(h_next, x, C, D) = Y(h_next, x, C, D)
ssm_step(h, x, A, B, C, D) = F(h, x, A, B, C, D)
