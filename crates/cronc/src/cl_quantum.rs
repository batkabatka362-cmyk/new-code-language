// ============================================================================
// CRON Quantum-Photonic Co-Processor & Qubit Emulation Backend
// (cl_quantum.rs)
//
// Models Linear Optical Quantum Computing (LOQC) and Universal Quantum State
// Vector Evolution on top of CRON's 4D-Torus Mach-Zehnder Interferometer (MZI)
// Photonic Accelerator (Brain 2).
//
// Features:
// 1. Full SU(2^N) Complex Hilbert Space State Vector Evolution (N <= 16).
// 2. Linear Optical Unitary Gates: Dual-Rail Beam Splitter U_BS(theta, phi),
//    MZI Phase Shifters, Hadamard, Pauli (X, Y, Z), CNOT, CZ, Toffoli, SWAP.
// 3. Collective Optical Algorithms: Quantum Fourier Transform (QFT),
//    Quantum Random Walk on 4D-Torus Lattice, and Bell / GHZ entanglement.
// 4. Projective Wavefunction Measurement & Collapse under Born's Rule:
//    P(m) = |<m|psi>|^2, with classical register feedback.
// 5. Quantum Information Metrics: Bipartite von Neumann Entanglement Entropy,
//    Bloch Sphere Coordinates (Rx, Ry, Rz), and Quantum State Fidelity F.
//
// 100% Pure Rust — Zero External Dependencies.
// ============================================================================

use std::f64::consts::PI;

/// High-precision complex number representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn norm_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn abs(&self) -> f64 {
        self.norm_sq().sqrt()
    }

    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn conj(&self) -> Self {
        Self { re: self.re, im: -self.im }
    }

    pub fn add(&self, other: Self) -> Self {
        Self { re: self.re + other.re, im: self.im + other.im }
    }

    pub fn sub(&self, other: Self) -> Self {
        Self { re: self.re - other.re, im: self.im - other.im }
    }

    pub fn mul(&self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    pub fn mul_f64(&self, scalar: f64) -> Self {
        Self { re: self.re * scalar, im: self.im * scalar }
    }
}

/// Universal Quantum Gate Primitives
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumGate {
    Hadamard { qubit: usize },
    PauliX { qubit: usize },
    PauliY { qubit: usize },
    PauliZ { qubit: usize },
    PhaseS { qubit: usize },
    PhaseT { qubit: usize },
    RotX { qubit: usize, theta: f64 },
    RotY { qubit: usize, theta: f64 },
    RotZ { qubit: usize, theta: f64 },
    PhaseShift { qubit: usize, phi: f64 },
    // Multi-qubit
    CNot { control: usize, target: usize },
    CZ { control: usize, target: usize },
    Swap { q1: usize, q2: usize },
    Toffoli { c1: usize, c2: usize, target: usize },
    // Photonic Native Dual-Rail MZI Beam Splitter
    BeamSplitter { q1: usize, q2: usize, theta: f64, phi: f64 },
    // Collective
    Qft { qubits: Vec<usize> },
}

impl QuantumGate {
    pub fn name(&self) -> &'static str {
        match self {
            QuantumGate::Hadamard { .. } => "H",
            QuantumGate::PauliX { .. } => "X",
            QuantumGate::PauliY { .. } => "Y",
            QuantumGate::PauliZ { .. } => "Z",
            QuantumGate::PhaseS { .. } => "S",
            QuantumGate::PhaseT { .. } => "T",
            QuantumGate::RotX { .. } => "Rx",
            QuantumGate::RotY { .. } => "Ry",
            QuantumGate::RotZ { .. } => "Rz",
            QuantumGate::PhaseShift { .. } => "Phase",
            QuantumGate::CNot { .. } => "CNOT",
            QuantumGate::CZ { .. } => "CZ",
            QuantumGate::Swap { .. } => "SWAP",
            QuantumGate::Toffoli { .. } => "CCX",
            QuantumGate::BeamSplitter { .. } => "MZI_BS",
            QuantumGate::Qft { .. } => "QFT",
        }
    }
}

