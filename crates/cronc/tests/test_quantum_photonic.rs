// ============================================================================
// Unit Tests for CRON Quantum-Photonic Co-Processor & Qubit Emulation Backend
// (test_quantum_photonic.rs)
// ============================================================================

use std::f64::consts::PI;
use cronc::cl_quantum::{
    run_quantum_simulation, Complex64, QuantumCircuit, QuantumGate, QuantumState,
};

#[test]
fn test_quantum_complex_arithmetic() {
    let a = Complex64::new(1.0, 2.0);
    let b = Complex64::new(3.0, 4.0);

    let sum = a.add(b);
    assert_eq!(sum.re, 4.0);
    assert_eq!(sum.im, 6.0);

    // (1 + 2i) * (3 + 4i) = (3 - 8) + (4 + 6)i = -5 + 10i
    let prod = a.mul(b);
    assert_eq!(prod.re, -5.0);
    assert_eq!(prod.im, 10.0);

    // Euler's formula: e^{i * pi} = -1
    let e_i_pi = Complex64::from_polar(1.0, PI);
    assert!((e_i_pi.re - (-1.0)).abs() < 1e-12);
    assert!(e_i_pi.im.abs() < 1e-12);
}

#[test]
fn test_quantum_hadamard_superposition() {
    let mut state = QuantumState::new(1);
    assert_eq!(state.probabilities(), vec![1.0, 0.0]);

    state.apply_gate(&QuantumGate::Hadamard { qubit: 0 });
    let probs = state.probabilities();

    assert!((probs[0] - 0.5).abs() < 1e-10, "P(|0>) must be 0.5");
    assert!((probs[1] - 0.5).abs() < 1e-10, "P(|1>) must be 0.5");

    let (rx, ry, rz) = state.bloch_vector(0);
    assert!((rx - 1.0).abs() < 1e-10, "<X> must be +1 for |+> state");
    assert!(ry.abs() < 1e-10);
    assert!(rz.abs() < 1e-10);
}

#[test]
fn test_quantum_bell_state_creation_and_entanglement() {
    let qc = QuantumCircuit::bell_pair();
    let (state, report) = run_quantum_simulation(&qc);

    let probs = state.probabilities();
    // |Phi+> = (|00> + |11>) / sqrt(2)
    assert!((probs[0] - 0.5).abs() < 1e-10, "P(|00>) must be 0.5");
    assert!(probs[1] < 1e-12, "P(|01>) must be 0");
    assert!(probs[2] < 1e-12, "P(|10>) must be 0");
    assert!((probs[3] - 0.5).abs() < 1e-10, "P(|11>) must be 0.5");

    // Maximal entanglement: von Neumann entropy S = ln(2) ~= 0.693147
    let expected_entropy = 2.0f64.ln();
    assert!(
        (report.entanglement_entropy - expected_entropy).abs() < 1e-4,
        "Bell state entanglement entropy must be ln(2) = {:.4}, got {:.4}",
        expected_entropy, report.entanglement_entropy
    );
}

#[test]
fn test_quantum_ghz_state_three_qubit() {
    let qc = QuantumCircuit::ghz_state();
    let (state, report) = run_quantum_simulation(&qc);

    let probs = state.probabilities();
    assert_eq!(probs.len(), 8);
    assert!((probs[0] - 0.5).abs() < 1e-10, "P(|000>) must be 0.5");
    assert!((probs[7] - 0.5).abs() < 1e-10, "P(|111>) must be 0.5");

    // All intermediate basis states should have 0 probability
    for (i, &p) in probs.iter().enumerate().take(7).skip(1) {
        assert!(p < 1e-12, "P(|{:03b}>) must be 0", i);
    }

    assert_eq!(report.num_qubits, 3);
    assert_eq!(report.total_gates, 3);
}

#[test]
fn test_quantum_pauli_and_phase_gates() {
    let mut state = QuantumState::new(1);

    // Pauli X: NOT gate
    state.apply_gate(&QuantumGate::PauliX { qubit: 0 });
    assert!((state.probabilities()[1] - 1.0).abs() < 1e-12, "X |0> = |1>");

    // Pauli Z on |1> -> -|1>
    state.apply_gate(&QuantumGate::PauliZ { qubit: 0 });
    assert!((state.amplitudes[1].re - (-1.0)).abs() < 1e-12);
    assert!((state.probabilities()[1] - 1.0).abs() < 1e-12);

    // Phase S gate
    let mut state2 = QuantumState::new(1);
    state2.apply_gate(&QuantumGate::Hadamard { qubit: 0 });
    state2.apply_gate(&QuantumGate::PhaseS { qubit: 0 });
    let (rx, ry, rz) = state2.bloch_vector(0);
    assert!(rx.abs() < 1e-10);
    assert!((ry - 1.0).abs() < 1e-10, "<Y> must be +1 for S |+>");
    assert!(rz.abs() < 1e-10);
}

