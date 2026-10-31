// ============================================================================
// CRON STANDARD MATHEMATICAL CALCULUS: MAMBA-2 SSD CORE
// libcl/math/mamba2_ssd.cl
// True Zero-Word Formulation: M(X, A, B, C, dt) = ssd_scan(X, A, B, C, dt)
// ============================================================================

// 1. True Zero-Word Mamba-2 Structured State Space Duality
M(X[T, D], A[D], B[D, N], C[D, N], dt[D]) = ssd_scan(X, A, B, C, dt)

// 2. Compatibility Aliases
mamba2_ssd_layer(X, A, B_proj, C_proj, dt) = M(X, A, B_proj, C_proj, dt)