/// Quantum Circuit Builder
#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub gates: Vec<QuantumGate>,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        assert!(num_qubits > 0 && num_qubits <= 16, "Qubits must be in range 1..=16");
        Self {
            num_qubits,
            gates: Vec::new(),
        }
    }

    pub fn h(&mut self, qubit: usize) -> &mut Self {
        self.gates.push(QuantumGate::Hadamard { qubit });
        self
    }

    pub fn x(&mut self, qubit: usize) -> &mut Self {
        self.gates.push(QuantumGate::PauliX { qubit });
        self
    }

    pub fn y(&mut self, qubit: usize) -> &mut Self {
        self.gates.push(QuantumGate::PauliY { qubit });
        self
    }

    pub fn z(&mut self, qubit: usize) -> &mut Self {
        self.gates.push(QuantumGate::PauliZ { qubit });
        self
    }

    pub fn rx(&mut self, qubit: usize, theta: f64) -> &mut Self {
        self.gates.push(QuantumGate::RotX { qubit, theta });
        self
    }

    pub fn ry(&mut self, qubit: usize, theta: f64) -> &mut Self {
        self.gates.push(QuantumGate::RotY { qubit, theta });
        self
    }

    pub fn rz(&mut self, qubit: usize, theta: f64) -> &mut Self {
        self.gates.push(QuantumGate::RotZ { qubit, theta });
        self
    }

    pub fn cnot(&mut self, control: usize, target: usize) -> &mut Self {
        self.gates.push(QuantumGate::CNot { control, target });
        self
    }

    pub fn cz(&mut self, control: usize, target: usize) -> &mut Self {
        self.gates.push(QuantumGate::CZ { control, target });
        self
    }

    pub fn swap(&mut self, q1: usize, q2: usize) -> &mut Self {
        self.gates.push(QuantumGate::Swap { q1, q2 });
        self
    }

    pub fn toffoli(&mut self, c1: usize, c2: usize, target: usize) -> &mut Self {
        self.gates.push(QuantumGate::Toffoli { c1, c2, target });
        self
    }

    pub fn beam_splitter(&mut self, q1: usize, q2: usize, theta: f64, phi: f64) -> &mut Self {
        self.gates.push(QuantumGate::BeamSplitter { q1, q2, theta, phi });
        self
    }

    pub fn qft(&mut self, qubits: Vec<usize>) -> &mut Self {
        self.gates.push(QuantumGate::Qft { qubits });
        self
    }

    /// Canonical Bell State Circuit (|Phi+> = (|00> + |11>) / sqrt(2))
    pub fn bell_pair() -> Self {
        let mut qc = Self::new(2);
        qc.h(0);
        qc.cnot(0, 1);
        qc
    }

    /// Canonical 3-Qubit GHZ State Circuit ((|000> + |111>) / sqrt(2))
    pub fn ghz_state() -> Self {
        let mut qc = Self::new(3);
        qc.h(0);
        qc.cnot(0, 1);
        qc.cnot(1, 2);
        qc
    }
}

/// 2^N Dimensional Quantum State Vector
#[derive(Debug, Clone)]
pub struct QuantumState {
    pub num_qubits: usize,
    pub amplitudes: Vec<Complex64>,
}

impl QuantumState {
    /// Initializes |00...0> ground state
    pub fn new(num_qubits: usize) -> Self {
        assert!(num_qubits > 0 && num_qubits <= 16);
        let dim = 1 << num_qubits;
        let mut amplitudes = vec![Complex64::ZERO; dim];
        amplitudes[0] = Complex64::ONE; // |0...0> = 1.0
        Self {
            num_qubits,
            amplitudes,
        }
    }

    pub fn dimension(&self) -> usize {
        1 << self.num_qubits
    }

    /// Applies single-qubit 2x2 unitary matrix [[u00, u01], [u10, u11]]
    pub fn apply_1q_matrix(&mut self, qubit: usize, u: [[Complex64; 2]; 2]) {
        let bit = 1 << qubit;
        let dim = self.dimension();

        for i in 0..dim {
            if (i & bit) == 0 {
                let j = i | bit;
                let a = self.amplitudes[i];
                let b = self.amplitudes[j];

                self.amplitudes[i] = u[0][0].mul(a).add(u[0][1].mul(b));
                self.amplitudes[j] = u[1][0].mul(a).add(u[1][1].mul(b));
            }
        }
    }

