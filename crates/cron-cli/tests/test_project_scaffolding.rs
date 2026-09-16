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
entry = "src/main.cr"

[hardware]
target = "4d-torus-256-core"
optical_precision = "int8"
max_thermal_threshold = 180
sentry_watchdog = true
"#;
    fs::write(test_dir.join("cron.toml"), manifest).unwrap();

    let main_code = r#"
.MODULE NeuroAgent
.ENTRY _main

_main:
    let lin packet: wave_t = pack_wave(amp=[10, 20], phase=[0, 0])
    let out = consume(packet)
.END
"#;
    fs::write(test_dir.join("src/main.cr"), main_code).unwrap();

    assert!(test_dir.join("cron.toml").exists());
    assert!(test_dir.join("src/main.cr").exists());

    let content = fs::read_to_string(test_dir.join("src/main.cr")).unwrap();
    let cl = cronc::compile_source(&content).expect("Failed to compile scaffolded code");
    assert!(cl.contains("B0001:"));

    let _ = fs::remove_dir_all(test_dir);
}
