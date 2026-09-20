// ============================================================================
// CRON Package Manager: Cognitive Package Registry & Cache Engine
// Module: cron_cli::pkg::registry
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use super::lockfile::sha256_hex;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RegistryPackageInfo {
    pub name: String,
    pub latest_version: String,
    pub description: String,
    pub hardware_target: String,
    pub dependencies: Vec<String>,
    pub checksum: String,
}

pub struct PackageRegistry {
    packages: BTreeMap<String, RegistryPackageInfo>,
}

impl Default for PackageRegistry {
    fn default() -> Self {
        let mut r = Self {
            packages: BTreeMap::new(),
        };
        r.seed_first_party_packages();
        r
    }
}

impl PackageRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    fn seed_first_party_packages(&mut self) {
        self.packages.insert(
            "cron/nn".to_string(),
            RegistryPackageInfo {
                name: "cron/nn".to_string(),
                latest_version: "1.2.0".to_string(),
                description: "Deep Learning, FlashAttention v3, BitNet 1.58b, and Mamba SSM primitives".to_string(),
                hardware_target: "4d-torus-256-core".to_string(),
                dependencies: vec![],
                checksum: sha256_hex(b"cron/nn@1.2.0-flash_attn-bitnet"),
            },
        );

        self.packages.insert(
            "cron/vision".to_string(),
            RegistryPackageInfo {
                name: "cron/vision".to_string(),
                latest_version: "1.0.0".to_string(),
                description: "Neuromorphic Dynamic Vision Sensor (DVS) temporal streaming & Optical Convolutions".to_string(),
                hardware_target: "4d-torus-256-core".to_string(),
                dependencies: vec!["cron/nn".to_string()],
                checksum: sha256_hex(b"cron/vision@1.0.0-dvs-mzi-optical"),
            },
        );

        self.packages.insert(
            "cron/audio".to_string(),
            RegistryPackageInfo {
                name: "cron/audio".to_string(),
                latest_version: "1.0.0".to_string(),
                description: "Neuromorphic Cochlea basilar membrane frequency bank & Optical Spectrograms".to_string(),
                hardware_target: "4d-torus-256-core".to_string(),
                dependencies: vec!["cron/nn".to_string()],
                checksum: sha256_hex(b"cron/audio@1.0.0-cochlea-snn-fft"),
            },
        );

        self.packages.insert(
            "cron/distributed".to_string(),
            RegistryPackageInfo {
                name: "cron/distributed".to_string(),
                latest_version: "2.0.0".to_string(),
                description: "4,096-Core Multi-Chip PGAS Collective Communications (all-reduce, ring-broadcast)".to_string(),
                hardware_target: "4d-torus-4096-cluster".to_string(),
                dependencies: vec![],
                checksum: sha256_hex(b"cron/distributed@2.0.0-pgas-torus-collectives"),
            },
        );

        self.packages.insert(
            "cron/crypto".to_string(),
            RegistryPackageInfo {
                name: "cron/crypto".to_string(),
                latest_version: "0.9.0".to_string(),
                description: "Landauer Zero-Dissipation Thermodynamic Reversible Hash & Quantum-Resistant Signatures".to_string(),
                hardware_target: "4d-torus-256-core".to_string(),
                dependencies: vec![],
                checksum: sha256_hex(b"cron/crypto@0.9.0-landauer-reversible-hash"),
            },
        );
    }

    pub fn lookup(&self, name: &str) -> Option<&RegistryPackageInfo> {
        self.packages.get(name)
    }

    pub fn list_all(&self) -> Vec<&RegistryPackageInfo> {
        self.packages.values().collect()
    }

    /// Package and publish a local directory project into a `.crpkg` distribution archive.
    pub fn publish_project<P: AsRef<Path>>(
        project_dir: P,
        output_pkg: Option<PathBuf>,
    ) -> Result<(PathBuf, String), String> {
        let root = project_dir.as_ref();
        let manifest_path = root.join("cron.toml");
        if !manifest_path.exists() {
            return Err(format!(
                "Cannot publish: missing 'cron.toml' in '{}'",
                root.display()
            ));
        }

        let manifest = super::manifest::Manifest::load_from_file(&manifest_path)?;
        let pkg_filename = format!(
            "{}-{}.crpkg",
            manifest.package.name.replace('/', "_"),
            manifest.package.version
        );

        let dest = output_pkg.unwrap_or_else(|| root.join(&pkg_filename));

        // Aggregate project files: cron.toml, src/**/*.cr
        let mut bundle_data = Vec::new();
        let manifest_bytes = fs::read(&manifest_path)
            .map_err(|e| format!("Failed reading manifest: {}", e))?;
        bundle_data.extend_from_slice(&manifest_bytes);

        let src_dir = root.join("src");
        if src_dir.exists() {
            collect_dir_bytes(&src_dir, &mut bundle_data)?;
        }

        let checksum = sha256_hex(&bundle_data);
        fs::write(&dest, &bundle_data)
            .map_err(|e| format!("Failed writing package bundle '{}': {}", dest.display(), e))?;

        Ok((dest, checksum))
    }
}

fn collect_dir_bytes(dir: &Path, out: &mut Vec<u8>) -> Result<(), String> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                collect_dir_bytes(&p, out)?;
            } else if p.is_file() {
                if let Ok(data) = fs::read(&p) {
                    out.extend_from_slice(&data);
                }
            }
        }
    }
    Ok(())
}