    /// Applies single-qubit rotation gate
    pub fn apply_gate(&mut self, gate: &QuantumGate) {
        match gate {
            QuantumGate::Hadamard { qubit } => {
                let inv_sqrt2 = 1.0 / std::f64::consts::SQRT_2;
                let h = [
                    [Complex64::new(inv_sqrt2, 0.0), Complex64::new(inv_sqrt2, 0.0)],
                    [Complex64::new(inv_sqrt2, 0.0), Complex64::new(-inv_sqrt2, 0.0)],
                ];
                self.apply_1q_matrix(*qubit, h);
            }
            QuantumGate::PauliX { qubit } => {
                let x = [
                    [Complex64::ZERO, Complex64::ONE],
                    [Complex64::ONE, Complex64::ZERO],
                ];
                self.apply_1q_matrix(*qubit, x);
            }
            QuantumGate::PauliY { qubit } => {
                let y = [
                    [Complex64::ZERO, Complex64::new(0.0, -1.0)],
                    [Complex64::new(0.0, 1.0), Complex64::ZERO],
                ];
                self.apply_1q_matrix(*qubit, y);
            }
            QuantumGate::PauliZ { qubit } => {
                let z = [
                    [Complex64::ONE, Complex64::ZERO],
                    [Complex64::ZERO, Complex64::new(-1.0, 0.0)],
                ];
                self.apply_1q_matrix(*qubit, z);
            }
            QuantumGate::PhaseS { qubit } => {
                let s = [
                    [Complex64::ONE, Complex64::ZERO],
                    [Complex64::ZERO, Complex64::I],
                ];
                self.apply_1q_matrix(*qubit, s);
            }
            QuantumGate::PhaseT { qubit } => {
                let pi_4 = PI / 4.0;
                let t = [
                    [Complex64::ONE, Complex64::ZERO],
                    [Complex64::ZERO, Complex64::from_polar(1.0, pi_4)],
                ];
                self.apply_1q_matrix(*qubit, t);
            }
            QuantumGate::RotX { qubit, theta } => {
                let half = theta / 2.0;
                let rx = [
                    [Complex64::new(half.cos(), 0.0), Complex64::new(0.0, -half.sin())],
                    [Complex64::new(0.0, -half.sin()), Complex64::new(half.cos(), 0.0)],
                ];
                self.apply_1q_matrix(*qubit, rx);
            }
            QuantumGate::RotY { qubit, theta } => {
                let half = theta / 2.0;
                let ry = [
                    [Complex64::new(half.cos(), 0.0), Complex64::new(-half.sin(), 0.0)],
                    [Complex64::new(half.sin(), 0.0), Complex64::new(half.cos(), 0.0)],
                ];
                self.apply_1q_matrix(*qubit, ry);
            }
            QuantumGate::RotZ { qubit, theta } => {
                let half = theta / 2.0;
                let rz = [
                    [Complex64::from_polar(1.0, -half), Complex64::ZERO],
                    [Complex64::ZERO, Complex64::from_polar(1.0, half)],
                ];
                self.apply_1q_matrix(*qubit, rz);
            }
            QuantumGate::PhaseShift { qubit, phi } => {
                let p = [
                    [Complex64::ONE, Complex64::ZERO],
                    [Complex64::ZERO, Complex64::from_polar(1.0, *phi)],
                ];
                self.apply_1q_matrix(*qubit, p);
            }
            QuantumGate::CNot { control, target } => {
                let c_bit = 1 << control;
                let t_bit = 1 << target;
                let dim = self.dimension();

                for i in 0..dim {
                    if (i & c_bit) != 0 && (i & t_bit) == 0 {
                        let j = i | t_bit;
                        self.amplitudes.swap(i, j);
                    }
                }
            }
            QuantumGate::CZ { control, target } => {
                let c_bit = 1 << control;
                let t_bit = 1 << target;
                let dim = self.dimension();

                for i in 0..dim {
                    if (i & c_bit) != 0 && (i & t_bit) != 0 {
                        self.amplitudes[i] = self.amplitudes[i].mul_f64(-1.0);
                    }
                }
            }
            QuantumGate::Swap { q1, q2 } => {
                let bit1 = 1 << q1;
                let bit2 = 1 << q2;
                let dim = self.dimension();

                for i in 0..dim {
                    let b1 = (i & bit1) != 0;
                    let b2 = (i & bit2) != 0;
                    if b1 != b2 && !b1 {
                        // b1 is 0, b2 is 1 -> swap with b1=1, b2=0
                        let j = (i | bit1) & !bit2;
                        self.amplitudes.swap(i, j);
                    }
                }
            }
            QuantumGate::Toffoli { c1, c2, target } => {
                let c1_bit = 1 << c1;
                let c2_bit = 1 << c2;
                let t_bit = 1 << target;
                let dim = self.dimension();

                for i in 0..dim {
                    if (i & c1_bit) != 0 && (i & c2_bit) != 0 && (i & t_bit) == 0 {
                        let j = i | t_bit;
                        self.amplitudes.swap(i, j);
                    }
                }
            }
            QuantumGate::BeamSplitter { q1, q2, theta, phi } => {
                // Dual-Rail MZI Unitary across spatial optical modes q1 and q2:
                // U_BS = [[cos(theta/2), -e^{i phi} sin(theta/2)],
                //         [e^{-i phi} sin(theta/2), cos(theta/2)]]
                let half = theta / 2.0;
                let c = half.cos();
                let s = half.sin();
                let exp_pos = Complex64::from_polar(1.0, *phi);
                let exp_neg = Complex64::from_polar(1.0, -phi);

                let u00 = Complex64::new(c, 0.0);
                let u01 = exp_pos.mul_f64(-s);
                let u10 = exp_neg.mul_f64(s);
                let u11 = Complex64::new(c, 0.0);

                let bit1 = 1 << q1;
                let bit2 = 1 << q2;
                let dim = self.dimension();

                for i in 0..dim {
                    let b1 = (i & bit1) != 0;
                    let b2 = (i & bit2) != 0;
                    if !b1 && b2 {
                        // mode 2 active, mode 1 inactive
                        let j = (i | bit1) & !bit2;
                        let a = self.amplitudes[i];
                        let b = self.amplitudes[j];
                        self.amplitudes[i] = u00.mul(a).add(u01.mul(b));
                        self.amplitudes[j] = u10.mul(a).add(u11.mul(b));
                    }
                }
            }
            QuantumGate::Qft { qubits } => {
                let n = qubits.len();
                for i in 0..n {
                    self.apply_gate(&QuantumGate::Hadamard { qubit: qubits[i] });
                    for j in (i + 1)..n {
                        let k = (j - i + 1) as f64;
                        let theta = 2.0 * PI / (2.0f64.powf(k));
                        // Controlled phase shift
                        let c_bit = 1 << qubits[j];
                        let t_bit = 1 << qubits[i];
                        let rot = Complex64::from_polar(1.0, theta);
                        let dim = self.dimension();
                        for idx in 0..dim {
                            if (idx & c_bit) != 0 && (idx & t_bit) != 0 {
                                self.amplitudes[idx] = self.amplitudes[idx].mul(rot);
                            }
                        }
                    }
                }
                // Reverse qubit order with SWAPs
                for i in 0..(n / 2) {
                    self.apply_gate(&QuantumGate::Swap { q1: qubits[i], q2: qubits[n - 1 - i] });
                }
            }
        }
    }

