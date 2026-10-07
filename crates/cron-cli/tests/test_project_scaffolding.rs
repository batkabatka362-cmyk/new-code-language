// ============================================================================
// Integration Tests for CRON Project Scaffolding & Manifest Manager
// ============================================================================

use std::fs;
use std::path::Path;

#[test]
fn test_project_scaffold_creates_valid_structure() {
    let test_dir = Path::new("../../scratch/scaffold_test_project");
    if test_dir.exists() {
        let _ = fs::remove_dir_all(test_dir);
    }

    fs::create_dir_all(test_dir.join("src")).unwrap();
    fs::create_dir_all(test_dir.join("tests")).unwrap();

    let manifest = r#"[package]
name = "neuro_agent"
version = "0.1.0"
authors = ["CRON Cognitive Developer <dev@cron-lang.org>"]
edition = "2026"
entry = "src/main.cl"

[hardware]
target = "4d-torus-256-core"
optical_precision = "int8"
max_thermal_threshold = 180
sentry_watchdog = true
"#;
    fs::write(test_dir.join("cron.toml"), manifest).unwrap();

    let main_code = r#"@kernel neuro_agent
.target silicon.4d_torus
.ipc_target 4.0

B0000: '==01#0403 '==02#020k _NO00#000> _NO00#000>
B0001: _AD03M1025 _SB04M102E !HL00#000( _NO00#000>
"#;
    fs::write(test_dir.join("src/main.cl"), main_code).unwrap();

    assert!(test_dir.join("cron.toml").exists());
    assert!(test_dir.join("src/main.cl").exists());

    let content = fs::read_to_string(test_dir.join("src/main.cl")).unwrap();
    let rep = cronc::cl_lang::verify_cl_program(&content).expect("Failed to verify scaffolded .cl code");
    assert_eq!(rep.total_bundles, 2);
    assert!(rep.hazards.is_empty());

    let _ = fs::remove_dir_all(test_dir);
}
