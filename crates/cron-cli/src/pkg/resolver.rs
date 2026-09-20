// ============================================================================
// CRON Package Manager: SSS+ Dependency Resolver & Graph Analyzer
// Module: cron_cli::pkg::resolver
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use super::lockfile::{sha256_hex, LockPackage, Lockfile};
use super::manifest::{DependencySource, Manifest};
use super::registry::PackageRegistry;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ResolutionResult {
    pub lockfile: Lockfile,
    pub total_packages: usize,
    pub resolved_order: Vec<String>,
}

pub struct DependencyResolver<'a> {
    registry: &'a PackageRegistry,
}

impl<'a> DependencyResolver<'a> {
    pub fn new(registry: &'a PackageRegistry) -> Self {
        Self { registry }
    }

    /// Resolve all direct and transitive dependencies from a manifest into a deterministic Lockfile.
    pub fn resolve(&self, root_manifest: &Manifest, project_root: &Path) -> Result<ResolutionResult, String> {
        let mut resolved = BTreeMap::new();
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        let mut order = Vec::new();

        for (dep_name, dep_spec) in &root_manifest.dependencies {
            self.resolve_node(
                dep_name,
                dep_spec.source.clone(),
                project_root,
                &mut visiting,
                &mut visited,
                &mut resolved,
                &mut order,
            )?;
        }

        let total_packages = resolved.len();
        let lockfile = Lockfile {
            lock_version: 1,
            packages: resolved,
        };

        Ok(ResolutionResult {
            lockfile,
            total_packages,
            resolved_order: order,
        })
    }

    fn resolve_node(
        &self,
        name: &str,
        source: DependencySource,
        project_root: &Path,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        resolved: &mut BTreeMap<String, LockPackage>,
        order: &mut Vec<String>,
    ) -> Result<(), String> {
        if visiting.contains(name) {
            return Err(format!(
                "Circular dependency detected involving package '{}'",
                name
            ));
        }

        if visited.contains(name) {
            return Ok(());
        }

        visiting.insert(name.to_string());

        let mut children = Vec::new();
        let version;
        let checksum;
        let source_str;
        let hardware_target;

        match source {
            DependencySource::Registry { version: req_ver } => {
                source_str = "registry".to_string();
                if let Some(info) = self.registry.lookup(name) {
                    version = info.latest_version.clone();
                    checksum = info.checksum.clone();
                    hardware_target = info.hardware_target.clone();
                    for child in &info.dependencies {
                        children.push((child.clone(), DependencySource::Registry { version: "latest".to_string() }));
                    }
                } else {
                    // Fallback generic registry resolution
                    version = req_ver;
                    checksum = sha256_hex(format!("{}@{}", name, version).as_bytes());
                    hardware_target = "4d-torus-256-core".to_string();
                }
            }
            DependencySource::Path { path } => {
                source_str = format!("path+{}", path.display());
                let full_path = if path.is_absolute() {
                    path.clone()
                } else {
                    project_root.join(&path)
                };

                let manifest_path = full_path.join("cron.toml");
                if manifest_path.exists() {
                    let sub_manifest = Manifest::load_from_file(&manifest_path)?;
                    version = sub_manifest.package.version;
                    hardware_target = sub_manifest.hardware.target;
                    checksum = sha256_hex(format!("{}-{}", name, version).as_bytes());

                    for (c_name, c_spec) in sub_manifest.dependencies {
                        children.push((c_name, c_spec.source));
                    }
                } else {
                    version = "0.1.0".to_string();
                    hardware_target = "4d-torus-256-core".to_string();
                    checksum = sha256_hex(format!("local-path-{}", name).as_bytes());
                }
            }
            DependencySource::Builtin { version: v } => {
                source_str = "builtin".to_string();
                version = v;
                checksum = sha256_hex(format!("libcr-{}", name).as_bytes());
                hardware_target = "4d-torus-256-core".to_string();
            }
        }

        let child_names: Vec<String> = children.iter().map(|(c, _)| c.clone()).collect();

        // Recursively resolve children
        for (child_name, child_source) in children {
            self.resolve_node(
                &child_name,
                child_source,
                project_root,
                visiting,
                visited,
                resolved,
                order,
            )?;
        }

        visiting.remove(name);
        visited.insert(name.to_string());
        order.push(name.to_string());

        resolved.insert(
            name.to_string(),
            LockPackage {
                name: name.to_string(),
                version,
                source: source_str,
                checksum,
                hardware_target,
                dependencies: child_names,
            },
        );

        Ok(())
    }

    /// Format a printable visual dependency tree.
    pub fn render_tree(&self, root_manifest: &Manifest, lockfile: &Lockfile) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "📦 {} v{} ({})\n",
            root_manifest.package.name, root_manifest.package.version, root_manifest.hardware.target
        ));

        let keys: Vec<_> = root_manifest.dependencies.keys().collect();
        for (i, key) in keys.iter().enumerate() {
            let is_last = i == keys.len() - 1;
            let prefix = if is_last { "└── " } else { "├── " };
            let sub_prefix = if is_last { "    " } else { "│   " };

            if let Some(pkg) = lockfile.packages.get(*key) {
                out.push_str(&format!(
                    "{}{} v{} [{}] (target: {})\n",
                    prefix, pkg.name, pkg.version, pkg.source, pkg.hardware_target
                ));
                for (j, dep) in pkg.dependencies.iter().enumerate() {
                    let is_dep_last = j == pkg.dependencies.len() - 1;
                    let dep_branch = if is_dep_last { "└── " } else { "├── " };
                    if let Some(child_pkg) = lockfile.packages.get(dep) {
                        out.push_str(&format!(
                            "{}{}{} v{}\n",
                            sub_prefix, dep_branch, child_pkg.name, child_pkg.version
                        ));
                    }
                }
            } else {
                out.push_str(&format!("{}{} (unresolved)\n", prefix, key));
            }
        }

        out
    }
}
