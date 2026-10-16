// ============================================================================
// CRON CLI WebAssembly Playground Integration Tests
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_playground_bundle_standalone_html() {
    let out_file = "test_cli_playground.html";
    let _ = fs::remove_file(out_file);

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["playground", "--bundle", "-o", out_file])
        .output()
        .expect("Failed to run cron playground --bundle");

    assert!(out.status.success(), "cron playground --bundle must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON WEBASSEMBLY IN-BROWSER PLAYGROUND"));
    assert!(stdout.contains("Standalone Single-File WebAssembly Playground Created"));

    assert!(std::path::Path::new(out_file).exists());
    let html_content = fs::read_to_string(out_file).expect("Failed to read bundled html");
    assert!(html_content.contains("<!DOCTYPE html>"));
    assert!(html_content.contains("CRON Interactive Playground"));
    assert!(html_content.contains("<style>"));
    assert!(html_content.contains("<script>"));
    assert!(html_content.contains("4D-Torus"));

    let _ = fs::remove_file(out_file);
}

#[test]
fn test_cli_playground_step_simulation() {
    let test_kernel = "test_pg_kernel.cl";
    let _ = fs::remove_file(test_kernel);

    let cl_code = r#"
@entry:
B0000: '==01#000> '==02#004> _MD03*120> _NO00#000>
"#;
    fs::write(test_kernel, cl_code).expect("Failed to write test kernel");

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["cl-playground", test_kernel, "--cycles", "5"])
        .output()
        .expect("Failed to run cron cl-playground");

    assert!(out.status.success(), "cron cl-playground must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON 256-CORE MICROCODE INTERACTIVE PLAYGROUND"));
    assert!(stdout.contains("HARDWARE REGISTERS (R0..RF)"));
    assert!(stdout.contains("NOMINAL 4.0 IPC"));

    let _ = fs::remove_file(test_kernel);
}
