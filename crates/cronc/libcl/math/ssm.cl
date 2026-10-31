// ============================================================================
// CRON STANDARD MATHEMATICAL LIBRARY: SELECTIVE STATE-SPACE MODEL (Mamba)
// libcl/math/ssm.cl
// Recurrent Hidden State Scan & Output Projection in Zero Verbal Keywords
// ============================================================================

// 1. Single-Step Continuous-to-Discrete State Update: h' = A * h + B * x
ssm_state(h: f32, x: f32, A: f32, B: f32) = A * h + B * x

// 2. State-Space Output Projection: y = C * h' + D * x
ssm_out(h_next: f32, x: f32, C: f32, D: f32) = C * h_next + D * x

// 3. Fused 1-Step State-Space Step
ssm_step(h: f32, x: f32, A: f32, B: f32, C: f32, D: f32) =
    [A * h + B * x, C * (A * h + B * x) + D * x]

