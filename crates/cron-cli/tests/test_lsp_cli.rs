// ============================================================================
// CRON CLI Language Server Protocol (LSP 3.17) & DAP Integration Tests
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_lsp_version() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["lsp", "--version"])
        .output()
        .expect("Failed to run cron lsp --version");

    assert!(out.status.success(), "cron lsp --version must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("cron-lsp 3.17.0"));
    assert!(stdout.contains("CRON Cognitive Language Server Protocol Engine"));
}

#[test]
fn test_cli_dap_version() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["dap", "--version"])
        .output()
        .expect("Failed to run cron dap --version");

    assert!(out.status.success(), "cron dap --version must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("cron-dap 1.0.0"));
    assert!(stdout.contains("CRON Debug Adapter Protocol Engine"));
}

#[test]
fn test_cli_cl_check_offline_diagnostics() {
    let test_kernel = "test_lsp_diag.cl";
    let _ = fs::remove_file(test_kernel);

    // Valid VLIW code
    let cl_code = r#"
@entry:
B0000: '==01#000> '==02#004> _MD03*120> _NO00#000>
"#;
    fs::write(test_kernel, cl_code).expect("Failed to write test kernel");

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["cl-check", test_kernel])
        .output()
        .expect("Failed to run cron cl-check");

    assert!(out.status.success(), "cron cl-check must succeed");

    let _ = fs::remove_file(test_kernel);
}
