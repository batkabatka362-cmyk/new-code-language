// ============================================================================
// CRON STANDARD MATHEMATICAL LIBRARY: ROTARY POSITION EMBEDDING (RoPE)
// libcl/math/rotary.cl
// 2D Complex / Clifford Phase Rotation in Zero Verbal Keywords
// ============================================================================

// 1. 2D Coordinate Rotation: [x1*cos - x2*sin, x1*sin + x2*cos]
rope_2d(x1: f32, x2: f32, cos_th: f32, sin_th: f32) =
    [x1 * cos_th - x2 * sin_th, x1 * sin_th + x2 * cos_th]

// 2. Complex Conjugate / Reverse Rotation
rope_inv_2d(x1: f32, x2: f32, cos_th: f32, sin_th: f32) =
    [x1 * cos_th + x2 * sin_th, -x1 * sin_th + x2 * cos_th]

