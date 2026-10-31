use std::path::{Path, PathBuf};
use std::process::Command;

fn get_workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

#[test]
fn test_cli_dual_profile_max_quality_and_min_resource() {
    let cron_bin = env!("CARGO_BIN_EXE_cron");
    let root = get_workspace_root();
    let example_path = "examples/dual_profile_demonstration.cl";

    // 1. Test Max-Quality Profile
    let out_max = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", example_path, "--profile", "max-quality"])
        .output()
        .expect("Failed to execute cron math --profile max-quality");

    assert!(out_max.status.success(), "Max-quality execution failed: {:?}", String::from_utf8_lossy(&out_max.stderr));
    let stdout_max = String::from_utf8_lossy(&out_max.stdout);
    assert!(stdout_max.contains("Max-Quality (IEEE 754 F32 / CUDA PTX / WGMMA)"), "stdout: {}", stdout_max);
    assert!(stdout_max.contains("Peak Tensor RAM"));
    assert!(stdout_max.contains("Evaluation Result"));

    // 2. Test Min-Resource Profile
    let out_min = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", example_path, "--profile", "min-resource"])
        .output()
        .expect("Failed to execute cron math --profile min-resource");

    assert!(out_min.status.success(), "Min-resource execution failed: {:?}", String::from_utf8_lossy(&out_min.stderr));
    let stdout_min = String::from_utf8_lossy(&out_min.stdout);
    assert!(stdout_min.contains("Min-Resource (1.03-bit / DPSTA / Popcount MAC)"), "stdout: {}", stdout_min);
    assert!(stdout_min.contains("Peak Tensor RAM"));
    assert!(stdout_min.contains("Evaluation Result"));

    // 3. Test Dual-Profile Comparative Side-by-Side Telemetry (--compare-profiles)
    let out_comp = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", example_path, "--compare-profiles"])
        .output()
        .expect("Failed to execute cron math --compare-profiles");

    assert!(out_comp.status.success(), "Compare-profiles failed: {:?}", String::from_utf8_lossy(&out_comp.stderr));
    let stdout_comp = String::from_utf8_lossy(&out_comp.stdout);
    assert!(stdout_comp.contains("CRON DUAL-PROFILE COMPILER TELEMETRY: MAX-QUALITY vs. MIN-RESOURCE"), "stdout: {}", stdout_comp);
    assert!(stdout_comp.contains("Peak Tensor Memory"));
    assert!(stdout_comp.contains("Less Memory"));
    assert!(stdout_comp.contains("100% PARITY PASSED"));

    // 4. Test Empirical Precision Audit of QuaRot FWHT + DPSTA Sub-Byte Packing (--audit-precision)
    let out_audit = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", example_path, "--audit-precision"])
        .output()
        .expect("Failed to execute cron math --audit-precision");

    assert!(out_audit.status.success(), "Audit-precision failed: {:?}", String::from_utf8_lossy(&out_audit.stderr));
    let stdout_audit = String::from_utf8_lossy(&out_audit.stdout);
    assert!(stdout_audit.contains("CRON EMPIRICAL PRECISION AUDIT: QUA-ROT (FWHT) + DPSTA 1.03-BIT SUB-BYTE PACKING"), "stdout: {}", stdout_audit);
    assert!(stdout_audit.contains("Mean Directional Cosine Similarity"));
    assert!(stdout_audit.contains("20.0x RAM reduction"));
    assert!(stdout_audit.contains("100% Multiplier-Free"));
}

