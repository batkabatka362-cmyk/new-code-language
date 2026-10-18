// ============================================================================
// CRON CLI TinyZero Pure RL Training Integration Test
// ============================================================================

use std::process::Command;

#[test]
fn test_cli_train_rl_help() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["train-rl", "--help"])
        .output()
        .expect("run cron train-rl --help");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("TinyZero-Style Pure RL Training Engine"));
    assert!(stdout.contains("--epochs"));
    assert!(stdout.contains("--group-size"));
    assert!(stdout.contains("--max-steps"));
}

#[test]
fn test_cli_train_rl_execution() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "train-rl",
            "--epochs", "2",
            "--group-size", "2",
            "--max-steps", "3",
            "--task", "4D Sphere Curvature Matrix Geodesic",
        ])
        .output()
        .expect("run cron train-rl");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON TINYZERO PURE REINFORCEMENT LEARNING"));
    assert!(stdout.contains("Epoch 01/02"));
    assert!(stdout.contains("Epoch 02/02"));
    assert!(stdout.contains("<think>"));
    assert!(stdout.contains("TINYZERO PURE RL TRAINING COMPLETED SUCCESSFULLY"));
}

#[test]
fn test_cli_train_rl_json_output() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "train-rl",
            "--epochs", "1",
            "--group-size", "2",
            "--json",
        ])
        .output()
        .expect("run cron train-rl --json");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"algorithm\": \"GRPO_TinyZero\""));
    assert!(stdout.contains("\"status\": \"pure_rl_convergence_verified\""));
}
