// ============================================================================
// CRON Non-Conventional 4D Clifford Topological Capabilities Test Suite
// File: crates/cronc/tests/test_cl_topological_capabilities.rs
//
// Verifies all 6 non-conventional paradigms natively implemented in Rust:
// 1. 128-Bit Topological Coordinate Register (TCA)
// 2. Discrete Non-Euclidean Clifford Kinematics & Braid Deflector
// 3. Zero-Anchor Homeostasis & Parity Rotation (Hamiltonian H == 100%)
// 4. 10-10-10-10 Base-94 VLIW Microcode Bundle
// 5. Wave Interference Attention (Zero-GEMM, Zero-Softmax)
// 6. Unified Topological Runtime End-to-End Execution
// ============================================================================

use cronc::cl_topological::{
    CliffordKinematicsEngine, CliffordVliwBundle, SnakeState, TopologicalHydraulicUnit,
    TopologicalUniverse128, TransitionMode, UnifiedTopologicalRuntime, WaveInterferenceAttention,
};

#[test]
fn test_128bit_register_universe_packing_and_verilog_hex() {
    let snakes = [
        SnakeState::new(0, TransitionMode::Wrap, 12, 45, 89, 1, 25),
        SnakeState::new(1, TransitionMode::Tunnel, 60, 10, 20, 2, 25),
        SnakeState::new(2, TransitionMode::Wrap, 30, 5, 55, 0, 25),
        SnakeState::new(3, TransitionMode::Anchor, 0, 0, 0, 3, 25),
    ];

    let universe = TopologicalUniverse128::pack_snakes(&snakes);
    let hex_rom = universe.to_verilog_hex();
    assert_eq!(hex_rom.len(), 32, "Verilog ROM hex string must be exactly 32 chars (128 bits)");

    let (low64, high64) = universe.to_c_simd_u64();
    assert!(low64 != 0 && high64 != 0, "SIMD words must contain packed coordinates & volumes");

    // Lossless Round-Trip Unpack
    let unpacked = universe.unpack_snakes();
    for i in 0..4 {
        assert_eq!(unpacked[i].w, snakes[i].w, "Snake {} W mismatch", i);
        assert_eq!(unpacked[i].x, snakes[i].x, "Snake {} X mismatch", i);
        assert_eq!(unpacked[i].z, snakes[i].z, "Snake {} Z mismatch", i);
        assert_eq!(unpacked[i].spin, snakes[i].spin, "Snake {} Spin mismatch", i);
        assert_eq!(unpacked[i].vol, snakes[i].vol, "Snake {} Volume mismatch", i);
    }
}

#[test]
fn test_clifford_kinematics_dual_modes_and_braid_deflector() {
    let mut engine = CliffordKinematicsEngine::new();

    // Mode A (Full Orbital Wrap): 100 steps on surface before radial step
    for _ in 0..100 {
        engine.step_full_wrap_mode(0, 1);
    }
    assert_eq!(engine.w[0], 21, "After 100 orbital steps, radial depth W should advance from 20 to 21");

    // Mode B (Direct Radial Tunneling): Direct O(1) step
    engine.step_direct_tunnel_mode(1, 5, 2, 3);
    assert_eq!(engine.w[1], 45, "Direct radial tunnel should advance W from 40 to 45");

    // Braid Deflector Deterministic Test: Force 2 snakes onto identical 4D coordinate
    engine.x[0] = 50; engine.z[0] = 50; engine.w[0] = 30;
    engine.x[1] = 50; engine.z[1] = 50; engine.w[1] = 30;
    let def_count = engine.apply_braid_deflector();
    assert_eq!(def_count, 1, "Braid deflector must trigger exactly once for surface collision");
    assert_ne!(engine.w[0], engine.w[1], "Braid deflector must separate radial depth (|W0 - W1| >= 1)");

    // Braid Deflector: 20,000 stress cycles with varied trajectories and zero collisions
    let mut total_deflections = def_count;
    for t in 0..20_000 {
        // Varied snake directions ensuring frequent trajectory crossings
        for k in 0..4 {
            let dw = match (t + k) % 3 { 0 => 1, 1 => -1, _ => 0 };
            let dx = ((k as i8 * 7 + (t as i8 % 11)) % 5) as i8;
            let dz = (((k as i8 * 13) + (t as i8 % 7)) % 4) as i8;
            engine.step_direct_tunnel_mode(k, dw, dx, dz);
        }

        total_deflections += engine.apply_braid_deflector();

        // Formal verification: no 4D collision
        for i in 0..4 {
            for j in (i + 1)..4 {
                let collision = engine.x[i] == engine.x[j]
                    && engine.z[i] == engine.z[j]
                    && engine.w[i] == engine.w[j];
                assert!(!collision, "Collision detected at cycle {} between snake {} and {}", t, i, j);
            }
        }
    }
    assert!(total_deflections > 0, "Braid deflector must have intervened to resolve potential collisions");
}

