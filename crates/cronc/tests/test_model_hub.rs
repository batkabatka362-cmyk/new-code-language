// ============================================================================
// CRON Compiler: Model Hub & Hugging Face Downloader Test Suite
// (C) 2026 CRON Language Project - 100% Native Pure Rust
// ============================================================================

use cronc::{
    get_cache_dir, get_local_model_path, get_model_catalog, is_model_cached,
    list_cached_models, resolve_model,
};

#[test]
fn test_model_catalog_resolution() {
    let catalog = get_model_catalog();
    assert!(catalog.len() >= 5);

    // 1. Resolve BitNet
    let bitnet = resolve_model("bitnet").expect("Must resolve bitnet");
    assert_eq!(bitnet.repo_id, "1bitLLM/bitnet_b1_58-large");
    assert_eq!(bitnet.default_file, "model.safetensors");
    assert_eq!(bitnet.default_mode, "ternary");

    // 2. Resolve DeepSeek-R1
    let r1 = resolve_model("deepseek-r1").expect("Must resolve deepseek-r1");
    assert!(r1.supports_reasoning);
    assert!(r1.default_file.ends_with(".gguf"));

    // 3. Resolve Llama-3
    let llama = resolve_model("llama3").expect("Must resolve llama3");
    assert_eq!(llama.repo_id, "bartowski/Llama-3.2-1B-Instruct-GGUF");

    // 4. Resolve SmolLM
    let smol = resolve_model("smollm").expect("Must resolve smollm");
    assert_eq!(smol.repo_id, "HuggingFaceTB/SmolLM2-135M");

    // 5. Resolve Custom dynamic repo
    let custom = resolve_model("mistralai/Mistral-7B-v0.1").expect("Must resolve custom repo");
    assert_eq!(custom.repo_id, "mistralai/Mistral-7B-v0.1");
}

#[test]
fn test_cache_paths_and_sanitization() {
    let cache_dir = get_cache_dir();
    assert!(cache_dir.to_str().unwrap().contains(".cron") || cache_dir.to_str().unwrap().contains(".cron_cache"));

    let local_path = get_local_model_path("unsloth/DeepSeek-R1", "model.gguf");
    assert!(local_path.to_str().unwrap().contains("unsloth--DeepSeek-R1"));
    assert!(local_path.to_str().unwrap().ends_with("model.gguf"));

    // Check non-existent model
    assert!(!is_model_cached("dummy_org/dummy_model", "dummy.bin"));

    // Listing cached models shouldn't crash
    let _ = list_cached_models();
}
