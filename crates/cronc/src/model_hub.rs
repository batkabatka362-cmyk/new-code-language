// ============================================================================
// CRON Compiler: Hugging Face Model Hub & Open-Weight Runner Engine
// Module: cronc::model_hub
// (C) 2026 CRON Language Project - 100% Native Pure Rust
// ============================================================================

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Model Metadata Entry in the CRON Open-Weight Catalog
#[derive(Debug, Clone)]
pub struct HfModel {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub repo_id: &'static str,
    pub default_file: &'static str,
    pub size_mb: usize,
    pub param_count: &'static str,
    pub precision: &'static str,
    pub supports_reasoning: bool,
    pub default_mode: &'static str, // "ternary", "float", or "hybrid"
    pub description: &'static str,
}

/// Catalog of curated open-weight frontier AI models optimized for CRON
pub fn get_model_catalog() -> Vec<HfModel> {
    vec![
        HfModel {
            name: "BitNet b1.58 Large (2B)",
            aliases: &["bitnet", "bitnet-2b", "bitnet-b1.58", "1bit"],
            repo_id: "1bitLLM/bitnet_b1_58-large",
            default_file: "model.safetensors",
            size_mb: 720,
            param_count: "2.0B",
            precision: "Ternary {-1, 0, +1} 1.58-bit",
            supports_reasoning: false,
            default_mode: "ternary",
            description: "Microsoft 1.58-bit ternary foundation model, 100% multiplier-free 0 FLOPs.",
        },
        HfModel {
            name: "DeepSeek-R1 Distill Qwen (1.5B)",
            aliases: &["deepseek-r1", "deepseek", "r1", "r1-1.5b"],
            repo_id: "unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF",
            default_file: "DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.gguf",
            size_mb: 1120,
            param_count: "1.5B",
            precision: "GGUF Q4_K_M (Int4/Int8 Hybrid)",
            supports_reasoning: true,
            default_mode: "hybrid",
            description: "DeepSeek-R1 test-time reasoning model with <think> trajectory streaming.",
        },
        HfModel {
            name: "Meta Llama-3.2 (1B Instruct)",
            aliases: &["llama3", "llama-3.2", "llama", "llama-1b"],
            repo_id: "bartowski/Llama-3.2-1B-Instruct-GGUF",
            default_file: "Llama-3.2-1B-Instruct-Q4_K_M.gguf",
            size_mb: 810,
            param_count: "1.2B",
            precision: "GGUF Q4_K_M",
            supports_reasoning: false,
            default_mode: "float",
            description: "Meta lightweight mobile & edge instruction-tuned transformer.",
        },
        HfModel {
            name: "HuggingFace SmolLM2 (135M)",
            aliases: &["smollm", "smollm2", "smol", "edge"],
            repo_id: "HuggingFaceTB/SmolLM2-135M",
            default_file: "model.safetensors",
            size_mb: 270,
            param_count: "135M",
            precision: "SafeTensors BF16/FP16",
            supports_reasoning: false,
            default_mode: "hybrid",
            description: "Ultra-fast lightweight edge model for instant real-time CPU inference.",
        },
        HfModel {
            name: "TinyLlama (1.1B Chat)",
            aliases: &["tinyllama", "tiny", "llama-tiny"],
            repo_id: "TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF",
            default_file: "tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf",
            size_mb: 670,
            param_count: "1.1B",
            precision: "GGUF Q4_K_M",
            supports_reasoning: false,
            default_mode: "hybrid",
            description: "Compact 1.1B parameter Llama-architecture chat model.",
        },
    ]
}

/// Resolve a query string (either alias like "deepseek-r1" or repo "org/model")
pub fn resolve_model(query: &str) -> Option<HfModel> {
    let q = query.trim().to_lowercase();
    let catalog = get_model_catalog();

    // 1. Exact alias match
    for m in &catalog {
        if m.aliases.iter().any(|&a| a == q) {
            return Some(m.clone());
        }
    }

    // 2. Exact repo ID match
    for m in &catalog {
        if m.repo_id.to_lowercase() == q {
            return Some(m.clone());
        }
    }

    // 3. Partial name match
    for m in &catalog {
        if m.name.to_lowercase().contains(&q) || m.repo_id.to_lowercase().contains(&q) {
            return Some(m.clone());
        }
    }

    // 4. If query looks like "org/model", construct dynamic model entry
    if query.contains('/') {
        let is_gguf = query.to_lowercase().contains("gguf");
        let default_file = if is_gguf { "model.gguf" } else { "model.safetensors" };
        let is_r1 = query.to_lowercase().contains("r1") || query.to_lowercase().contains("reason");
        return Some(HfModel {
            name: "Custom HuggingFace Model",
            aliases: &[],
            repo_id: Box::leak(query.to_string().into_boxed_str()),
            default_file,
            size_mb: 1000,
            param_count: "Custom",
            precision: if is_gguf { "GGUF" } else { "SafeTensors" },
            supports_reasoning: is_r1,
            default_mode: "hybrid",
            description: "User-specified custom Hugging Face repository.",
        });
    }

    None
}

/// Get the system cache directory for CRON models (~/.cron/models)
pub fn get_cache_dir() -> PathBuf {
    if let Some(user_home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        let p = PathBuf::from(user_home).join(".cron").join("models");
        let _ = fs::create_dir_all(&p);
        p
    } else {
        let p = PathBuf::from(".cron_cache").join("models");
        let _ = fs::create_dir_all(&p);
        p
    }
}

/// Calculate the local destination path for a given model repo and filename
pub fn get_local_model_path(repo_id: &str, filename: &str) -> PathBuf {
    let sanitized_repo = repo_id.replace('/', "--");
    get_cache_dir().join(sanitized_repo).join(filename)
}

