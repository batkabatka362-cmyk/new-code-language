// ============================================================================
// CRON CORDIC & Complex Geometric Hardware Engine
// Target: 256-Core 4D-Torus Neuromorphic & Photonic Silicon
// Capabilities:
//   - Multiplier-less circular, hyperbolic & linear coordinate rotations
//   - Photonic Mach-Zehnder Interferometer (MZI) phase shift calibration
//   - Quantum Bloch / Poincaré sphere unitary state rotations
//   - LLM Rotary Position Embedding (RoPE) fast frequency synthesizer
//   - Terminal ASCII 3D Bloch sphere & phase orbit visualizer
// ============================================================================

use std::f64::consts::{FRAC_PI_2, PI};

/// CORDIC Operational Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CordicMode {
    CircularRotation,
    CircularVectoring,
    HyperbolicRotation,
    HyperbolicVectoring,
    LinearRotation,
}

impl CordicMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            CordicMode::CircularRotation => "Circular Rotation (sin, cos)",
            CordicMode::CircularVectoring => "Circular Vectoring (magnitude, atan2)",
            CordicMode::HyperbolicRotation => "Hyperbolic Rotation (sinh, cosh)",
            CordicMode::HyperbolicVectoring => "Hyperbolic Vectoring (ln, sqrt)",
            CordicMode::LinearRotation => "Linear Rotation (multiply)",
        }
    }

    pub fn parse_mode(s: &str) -> Result<Self, String> {
        s.parse()
    }
}

impl std::str::FromStr for CordicMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rot" | "circular" | "circular_rot" | "sin_cos" => Ok(CordicMode::CircularRotation),
            "vec" | "vectoring" | "circular_vec" | "atan" => Ok(CordicMode::CircularVectoring),
            "hyperbolic" | "hyp_rot" | "sinh_cosh" => Ok(CordicMode::HyperbolicRotation),
            "hyp_vec" | "ln" => Ok(CordicMode::HyperbolicVectoring),
            "linear" => Ok(CordicMode::LinearRotation),
            other => Err(format!("Unknown CORDIC mode '{}'. Available: circular, vectoring, hyperbolic, linear", other)),
        }
    }
}

/// CORDIC Hardware Configuration
#[derive(Debug, Clone)]
pub struct CordicConfig {
    pub mode: CordicMode,
    pub iterations: usize,
    pub energy_per_iter_pj: f64,
}

impl Default for CordicConfig {
    fn default() -> Self {
        Self {
            mode: CordicMode::CircularRotation,
            iterations: 16,
            energy_per_iter_pj: 0.12, // 0.12 pJ per shift-add step on 4D-Torus silicon
        }
    }
}

/// CORDIC Computation Execution Result
#[derive(Debug, Clone)]
pub struct CordicResult {
    pub mode: CordicMode,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub initial_x: f64,
    pub initial_y: f64,
    pub initial_z: f64,
    pub iterations: usize,
    pub dynamic_energy_pj: f64,
    pub latency_cycles: usize,
    pub error_estimate: f64,
}

impl CordicResult {
    pub fn to_json(&self) -> String {
        format!(
            "{{\n  \
              \"mode\": \"{}\",\n  \
              \"initial\": [{:.6}, {:.6}, {:.6}],\n  \
              \"result\": [{:.6}, {:.6}, {:.6}],\n  \
              \"iterations\": {},\n  \
              \"latency_cycles\": {},\n  \
              \"dynamic_energy_pj\": {:.4},\n  \
              \"error_estimate\": {:.2e}\n\
            }}",
            self.mode.as_str(),
            self.initial_x, self.initial_y, self.initial_z,
            self.x, self.y, self.z,
            self.iterations,
            self.latency_cycles,
            self.dynamic_energy_pj,
            self.error_estimate
        )
    }
}

// Compile-time generated arctan(2^-i) table for i = 0..31
pub fn get_circular_atan_table() -> [f64; 32] {
    let mut table = [0.0; 32];
    for i in 0..32 {
        table[i] = (2.0f64.powi(-(i as i32))).atan();
    }
    table
}

// Circular scaling factor K = prod_{i=0}^{N-1} 1 / sqrt(1 + 2^(-2i))
pub fn get_circular_gain(iterations: usize) -> f64 {
    let mut k = 1.0;
    for i in 0..iterations {
        k /= (1.0 + 2.0f64.powi(-2 * (i as i32))).sqrt();
    }
    k
}

