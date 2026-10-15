// ============================================================================
// CRON CLI Liquid Neural Network (LNN / LTC) Test Suite
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_liquid_help() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["cl-liquid", "--help"])
        .output()
        .expect("run cron cl-liquid --help");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Liquid Continuous-Time Neural Network"));
    assert!(stdout.contains("--neurons"));
    assert!(stdout.contains("--inputs"));
    assert!(stdout.contains("--steps"));
    assert!(stdout.contains("--compile"));
    assert!(stdout.contains("--json"));
}

#[test]
fn test_cli_liquid_simulation_telemetry() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["cl-liquid", "--neurons", "8", "--inputs", "4", "--steps", "100"])
        .output()
        .expect("run cron cl-liquid");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON LIQUID CONTINUOUS-TIME NEURAL NETWORK"));
    assert!(stdout.contains("TIME-DILATION & ADAPTIVE REFLEX TELEMETRY:"));
    assert!(stdout.contains("Calm Baseline Tau"));
    assert!(stdout.contains("Shock Reflex Tau"));
    assert!(stdout.contains("UNCONDITIONALLY STABLE"));
}

#[test]
fn test_cli_liquid_json_output() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["cl-liquid", "--neurons", "4", "--steps", "50", "--json"])
        .output()
        .expect("run cron cl-liquid --json");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"is_unconditionally_stable\":true"));
    assert!(stdout.contains("\"time_dilation_speedup\":"));
    assert!(stdout.contains("\"memory_footprint_bytes\":"));
}

#[test]
fn test_cli_liquid_compile_to_cl() {
    let temp_dir = std::env::temp_dir().join(format!("cron_liquid_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let _ = fs::create_dir_all(&temp_dir);
    let out_cl = temp_dir.join("liquid_test.cl");

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["cl-liquid", "--compile", "--neurons", "8", "-o", out_cl.to_str().unwrap()])
        .output()
        .expect("run cron cl-liquid --compile");

    assert!(out.status.success());
    assert!(out_cl.exists());
    let content = fs::read_to_string(&out_cl).unwrap();
    assert!(content.contains("@liquid_nn_entry:"));
    assert!(content.contains("B0000:"));
    assert!(content.contains("B0001:"));
    assert!(content.contains("B0002:"));
    assert!(content.contains("B0003:"));

    let _ = fs::remove_file(&out_cl);
    let _ = fs::remove_dir(&temp_dir);
}
