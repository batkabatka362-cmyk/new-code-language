// ============================================================================
// CRON Package Manager: Manifest (cron.toml) Engine
// Module: cron_cli::pkg::manifest
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencySource {
    Registry { version: String },
    Path { path: PathBuf },
    Builtin { version: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencySpec {
    pub name: String,
    pub source: DependencySource,
    pub features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PackageSection {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub edition: String,
    pub entry: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct HardwareSection {
    pub target: String,
    pub optical_precision: String,
    pub max_thermal_threshold: u32,
    pub sentry_watchdog: bool,
}

#[derive(Debug, Clone)]
pub struct Manifest {
    pub package: PackageSection,
    pub hardware: HardwareSection,
    pub dependencies: BTreeMap<String, DependencySpec>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            package: PackageSection {
                name: "unnamed_cron_app".to_string(),
                version: "0.1.0".to_string(),
                authors: vec!["CRON Cognitive Developer <dev@cron-lang.org>".to_string()],
                edition: "2026".to_string(),
                entry: "src/main.cr".to_string(),
                description: "Cognitive Neural Application for 4D-Torus Silicon".to_string(),
            },
            hardware: HardwareSection {
                target: "4d-torus-256-core".to_string(),
                optical_precision: "int8".to_string(),
                max_thermal_threshold: 180,
                sentry_watchdog: true,
            },
            dependencies: BTreeMap::new(),
        }
    }
}

impl Manifest {
    pub fn new(name: &str) -> Self {
        let mut m = Self::default();
        m.package.name = name.to_string();
        m
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read manifest file '{}': {}", path.as_ref().display(), e))?;
        Self::parse(&content)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let toml = self.to_toml_string();
        fs::write(path.as_ref(), toml)
            .map_err(|e| format!("Failed to write manifest file '{}': {}", path.as_ref().display(), e))
    }

    pub fn parse(toml_str: &str) -> Result<Self, String> {
        let mut manifest = Self::default();
        let mut current_section = "";

        for line in toml_str.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = &trimmed[1..trimmed.len() - 1].trim();
                continue;
            }

            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim();
                let raw_val = v.trim();

                match current_section {
                    "package" => match key {
                        "name" => manifest.package.name = clean_string(raw_val),
                        "version" => manifest.package.version = clean_string(raw_val),
                        "entry" => manifest.package.entry = clean_string(raw_val),
                        "edition" => manifest.package.edition = clean_string(raw_val),
                        "description" => manifest.package.description = clean_string(raw_val),
                        "authors" => manifest.package.authors = parse_string_list(raw_val),
                        _ => {}
                    },
                    "hardware" => match key {
                        "target" => manifest.hardware.target = clean_string(raw_val),
                        "optical_precision" => manifest.hardware.optical_precision = clean_string(raw_val),
                        "max_thermal_threshold" => {
                            if let Ok(n) = raw_val.parse::<u32>() {
                                manifest.hardware.max_thermal_threshold = n;
                            }
                        }
                        "sentry_watchdog" => {
                            manifest.hardware.sentry_watchdog = raw_val == "true";
                        }
                        _ => {}
                    },
                    "dependencies" => {
                        let dep = parse_dependency(key, raw_val)?;
                        manifest.dependencies.insert(key.to_string(), dep);
                    }
                    _ => {}
                }
            }
        }

        Ok(manifest)
    }

    pub fn add_dependency(&mut self, spec: DependencySpec) {
        self.dependencies.insert(spec.name.clone(), spec);
    }

    pub fn remove_dependency(&mut self, name: &str) -> bool {
        self.dependencies.remove(name).is_some()
    }

    pub fn to_toml_string(&self) -> String {
        let mut out = String::new();
        out.push_str("[package]\n");
        out.push_str(&format!("name = \"{}\"\n", self.package.name));
        out.push_str(&format!("version = \"{}\"\n", self.package.version));
        out.push_str(&format!("edition = \"{}\"\n", self.package.edition));
        out.push_str(&format!("entry = \"{}\"\n", self.package.entry));
        out.push_str(&format!("description = \"{}\"\n", self.package.description));
        let authors_str = self
            .package
            .authors
            .iter()
            .map(|a| format!("\"{}\"", a))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!("authors = [{}]\n\n", authors_str));

        out.push_str("[hardware]\n");
        out.push_str(&format!("target = \"{}\"\n", self.hardware.target));
        out.push_str(&format!("optical_precision = \"{}\"\n", self.hardware.optical_precision));
        out.push_str(&format!("max_thermal_threshold = {}\n", self.hardware.max_thermal_threshold));
        out.push_str(&format!("sentry_watchdog = {}\n\n", self.hardware.sentry_watchdog));

        out.push_str("[dependencies]\n");
        for (name, dep) in &self.dependencies {
            match &dep.source {
                DependencySource::Registry { version } => {
                    if dep.features.is_empty() {
                        out.push_str(&format!("{} = \"{}\"\n", name, version));
                    } else {
                        let feat_str = dep
                            .features
                            .iter()
                            .map(|f| format!("\"{}\"", f))
                            .collect::<Vec<_>>()
                            .join(", ");
                        out.push_str(&format!(
                            "{} = {{ version = \"{}\", features = [{}] }}\n",
                            name, version, feat_str
                        ));
                    }
                }
                DependencySource::Path { path } => {
                    out.push_str(&format!(
                        "{} = {{ path = \"{}\" }}\n",
                        name,
                        path.display().to_string().replace('\\', "/")
                    ));
                }
                DependencySource::Builtin { version } => {
                    out.push_str(&format!("{} = {{ builtin = true, version = \"{}\" }}\n", name, version));
                }
            }
        }

        out
    }
}

