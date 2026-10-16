// ============================================================================
// CRON SOTA Benchmark Comparison Integration Tests
// ============================================================================

use std::process::Command;

#[test]
fn test_cli_sota_benchmark_report_and_html_generation() {
    let cron_bin = env!("CARGO_BIN_EXE_cron");
    let temp_dir = std::env::temp_dir().join(format!("test_sota_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let html_out = temp_dir.join("sota_dashboard.html");

    // 1. Run cron bench --report -o <html_out>
    let status = Command::new(cron_bin)
        .args(["bench", "--report", "-o", html_out.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron bench --report");

    assert!(status.status.success(), "cron bench --report should succeed");
    let stdout = String::from_utf8_lossy(&status.stdout);

    assert!(stdout.contains("CRON FRONTIER AI & SOTA HARDWARE BENCHMARK COMPARISON REPORT"));
    assert!(stdout.contains("CRON (4D-Torus Silicon)"));
    assert!(stdout.contains("ONNX Runtime"));
    assert!(stdout.contains("Throughput Speedup"));

    // 2. Verify HTML output
    assert!(html_out.exists(), "HTML report must exist");
    let html_content = std::fs::read_to_string(&html_out).unwrap();
    assert!(html_content.contains("CRON SOTA Hardware Benchmark Scoreboard"));
    assert!(html_content.contains("1.58-Bit Ternary"));
    assert!(html_content.contains("0 DSP (Free)"));

    let _ = std::fs::remove_dir_all(&temp_dir);
}
