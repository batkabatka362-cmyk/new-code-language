// ============================================================================
// CRON STANDARD MATHEMATICAL LIBRARY: RMSNORM
// libcl/math/rms_norm.cl
// Root Mean Square Layer Normalization (Zero-Verbal)
// ============================================================================

eps = 1e-5

// Standard RMSNorm for 1D / 2D activation tensors:
// rms_norm(X, gamma) = (X / sqrt(mean(X * X) + eps)) * gamma
rms_norm(X: f32[B, D], gamma: f32[D]) = rmsnorm(X, gamma)

// Scaled RMSNorm with custom epsilon parameter:
rms_norm_eps(X: f32[B, D], gamma: f32[D], e: f32) = (X / sqrt((X * X) + e)) * gamma
