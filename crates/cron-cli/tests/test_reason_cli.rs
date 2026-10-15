// ============================================================================
// CRON CLI Test-Time Reasoning (DeepSeek-R1 / GRPO) Test Suite
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_reason_help() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["reason", "--help"])
        .output()
        .expect("run cron reason --help");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("DeepSeek-R1"));
    assert!(stdout.contains("<think>"));
    assert!(stdout.contains("--group"));
    assert!(stdout.contains("--no-cot"));
    assert!(stdout.contains("--json"));
}

#[test]
fn test_cli_reason_cot_and_microcode() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["reason", "FlashAttention 4.0 IPC Polyhedral Tiling", "--group", "4"])
        .output()
        .expect("run cron reason");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON TEST-TIME REASONING ENGINE"));
    assert!(stdout.contains("<think>"));
    assert!(stdout.contains("</think>"));
    assert!(stdout.contains("GRPO Selection"));
    assert!(stdout.contains(".stage"));
    assert!(stdout.contains("@entry:"));
    assert!(stdout.contains("Formal Verification"));
}

#[test]
fn test_cli_reason_json_output() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["reason", "BitNet SwiGLU 1.58b 4-slot bundle", "--json"])
        .output()
        .expect("run cron reason --json");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"full_cot_monologue\":"));
    assert!(stdout.contains("\"verified_code\":"));
    assert!(stdout.contains("\"proof_hash\":"));
    assert!(stdout.contains("\"best_reward\":"));
}

#[test]
fn test_cli_reason_save_output_file() {
    let temp_dir = std::env::temp_dir().join(format!("cron_reason_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let _ = fs::create_dir_all(&temp_dir);
    let out_cl = temp_dir.join("reasoned_output.cl");

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["reason", "Clifford Rotor Pipeline", "-o", out_cl.to_str().unwrap()])
        .output()
        .expect("run cron reason -o");

    assert!(out.status.success());
    assert!(out_cl.exists());
    let content = fs::read_to_string(&out_cl).unwrap();
    assert!(content.contains(".stage"));
    assert!(content.contains("@entry:"));

    let _ = fs::remove_file(&out_cl);
    let _ = fs::remove_dir(&temp_dir);
}