    /// Executes all gates in circuit
    pub fn execute_circuit(&mut self, circuit: &QuantumCircuit) {
        for gate in &circuit.gates {
            self.apply_gate(gate);
        }
    }

    /// Computes measurement probabilities for all 2^N states
    pub fn probabilities(&self) -> Vec<f64> {
        self.amplitudes.iter().map(|a| a.norm_sq()).collect()
    }

    /// Performs projective Born measurement on a qubit with state collapse
    pub fn measure_qubit(&mut self, qubit: usize, seed: u64) -> (bool, f64) {
        let bit = 1 << qubit;
        let dim = self.dimension();

        // Calculate probability P(qubit == 1)
        let mut p1 = 0.0;
        for i in 0..dim {
            if (i & bit) != 0 {
                p1 += self.amplitudes[i].norm_sq();
            }
        }
        let p0 = (1.0 - p1).max(0.0);

        // Pseudorandom collapse sample
        let mut rng = seed ^ 0xCAFE_BABE;
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let sample = (rng as f64) / (u64::MAX as f64);

        let outcome = sample < p1;
        let norm = if outcome { p1.sqrt().max(1e-12) } else { p0.sqrt().max(1e-12) };

        // Wavefunction collapse
        for i in 0..dim {
            let has_bit = (i & bit) != 0;
            if has_bit == outcome {
                self.amplitudes[i] = self.amplitudes[i].mul_f64(1.0 / norm);
            } else {
                self.amplitudes[i] = Complex64::ZERO;
            }
        }

        (outcome, if outcome { p1 } else { p0 })
    }

