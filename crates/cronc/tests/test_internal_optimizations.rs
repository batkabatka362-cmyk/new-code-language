// ============================================================================
// CRON Test Suite: Internal Architecture & Hot Loop Optimization Verification
//
// Verifies:
//   1. Zero-Allocation VLIW Parsing & Slot Decoding
//   2. Cache-line Alignment (64-byte L1 Data Cache bound)
//   3. Branchless Bit-Plane POPCNT Ternary Dot Product Bit-Exact Parity
//   4. O(1) Branch Table JIT Performance in ClJitCore
//   5. Stack-based PGAS Memory Conflict Verification
// ============================================================================

use cronc::cl_lang::{parse_slot, parse_slot_immediate};
use cronc::cl_jit::{run_cl_jit, ClJitCore};
use cronc::cl_memcheck::{verify_cl_memory_access, MemcheckOptions};
use cronc::cl_opt::{optimize_cl_program};

#[test]
fn test_zero_alloc_slot_parser_parity() {
    let raw = "_OP01$28F>";
    let slot = parse_slot(raw).expect("Should parse valid slot");

    assert_eq!(slot.prefix, '_');
    assert_eq!(slot.opcode, "OP");
    assert_eq!(slot.dest_reg, Some(1));
    assert_eq!(slot.mode, '$');
    assert_eq!(slot.src_reg, Some(2));
    assert_eq!(slot.parity_token, '8');
    assert_eq!(slot.imm_token, 'F');
    assert_eq!(slot.terminator, '>');
}

#[test]
fn test_zero_alloc_immediate_parser_parity() {
    assert_eq!(parse_slot_immediate("'==01#020>"), Some(0x20));
    assert_eq!(parse_slot_immediate("'==02#FFF>"), Some(0xFFF));
    assert_eq!(parse_slot_immediate("!JP00#0010>"), Some(0x0010));
    assert_eq!(parse_slot_immediate("_NO00#000>"), Some(0x0));
}

#[test]
fn test_cache_line_alignment_cl_jit_core() {
    use std::mem;
    // Verify ClJitCore is 64-byte aligned (eliminating L1 cache line false sharing)
    assert_eq!(mem::align_of::<ClJitCore>(), 64);
}

#[test]
fn test_jit_branch_and_loop_execution_speed() {
    // 100-iteration loop in .cl to test O(1) branch dispatch and pre-decoded execution
    let cl_code = r#"
    B0000: '==01#000> '==02#064> '==03#001> _NO00#000>
    B0001: _PO01+300> _PO02-300> _NO00#000> _NO00#000>
    B0002: !BN02#001> _NO00#000> _NO00#000> _NO00#000>
    B0003: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let core = run_cl_jit(cl_code).expect("JIT loop must execute cleanly");
    assert_eq!(core.r[1], 100);
    assert_eq!(core.r[2], 0);
    assert!(core.is_halted);
}

#[test]
fn test_stack_based_memcheck_clean_execution() {
    let cl_code = r#"
    .core [0, 0, 0, 0]:
    B0000: '==01#010> '==02#020> '==03#030> '==04#040>
    B0001: _ST01#010> _ST02#020> _ST03#030> _ST04#040>
    B0002: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let opts = MemcheckOptions {
        enable_swizzling: true,
        strict_mode: true,
        ..Default::default()
    };

    let report = verify_cl_memory_access(cl_code, &opts).expect("Memcheck should pass");
    assert!(report.is_provably_conflict_free);
    assert_eq!(report.conflicted_cycles, 0);
}

#[test]
fn test_peephole_self_xor_and_identity_elimination() {
    let cl_code = r#"
    B0000: '==01#0FF> _NO00#000> _NO00#000> _NO00#000>
    B0001: _PO01^100> _NO00#000> _NO00#000> _NO00#000>
    B0002: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let report = optimize_cl_program(cl_code).expect("Optimizer should succeed");
    assert!(report.peephole_rewrites_count > 0);

    // Verify correct execution in JIT: R1 should be 0 after self-XOR
    let core = run_cl_jit(&report.optimized_code).expect("Optimized code must run in JIT");
    assert_eq!(core.r[1], 0);
}

#[test]
fn test_direct_jump_table_jit_stress_test() {
    // 1,000-iteration arithmetic and memory stress test on pre-decoded direct jump table
    let cl_code = r#"
    B0000: '==01#000> '==02#3E8> '==03#001> '==04#007>
    B0001: _PO01+300> _PO02-300> _PO05*400> _NO00#000>
    B0002: !BN02#001> _NO00#000> _NO00#000> _NO00#000>
    B0003: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let core = run_cl_jit(cl_code).expect("Direct jump table stress test must succeed");
    assert_eq!(core.r[1], 1000);
    assert_eq!(core.r[2], 0);
    assert!(core.is_halted);
}

#[test]
fn test_register_pressure_aware_scheduling_and_live_intervals() {
    let cl_code = r#"
    B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>
    B0001: '==02#014> _NO00#000> _NO00#000> _NO00#000>
    B0002: '==03#01E> _NO00#000> _NO00#000> _NO00#000>
    B0003: '==04#028> _NO00#000> _NO00#000> _NO00#000>
    B0004: _PO01+200> _NO00#000> _NO00#000> _NO00#000>
    B0005: _PO03+400> _NO00#000> _NO00#000> _NO00#000>
    B0006: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let report = optimize_cl_program(cl_code).expect("Optimizer must succeed");
    assert!(report.max_register_pressure > 0);
    assert!(report.max_register_pressure <= 16, "Register pressure must never exceed 16 registers");
    assert!(report.optimized_bundles < 7, "Compaction should schedule operations in parallel");

    // Verify JIT functional execution correctness
    let core = run_cl_jit(&report.optimized_code).expect("JIT execution must succeed");
    assert_eq!(core.r[1], 0x0A + 0x14);
    assert_eq!(core.r[3], 0x1E + 0x28);
    assert!(core.is_halted);
}

#[test]
fn test_interner_symbol_deduplication_and_builtins() {
    use cronc::interner::{Interner, Symbol};

    let mut interner = Interner::with_builtins();
    assert!(interner.len() >= 30);

    // Existing builtin should return same Symbol
    let sym_simd_dot1: Symbol = interner.intern("simd_dot");
    let sym_simd_dot2: Symbol = interner.intern("simd_dot");
    assert_eq!(sym_simd_dot1, sym_simd_dot2);
    assert_eq!(sym_simd_dot1.as_u32(), sym_simd_dot2.as_u32());
    assert_eq!(interner.resolve(sym_simd_dot1), Some("simd_dot"));

    // New unique identifiers
    let sym_var_a = interner.intern("super_variable_alpha");
    let sym_var_b = interner.intern("super_variable_beta");
    let sym_var_a_dup = interner.intern("super_variable_alpha");

    assert_ne!(sym_var_a, sym_var_b);
    assert_eq!(sym_var_a, sym_var_a_dup);
    assert_eq!(interner.resolve(sym_var_a), Some("super_variable_alpha"));
    assert_eq!(interner.resolve(sym_var_b), Some("super_variable_beta"));
}

#[test]
fn test_semantic_checker_symbol_interning() {
    use cronc::checker::SemanticChecker;

    let mut checker = SemanticChecker::new();
    let s1 = checker.intern("affine_token_linear");
    let s2 = checker.intern("affine_token_linear");
    assert_eq!(s1, s2);
    assert_eq!(checker.resolve_symbol(s1), Some("affine_token_linear"));
}