#[test]
fn test_zero_anchor_homeostasis_and_parity_rotation() {
    let mut hydraulics = TopologicalHydraulicUnit::new();

    // 4 active: 25% each
    hydraulics.set_activity([true, true, true, true]);
    assert_eq!(hydraulics.volumes, [25, 25, 25, 25]);
    assert_eq!(hydraulics.get_hamiltonian(), 100);

    // 2 active: 50% each
    hydraulics.set_activity([true, false, true, false]);
    assert_eq!(hydraulics.volumes, [50, 0, 50, 0]);
    assert_eq!(hydraulics.get_hamiltonian(), 100);

    // 1 active: 100% singular focus
    hydraulics.set_activity([false, false, false, true]);
    assert_eq!(hydraulics.volumes, [0, 0, 0, 100]);
    assert_eq!(hydraulics.get_hamiltonian(), 100);

    // 3 active: deterministic parity rotation [34,33,33] -> [33,34,33] -> [33,33,34]
    hydraulics.set_activity([true, true, true, false]);
    assert_eq!(hydraulics.volumes, [34, 33, 33, 0]);
    assert_eq!(hydraulics.get_hamiltonian(), 100);

    hydraulics.tick_parity();
    assert_eq!(hydraulics.volumes, [33, 34, 33, 0]);
    assert_eq!(hydraulics.get_hamiltonian(), 100);

    hydraulics.tick_parity();
    assert_eq!(hydraulics.volumes, [33, 33, 34, 0]);
    assert_eq!(hydraulics.get_hamiltonian(), 100);

    hydraulics.tick_parity();
    assert_eq!(hydraulics.volumes, [34, 33, 33, 0]);
    assert_eq!(hydraulics.get_hamiltonian(), 100);

    // Fluidic exchange conservation
    hydraulics.fluidic_exchange(0, 1, 10);
    assert_eq!(hydraulics.get_hamiltonian(), 100);
}

#[test]
fn test_vliw_base94_microcode_parsing_and_synthesis() {
    let line = "B0001:W124589125T601020225W300555025A000000325";
    let bundle = CliffordVliwBundle::from_vliw_line(line).expect("VLIW bundle parsing should succeed");

    assert_eq!(bundle.label, "B0001");
    assert_eq!(bundle.snakes[0].mode, TransitionMode::Wrap);
    assert_eq!(bundle.snakes[0].w, 12);
    assert_eq!(bundle.snakes[0].x, 45);
    assert_eq!(bundle.snakes[0].z, 89);
    assert_eq!(bundle.snakes[0].spin, 1);
    assert_eq!(bundle.snakes[0].vol, 25);

    assert_eq!(bundle.snakes[1].mode, TransitionMode::Tunnel);
    assert_eq!(bundle.snakes[1].w, 60);

    // Round-trip serialize
    let serialized = bundle.to_vliw_line();
    assert_eq!(serialized, line);

    // Hamiltonian violation rejection
    let bad_line = "B0002:W102030050W102030050W102030020A000000000"; // sum is 120%
    assert!(CliffordVliwBundle::from_vliw_line(bad_line).is_err());
}

#[test]
fn test_wave_interference_attention_zero_gemm() {
    let snakes = [
        SnakeState::new(0, TransitionMode::Wrap, 10, 20, 30, 0, 25), // 0°
        SnakeState::new(1, TransitionMode::Wrap, 10, 22, 30, 0, 25), // 0° (in-phase, close)
        SnakeState::new(2, TransitionMode::Wrap, 10, 20, 30, 2, 25), // 180° (anti-phase notch)
        SnakeState::new(3, TransitionMode::Wrap, 80, 90, 90, 1, 25), // 90° (far away)
    ];

    let att = WaveInterferenceAttention::compute_attention_matrix(&snakes);

    for i in 0..4 {
        let row_sum: u8 = att[i].iter().sum();
        assert_eq!(row_sum, 100, "Attention row {} sum must be exactly 100%", i);
    }

    // Snake 0 -> Snake 1 is in-phase and close, must resonate strongly
    assert!(att[0][1] > 15, "In-phase close snake must have strong resonance weight");

    // Snake 0 -> Snake 2 is exactly anti-phase (180°), must experience destructive notch cancellation
    assert_eq!(att[0][2], 0, "Anti-phase (180°) must be completely cancelled to 0%");
}

#[test]
fn test_unified_topological_runtime_end_to_end() {
    let mut runtime = UnifiedTopologicalRuntime::new();

    let bundle0 = CliffordVliwBundle::from_vliw_line("B0000:W102030025W102230025T405060125A000000025").unwrap();
    let bundle1 = CliffordVliwBundle::from_vliw_line("B0001:W112131134W112331233T415161333A000000000").unwrap();

    runtime.load_bundle(&bundle0);
    for _ in 0..5 {
        let telem = runtime.step_cycle();
        assert_eq!(telem.hamiltonian, 100, "Hamiltonian must be conserved at 100%");
    }

    runtime.load_bundle(&bundle1);
    for _ in 0..6 {
        let telem = runtime.step_cycle();
        assert_eq!(telem.hamiltonian, 100, "Hamiltonian must be conserved at 100%");
    }

    assert_eq!(runtime.cycle_count, 11);
}

