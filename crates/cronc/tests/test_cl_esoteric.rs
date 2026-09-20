// ============================================================================
// Unit Tests: CRON Esolang-Inspired Silicon Coprocessor (test_cl_esoteric.rs)
// ============================================================================

use cronc::cl_esoteric::{
    render_ascii_esoteric_hud, synthesize_verilog_esoteric_coprocessor,
    BefungeSystolicGrid, EsotericCoprocessor, HardwareStatusFlags,
    HardwareTapePointers, PrologUnificationEngine, SystolicDirection,
    Trit, TritWord, ZohlState,
};
use cronc::cl_jit::run_cl_jit;
use cronc::cl_lang::verify_cl_program;

#[test]
fn test_brainfuck_tape_pointers_and_zohl() {
    let mut tape = HardwareTapePointers::new();
    assert_eq!(tape.tp0, 0);
    assert!(!tape.wrap_flag);

    // Auto-increment & write
    tape.write_advance_tp0(42);
    assert_eq!(tape.tp0, 1);
    assert_eq!(tape.tape_memory[0], 42);

    // Auto-read & advance
    tape.tape_memory[1] = 99;
    let val = tape.read_advance_tp0();
    assert_eq!(val, 99);
    assert_eq!(tape.tp0, 2);

    // Test modulo wrap
    tape.tp0 = 255;
    tape.inc_tp0();
    assert_eq!(tape.tp0, 0);
    assert!(tape.wrap_flag);

    // Test ZOHL zero-overhead hardware loop
    let mut zohl = ZohlState::default();
    zohl.set_loop(3, 10, 14); // loop 3 times from PC 10 to 14
    assert!(zohl.active);

    let next_pc = zohl.step(14); // end of loop 1
    assert_eq!(next_pc, 10);
    assert_eq!(zohl.loop_counter, 2);

    let next_pc = zohl.step(14); // end of loop 2
    assert_eq!(next_pc, 10);
    assert_eq!(zohl.loop_counter, 1);

    let next_pc = zohl.step(14); // end of loop 3 (exit)
    assert_eq!(next_pc, 15);
    assert_eq!(zohl.loop_counter, 0);
    assert!(!zohl.active);
}

#[test]
fn test_malbolge_balanced_ternary_and_trit_mac() {
    // Test Trit conversions
    assert_eq!(Trit::Pos.to_int(), 1);
    assert_eq!(Trit::Neg.to_int(), -1);
    assert_eq!(Trit::Zero.to_int(), 0);

    assert_eq!(Trit::Pos.to_code(), 0b01);
    assert_eq!(Trit::Neg.to_code(), 0b10);
    assert_eq!(Trit::Zero.to_code(), 0b00);

    // Test Crazy Op
    assert_eq!(TritWord::crazy_op(Trit::Zero, Trit::Zero), Trit::Pos);
    assert_eq!(TritWord::crazy_op(Trit::Pos, Trit::Pos), Trit::Zero);
    assert_eq!(TritWord::crazy_op(Trit::Neg, Trit::Neg), Trit::Pos);

    // Test packed TritWord
    let trits = [
        Trit::Pos, Trit::Neg, Trit::Zero, Trit::Pos,
        Trit::Zero, Trit::Zero, Trit::Pos, Trit::Neg,
        Trit::Pos, Trit::Pos, Trit::Zero, Trit::Neg,
        Trit::Neg, Trit::Zero, Trit::Pos, Trit::Zero,
    ];
    let word = TritWord::from_trits(&trits);
    let unpacked = word.to_trits();
    assert_eq!(trits, unpacked);

    // Test Multiplier-Free Trit-MAC
    let activations: [i8; 16] = [
        10, 20, 30, 40,
        50, 60, 70, 80,
        90, 100, 110, 120,
        -126, 14, 15, 16,
    ];
    let result = TritWord::trit_mac_int8(word, &activations);
    // Pos items: 10 + 40 + 70 + 90 + 100 + 15 = 325
    // Neg items: 20 + 80 + 120 + (-126) = 94
    // 325 - 94 = 231
    assert_eq!(result, 325 - 94);
}