fn clean_string(s: &str) -> String {
    s.trim().trim_matches('"').trim_matches('\'').to_string()
}

fn parse_string_list(s: &str) -> Vec<String> {
    let inner = s.trim().trim_start_matches('[').trim_end_matches(']');
    inner
        .split(',')
        .map(|item| clean_string(item))
        .filter(|item| !item.is_empty())
        .collect()
}

fn split_top_level_commas(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut bracket_depth = 0;
    let mut in_quote = false;

    for c in s.chars() {
        if c == '"' {
            in_quote = !in_quote;
            current.push(c);
        } else if !in_quote && (c == '[' || c == '{') {
            bracket_depth += 1;
            current.push(c);
        } else if !in_quote && (c == ']' || c == '}') {
            bracket_depth -= 1;
            current.push(c);
        } else if !in_quote && bracket_depth == 0 && c == ',' {
            parts.push(current.trim().to_string());
            current.clear();
        } else {
            current.push(c);
        }
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}

fn parse_dependency(name: &str, raw: &str) -> Result<DependencySpec, String> {
    let trimmed = raw.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        // Table format: { path = "...", version = "...", features = [...] }
        let inner = &trimmed[1..trimmed.len() - 1];
        let mut path = None;
        let mut version = "0.1.0".to_string();
        let mut is_builtin = false;
        let mut features = Vec::new();

        for part in split_top_level_commas(inner) {
            if let Some((k, v)) = part.split_once('=') {
                let k = k.trim();
                let v = v.trim();
                match k {
                    "path" => path = Some(PathBuf::from(clean_string(v))),
                    "version" => version = clean_string(v),
                    "builtin" => is_builtin = v == "true",
                    "features" => features = parse_string_list(v),
                    _ => {}
                }
            }
        }

        let source = if let Some(p) = path {
            DependencySource::Path { path: p }
        } else if is_builtin {
            DependencySource::Builtin { version }
        } else {
            DependencySource::Registry { version }
        };

        Ok(DependencySpec {
            name: name.to_string(),
            source,
            features,
        })
    } else {
        // Simple version string format: "1.0.0"
        let version = clean_string(trimmed);
        Ok(DependencySpec {
            name: name.to_string(),
            source: DependencySource::Registry { version },
            features: Vec::new(),
        })
    }
}
