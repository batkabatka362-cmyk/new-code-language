// ============================================================================
// CRON STANDARD MATHEMATICAL LIBRARY: MAMBA-2 SSD CORE
// libcl/math/mamba2_ssd.cl
// Structured State Space Duality (SSD) 1-D Recurrent Linear Attention
// ============================================================================

// 1. Mamba-2 State Space Duality Associative Scan:
// ssd_scan(X, A, B, C, dt)
mamba2_ssd_layer(X: f32[T, D], A: f32[D], B_proj: f32[D, N], C_proj: f32[D, N], dt: f32[D]) =
    ssd_scan(X, A, B_proj, C_proj, dt)