#[test]
fn test_quantum_rotations_rx_ry_rz() {
    let mut state = QuantumState::new(1);
    // Ry(pi) on |0> rotates to |1>
    state.apply_gate(&QuantumGate::RotY { qubit: 0, theta: PI });
    let probs = state.probabilities();
    assert!((probs[1] - 1.0).abs() < 1e-10);

    // Rz(pi) adds relative phase
    state.apply_gate(&QuantumGate::RotZ { qubit: 0, theta: PI });
    assert!((state.probabilities()[1] - 1.0).abs() < 1e-10);
}

#[test]
fn test_quantum_beam_splitter_mzi_equivalence() {
    let mut state = QuantumState::new(2);
    // Prep mode 1 into |1> so mode 1 has photon
    state.apply_gate(&QuantumGate::PauliX { qubit: 1 });

    // 50:50 MZI Beam Splitter with theta = pi/2, phi = 0
    state.apply_gate(&QuantumGate::BeamSplitter {
        q1: 0,
        q2: 1,
        theta: PI / 2.0,
        phi: 0.0,
    });

    let probs = state.probabilities();
    // Mode 0 and mode 1 each have 50% probability
    assert!((probs[1] - 0.5).abs() < 1e-10, "MZI Beam splitter mode 0 must be 0.5");
    assert!((probs[2] - 0.5).abs() < 1e-10, "MZI Beam splitter mode 1 must be 0.5");
}

#[test]
fn test_quantum_qft_unitary() {
    let mut qc = QuantumCircuit::new(3);
    qc.x(0); // input |001>
    qc.qft(vec![0, 1, 2]);

    let (state, _) = run_quantum_simulation(&qc);
    let probs = state.probabilities();

    // QFT of any basis state produces a completely uniform probability distribution across all 2^N states
    for (i, &p) in probs.iter().enumerate() {
        assert!((p - 0.125).abs() < 1e-10, "QFT basis state {} must have probability 1/8", i);
    }
}

#[test]
fn test_quantum_projective_measurement_collapse() {
    let mut state = QuantumState::new(1);
    state.apply_gate(&QuantumGate::Hadamard { qubit: 0 });

    let (outcome, prob) = state.measure_qubit(0, 0x1234_5678);
    assert!((prob - 0.5).abs() < 1e-10);

    // After collapse, state must be an eigenstate with 100% certainty
    let probs = state.probabilities();
    if outcome {
        assert!((probs[1] - 1.0).abs() < 1e-10);
        assert!(probs[0] < 1e-10);
    } else {
        assert!((probs[0] - 1.0).abs() < 1e-10);
        assert!(probs[1] < 1e-10);
    }
}

#[test]
fn test_quantum_fidelity_metric() {
    let state1 = QuantumState::new(2);
    let mut state2 = QuantumState::new(2);

    // Identical states have fidelity 1.0
    assert!((state1.fidelity(&state2) - 1.0).abs() < 1e-12);

    // Orthogonal states have fidelity 0.0
    state2.apply_gate(&QuantumGate::PauliX { qubit: 0 });
    assert!(state1.fidelity(&state2) < 1e-12);
}

#[test]
fn test_quantum_simulation_report_json_and_hud() {
    let qc = QuantumCircuit::bell_pair();
    let (_, report) = run_quantum_simulation(&qc);

    let hud = report.render_ascii_hud();
    assert!(hud.contains("CRON QUANTUM-PHOTONIC CO-PROCESSOR"));
    assert!(hud.contains("Entanglement Entropy:"));
    assert!(hud.contains("BLOCH SPHERE COORDINATES"));

    let json = report.to_json();
    assert!(json.contains("\"num_qubits\": 2"));
    assert!(json.contains("\"entanglement_entropy\":"));
    assert!(json.contains("\"bloch_vectors\":"));
    assert!(json.contains("\"probabilities\":"));
}