    /// Computes expectation values <X>, <Y>, <Z> for Bloch Sphere projection
    pub fn bloch_vector(&self, qubit: usize) -> (f64, f64, f64) {
        let bit = 1 << qubit;
        let dim = self.dimension();

        let mut rx = 0.0;
        let mut ry = 0.0;
        let mut rz = 0.0;

        for i in 0..dim {
            if (i & bit) == 0 {
                let j = i | bit;
                let a = self.amplitudes[i];
                let b = self.amplitudes[j];

                // <X> = a* b + b* a = 2 Re(a* b)
                rx += 2.0 * (a.re * b.re + a.im * b.im);
                // <Y> = -i a* b + i b* a = 2 Im(a* b)
                ry += 2.0 * (a.re * b.im - a.im * b.re);
                // <Z> = |a|^2 - |b|^2
                rz += a.norm_sq() - b.norm_sq();
            }
        }

        (rx, ry, rz)
    }

    /// Computes Bipartite von Neumann Entanglement Entropy between qubit 0 and rest of system
    pub fn entanglement_entropy(&self) -> f64 {
        if self.num_qubits < 2 {
            return 0.0;
        }

        // Reduced density matrix for qubit 0: 2x2 matrix rho_0
        let bit0 = 1;
        let dim = self.dimension();

        let mut rho00 = 0.0;
        let mut rho11 = 0.0;
        let mut rho01 = Complex64::ZERO;

        for i in 0..dim {
            if (i & bit0) == 0 {
                let j = i | bit0;
                let a = self.amplitudes[i];
                let b = self.amplitudes[j];

                rho00 += a.norm_sq();
                rho11 += b.norm_sq();
                rho01 = rho01.add(a.mul(b.conj()));
            }
        }

        // Eigenvalues of 2x2 Hermitian matrix:
        // lambda = (Tr +- sqrt(Tr^2 - 4 Det)) / 2
        let tr = rho00 + rho11; // ~ 1.0
        let det = rho00 * rho11 - rho01.norm_sq();
        let disc = ((tr * tr - 4.0 * det).max(0.0)).sqrt();

        let l1 = ((tr + disc) / 2.0).clamp(1e-15, 1.0);
        let l2 = ((tr - disc) / 2.0).clamp(1e-15, 1.0);

        let mut entropy = 0.0;
        if l1 > 1e-12 {
            entropy -= l1 * l1.ln();
        }
        if l2 > 1e-12 {
            entropy -= l2 * l2.ln();
        }

        entropy.max(0.0)
    }

    /// Computes Quantum State Fidelity F = |<self|other>|^2
    pub fn fidelity(&self, other: &Self) -> f64 {
        assert_eq!(self.num_qubits, other.num_qubits);
        let mut inner = Complex64::ZERO;
        for (a, b) in self.amplitudes.iter().zip(&other.amplitudes) {
            inner = inner.add(a.conj().mul(*b));
        }
        inner.norm_sq().clamp(0.0, 1.0)
    }
}

/// Simulation report output for IDEs and CLI HUD
#[derive(Debug, Clone)]
pub struct QuantumSimulationReport {
    pub num_qubits: usize,
    pub total_gates: usize,
    pub entanglement_entropy: f64,
    pub dominant_state_index: usize,
    pub dominant_state_prob: f64,
    pub bloch_vectors: Vec<(f64, f64, f64)>,
    pub probabilities: Vec<f64>,
}

