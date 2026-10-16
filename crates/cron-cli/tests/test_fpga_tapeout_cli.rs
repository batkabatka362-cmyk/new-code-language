// ============================================================================
// CRON CLI FPGA & Silicon Tape-Out Deployment Integration Tests
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_fpga_synthesis_and_manifest() {
    let test_kernel = "test_fpga_kernel.cl";
    let _ = fs::remove_file(test_kernel);
    let out_dir = "test_fpga_out";
    let _ = fs::remove_dir_all(out_dir);

    let cl_code = r#"
@entry:
B0000: '==01#000> '==02#004> _MD03*120> _NO00#000>
"#;
    fs::write(test_kernel, cl_code).expect("Failed to write test kernel");

    // 1. Run cron fpga --board icebreaker --dry-run
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["fpga", test_kernel, "--board", "icebreaker", "-o", out_dir, "--dry-run"])
        .output()
        .expect("Failed to run cron fpga");

    assert!(out.status.success(), "cron fpga must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON PHYSICAL HARDWARE & FPGA DEPLOYMENT BRIDGE"));
    assert!(stdout.contains("1BitSquared iCEBreaker"));
    assert!(stdout.contains("pins.pcf"));
    assert!(stdout.contains("0 DSPs"));

    // Verify generated files on disk
    assert!(std::path::Path::new(out_dir).join("cron_top.v").exists());
    assert!(std::path::Path::new(out_dir).join("cron_uart.v").exists());
    assert!(std::path::Path::new(out_dir).join("synth_yosys.ys").exists());
    assert!(std::path::Path::new(out_dir).join("flash_manifest.json").exists());

    // 2. Run cron fpga --json
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["fpga", test_kernel, "--board", "colorlight-i5", "-o", out_dir, "--json"])
        .output()
        .expect("Failed to run cron fpga --json");

    assert!(json_out.status.success(), "cron fpga --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"package\": \"cron_silicon_deployment\""));
    assert!(json_str.contains("Colorlight i5"));
    assert!(json_str.contains("\"multiplier_free\": true"));

    let _ = fs::remove_file(test_kernel);
    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn test_cli_tapeout_sky130_manifest() {
    let test_kernel = "test_tapeout_kernel.cl";
    let _ = fs::remove_file(test_kernel);
    let out_dir = "test_tapeout_out";
    let _ = fs::remove_dir_all(out_dir);

    let cl_code = r#"
@entry:
B0000: '==01#000> '==02#004> _MD03*120> _NO00#000>
"#;
    fs::write(test_kernel, cl_code).expect("Failed to write test kernel");

    // Run cron tapeout --pdk sky130 --json
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["tapeout", test_kernel, "--pdk", "sky130", "-o", out_dir, "--json"])
        .output()
        .expect("Failed to run cron tapeout --json");

    assert!(out.status.success(), "cron tapeout --json must succeed");
    let json_str = String::from_utf8_lossy(&out.stdout);
    assert!(json_str.contains("OpenLane SkyWater 130nm ASIC"));
    assert!(json_str.contains("sky130_fd_sc_hd"));
    assert!(json_str.contains("\"dsps\": 0"));

    // Verify OpenLane artifacts
    assert!(std::path::Path::new(out_dir).join("config.json").exists());
    assert!(std::path::Path::new(out_dir).join("run_openlane.sh").exists());

    let _ = fs::remove_file(test_kernel);
    let _ = fs::remove_dir_all(out_dir);
}
