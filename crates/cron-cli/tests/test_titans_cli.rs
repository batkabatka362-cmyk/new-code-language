// ============================================================================
// CRON CLI Google Titans & RWKV-7 Neural Long-Term Memory Test Suite
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_titans_help() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["titans", "--help"])
        .output()
        .expect("run cron titans --help");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Google Titans"));
    assert!(stdout.contains("RWKV-7"));
    assert!(stdout.contains("--dim"));
    assert!(stdout.contains("--heads"));
    assert!(stdout.contains("--mem-dim"));
    assert!(stdout.contains("--seq"));
    assert!(stdout.contains("--ternary"));
    assert!(stdout.contains("--json"));
}

#[test]
fn test_cli_titans_benchmark_and_telemetry() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["titans", "--dim", "64", "--heads", "4", "--mem-dim", "16", "--seq", "256", "--ternary"])
        .output()
        .expect("run cron titans benchmark");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON GOOGLE TITANS & RWKV-7"));
    assert!(stdout.contains("STRICT CONSTANT O(1) FLAT MEMORY"));
    assert!(stdout.contains("MatMul-Free Ternary"));
    assert!(stdout.contains("VRAM Savings Efficiency"));
    assert!(stdout.contains("Flat O(1) Memory Verified"));
    assert!(stdout.contains("YES"));
    assert!(stdout.contains("Surprise Memory Updates"));
}

#[test]
fn test_cli_titans_json_output() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["titans", "--seq", "128", "--json"])
        .output()
        .expect("run cron titans --json");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"is_flat_o1_verified\":true"));
    assert!(stdout.contains("\"memory_savings_pct\":"));
    assert!(stdout.contains("\"titans_flat_memory_bytes\":"));
}

#[test]
fn test_cli_titans_save_output_file() {
    let temp_dir = std::env::temp_dir().join(format!("cron_titans_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let _ = fs::create_dir_all(&temp_dir);
    let out_report = temp_dir.join("titans_report.txt");

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["titans", "--seq", "64", "-o", out_report.to_str().unwrap()])
        .output()
        .expect("run cron titans -o");

    assert!(out.status.success());
    assert!(out_report.exists());
    let content = fs::read_to_string(&out_report).unwrap();
    assert!(content.contains("TitansBenchmarkReport"));

    let _ = fs::remove_file(&out_report);
    let _ = fs::remove_dir(&temp_dir);
}
