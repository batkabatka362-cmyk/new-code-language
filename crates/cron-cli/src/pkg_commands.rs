// ============================================================================
// CRON CLI Package Manager Commands
// Module: cron_cli::pkg_commands
// ============================================================================

use crate::pkg;
use std::env;

pub fn handle_pkg_command(args: &[String]) {
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        println!("================================================================================");
        println!(" CRON PACKAGE MANAGER (cron pkg)");
        println!("================================================================================");
        println!("USAGE:");
        println!("    cron pkg <subcommand> [options]");
        println!();
        println!("SUBCOMMANDS:");
        println!("    init <name> [dir]    Initialize a new CRON package project");
        println!("    add <dep>            Add a dependency to cron.toml and update lockfile");
        println!("    remove <dep>         Remove a dependency from cron.toml");
        println!("    install              Resolve and fetch dependencies from registry");
        println!("    build                Build all .cl kernels in project");
        println!("    test                 Run tests in tests/ directory");
        println!("    tree, list           Display dependency tree graph");
        println!("    verify               Cryptographically verify SHA-256 package checksums");
        println!("    search [query]       Search online CRON package registry");
        println!("    publish              Package and publish project to registry bundle");
        return;
    }

    let cur_dir = env::current_dir().unwrap();
    match args[0].as_str() {
        "init" | "new" => {
            let name = if args.len() >= 2 { &args[1] } else { "cron_project" };
            let target_dir = if args.len() >= 3 {
                cur_dir.join(&args[2])
            } else if args.len() == 2 && name != "." {
                cur_dir.join(name)
            } else {
                cur_dir.clone()
            };
            match pkg::init_project(&target_dir, name) {
                Ok(msg) => println!("[SUCCESS] {}", msg),
                Err(e) => {
                    eprintln!("[ERROR] Init failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "add" => {
            if args.len() < 2 {
                eprintln!("Usage: cron pkg add <dependency-name>");
                std::process::exit(1);
            }
            let dep_name = &args[1];
            match pkg::add_dependency_to_project(&cur_dir, dep_name, None, Vec::new()) {
                Ok(msg) => println!("[SUCCESS] {}", msg),
                Err(e) => {
                    eprintln!("[ERROR] Add failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "remove" | "rm" => {
            if args.len() < 2 {
                eprintln!("Usage: cron pkg remove <dependency-name>");
                std::process::exit(1);
            }
            let dep_name = &args[1];
            match pkg::remove_dependency_from_project(&cur_dir, dep_name) {
                Ok(msg) => println!("[SUCCESS] {}", msg),
                Err(e) => {
                    eprintln!("[ERROR] Remove failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "install" | "restore" => {
            match pkg::install_project_dependencies(&cur_dir) {
                Ok(res) => {
                    println!("[SUCCESS] Resolved and installed {} package(s)", res.total_packages);
                    let lock_path = cur_dir.join("cron.lock");
                    println!("  Updated lockfile: '{}'", lock_path.display());
                }
                Err(e) => {
                    eprintln!("[ERROR] Install failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "build" => {
            match pkg::build_project(&cur_dir) {
                Ok(msg) => println!("[BUILD SUCCESS] {}", msg),
                Err(e) => {
                    eprintln!("[BUILD ERROR] {}", e);
                    std::process::exit(1);
                }
            }
        }
        "test" => {
            match pkg::test_project(&cur_dir) {
                Ok(msg) => println!("[TEST SUCCESS] {}", msg),
                Err(e) => {
                    eprintln!("[TEST ERROR] {}", e);
                    std::process::exit(1);
                }
            }
        }
        "tree" | "list" => {
            let manifest_path = cur_dir.join("cron.toml");
            let lock_path = cur_dir.join("cron.lock");
            if !manifest_path.exists() {
                eprintln!("[ERROR] Missing 'cron.toml' in current directory");
                std::process::exit(1);
            }
            let manifest = match pkg::Manifest::load_from_file(&manifest_path) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("[ERROR] {}", e);
                    std::process::exit(1);
                }
            };
            let registry = pkg::PackageRegistry::new();
            let resolver = pkg::DependencyResolver::new(&registry);
            let lockfile = if lock_path.exists() {
                pkg::Lockfile::load_from_file(&lock_path).unwrap_or_default()
            } else {
                match resolver.resolve(&manifest, &cur_dir) {
                    Ok(res) => res.lockfile,
                    Err(e) => {
                        eprintln!("[ERROR] Resolution failed: {}", e);
                        std::process::exit(1);
                    }
                }
            };
            println!("{}", resolver.render_tree(&manifest, &lockfile));
        }
        "publish" => {
            println!("[PUBLISH] Packaging CRON project into distribution bundle...");
            match pkg::PackageRegistry::publish_project(&cur_dir, None) {
                Ok((bundle_path, checksum)) => {
                    println!("[SUCCESS] Package published: '{}'", bundle_path.display());
                    println!("  SHA-256 Checksum: {}", checksum);
                }
                Err(e) => {
                    eprintln!("[ERROR] Publish failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "verify" => {
            match pkg::verify_project_integrity(&cur_dir) {
                Ok(msg) => println!("[VERIFY SUCCESS] {}", msg),
                Err(e) => {
                    eprintln!("[VERIFY FAILED] {}", e);
                    std::process::exit(1);
                }
            }
        }
        "search" => {
            let query = if args.len() >= 2 { &args[1] } else { "" };
            let registry = pkg::PackageRegistry::new();
            let matches = registry.search_packages(query);
            println!("[CRON PACKAGE REGISTRY] Found {} matching package(s):", matches.len());
            println!("{:<20} {:<10} {:<24} DESCRIPTION", "PACKAGE", "VERSION", "TARGET");
            println!("{:-<20} {:-<10} {:-<24} {:-<40}", "", "", "", "");
            for pkg in matches {
                println!("{:<20} {:<10} {:<24} {}", pkg.name, pkg.latest_version, pkg.hardware_target, pkg.description);
            }
        }
        other => {
            eprintln!("Unknown pkg subcommand '{}'. Valid: init, add, remove, install, build, test, tree, list, publish, verify, search", other);
            std::process::exit(1);
        }
    }
}
