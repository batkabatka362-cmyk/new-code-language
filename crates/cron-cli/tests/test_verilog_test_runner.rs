// ============================================================================
// CRON CLI Physical Verilog Testbench & Co-Simulation Test Suite
// Module: cron-cli::tests::test_verilog_test_runner
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_test_help_includes_verilog() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["test", "--help"])
        .output()
        .expect("run cron test --help");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("cron test"));
    assert!(stdout.contains("--verilog"));
    assert!(stdout.contains("--cycles"));
    assert!(stdout.contains("--strict"));
}

#[test]
fn test_cli_test_verilog_single_kernel() {
    let temp_dir = std::env::temp_dir().join("cron_test_verilog_single");
    let _ = fs::create_dir_all(&temp_dir);
    let cl_file = temp_dir.join("mini_alu.cl");
    let out_dir = temp_dir.join("synth_verilog");

    // Standard 40-character 4-slot VLIW microcode bundles with CRC8
    let cl_code = r#"
.core [0, 0, 0, 0]:
@entry:
B0000: _NO00#070> _NO00#070> _NO00#070> _NO00#070>
B0001: _AD00$1F1> _NO00#070> _NO00#070> _HL00#0E1!
"#;
    fs::write(&cl_file, cl_code).expect("Write mini_alu.cl");

    // 1. Run cron test --verilog
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "test",
            cl_file.to_str().unwrap(),
            "--verilog",
            "-o",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("run cron test --verilog");

    assert!(out.status.success(), "cron test --verilog should succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON PHYSICAL VERILOG RTL TESTBENCH & CO-SIMULATION"));
    assert!(stdout.contains("[PASS]"));
    assert!(stdout.contains("VERILOG RTL COSIM"));
    assert!(stdout.contains("0 DSPs"));
    assert!(stdout.contains("100.000% Bit-Exact"));

    // 2. Verify synthesized Verilog HDL and Testbench on disk
    let rtl_path = out_dir.join("cksl_mini_alu_core.v");
    let tb_path = out_dir.join("tb_cksl_mini_alu_core.v");
    assert!(rtl_path.exists(), "Synthesized Verilog core must exist");
    assert!(tb_path.exists(), "Synthesized Verilog testbench must exist");

    let rtl_content = fs::read_to_string(&rtl_path).expect("Read rtl");
    assert!(rtl_content.contains("module cksl_mini_alu_core"));
    assert!(rtl_content.contains("endmodule"));

    let tb_content = fs::read_to_string(&tb_path).expect("Read tb");
    assert!(tb_content.contains("module tb_cksl_mini_alu_core"));
    assert!(tb_content.contains("initial begin"));

    // 3. Run with --json output
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "test",
            cl_file.to_str().unwrap(),
            "--verilog",
            "--json",
        ])
        .output()
        .expect("run cron test --verilog --json");

    assert!(json_out.status.success());
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"dsp_multipliers\":0"));
    assert!(json_str.contains("\"parity\":true"));
    assert!(json_str.contains("\"divergences\":0"));

    // Cleanup
    let _ = fs::remove_file(&cl_file);
    let _ = fs::remove_file(&rtl_path);
    let _ = fs::remove_file(&tb_path);
    let _ = fs::remove_dir(&out_dir);
    let _ = fs::remove_dir(&temp_dir);
}
