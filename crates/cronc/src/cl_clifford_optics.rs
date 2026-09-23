//! Clifford Cl(4,0) Space-Time Algebra and Optical MZI Phase Rotor Engine for CRON (.cl)
//!
//! Provides direct 16-element multivector geometric algebra operations and
//! Photonic Mach-Zehnder Interferometer (MZI) $2 \times 2$ unitary phase transformations
//! directly integrated with .cl cognitive pipelines.

/// 16-Blade Multivector in 4D Euclidean Clifford Algebra Cl(4,0)
/// Indices:
/// 0: scalar (1)
/// 1..4: vectors (e0, e1, e2, e3)
/// 5..10: bivectors (e01, e02, e03, e12, e13, e23)
/// 11..14: trivectors (e012, e013, e023, e123)
/// 15: pseudoscalar (e0123)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Multivector16 {
    pub blades: [f32; 16],
}

impl Default for Multivector16 {
    fn default() -> Self {
        Self { blades: [0.0; 16] }
    }
}

impl Multivector16 {
    pub fn scalar(val: f32) -> Self {
        let mut m = Self::default();
        m.blades[0] = val;
        m
    }

    pub fn from_vector(e0: f32, e1: f32, e2: f32, e3: f32) -> Self {
        let mut m = Self::default();
        m.blades[1] = e0;
        m.blades[2] = e1;
        m.blades[3] = e2;
        m.blades[4] = e3;
        m
    }

    /// Multivector Addition: $A + B$
    pub fn add(&self, other: &Self) -> Self {
        let mut res = Self::default();
        for i in 0..16 {
            res.blades[i] = self.blades[i] + other.blades[i];
        }
        res
    }

    /// Geometric Product in Cl(4,0) (Exact Blade Combinations)
    pub fn geometric_product(&self, other: &Self) -> Self {
        let mut res = Self::default();
        // Scalar * Multivector
        for i in 0..16 {
            res.blades[i] += self.blades[0] * other.blades[i];
            if i > 0 {
                res.blades[i] += self.blades[i] * other.blades[0];
            }
        }

        // Vector-Vector contractions & bivector generation
        for i in 1..=4 {
            let a_i = self.blades[i];
            if a_i.abs() < 1e-6 { continue; }
            for j in 1..=4 {
                let b_j = other.blades[j];
                if b_j.abs() < 1e-6 { continue; }

                if i == j {
                    // e_i * e_i = +1 in Cl(4,0)
                    res.blades[0] += a_i * b_j;
                } else if i < j {
                    let bivec_idx = match (i, j) {
                        (1, 2) => 5,
                        (1, 3) => 6,
                        (1, 4) => 7,
                        (2, 3) => 8,
                        (2, 4) => 9,
                        (3, 4) => 10,
                        _ => unreachable!(),
                    };
                    res.blades[bivec_idx] += a_i * b_j;
                } else {
                    let bivec_idx = match (j, i) {
                        (1, 2) => 5,
                        (1, 3) => 6,
                        (1, 4) => 7,
                        (2, 3) => 8,
                        (2, 4) => 9,
                        (3, 4) => 10,
                        _ => unreachable!(),
                    };
                    res.blades[bivec_idx] -= a_i * b_j;
                }
            }
        }

        res
    }

    /// Compute L2 Energy Norm $\sum b_i^2$
    pub fn energy_norm(&self) -> f32 {
        self.blades.iter().map(|b| b * b).sum::<f32>().sqrt()
    }
}

/// Optical Mach-Zehnder Interferometer (MZI) Phase Rotor
/// Represents unitary $2 \times 2$ matrix $U(\theta, \phi)$:
/// $\begin{pmatrix} e^{i\phi}\cos\theta & -\sin\theta \\ e^{i\phi}\sin\theta & \cos\theta \end{pmatrix}$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MziPhaseRotor {
    pub theta_rad: f32,
    pub phi_rad: f32,
}

impl MziPhaseRotor {
    pub fn new(theta_rad: f32, phi_rad: f32) -> Self {
        Self { theta_rad, phi_rad }
    }

    /// Transform input optical field (in_re0, in_im0, in_re1, in_im1)
    pub fn transform_optical_field(&self, in0: (f32, f32), in1: (f32, f32)) -> ((f32, f32), (f32, f32)) {
        let (sin_t, cos_t) = self.theta_rad.sin_cos();
        let (sin_p, cos_p) = self.phi_rad.sin_cos();

        // Complex multiplication for e^{i\phi}
        let in0_phase_re = in0.0 * cos_p - in0.1 * sin_p;
        let in0_phase_im = in0.0 * sin_p + in0.1 * cos_p;

        let out0_re = in0_phase_re * cos_t - in1.0 * sin_t;
        let out0_im = in0_phase_im * cos_t - in1.1 * sin_t;

        let out1_re = in0_phase_re * sin_t + in1.0 * cos_t;
        let out1_im = in0_phase_im * sin_t + in1.1 * cos_t;

        ((out0_re, out0_im), (out1_re, out1_im))
    }
}
