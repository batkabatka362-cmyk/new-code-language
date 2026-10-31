use std::process::Command;

#[test]
fn test_cli_bench_math_execution() {
    let cron_bin = env!("CARGO_BIN_EXE_cron");

    // 1. Test standard table output
    let output = Command::new(cron_bin)
        .arg("bench-math")
        .output()
        .expect("Failed to execute cron bench-math");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON ZERO-VERBAL MATHEMATICAL SILICON ENGINE BENCHMARK"));
    assert!(stdout.contains("FP32 GEMM"));
    assert!(stdout.contains("Causal FlashAttn"));
    assert!(stdout.contains("Fused RMSNorm"));
    assert!(stdout.contains("Fused SwiGLU MLP"));
    assert!(stdout.contains("BitNet b1.58 BitLinear"));
    assert!(stdout.contains("100% SSS+ OPTIMIZED (PASSED)"));

    // 2. Test JSON structured report output
    let json_output = Command::new(cron_bin)
        .args(["bench-math", "--json"])
        .output()
        .expect("Failed to execute cron bench-math --json");

    assert!(json_output.status.success());
    let json_str = String::from_utf8_lossy(&json_output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&json_str).expect("Valid JSON output");
    assert_eq!(parsed["status"], "PASSED");
    assert_eq!(parsed["version"], "v1.0-Phase12");
    assert!(parsed["results"].as_array().unwrap().len() >= 5);
}

