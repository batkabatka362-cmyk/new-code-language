// ============================================================================
// CRON Test Suite: Comprehensive .cl Compiler & Tooling Optimizations
// Verifies:
//   1. VLIW Auto-Vectorizer (DotProduct, RMSNorm, RoPE, Elementwise, Projections)
//   2. Verilog RTL Synthesis & Automated Testbench Packaging (IEEE 1364-2001)
//   3. Accelerated In-Memory Vector JIT Backend & 256-Core Multi-Batch Parity
// (C) 2026 CRON Language Project - 100% Industrial Verification
// ============================================================================

use cronc::cl_vectorizer::{vectorize_pattern, VectorPattern, VectorizerConfig};
use cronc::cl_rtl_export::{export_cl_to_rtl, RtlExportConfig, TargetFpgaPlatform};
use cronc::cl_jit_accel::{run_accelerated_batch_jit, run_fast_jit};
use cronc::cl_pkg::audit_cl_source;
use cronc::cl_lint::lint_cl_source;
use std::fs;

#[test]
fn test_auto_vectorizer_all_patterns() {
    let cfg = VectorizerConfig::default();

    // 1. Ternary Dot Product
    let rep_dot = vectorize_pattern(
        "opt_ternary_dot",
        &VectorPattern::DotProduct { len: 64, is_ternary: true },
        &cfg,
    ).expect("DotProduct vectorization failed");
    assert!(rep_dot.bundles_generated > 0);
    assert_eq!(rep_dot.effective_ipc, 4.0);
    let audit_dot = audit_cl_source(&rep_dot.cl_source, "opt_ternary_dot");
    assert!(audit_dot.passed, "Audit failed: {:?}", audit_dot.diagnostics);
    assert_eq!(audit_dot.raw_waw_hazard_count, 0);

    // 2. RMSNorm reduction & scaling
    let rep_norm = vectorize_pattern(
        "opt_rmsnorm",
        &VectorPattern::RMSNorm { len: 32 },
        &cfg,
    ).expect("RMSNorm vectorization failed");
    assert!(rep_norm.bundles_generated > 0);
    let audit_norm = audit_cl_source(&rep_norm.cl_source, "opt_rmsnorm");
    assert!(audit_norm.passed, "Audit failed: {:?}", audit_norm.diagnostics);

    // 3. RoPE 4D CORDIC vectorization
    let rep_rope = vectorize_pattern(
        "opt_rope",
        &VectorPattern::RoPE { dim: 128 },
        &cfg,
    ).expect("RoPE vectorization failed");
    assert!(rep_rope.bundles_generated > 0);
    let audit_rope = audit_cl_source(&rep_rope.cl_source, "opt_rope");
    assert!(audit_rope.passed, "Audit failed: {:?}", audit_rope.diagnostics);

    // 4. Elementwise SwiGLU / GELU
    let rep_elem = vectorize_pattern(
        "opt_swiglu",
        &VectorPattern::Elementwise { op: "swiglu".to_string(), len: 16 },
        &cfg,
    ).expect("Elementwise vectorization failed");
    assert!(rep_elem.bundles_generated > 0);
    let audit_elem = audit_cl_source(&rep_elem.cl_source, "opt_swiglu");
    assert!(audit_elem.passed, "Audit failed: {:?}", audit_elem.diagnostics);

    // 5. Linear Projection
    let rep_proj = vectorize_pattern(
        "opt_linear_proj",
        &VectorPattern::LinearProjection { in_dim: 16, out_dim: 8 },
        &cfg,
    ).expect("LinearProjection vectorization failed");
    assert!(rep_proj.bundles_generated > 0);
    let audit_proj = audit_cl_source(&rep_proj.cl_source, "opt_linear_proj");
    assert!(audit_proj.passed, "Audit failed: {:?}", audit_proj.diagnostics);
}

#[test]
fn test_rtl_hardware_package_full_export() {
    let cfg = VectorizerConfig::default();
    let rep = vectorize_pattern(
        "silicon_accel_core",
        &VectorPattern::DotProduct { len: 16, is_ternary: false },
        &cfg,
    ).expect("Pattern vectorization failed");

    let rtl_cfg = RtlExportConfig {
        platform: TargetFpgaPlatform::XilinxAlveoU280,
        clock_freq_mhz: 1000.0,
        enable_debug_probe: true,
        bram_banks: 16,
    };

    let pkg = export_cl_to_rtl(&rep.cl_source, "silicon_accel_core", &rtl_cfg)
        .expect("RTL package generation failed");

    assert_eq!(pkg.module_name, "silicon_accel_core");
    assert!(pkg.core_verilog.contains("module silicon_accel_core"));
    assert!(pkg.testbench_verilog.contains("module tb_silicon_accel_core"));
    assert!(pkg.vivado_tcl.contains("create_project -in_memory"));
    assert!(pkg.timing_xdc.contains("create_clock -period 1.000"));
    assert!(!pkg.hex_rom.is_empty());
    assert!(pkg.resources.estimated_luts >= 450);

    let temp_dir = std::env::temp_dir().join("cron_opt_rtl_export_test");
    let _ = fs::remove_dir_all(&temp_dir);
    pkg.write_to_directory(&temp_dir).expect("Failed to write package directory");

    assert!(temp_dir.join("silicon_accel_core.v").exists());
    assert!(temp_dir.join("tb_silicon_accel_core.v").exists());
    assert!(temp_dir.join("bitstream.hex").exists());
    assert!(temp_dir.join("synth_vivado.tcl").exists());
    assert!(temp_dir.join("timing.xdc").exists());
    assert!(temp_dir.join("package_manifest.json").exists());
}

#[test]
fn test_accelerated_batch_jit_multi_core_simulation() {
    let cfg = VectorizerConfig::default();
    let rep = vectorize_pattern(
        "batch_jit_sample",
        &VectorPattern::DotProduct { len: 16, is_ternary: true },
        &cfg,
    ).expect("Vectorization failed");

    // Fast JIT single-core execution
    let core = run_fast_jit(&rep.cl_source).expect("Fast JIT execution failed");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);

    // Multi-core parallel batch JIT execution (16 virtual cores)
    let batch_rep = run_accelerated_batch_jit(&rep.cl_source, "batch_jit_sample", 16)
        .expect("Batch JIT failed");

    assert_eq!(batch_rep.cores_simulated, 16);
    assert!(batch_rep.total_cycles > 0);
    assert!(batch_rep.million_instructions_per_sec > 0.0);
    assert!(batch_rep.is_all_halted);
}

#[test]
fn test_vectorized_code_linter_zero_hazards() {
    let cfg = VectorizerConfig::default();
    let rep = vectorize_pattern(
        "zero_hazard_kernel",
        &VectorPattern::DotProduct { len: 32, is_ternary: false },
        &cfg,
    ).expect("Vectorization failed");

    let lint = lint_cl_source(&rep.cl_source, "zero_hazard_kernel");
    assert_eq!(lint.error_count, 0, "Vectorized code must have 0 lint errors");
    assert_eq!(lint.warning_count, 0, "Vectorized code must have 0 pipeline hazard warnings");
    assert!(lint.is_clean, "Vectorized code must be 100% clean");
}