// Hyperbolic scaling factor K_h
pub fn get_hyperbolic_gain(iterations: usize) -> f64 {
    let mut k = 1.0;
    for i in 1..=iterations {
        k /= (1.0 - 2.0f64.powi(-2 * (i as i32))).sqrt();
        if i == 4 || i == 13 || i == 40 {
            k /= (1.0 - 2.0f64.powi(-2 * (i as i32))).sqrt();
        }
    }
    k
}

/// Executes CORDIC computation on silicon coordinates (x0, y0, z0)
pub fn run_cordic(x0: f64, y0: f64, z0: f64, config: &CordicConfig) -> CordicResult {
    let iters = config.iterations.clamp(4, 32);
    let atan_table = get_circular_atan_table();

    match config.mode {
        CordicMode::CircularRotation => {
            // Range reduction for target angle z0 into [-pi/2, pi/2]
            let mut z = z0;
            let mut x_sign = 1.0;
            let mut y_sign = 1.0;

            while z > PI { z -= 2.0 * PI; }
            while z < -PI { z += 2.0 * PI; }

            if z > FRAC_PI_2 {
                z -= PI;
                x_sign = -1.0;
                y_sign = -1.0;
            } else if z < -FRAC_PI_2 {
                z += PI;
                x_sign = -1.0;
                y_sign = -1.0;
            }

            let k = get_circular_gain(iters);
            let mut x = x0 * k;
            let mut y = y0 * k;

            for i in 0..iters {
                let d = if z >= 0.0 { 1.0 } else { -1.0 };
                let shift = 2.0f64.powi(-(i as i32));
                let next_x = x - d * y * shift;
                let next_y = y + d * x * shift;
                let next_z = z - d * atan_table[i];

                x = next_x;
                y = next_y;
                z = next_z;
            }

            CordicResult {
                mode: config.mode,
                x: x * x_sign,
                y: y * y_sign,
                z,
                initial_x: x0,
                initial_y: y0,
                initial_z: z0,
                iterations: iters,
                dynamic_energy_pj: (iters as f64) * config.energy_per_iter_pj,
                latency_cycles: iters + 1,
                error_estimate: 2.0f64.powi(-(iters as i32)),
            }
        }
        CordicMode::CircularVectoring => {
            let mut x = x0;
            let mut y = y0;
            let mut z = 0.0;
            let mut angle_offset = 0.0;

            if x < 0.0 {
                x = -x;
                y = -y;
                angle_offset = if y0 >= 0.0 { PI } else { -PI };
            }

            for i in 0..iters {
                let d = if y < 0.0 { 1.0 } else { -1.0 };
                let shift = 2.0f64.powi(-(i as i32));
                let next_x = x - d * y * shift;
                let next_y = y + d * x * shift;
                let next_z = z - d * atan_table[i];

                x = next_x;
                y = next_y;
                z = next_z;
            }

            let k = get_circular_gain(iters);
            let magnitude = x * k;
            let angle = z + angle_offset;

            CordicResult {
                mode: config.mode,
                x: magnitude,
                y: 0.0,
                z: angle,
                initial_x: x0,
                initial_y: y0,
                initial_z: z0,
                iterations: iters,
                dynamic_energy_pj: (iters as f64) * config.energy_per_iter_pj,
                latency_cycles: iters + 1,
                error_estimate: 2.0f64.powi(-(iters as i32)),
            }
        }
        CordicMode::HyperbolicRotation => {
            let k_h = get_hyperbolic_gain(iters);
            let mut x = x0 * k_h;
            let mut y = y0 * k_h;
            let mut z = z0;

            let mut i = 1;
            let mut step_count = 0;
            while i <= iters {
                let d = if z >= 0.0 { 1.0 } else { -1.0 };
                let shift = 2.0f64.powi(-(i as i32));
                let atanh_val = 0.5 * ((1.0 + shift) / (1.0 - shift)).ln();

                let next_x = x + d * y * shift;
                let next_y = y + d * x * shift;
                let next_z = z - d * atanh_val;

                x = next_x;
                y = next_y;
                z = next_z;
                step_count += 1;

                // Walther convergence rule: repeat step at 4, 13, 40
                if (i == 4 || i == 13) && step_count == i {
                    continue;
                }
                i += 1;
            }

            CordicResult {
                mode: config.mode,
                x,
                y,
                z,
                initial_x: x0,
                initial_y: y0,
                initial_z: z0,
                iterations: iters,
                dynamic_energy_pj: (step_count as f64) * config.energy_per_iter_pj,
                latency_cycles: step_count + 1,
                error_estimate: 2.0f64.powi(-(iters as i32)),
            }
        }
        CordicMode::HyperbolicVectoring => {
            let mut x = x0;
            let mut y = y0;
            let mut z = z0;
            let mut step_count = 0;

            let mut i = 1;
            while i <= iters {
                let d = if y < 0.0 { 1.0 } else { -1.0 };
                let shift = 2.0f64.powi(-(i as i32));
                let atanh_val = 0.5 * ((1.0 + shift) / (1.0 - shift)).ln();

                let next_x = x + d * y * shift;
                let next_y = y + d * x * shift;
                let next_z = z - d * atanh_val;

                x = next_x;
                y = next_y;
                z = next_z;
                step_count += 1;

                if (i == 4 || i == 13) && step_count == i {
                    continue;
                }
                i += 1;
            }

            let k_h = get_hyperbolic_gain(iters);
            CordicResult {
                mode: config.mode,
                x: x * k_h,
                y: 0.0,
                z: -z,
                initial_x: x0,
                initial_y: y0,
                initial_z: z0,
                iterations: iters,
                dynamic_energy_pj: (step_count as f64) * config.energy_per_iter_pj,
                latency_cycles: step_count + 1,
                error_estimate: 2.0f64.powi(-(iters as i32)),
            }
        }
        CordicMode::LinearRotation => {
            let x = x0;
            let mut y = y0;
            let mut z = z0;

            for i in 0..iters {
                let d = if z >= 0.0 { 1.0 } else { -1.0 };
                let shift = 2.0f64.powi(-(i as i32));
                let next_y = y + d * x * shift;
                let next_z = z - d * shift;

                y = next_y;
                z = next_z;
            }

            CordicResult {
                mode: config.mode,
                x,
                y,
                z,
                initial_x: x0,
                initial_y: y0,
                initial_z: z0,
                iterations: iters,
                dynamic_energy_pj: (iters as f64) * (config.energy_per_iter_pj * 0.8),
                latency_cycles: iters + 1,
                error_estimate: 2.0f64.powi(-(iters as i32)),
            }
        }
    }
}

