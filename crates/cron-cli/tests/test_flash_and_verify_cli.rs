// ============================================================================
// Integration Tests for CRON CLI Physical Flash & Formal Verify Commands
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_flash_hardware_package_generation() {
    let temp_dir = std::env::temp_dir();
    let sample_cr = temp_dir.join("test_flash_source.cr");
    let out_deploy_dir = temp_dir.join("test_flash_deploy_output");

    let cr_content = r#"
    .MODULE FlashSourceMod
    .ENTRY main

    def main() -> i64 {
        let x: i64 = 42;
        return x;
    }
    .END
    "#;

    fs::write(&sample_cr, cr_content).expect("Failed to write sample cr file");

    let status = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "flash",
            sample_cr.to_str().unwrap(),
            "--target",
            "xilinx_u280",
            "--interface",
            "pcie",
            "--out-dir",
            out_deploy_dir.to_str().unwrap(),
            "--dry-run",
        ])
        .status()
        .expect("Failed to execute cron flash");

    assert!(status.success(), "cron flash should exit with code 0");

    assert!(out_deploy_dir.join("cron_top.v").exists());
    assert!(out_deploy_dir.join("synth.tcl").exists());
    assert!(out_deploy_dir.join("timing.xdc").exists());
    assert!(out_deploy_dir.join("cron_pcie_dma.h").exists());
    assert!(out_deploy_dir.join("cron_pcie_dma.c").exists());
    assert!(out_deploy_dir.join("flash_manifest.json").exists());

    let manifest = fs::read_to_string(out_deploy_dir.join("flash_manifest.json")).expect("Read manifest");
    assert!(manifest.contains("xcu280-fsvh2892-2L-e"), "Manifest must record Alveo U280 part number");
    assert!(manifest.contains("PcieGen5x16"), "Manifest must record PCIe Gen5 interface");

    let _ = fs::remove_file(&sample_cr);
    let _ = fs::remove_dir_all(&out_deploy_dir);
}

#[test]
fn test_cli_verify_formal_safety_pass() {
    let temp_dir = std::env::temp_dir();
    let sample_cr = temp_dir.join("test_verify_pass.cr");

    let cr_content = r#"
    .MODULE VerifyPassMod
    .ENTRY main

    schedule main for "256-Core-4D-Torus" {
        distribute_4d(axis: X+, cores: 4);
        distribute_4d(axis: Y+, cores: 4);
        distribute_4d(axis: Z+, cores: 4);
        distribute_4d(axis: W+, cores: 4);
    }

    def main() -> i64 {
        let mut x: i64 = 100;
        x = 200;
        return x;
    }
    .END
    "#;

    fs::write(&sample_cr, cr_content).expect("Failed to write sample cr file");

    let output = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "verify",
            sample_cr.to_str().unwrap(),
            "--temp",
            "310.0",
            "--freq",
            "1.2",
        ])
        .output()
        .expect("Failed to execute cron verify");

    assert!(output.status.success(), "cron verify on valid code must exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DOR Deadlock-Freedom Proof:   PROVEN ACYCLIC"), "Deadlock proof must pass");
    assert!(stdout.contains("Landauer Energy Dissipation"), "Must report Landauer energy");
    assert!(stdout.contains("SSS+ PROVABLY SAFE"), "Must report SSS+ provably safe status");

    let _ = fs::remove_file(&sample_cr);
}

#[test]
fn test_cli_verify_deadlock_detection_failure() {
    let temp_dir = std::env::temp_dir();
    let sample_cr = temp_dir.join("test_verify_deadlock.cr");

    let cr_content = r#"
    .MODULE VerifyDeadlockMod
    .ENTRY main

    schedule main for "256-Core-4D-Torus" {
        distribute_4d(axis: W+, cores: 4);
        distribute_4d(axis: X+, cores: 4); // Inversion triggers CDG cycle
    }

    def main() -> i64 {
        return 0;
    }
    .END
    "#;

    fs::write(&sample_cr, cr_content).expect("Failed to write sample cr file");

    let output = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "verify",
            sample_cr.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron verify");

    assert!(!output.status.success(), "cron verify on cyclic deadlock routing must exit with non-zero error");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SAFETY HAZARDS DETECTED") || stdout.contains("Deadlock hazard"), "Must report deadlock hazard");

    let _ = fs::remove_file(&sample_cr);
}