/// Check if a model file is already present and non-empty in the cache
pub fn is_model_cached(repo_id: &str, filename: &str) -> bool {
    let path = get_local_model_path(repo_id, filename);
    if path.exists() {
        if let Ok(meta) = fs::metadata(&path) {
            return meta.len() > 1024; // Must be at least 1 KB
        }
    }
    false
}

/// Summary of a model located in the local CRON cache
#[derive(Debug, Clone)]
pub struct CachedModelInfo {
    pub repo_id: String,
    pub filename: String,
    pub local_path: PathBuf,
    pub size_bytes: u64,
    pub size_mb: f64,
}

/// List all downloaded model files in the CRON cache
pub fn list_cached_models() -> Vec<CachedModelInfo> {
    let cache_dir = get_cache_dir();
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(&cache_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let repo_dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                let repo_id = repo_dir_name.replace("--", "/");
                if let Ok(files) = fs::read_dir(&path) {
                    for f in files.flatten() {
                        let fpath = f.path();
                        if fpath.is_file() {
                            let fname = fpath.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
                            if fname.ends_with(".gguf") || fname.ends_with(".safetensors") || fname.ends_with(".bin") {
                                if let Ok(m) = fs::metadata(&fpath) {
                                    let sz = m.len();
                                    results.push(CachedModelInfo {
                                        repo_id: repo_id.clone(),
                                        filename: fname,
                                        local_path: fpath,
                                        size_bytes: sz,
                                        size_mb: sz as f64 / (1024.0 * 1024.0),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    results
}

/// Download a model file from Hugging Face Hub with progress streaming
pub fn download_hf_file(
    repo_id: &str,
    filename: &str,
    dest_path: &Path,
    show_progress: bool,
) -> Result<(), String> {
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create model cache directory '{}': {}", parent.display(), e))?;
    }

    let url = format!("https://huggingface.co/{}/resolve/main/{}", repo_id, filename);

    if show_progress {
        println!("┌────────────────────────────────────────────────────────────────────────┐");
        println!("│ CRON HIGH-SPEED HUGGING FACE DOWNLOADER                                │");
        println!("├────────────────────────────────────────────────────────────────────────┤");
        println!("│ Repository : {: <58} │", repo_id);
        println!("│ Target File: {: <58} │", filename);
        println!("│ Local Path : {: <58} │", dest_path.display().to_string().chars().take(58).collect::<String>());
        println!("│ Source URL : {: <58} │", url.chars().take(58).collect::<String>());
        println!("└────────────────────────────────────────────────────────────────────────┘");
        println!("[FETCH] Streaming open-weights from Hugging Face CDN (resumable)...");
    }

    // Try using curl.exe first (native on Windows 10/11 and Linux/macOS)
    let dest_str = dest_path.to_str().ok_or_else(|| "Invalid UTF-8 destination path".to_string())?;

    let curl_res = Command::new("curl.exe")
        .args([
            "-fL",                     // Fail on HTTP errors, follow redirects
            "-C", "-",                 // Auto-resume partially downloaded file
            if show_progress { "--progress-bar" } else { "-s" },
            "-o", dest_str,
            &url,
        ])
        .status();

    match curl_res {
        Ok(status) if status.success() => {
            if show_progress {
                println!("[SUCCESS] Model file downloaded and verified: '{}'", dest_path.display());
            }
            Ok(())
        }
        _ => {
            // Fallback: try standard curl if curl.exe fails
            let std_curl = Command::new("curl")
                .args(["-fL", "-C", "-", if show_progress { "--progress-bar" } else { "-s" }, "-o", dest_str, &url])
                .status();

            if let Ok(st) = std_curl {
                if st.success() {
                    return Ok(());
                }
            }

            // Fallback: PowerShell Invoke-WebRequest on Windows
            if cfg!(windows) {
                if show_progress {
                    println!("[FALLBACK] Using PowerShell web client to complete download...");
                }
                let ps_cmd = format!(
                    "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; (New-Object Net.WebClient).DownloadFile('{}', '{}')",
                    url, dest_str.replace('\\', "\\\\")
                );
                let ps_res = Command::new("powershell")
                    .args(["-NoProfile", "-Command", &ps_cmd])
                    .status();

                if let Ok(st) = ps_res {
                    if st.success() && dest_path.exists() {
                        return Ok(());
                    }
                }
            }

            Err(format!(
                "Failed to download '{}' from Hugging Face Hub (URL: {}). Please verify repository name and internet connection.",
                filename, url
            ))
        }
    }
}

/// Ensure a model is available locally, downloading it automatically if needed
pub fn ensure_model_available(
    query: &str,
    custom_file: Option<&str>,
    show_progress: bool,
) -> Result<(HfModel, PathBuf), String> {
    let model = resolve_model(query)
        .ok_or_else(|| format!("Unknown model '{}'. Run 'cron models' to inspect the supported catalog.", query))?;

    let filename = custom_file.unwrap_or(model.default_file);
    let local_path = get_local_model_path(model.repo_id, filename);

    if !is_model_cached(model.repo_id, filename) {
        if show_progress {
            println!("[CACHE MISS] Model '{}' ({}) is not present locally.", model.name, filename);
            println!("             Initiating automatic high-speed download into CRON cache...");
        }
        download_hf_file(model.repo_id, filename, &local_path, show_progress)?;
    } else if show_progress {
        println!("[CACHE HIT] Model '{}' found in local vault: '{}'", model.name, local_path.display());
    }

    Ok((model, local_path))
}
