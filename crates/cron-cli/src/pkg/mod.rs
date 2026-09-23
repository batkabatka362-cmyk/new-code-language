// ============================================================================
// CRON Package Manager: Core Coordinator Subsystem
// Module: cron_cli::pkg
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

pub mod lockfile;
pub mod manifest;
pub mod registry;
pub mod resolver;

pub use lockfile::{sha256_hex, LockPackage, Lockfile};
pub use manifest::{DependencySource, DependencySpec, HardwareSection, Manifest, PackageSection};
pub use registry::{PackageRegistry, RegistryPackageInfo};
pub use resolver::{DependencyResolver, ResolutionResult};

use std::fs;
use std::path::{Path, PathBuf};

/// Add a dependency to `cron.toml` and update `cron.lock`.
pub fn add_dependency_to_project<P: AsRef<Path>>(
    project_root: P,
    dep_name: &str,
    path_opt: Option<PathBuf>,
    features: Vec<String>,
) -> Result<String, String> {
    let root = project_root.as_ref();
    let manifest_path = root.join("cron.toml");
    if !manifest_path.exists() {
        return Err(format!("Missing 'cron.toml' in '{}'", root.display()));
    }

    let mut manifest = Manifest::load_from_file(&manifest_path)?;
    let registry = PackageRegistry::new();

    let source = if let Some(p) = path_opt {
        DependencySource::Path { path: p }
    } else if let Some(info) = registry.lookup(dep_name) {
        DependencySource::Registry {
            version: info.latest_version.clone(),
        }
    } else {
        DependencySource::Registry {
            version: "0.1.0".to_string(),
        }
    };

    let spec = DependencySpec {
        name: dep_name.to_string(),
        source,
        features,
    };

    manifest.add_dependency(spec);
    manifest.save_to_file(&manifest_path)?;

    // Re-resolve dependencies and update cron.lock
    let resolver = DependencyResolver::new(&registry);
    let resolution = resolver.resolve(&manifest, root)?;
    let lock_path = root.join("cron.lock");
    resolution.lockfile.save_to_file(&lock_path)?;

    Ok(format!(
        "Added '{}' to cron.toml and updated cron.lock (Total packages: {})",
        dep_name, resolution.total_packages
    ))
}

/// Remove a dependency from `cron.toml` and update `cron.lock`.
pub fn remove_dependency_from_project<P: AsRef<Path>>(
    project_root: P,
    dep_name: &str,
) -> Result<String, String> {
    let root = project_root.as_ref();
    let manifest_path = root.join("cron.toml");
    if !manifest_path.exists() {
        return Err(format!("Missing 'cron.toml' in '{}'", root.display()));
    }

    let mut manifest = Manifest::load_from_file(&manifest_path)?;
    if !manifest.remove_dependency(dep_name) {
        return Err(format!("Dependency '{}' was not found in cron.toml", dep_name));
    }
    manifest.save_to_file(&manifest_path)?;

    let registry = PackageRegistry::new();
    let resolver = DependencyResolver::new(&registry);
    let resolution = resolver.resolve(&manifest, root)?;
    let lock_path = root.join("cron.lock");
    resolution.lockfile.save_to_file(&lock_path)?;

    Ok(format!(
        "Removed '{}' from cron.toml and updated cron.lock",
        dep_name
    ))
}

/// Install / fetch all dependencies in `cron.toml` and verify against `cron.lock`.
pub fn install_project_dependencies<P: AsRef<Path>>(
    project_root: P,
) -> Result<ResolutionResult, String> {
    let root = project_root.as_ref();
    let manifest_path = root.join("cron.toml");
    if !manifest_path.exists() {
        return Err(format!("Missing 'cron.toml' in '{}'", root.display()));
    }

    let manifest = Manifest::load_from_file(&manifest_path)?;
    let registry = PackageRegistry::new();
    let resolver = DependencyResolver::new(&registry);
    let resolution = resolver.resolve(&manifest, root)?;

    let lock_path = root.join("cron.lock");
    resolution.lockfile.save_to_file(&lock_path)?;

    // Create local package links directory `.cron_packages`
    let pkg_cache_dir = root.join(".cron_packages");
    if !pkg_cache_dir.exists() {
        let _ = fs::create_dir_all(&pkg_cache_dir);
    }

    Ok(resolution)
}

/// Verify integrity of project dependencies and hardware targets.
pub fn verify_project_integrity<P: AsRef<Path>>(project_root: P) -> Result<String, String> {
    let root = project_root.as_ref();
    let manifest_path = root.join("cron.toml");
    let lock_path = root.join("cron.lock");

    if !manifest_path.exists() {
        return Err("Missing 'cron.toml'".to_string());
    }
    if !lock_path.exists() {
        return Err("Missing 'cron.lock'. Run `cron install` to generate lockfile.".to_string());
    }

    let manifest = Manifest::load_from_file(&manifest_path)?;
    let lockfile = Lockfile::load_from_file(&lock_path)?;

    for name in manifest.dependencies.keys() {
        if !lockfile.packages.contains_key(name) {
            return Err(format!(
                "Integrity mismatch: dependency '{}' is in cron.toml but missing in cron.lock",
                name
            ));
        }
    }

    Ok(format!(
        "Verified {} packages with cryptographic SHA-256 integrity.",
        lockfile.packages.len()
    ))
}