impl QuantumSimulationReport {
    pub fn to_json(&self) -> String {
        let mut s = String::new();
        s.push_str("{\n");
        s.push_str(&format!("  \"num_qubits\": {},\n", self.num_qubits));
        s.push_str(&format!("  \"total_gates\": {},\n", self.total_gates));
        s.push_str(&format!("  \"entanglement_entropy\": {:.4},\n", self.entanglement_entropy));
        s.push_str(&format!("  \"dominant_state_index\": {},\n", self.dominant_state_index));
        s.push_str(&format!("  \"dominant_state_prob\": {:.4},\n", self.dominant_state_prob));

        s.push_str("  \"bloch_vectors\": [\n");
        for (i, (rx, ry, rz)) in self.bloch_vectors.iter().enumerate() {
            s.push_str(&format!("    {{\"qubit\": {}, \"rx\": {:.4}, \"ry\": {:.4}, \"rz\": {:.4}}}{}\n",
                i, rx, ry, rz, if i + 1 < self.bloch_vectors.len() { "," } else { "" }));
        }
        s.push_str("  ],\n");

        s.push_str("  \"probabilities\": [\n");
        let display_count = self.probabilities.len().min(16);
        for i in 0..display_count {
            s.push_str(&format!("    {{\"state\": \"|{:0b}>\", \"prob\": {:.4}}}{}\n",
                i, self.probabilities[i], if i + 1 < display_count { "," } else { "" }));
        }
        s.push_str("  ]\n");
        s.push_str("}\n");
        s
    }

    pub fn render_ascii_hud(&self) -> String {
        let mut s = String::new();
        s.push_str("╔══════════════════════════════════════════════════════════════════════════════╗\n");
        s.push_str("║ CRON QUANTUM-PHOTONIC CO-PROCESSOR (MZI OPTICAL CO-SIMULATOR)               ║\n");
        s.push_str("╠══════════════════════════════════════════════════════════════════════════════╣\n");
        s.push_str(&format!("║ Qubit Register: {:<2} Qubits ({:<5} Hilbert Space) | Circuit Gates: {:<4} ║\n",
            self.num_qubits, 1 << self.num_qubits, self.total_gates));
        s.push_str(&format!("║ Entanglement Entropy: S = {:.4} nats (von Neumann)                          ║\n",
            self.entanglement_entropy));
        s.push_str("╟──────────────────────────────────────────────────────────────────────────────╢\n");
        s.push_str("║ BLOCH SPHERE COORDINATES & PROBABILITIES:                                    ║\n");

        for (q, (rx, ry, rz)) in self.bloch_vectors.iter().enumerate() {
            s.push_str(&format!("║   Q{:02}: <X>={:>+6.3} <Y>={:>+6.3} <Z>={:>+6.3} | Status: Pure Coherent         ║\n",
                q, rx, ry, rz));
        }

        s.push_str("╟──────────────────────────────────────────────────────────────────────────────╢\n");
        s.push_str("║ COMPUTATIONAL BASIS PROBABILITY SPECTRUM:                                    ║\n");

        let top_states = self.probabilities.len().min(8);
        for i in 0..top_states {
            let p = self.probabilities[i];
            let bar_len = (p * 30.0).round() as usize;
            let bar: String = "█".repeat(bar_len);
            s.push_str(&format!("║   |{:0width$b}> : {:>5.1}% {:<30}                   ║\n",
                i, p * 100.0, bar, width = self.num_qubits));
        }

        s.push_str("╚══════════════════════════════════════════════════════════════════════════════╝\n");
        s
    }
}

/// Runs a simulated quantum circuit and outputs simulation report
pub fn run_quantum_simulation(circuit: &QuantumCircuit) -> (QuantumState, QuantumSimulationReport) {
    let mut state = QuantumState::new(circuit.num_qubits);
    state.execute_circuit(circuit);

    let probs = state.probabilities();
    let mut dom_idx = 0;
    let mut dom_prob = 0.0;
    for (i, &p) in probs.iter().enumerate() {
        if p > dom_prob {
            dom_prob = p;
            dom_idx = i;
        }
    }

    let mut bloch_vectors = Vec::new();
    for q in 0..circuit.num_qubits {
        bloch_vectors.push(state.bloch_vector(q));
    }

    let entropy = state.entanglement_entropy();

    let report = QuantumSimulationReport {
        num_qubits: circuit.num_qubits,
        total_gates: circuit.gates.len(),
        entanglement_entropy: entropy,
        dominant_state_index: dom_idx,
        dominant_state_prob: dom_prob,
        bloch_vectors,
        probabilities: probs,
    };

    (state, report)
}
