// ============================================================================
// CRON Hardware Silicon Roofline & Multi-Core Profiler Integration Tests
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_hardware_bench_suite_ascii_and_json() {
    // 1. Run cron bench with no arguments (Full 256-core AI suite)
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["bench"])
        .output()
        .expect("Failed to run cron bench");

    assert!(out.status.success(), "cron bench suite must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON 256-CORE 4D-TORUS HARDWARE SILICON ROOFLINE PROFILER MATRIX"));
    assert!(stdout.contains("FlashAttention-3"));
    assert!(stdout.contains("BitNet-b1.58-GEMM"));
    assert!(stdout.contains("20.48 TFLOPs"));
    assert!(stdout.contains("100.0% CONFLICT-FREE"));

    // 2. Run cron bench with --json
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["bench", "--json"])
        .output()
        .expect("Failed to run cron bench --json");

    assert!(json_out.status.success(), "cron bench --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"total_cores\": 256"));
    assert!(json_str.contains("\"peak_chip_tflops\": 20.48"));
    assert!(json_str.contains("\"is_provably_conflict_free\": true"));
}

#[test]
fn test_cli_hardware_bench_single_kernel_file() {
    let test_kernel = "test_custom_roofline.cl";
    let _ = fs::remove_file(test_kernel);

    let cl_code = r#"
@entry:
B0000: '==01#000> '==02#004> _MD03*120> _NO00#000>
B0001: _MD04*130> _PO05+340> _ST05#010> _NO00#000>
"#;
    fs::write(test_kernel, cl_code).expect("Failed to write test kernel");

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["bench", test_kernel])
        .output()
        .expect("Failed to run cron bench on single kernel");

    assert!(out.status.success(), "cron bench on single kernel must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("HARDWARE ROOFLINE & PERFORMANCE BENCHMARK"));
    assert!(stdout.contains("Operating Regime:"));
    assert!(stdout.contains("Operational Intensity:"));
    assert!(stdout.contains("Silicon Roofline Model Curve:"));

    let _ = fs::remove_file(test_kernel);
}