#[test]
fn test_phase_token_engine_and_3byte_kv_cache() {
    use cronc::cl_topological::{CorticalLayerKind, PhaseTokenEngine};

    let text = "CRON4D";
    let tokens = PhaseTokenEngine::encode_string(text, CorticalLayerKind::Layer4Sensory);
    assert_eq!(tokens.len(), 6);

    for tok in &tokens {
        // 3 bytes total
        let bytes = tok.to_bytes();
        assert_eq!(bytes.len(), 3);
        assert!(tok.depth_w >= 41 && tok.depth_w <= 60, "Must reside in Layer IV Sensory depth");
    }

    // Identical tokens produce maximum resonance
    let res_self = PhaseTokenEngine::compute_resonance(&tokens[0], &tokens[0]);
    assert_eq!(res_self, 100, "Self-resonance must be 100%");

    // Cross-token resonance is bounded in 0..100
    let res_cross = PhaseTokenEngine::compute_resonance(&tokens[0], &tokens[1]);
    assert!(res_cross <= 100);
}

#[test]
fn test_cortical_layer_mapping_and_clamping() {
    use cronc::cl_topological::CorticalLayerKind;

    assert_eq!(CorticalLayerKind::from_w(5), CorticalLayerKind::Layer1Apical);
    assert_eq!(CorticalLayerKind::from_w(25), CorticalLayerKind::Layer23Lateral);
    assert_eq!(CorticalLayerKind::from_w(50), CorticalLayerKind::Layer4Sensory);
    assert_eq!(CorticalLayerKind::from_w(75), CorticalLayerKind::Layer5Motor);
    assert_eq!(CorticalLayerKind::from_w(95), CorticalLayerKind::Layer6Predictive);

    let l4 = CorticalLayerKind::Layer4Sensory;
    assert_eq!(l4.clamp_w(10), 41);
    assert_eq!(l4.clamp_w(55), 55);
    assert_eq!(l4.clamp_w(90), 60);
}

#[test]
fn test_reversible_fsm8_computation() {
    use cronc::cl_topological::{ReversibleFsm8, SnakeState, TransitionMode};

    let mut fsm = ReversibleFsm8::new();
    assert_eq!(fsm.state, 0);

    // 8 steps forward returns to initial state (T^8 = I)
    for expected in 1..=8 {
        let s = fsm.step_forward();
        assert_eq!(s, expected % 8);
    }
    assert_eq!(fsm.state, 0, "T^8 must equal Identity (state 0)");

    // Step backward is exact inverse: T^-1 = T^7
    let s_back = fsm.step_backward();
    assert_eq!(s_back, 7);

    // Permutation of snakes
    let mut snakes = [
        SnakeState::new(0, TransitionMode::Wrap, 10, 20, 30, 0, 25),
        SnakeState::new(1, TransitionMode::Wrap, 11, 21, 31, 1, 25),
        SnakeState::new(2, TransitionMode::Wrap, 12, 22, 32, 2, 25),
        SnakeState::new(3, TransitionMode::Wrap, 13, 23, 33, 3, 25),
    ];
    fsm.state = 1;
    fsm.permute_snakes(&mut snakes);
    assert_eq!(snakes[0].w, 11, "Snake 0 should now have coordinates of previous Snake 1");
}

#[test]
fn test_dynamic_topological_isa_program_execution() {
    use cronc::cl_topological::ClTopologicalCompiler;

    let cl_source = r#"
    // Initial manifold bundle
    bundle InitState {
        S0: WRAP   W=10 X=20 Z=30 spin=0 vol=25
        S1: WRAP   W=10 X=22 Z=30 spin=0 vol=25
        S2: TUNNEL W=40 X=50 Z=60 spin=90 vol=25
        S3: ANCHOR W=00 X=00 Z=00 spin=0 vol=25
    }

    // Dynamic trajectory instructions
    flow 3;
    braid S0, S1;
    resonate;
    parity_rotate;
    fsm_step forward;
    layer IV S0;
    transfer S3 -> S0 vol=25;
    "#;

    let bundles = ClTopologicalCompiler::compile_source(cl_source).expect("Source compilation must succeed");
    assert!(!bundles.is_empty(), "Program must synthesize bundles");

    // All synthesized bundles must strictly conserve H == 100%
    for b in &bundles {
        assert_eq!(b.snakes.iter().map(|s| s.vol as usize).sum::<usize>(), 100, "Every synthesized bundle must conserve H == 100%");
    }

    // Verify 40-character 10-10-10-10 microcode emission
    let listing = ClTopologicalCompiler::emit_10char_vliw_program(&bundles);
    assert!(listing.contains("B0000:"));
    for line in listing.lines().filter(|l| l.starts_with("B")) {
        let parts: Vec<&str> = line.split(':').collect();
        assert_eq!(parts[1].len(), 40, "Each machine line must have exactly 40 chars for 4 ALUs");
    }
}