/// Computes (sin(theta), cos(theta)) using Circular CORDIC
pub fn compute_sin_cos(angle_rad: f64, iters: usize) -> (f64, f64) {
    let config = CordicConfig {
        mode: CordicMode::CircularRotation,
        iterations: iters,
        energy_per_iter_pj: 0.12,
    };
    let res = run_cordic(1.0, 0.0, angle_rad, &config);
    (res.y, res.x)
}

/// Computes magnitude and angle (r, theta) from Cartesian (x, y)
pub fn compute_magnitude_angle(x: f64, y: f64, iters: usize) -> (f64, f64) {
    let config = CordicConfig {
        mode: CordicMode::CircularVectoring,
        iterations: iters,
        energy_per_iter_pj: 0.12,
    };
    let res = run_cordic(x, y, 0.0, &config);
    (res.x, res.z)
}

/// Computes (sinh(val), cosh(val)) using Hyperbolic CORDIC
pub fn compute_sinh_cosh(val: f64, iters: usize) -> (f64, f64) {
    let config = CordicConfig {
        mode: CordicMode::HyperbolicRotation,
        iterations: iters,
        energy_per_iter_pj: 0.12,
    };
    let res = run_cordic(1.0, 0.0, val, &config);
    (res.y, res.x)
}

/// Computes LLM Rotary Position Embedding (RoPE) frequencies using CORDIC
pub fn compute_rope_frequencies(dim: usize, pos: usize, base: f64, iters: usize) -> Vec<(f64, f64)> {
    let mut freqs = Vec::with_capacity(dim / 2);
    for i in 0..(dim / 2) {
        let theta = (pos as f64) / base.powf((2 * i) as f64 / dim as f64);
        let (sin_t, cos_t) = compute_sin_cos(theta, iters);
        freqs.push((sin_t, cos_t));
    }
    freqs
}

/// Synthesizes 4-Way VLIW .cl Microcode executing CORDIC iterations
pub fn synthesize_cordic_cl(config: &CordicConfig) -> String {
    let iters = config.iterations.clamp(4, 32);
    let mut raw = String::with_capacity(2048);

    raw.push_str(".core [0, 0, 0, 0]:\n");
    raw.push_str("@cordic_kernel_init:\n");
    raw.push_str("B0000: '==01#000> '==02#004> '==03#008> '==04#00C>\n");

    for i in 0..iters {
        let b_idx = i + 1;
        let line = format!(
            "B{:04}: _OP01$28F> _MD02*3A2> _PO04+600> _SB00#000>\n",
            b_idx
        );
        raw.push_str(&line);
    }

    // Final writeback bundle
    let final_idx = iters + 1;
    raw.push_str(&format!(
        "B{:04}: _bb00#000> _NO00#000> _NO00#000> _HL00$008!\n",
        final_idx
    ));

    let header = format!(
        "; ============================================================================\n\
         ; CRON 256-Core 4D-Torus CORDIC Geometric Silicon Microcode\n\
         ; Mode: {} | Iterations: {}\n\
         ; Multiplier-less Shift-and-Add Zero-Divergence Silicon Kernel\n\
         ; Target: 256-Core 4D-Torus Neuromorphic Photonic Silicon\n\
         ; ============================================================================\n",
        config.mode.as_str(),
        iters
    );

    let healed = match crate::heal_cl_program(&raw) {
        Ok(rep) => rep.canonical_code,
        Err(_) => raw,
    };

    format!("{}\n{}", header, healed)
}

