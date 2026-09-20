use cronc::cl_opt::{optimize_cl_program, optimize_cl_program_advanced, ClOptConfig, ClOptLevel};
use cronc::cl_jit::run_cl_jit;
use cronc::cl_cosim::{run_cl_cosim, CosimOptions};

#[test]
fn test_cl_opt_dag_independent_loads_compaction() {
    let sparse_cl = r#"
    B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>
    B0001: '==02#00B> _NO00#000> _NO00#000> _NO00#000>
    B0002: '==03#00C> _NO00#000> _NO00#000> _NO00#000>
    B0003: '==04#00D> _NO00#000> _NO00#000> _NO00#000>
    "#;

    let report = optimize_cl_program(sparse_cl).expect("Super-optimization should succeed");

    assert_eq!(report.original_bundles, 4);
    assert_eq!(report.optimized_bundles, 1, "All 4 independent loads must be packed into exactly 1 bundle");
    assert_eq!(report.compacted_slots, 4);
    assert_eq!(report.optimized_ipc, 4.0);
    assert_eq!(report.speedup_percentage, 75.0);

    // Verify correct execution of optimized code
    let core = run_cl_jit(&report.optimized_code).expect("Optimized code must run in JIT");
    assert_eq!(core.r[1], 0x0A);
    assert_eq!(core.r[2], 0x0B);
    assert_eq!(core.r[3], 0x0C);
    assert_eq!(core.r[4], 0x0D);
}

#[test]
fn test_cl_opt_dag_out_of_order_hazard_bypassing() {
    // Instruction 0 writes R1
    // Instruction 1 adds R1 (depends on Instruction 0 - RAW hazard)
    // Instruction 2 writes R2 (independent of R1!)
    // An in-order packer blocks after Instruction 0, but DAG scheduler pulls Instruction 2 into cycle 0!
    let cl_code = r#"
    B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>
    B0001: _PO01+100> _NO00#000> _NO00#000> _NO00#000>
    B0002: '==02#005> _NO00#000> _NO00#000> _NO00#000>
    B0003: _HL00$008! _NO00$000> _NO00$000> _NO00#000>
    "#;

    let report = optimize_cl_program(cl_code).expect("DAG optimization should succeed");

    assert!(report.optimized_bundles < 4, "DAG scheduling must compact bundles out of order");
    assert!(report.critical_path_depth >= 2);

    // Check that both R1 and R2 are computed correctly in JIT
    let core = run_cl_jit(&report.optimized_code).expect("JIT execution must succeed");
    // R1 was 10, then 10 + 10 = 20 (0x14)
    assert_eq!(core.r[1], 0x14);
    assert_eq!(core.r[2], 0x05);
}

#[test]
fn test_cl_opt_dag_reversible_involution_cancellation() {
    // Two consecutive Fredkin swaps on R1 and R2 cancel out (RF * RF = I)
    let cl_code = r#"
    B0000: '==01#00A> '==02#005> _NO00#000> _NO00#000>
    B0001: _RF01$200> _NO00#000> _NO00#000> _NO00#000>
    B0002: _RF01$200> _NO00#000> _NO00#000> _NO00#000>
    B0003: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let report = optimize_cl_program(cl_code).expect("Optimization should succeed");

    assert!(report.peephole_rewrites_count >= 2, "Double RF must be cancelled by peephole");
    assert!(!report.optimized_code.contains("_RF01$200>"), "Cancelled RF must not appear in output");

    let core = run_cl_jit(&report.optimized_code).expect("JIT must succeed");
    assert_eq!(core.r[1], 0x0A);
    assert_eq!(core.r[2], 0x05);
}

#[test]
fn test_cl_opt_dag_silicon_heterogeneous_way_routing() {
    let cl_code = r#"
    B0000: '==01#00A> '==02#005> '==03#001> _NO00#000>
    B0001: _OP04$100> _MD05*200> _PO06+300> _SB00$300>
    B0002: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let report = optimize_cl_program(cl_code).expect("Optimization must succeed");

    // In B0001:
    // Slot 0 should have _OP (Way 0)
    // Slot 1 should have _MD (Way 1)
    // Slot 2 should have _PO (Way 2)
    // Slot 3 should have _SB (Way 3)
    let b1_line = report.optimized_code.lines().find(|l| l.starts_with("B0001:")).expect("B0001 must exist");
    let parts: Vec<&str> = b1_line.split_whitespace().collect();
    assert_eq!(parts.len(), 5);
    assert!(parts[1].starts_with("_OP"), "Way 0 must receive Optical GEMM");
    assert!(parts[2].starts_with("_MD"), "Way 1 must receive Sub-byte MAC");
    assert!(parts[3].starts_with("_PO"), "Way 2 must receive SIMD ALU");
    assert!(parts[4].starts_with("_SB"), "Way 3 must receive Spatial NoC");
}

#[test]
fn test_cl_opt_level1_vs_level2_comparison() {
    let cl_code = r#"
    B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>
    B0001: _PO01+100> _NO00#000> _NO00#000> _NO00#000>
    B0002: '==02#005> _NO00#000> _NO00#000> _NO00#000>
    B0003: '==03#002> _NO00#000> _NO00#000> _NO00#000>
    B0004: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let cfg_l1 = ClOptConfig { level: ClOptLevel::Level1, ..Default::default() };
    let cfg_l2 = ClOptConfig { level: ClOptLevel::Level2, ..Default::default() };

    let rep_l1 = optimize_cl_program_advanced(cl_code, &cfg_l1).expect("Level 1 success");
    let rep_l2 = optimize_cl_program_advanced(cl_code, &cfg_l2).expect("Level 2 success");

    assert!(rep_l2.optimized_bundles <= rep_l1.optimized_bundles, "Level 2 DAG must be at least as compact as Level 1");
    assert!(rep_l2.optimized_ipc >= rep_l1.optimized_ipc, "Level 2 IPC must be >= Level 1 IPC");
}

#[test]
fn test_cl_opt_mini_transformer_hardware_cosim_parity() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_path = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");
    let source = std::fs::read_to_string(kernel_path).expect("Read attention kernel");

    let report = optimize_cl_program(&source).expect("Optimize attention kernel");

    assert!(report.optimized_ipc > 0.0);

    // Verify optimized code still achieves 100% bit-exact hardware parity in Co-Simulation!
    let cosim_opts = CosimOptions::default();
    let cosim_report = run_cl_cosim(&report.optimized_code, &cosim_opts)
        .expect("Co-simulation of optimized kernel must succeed");

    assert!(cosim_report.is_100pct_parity, "Optimized kernel must maintain 100% bit-exact hardware parity!");
    assert_eq!(cosim_report.divergence_count, 0);
}
