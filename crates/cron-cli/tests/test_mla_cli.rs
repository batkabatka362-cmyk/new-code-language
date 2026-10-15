// ============================================================================
// CRON CLI Multi-Head Latent Attention (MLA / DeepSeek-V3) Test Suite
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_mla_help() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["mla", "--help"])
        .output()
        .expect("run cron mla --help");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Multi-Head Latent Attention"));
    assert!(stdout.contains("DeepSeek-V3"));
    assert!(stdout.contains("--dim"));
    assert!(stdout.contains("--heads"));
    assert!(stdout.contains("--latent"));
    assert!(stdout.contains("--seq"));
    assert!(stdout.contains("--ternary"));
    assert!(stdout.contains("--verilog"));
    assert!(stdout.contains("--json"));
}

#[test]
fn test_cli_mla_benchmark_and_telemetry() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["mla", "--dim", "64", "--heads", "4", "--latent", "16", "--seq", "128", "--ternary"])
        .output()
        .expect("run cron mla benchmark");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON MULTI-HEAD LATENT ATTENTION"));
    assert!(stdout.contains("Matrix Absorption in Latent Space"));
    assert!(stdout.contains("MatMul-Free Ternary"));
    assert!(stdout.contains("VRAM reduction"));
    assert!(stdout.contains("Bit-exact verified"));
    assert!(stdout.contains("GF(2^4) XOR Address Swizzling"));
}

#[test]
fn test_cli_mla_json_output() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["mla", "--seq", "64", "--json"])
        .output()
        .expect("run cron mla --json");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"vram_reduction_pct\":"));
    assert!(stdout.contains("\"parity_max_abs_error\":"));
    assert!(stdout.contains("\"kv_cache_bytes_mla\":"));
}

#[test]
fn test_cli_mla_verilog_synthesis() {
    let temp_dir = std::env::temp_dir().join(format!("cron_mla_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let _ = fs::create_dir_all(&temp_dir);
    let v_file = temp_dir.join("cksl_mla_absorb_core.v");

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["mla", "--verilog", "-o", v_file.to_str().unwrap()])
        .output()
        .expect("run cron mla --verilog");

    assert!(out.status.success());
    assert!(v_file.exists());
    let content = fs::read_to_string(&v_file).unwrap();
    assert!(content.contains("module cksl_mla_absorb_core"));
    assert!(content.contains("gf_swizzle"));
    assert!(content.contains("latent_kv_bank"));

    let _ = fs::remove_file(&v_file);
    let _ = fs::remove_dir(&temp_dir);
}