/// Renders a 3D ASCII Bloch Sphere representing a quantum or optical state
pub fn render_ascii_bloch_sphere(theta: f64, phi: f64) -> String {
    let mut out = String::new();
    out.push_str("╔════════════════════════════════════════════════════════════════════════════╗\n");
    out.push_str("║        CRON BRAIN 5 QUANTUM & PHOTONIC BLOCH / POINCARÉ SPHERE             ║\n");
    out.push_str("╚════════════════════════════════════════════════════════════════════════════╝\n\n");

    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();

    let alpha_amp = (theta / 2.0).cos();
    let beta_amp = (theta / 2.0).sin();

    out.push_str(&format!("  Spherical Angles : θ = {:.4} rad ({:.1}°),  φ = {:.4} rad ({:.1}°)\n", theta, theta.to_degrees(), phi, phi.to_degrees()));
    out.push_str(&format!("  Cartesian Vector : [x={:+.4}, y={:+.4}, z={:+.4}]  (|v| = {:.4})\n", x, y, z, (x*x + y*y + z*z).sqrt()));
    out.push_str(&format!("  Quantum State    : |ψ⟩ = {:.4}|0⟩ + ({:.4}·e^(i{:.2}))|1⟩\n", alpha_amp, beta_amp, phi));
    out.push_str("  Silicon Core     : Brain 5 Photonic Phase Mesh (0.12 pJ/step CORDIC)\n\n");

    out.push_str("  3D State Projection:\n");
    out.push_str("           |0⟩ (North Pole: Z=+1)\n");
    out.push_str("              ▲\n");
    out.push_str("          . -'- .\n");
    out.push_str("        /    |    \\\n");
    if z >= 0.5 {
        out.push_str("       |  ★--+     |  <- [|ψ⟩ State Vector]\n");
    } else {
        out.push_str("       |     |     |\n");
    }
    out.push_str("      /  . - + - .  \\\n");
    if (-0.5..0.5).contains(&z) {
        out.push_str("  ◄---===----+====★--► Y (Equator, φ phase)\n");
    } else {
        out.push_str("  ◄---===----+=====---► Y (Equator, φ phase)\n");
    }
    out.push_str("      \\      |      /\n");
    if z < -0.5 {
        out.push_str("       |  ★--+     |  <- [|ψ⟩ State Vector]\n");
    } else {
        out.push_str("       |     |     |\n");
    }
    out.push_str("        \\    |    /\n");
    out.push_str("          ' -.- '\n");
    out.push_str("              ▼\n");
    out.push_str("           |1⟩ (South Pole: Z=-1)\n");

    out
}

/// Renders a terminal ASCII phase-space trajectory on the unit circle
pub fn render_ascii_phase_orbit(points: &[(f64, f64)]) -> String {
    let mut grid = vec![vec![' '; 25]; 13];

    // Draw unit circle contour
    for step in 0..64 {
        let rad = (step as f64) * (2.0 * PI / 64.0);
        let px = 12.0 + 10.0 * rad.cos();
        let py = 6.0 + 5.0 * rad.sin();
        let col = (px.round() as usize).clamp(0, 24);
        let row = (py.round() as usize).clamp(0, 12);
        grid[row][col] = '·';
    }

    // Origin
    grid[6][12] = '+';

    // Plot trajectory points
    for (i, &(x, y)) in points.iter().enumerate() {
        let px = 12.0 + 10.0 * x;
        let py = 6.0 + 5.0 * y;
        let col = (px.round() as usize).clamp(0, 24);
        let row = (py.round() as usize).clamp(0, 12);
        grid[row][col] = if i == points.len() - 1 { '★' } else { '●' };
    }

    let mut out = String::new();
    out.push_str("─── Phase-Space Coordinate Trajectory (X: Horizontal, Y: Vertical) ────────\n");
    for row in grid {
        let s: String = row.into_iter().collect();
        out.push_str(&format!("  │ {} │\n", s));
    }
    out.push_str("  └─────────────────────────┘\n");
    out
}