#[test]
fn test_befunge_systolic_spatial_routing() {
    let mut grid = BefungeSystolicGrid::new();
    assert_eq!(grid.total_hops, 0);

    grid.push(SystolicDirection::EastX, 0x1111);
    grid.push(SystolicDirection::EastX, 0x2222);
    grid.push(SystolicDirection::NorthY, 0x3333);
    assert_eq!(grid.total_hops, 3);

    assert_eq!(grid.pop(SystolicDirection::EastX), Some(0x1111));
    assert_eq!(grid.pop(SystolicDirection::EastX), Some(0x2222));
    assert_eq!(grid.pop(SystolicDirection::EastX), None);
    assert_eq!(grid.pop(SystolicDirection::NorthY), Some(0x3333));
}

#[test]
fn test_prolog_1cycle_unification_and_shape_solving() {
    let mut vec_a = [0xFFFFu16; 16];
    let mut vec_b = [0xFFFFu16; 16];
    vec_a[0] = 5;
    vec_a[1] = 12;
    vec_a[2] = 20;

    vec_b[0] = 3;
    vec_b[1] = 12; // match
    vec_b[2] = 20; // match

    let res = PrologUnificationEngine::unify_indices_16(&vec_a, &vec_b);
    assert_eq!(res.match_count, 2);
    assert_eq!(res.match_mask, 0b110);
    assert_eq!(res.matched_indices, vec![(1, 1), (2, 2)]);

    // Test high-level tensor shape unification
    let shape_a = [Some(8), None, Some(64)];
    let shape_b = [None, Some(128), Some(64)];
    let unified = PrologUnificationEngine::unify_tensor_shapes(&shape_a, &shape_b).unwrap();
    assert_eq!(unified, vec![8, 128, 64]);

    // Test conflict: dimension 0 is 8 vs 16
    let shape_conflict = [Some(16), Some(32), Some(64)];
    assert!(PrologUnificationEngine::unify_tensor_shapes(&shape_a, &shape_conflict).is_err());
}

#[test]
fn test_assembly_hardware_status_flags() {
    let mut flags = HardwareStatusFlags::default();
    assert_eq!(flags.to_u32(), 0);

    flags.sat_fp8 = true;
    assert_eq!(flags.to_u32(), 1);

    flags.zero_sparse_hit = true;
    assert_eq!(flags.to_u32(), 3);

    flags.tape_wrap = true;
    assert_eq!(flags.to_u32(), 7);

    flags.unify_hit = true;
    assert_eq!(flags.to_u32(), 15);
}

#[test]
fn test_esoteric_coprocessor_integrated_and_verilog_synth() {
    let mut coproc = EsotericCoprocessor::new();
    coproc.exec_tape_write(100);
    assert_eq!(coproc.exec_tape_read(), 0); // read from tp0=1 which is 0
    assert!(coproc.dynamic_energy_pj > 0.0);

    let hud = render_ascii_esoteric_hud(&coproc);
    assert!(hud.contains("CRON ESOLANG-INSPIRED AI SILICON COPROCESSOR TELEMETRY HUD"));
    assert!(hud.contains("Brainfuck Hardware Tape Pointer"));
    assert!(hud.contains("Malbolge Balanced Ternary"));
    assert!(hud.contains("Befunge 2D/4D Spatial Systolic"));
    assert!(hud.contains("Prolog 1-Cycle Hardware Unifier"));

    let verilog = synthesize_verilog_esoteric_coprocessor();
    assert!(verilog.contains("module esoteric_coprocessor"));
    assert!(verilog.contains("tape_ptr_out"));
    assert!(verilog.contains("trit_mac_result"));
    assert!(verilog.contains("unify_match_mask"));
}

#[test]
fn test_cl_jit_esoteric_opcodes() {
    // Valid 4-slot VLIW bundle using new esoteric opcodes
    let cl_code = r#"
B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>
B0001: _TW00#100> _NO00#000> _NO00#000> _NO00#000>
B0002: _TD00#000> _NO00#000> _NO00#000> _NO00#000>
B0003: _TR02#000> _DE00#000> _NO00#000> _HL00#000!
"#;

    let report = verify_cl_program(cl_code).unwrap();
    assert_eq!(report.total_bundles, 4);

    let core = run_cl_jit(cl_code).unwrap();
    assert!(core.is_halted);
    assert_eq!(core.r[1], 0x0A);
    assert_eq!(core.tape_buf[0], 0x0A);
    assert_eq!(core.r[2], 0x0A);
    assert_eq!(core.systolic_hops_count, 1);
}
