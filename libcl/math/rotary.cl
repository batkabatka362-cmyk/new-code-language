// ============================================================================
// CRON STANDARD MATHEMATICAL CALCULUS: ROTARY POSITION EMBEDDING (RoPE)
// libcl/math/rotary.cl
// True Zero-Word Formulation: R(x1, x2, cosθ, sinθ) = [x1*cosθ - x2*sinθ, x1*sinθ + x2*cosθ]
// ============================================================================

// 1. 2D Coordinate Rotation
R(x1, x2, cos_th, sin_th) = [x1 * cos_th - x2 * sin_th, x1 * sin_th + x2 * cos_th]

// 2. Conjugate Inverse Rotation
R_inv(x1, x2, cos_th, sin_th) = [x1 * cos_th + x2 * sin_th, -x1 * sin_th + x2 * cos_th]

// Compatibility Aliases
rope_2d(x1, x2, cos_th, sin_th) = R(x1, x2, cos_th, sin_th)
rope_inv_2d(x1, x2, cos_th, sin_th) = R_inv(x1, x2, cos_th, sin_th)
