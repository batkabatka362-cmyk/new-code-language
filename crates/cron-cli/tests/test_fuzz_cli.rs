// ============================================================================
// CRON Memory Safety & AI Vibe-Fuzz CLI Integration Tests
// ============================================================================

use std::process::Command;

#[test]
fn test_cli_fuzz_engine_and_zero_crash_invariant() {
    let cron_bin = env!("CARGO_BIN_EXE_cron");

    // Run cron test --fuzz -n 200
    let status = Command::new(cron_bin)
        .args(["test", "--fuzz", "-n", "200"])
        .output()
        .expect("Failed to execute cron test --fuzz");

    assert!(status.status.success(), "cron test --fuzz should exit cleanly");
    let stdout = String::from_utf8_lossy(&status.stdout);

    assert!(stdout.contains("CRON AUTONOMOUS AI VIBE-FUZZ & MUTATION RESILIENCE REPORT"));
    assert!(stdout.contains("100% CRASH-RESILIENT"));
    assert!(stdout.contains("Unhandled Crashes/Panics:"));
    assert!(stdout.contains("0"));
}

#[test]
fn test_cli_fuzz_json_output() {
    let cron_bin = env!("CARGO_BIN_EXE_cron");

    // Run cron test --fuzz -n 50 --json
    let status = Command::new(cron_bin)
        .args(["test", "--fuzz", "-n", "50", "--json"])
        .output()
        .expect("Failed to execute cron test --fuzz --json");

    assert!(status.status.success(), "cron test --fuzz --json should succeed");
    let stdout = String::from_utf8_lossy(&status.stdout);

    assert!(stdout.contains("\"total_iterations\": 50"));
    assert!(stdout.contains("\"is_robust_and_crash_free\": true"));
    assert!(stdout.contains("\"unhandled_panics\": 0"));
}
