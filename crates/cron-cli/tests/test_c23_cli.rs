// ============================================================================
// Integration Tests for CRON CLI C23 & Native Binary Compilation
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_c23_transpilation() {
    let temp_dir = std::env::temp_dir();
    let sample_cr = temp_dir.join("test_sample.cr");
    let out_c = temp_dir.join("test_sample_out.c");

    // Use a pattern where variables are consumed to avoid dead-code elimination
    let cr_content = r#"
    .MODULE SampleMath
    .ENTRY _main

    def add_values(a: i32, b: i32) -> i32 {
        let sum = a + b
        return sum
    }

    _main:
        let val_a: i32 = 100
        let val_b: i32 = 250
        let val_sum: i32 = add_values(val_a, val_b)
    .END
    "#;

    fs::write(&sample_cr, cr_content).expect("write sample.cr");

    // Invoke cron c23
    let status = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "c23",
            sample_cr.to_str().unwrap(),
            "-o",
            out_c.to_str().unwrap(),
        ])
        .status()
        .expect("run cron c23");

    assert!(status.success(), "cron c23 should exit 0");
    assert!(out_c.exists(), "out_c file must exist");

    let c_text = fs::read_to_string(&out_c).expect("read generated c");
    assert!(c_text.contains("add_values"), "Must contain transpiled function");
    assert!(c_text.contains("[CRON Native C23 Engine]"), "Must contain runtime banner");

    let _ = fs::remove_file(&sample_cr);
    let _ = fs::remove_file(&out_c);
}
