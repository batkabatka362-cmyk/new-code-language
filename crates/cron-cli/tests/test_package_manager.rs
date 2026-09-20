// ============================================================================
// CRON Test Suite: SSS+ Package Manager & Ecosystem (Phase 15)
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use cron_cli::pkg::lockfile::{sha256_hex, Lockfile};
use cron_cli::pkg::manifest::{DependencySource, DependencySpec, Manifest};
use cron_cli::pkg::registry::PackageRegistry;
use cron_cli::pkg::resolver::DependencyResolver;
use cron_cli::pkg::{
    add_dependency_to_project, install_project_dependencies, remove_dependency_from_project,
    verify_project_integrity,
};
use cronc::compile_source;
use std::fs;
use std::path::Path;

#[test]
fn test_sha256_cryptographic_engine() {
    // Standard test vectors
    let empty_hash = sha256_hex(b"");
    assert_eq!(
        empty_hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );

    let hello_hash = sha256_hex(b"hello world");
    assert_eq!(
        hello_hash,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}

#[test]
fn test_manifest_parsing_and_serialization() {
    let toml_sample = r#"
[package]
name = "torus_agent_app"
version = "1.0.0"
edition = "2026"
entry = "src/main.cr"
description = "High-performance SNN on 4D-Torus"
authors = ["Dev One <dev1@cron.org>", "Dev Two <dev2@cron.org>"]

[hardware]
target = "4d-torus-256-core"
optical_precision = "int8"
max_thermal_threshold = 175
sentry_watchdog = true

[dependencies]
cron_nn = "1.2.0"
local_math = { path = "../math_lib" }
cron_vision = { version = "1.0.0", features = ["dvs", "optical"] }
"#;

    let mut manifest = Manifest::parse(toml_sample).expect("Should parse valid TOML manifest");
    assert_eq!(manifest.package.name, "torus_agent_app");
    assert_eq!(manifest.package.version, "1.0.0");
    assert_eq!(manifest.hardware.target, "4d-torus-256-core");
    assert_eq!(manifest.dependencies.len(), 3);

    // Verify parsed dependencies
    let dep_math = &manifest.dependencies["local_math"];
    assert_eq!(
        dep_math.source,
        DependencySource::Path {
            path: "../math_lib".into()
        }
    );

    let dep_vis = &manifest.dependencies["cron_vision"];
    assert_eq!(dep_vis.features, vec!["dvs", "optical"]);

    // Test adding a dependency programmatically
    manifest.add_dependency(DependencySpec {
        name: "cron/audio".to_string(),
        source: DependencySource::Registry {
            version: "1.0.0".to_string(),
        },
        features: vec!["cochlea".to_string()],
    });
    assert_eq!(manifest.dependencies.len(), 4);

    // Test removing a dependency
    let removed = manifest.remove_dependency("cron_nn");
    assert!(removed);
    assert_eq!(manifest.dependencies.len(), 3);

    // Verify round-trip serialization
    let serialized = manifest.to_toml_string();
    assert!(serialized.contains("torus_agent_app"));
    assert!(serialized.contains("cron/audio"));
    assert!(!serialized.contains("cron_nn"));
}

#[test]
fn test_dependency_resolution_and_lockfile_generation() {
    let mut manifest = Manifest::new("vision_pipeline");
    manifest.add_dependency(DependencySpec {
        name: "cron/vision".to_string(),
        source: DependencySource::Registry {
            version: "1.0.0".to_string(),
        },
        features: vec![],
    });

    let registry = PackageRegistry::new();
    let resolver = DependencyResolver::new(&registry);

    // cron/vision transitively depends on cron/nn
    let res = resolver
        .resolve(&manifest, Path::new("."))
        .expect("Should resolve transitive dependencies");

    assert_eq!(res.total_packages, 2);
    assert!(res.lockfile.packages.contains_key("cron/vision"));
    assert!(res.lockfile.packages.contains_key("cron/nn"));

    let lock_text = res.lockfile.to_lock_string();
    let parsed_lock = Lockfile::parse(&lock_text).expect("Should parse generated lockfile");
    assert_eq!(parsed_lock.packages.len(), 2);
    assert_eq!(parsed_lock.packages["cron/nn"].hardware_target, "4d-torus-256-core");
    assert!(!parsed_lock.packages["cron/nn"].checksum.is_empty());
}

#[test]
fn test_circular_dependency_rejection() {
    let temp_dir = std::env::temp_dir();
    let dir_a = temp_dir.join("cron_test_pkg_a");
    let dir_b = temp_dir.join("cron_test_pkg_b");

    let _ = fs::create_dir_all(&dir_a);
    let _ = fs::create_dir_all(&dir_b);

    // Package A depends on B
    let mut man_a = Manifest::new("pkg_a");
    man_a.add_dependency(DependencySpec {
        name: "pkg_b".to_string(),
        source: DependencySource::Path {
            path: dir_b.clone(),
        },
        features: vec![],
    });
    man_a.save_to_file(dir_a.join("cron.toml")).expect("Save A");

    // Package B depends on A (Cycle!)
    let mut man_b = Manifest::new("pkg_b");
    man_b.add_dependency(DependencySpec {
        name: "pkg_a".to_string(),
        source: DependencySource::Path {
            path: dir_a.clone(),
        },
        features: vec![],
    });
    man_b.save_to_file(dir_b.join("cron.toml")).expect("Save B");

    let registry = PackageRegistry::new();
    let resolver = DependencyResolver::new(&registry);
    let result = resolver.resolve(&man_a, &dir_a);

    assert!(result.is_err(), "Must reject circular dependency");
    assert!(result.unwrap_err().contains("Circular dependency detected"));

    let _ = fs::remove_dir_all(dir_a);
    let _ = fs::remove_dir_all(dir_b);
}

#[test]
fn test_cli_add_remove_install_tree_workflow() {
    let temp_dir = std::env::temp_dir();
    let proj_dir = temp_dir.join("test_cron_project_workflow");
    let _ = fs::remove_dir_all(&proj_dir);
    fs::create_dir_all(&proj_dir).expect("Create project dir");

    // 1. Initialize project
    let manifest = Manifest::new("robotics_brain");
    manifest.save_to_file(proj_dir.join("cron.toml")).expect("Save initial manifest");

    // 2. Add dependency via package manager
    let msg = add_dependency_to_project(&proj_dir, "cron/distributed", None, vec![])
        .expect("Add dependency");
    assert!(msg.contains("cron/distributed"));

    // 3. Verify lockfile exists and has entry
    assert!(proj_dir.join("cron.lock").exists());
    let verify_res = verify_project_integrity(&proj_dir);
    assert!(verify_res.is_ok(), "Project integrity check must pass: {:?}", verify_res);

    // 4. Test render tree
    let lockfile = Lockfile::load_from_file(proj_dir.join("cron.lock")).expect("Load lockfile");
    let loaded_man = Manifest::load_from_file(proj_dir.join("cron.toml")).expect("Load manifest");
    let registry = PackageRegistry::new();
    let resolver = DependencyResolver::new(&registry);
    let tree = resolver.render_tree(&loaded_man, &lockfile);
    assert!(tree.contains("robotics_brain"));
    assert!(tree.contains("cron/distributed"));
    assert!(tree.contains("4d-torus-4096-cluster"));

    // 5. Test install
    let install_res = install_project_dependencies(&proj_dir).expect("Install dependencies");
    assert_eq!(install_res.total_packages, 1);
    assert!(proj_dir.join(".cron_packages").exists());

    // 6. Test remove
    let rem_msg = remove_dependency_from_project(&proj_dir, "cron/distributed")
        .expect("Remove dependency");
    assert!(rem_msg.contains("Removed"));

    let _ = fs::remove_dir_all(proj_dir);
}

#[test]
fn test_libcr_vision_audio_collectives_compilation() {
    let (dvs_path, cochlea_path, coll_path) = if Path::new("libcr").exists() {
        (
            Path::new("libcr/vision/dvs_stream.cr").to_path_buf(),
            Path::new("libcr/audio/cochlea.cr").to_path_buf(),
            Path::new("libcr/distributed/collectives.cr").to_path_buf(),
        )
    } else {
        (
            Path::new("../../libcr/vision/dvs_stream.cr").to_path_buf(),
            Path::new("../../libcr/audio/cochlea.cr").to_path_buf(),
            Path::new("../../libcr/distributed/collectives.cr").to_path_buf(),
        )
    };

    let dvs_code = fs::read_to_string(&dvs_path)
        .expect("Read dvs_stream.cr");
    let vliw_dvs = compile_source(&dvs_code);
    assert!(vliw_dvs.is_ok(), "dvs_stream.cr must compile cleanly: {:?}", vliw_dvs.err());

    let cochlea_code = fs::read_to_string(&cochlea_path)
        .expect("Read cochlea.cr");
    let vliw_cochlea = compile_source(&cochlea_code);
    assert!(vliw_cochlea.is_ok(), "cochlea.cr must compile cleanly: {:?}", vliw_cochlea.err());

    let collectives_code = fs::read_to_string(&coll_path)
        .expect("Read collectives.cr");
    let vliw_coll = compile_source(&collectives_code);
    assert!(vliw_coll.is_ok(), "collectives.cr must compile cleanly: {:?}", vliw_coll.err());
}
