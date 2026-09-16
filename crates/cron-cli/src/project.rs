// ============================================================================
// CRON Project Scaffolding & Package Manifest Manager
// Handles `cron init`, `cron new`, and `cron.toml` manifest management.
// ============================================================================

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProjectManifest {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub target_arch: String,
}

impl ProjectManifest {
    #[allow(dead_code)]
    pub fn parse(toml_str: &str) -> Result<Self, String> {
        let mut name = "unnamed".to_string();
        let mut version = "0.1.0".to_string();
        let mut entry = "src/main.cr".to_string();
        let mut target_arch = "4d-torus-256-core".to_string();

        for line in toml_str.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('[') {
                continue;
            }

            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim();
                let val = v.trim().trim_matches('"').trim_matches('\'');
                match key {
                    "name" => name = val.to_string(),
                    "version" => version = val.to_string(),
                    "entry" => entry = val.to_string(),
                    "target" => target_arch = val.to_string(),
                    _ => {}
                }
            }
        }

        Ok(Self {
            name,
            version,
            entry,
            target_arch,
        })
    }

    pub fn to_toml_string(&self) -> String {
        format!(
            r#"[package]
name = "{}"
version = "{}"
authors = ["CRON Cognitive Developer <dev@cron-lang.org>"]
edition = "2026"
entry = "{}"

[hardware]
target = "{}"
optical_precision = "int8"
max_thermal_threshold = 180
sentry_watchdog = true

[dependencies]
# Standard library (libcr) modules are linked by default
"#,
            self.name, self.version, self.entry, self.target_arch
        )
    }
}

pub fn scaffold_project(target_dir: &Path, project_name: &str) -> Result<(), String> {
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)
            .map_err(|e| format!("Failed to create project directory '{:?}': {}", target_dir, e))?;
    }

    let src_dir = target_dir.join("src");
    fs::create_dir_all(&src_dir)
        .map_err(|e| format!("Failed to create 'src' directory: {}", e))?;

    let tests_dir = target_dir.join("tests");
    fs::create_dir_all(&tests_dir)
        .map_err(|e| format!("Failed to create 'tests' directory: {}", e))?;

    // 1. Generate cron.toml
    let manifest = ProjectManifest {
        name: project_name.to_string(),
        version: "0.1.0".to_string(),
        entry: "src/main.cr".to_string(),
        target_arch: "4d-torus-256-core".to_string(),
    };
    let manifest_path = target_dir.join("cron.toml");
    fs::write(&manifest_path, manifest.to_toml_string())
        .map_err(|e| format!("Failed to write cron.toml: {}", e))?;

    // 2. Generate src/main.cr
    let main_cr = format!(
        r#"// ============================================================================
// CRON Cognitive Application: {}
// Target: 256-Core 4D-Torus Neuromorphic Hardware
// ============================================================================

.MODULE Main
.ENTRY _main

def dense_forward(input_signal: i32, weight: i32) -> i32 {{
    let scaled = (input_signal * weight) / 64
    let activated = if scaled > 0 {{ scaled }} else {{ 0 }}
    return activated
}}

_main:
    // Initialize external high-bandwidth base address
    let ext_hbm_base: u32 = 0x000A_0000

    // Affine linear photonic wave packet
    let lin photon_bundle: wave_t = pack_wave(amp=[64, 32, 16, 8], phase=[0, 0, 0, 0])

    // Forward inference step
    let sensor_val = 0x00FF
    let result = dense_forward(sensor_val, 16)

    // Safely consume linear hardware resource
    let consumed_state = consume(photon_bundle)
.END
"#,
        project_name
    );
    let main_path = src_dir.join("main.cr");
    fs::write(&main_path, main_cr)
        .map_err(|e| format!("Failed to write src/main.cr: {}", e))?;

    // 3. Generate tests/test_sanity.cr
    let test_cr = r#"// Sanity integration test for CRON project
.MODULE TestSanity
.ENTRY _main

_main:
    let expected = 42
    let actual = 20 + 22
.END
"#;
    let test_path = tests_dir.join("test_sanity.cr");
    fs::write(&test_path, test_cr)
        .map_err(|e| format!("Failed to write tests/test_sanity.cr: {}", e))?;

    // 4. Generate .gitignore
    let gitignore = r#"target/
*.cl
*.v
*.prof
*.log
.DS_Store
"#;
    let gitignore_path = target_dir.join(".gitignore");
    fs::write(&gitignore_path, gitignore)
        .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

    Ok(())
}

#[allow(dead_code)]
pub fn find_project_root() -> Option<PathBuf> {
    let mut curr = std::env::current_dir().ok()?;
    loop {
        if curr.join("cron.toml").exists() {
            return Some(curr);
        }
        if !curr.pop() {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_parse_and_serialize() {
        let toml = r#"
        [package]
        name = "test_agent"
        version = "0.2.1"
        entry = "src/agent.cr"

        [hardware]
        target = "4d-torus-512-core"
        "#;

        let m = ProjectManifest::parse(toml).unwrap();
        assert_eq!(m.name, "test_agent");
        assert_eq!(m.version, "0.2.1");
        assert_eq!(m.entry, "src/agent.cr");
        assert_eq!(m.target_arch, "4d-torus-512-core");
    }
}
