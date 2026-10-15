// ============================================================================
// CRON CLI 3D Multi-Topology Visualizer Test Suite
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use cronc::cl_lora::{ClLoraAdapter, ClLoraConfig, DiscreteLoraLayer};
use std::fs;
use std::process::Command;

#[test]
fn test_cli_visualizer_help() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["visualize", "--help"])
        .output()
        .expect("run cron visualize --help");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON 3D MULTI-TOPOLOGY INTERACTIVE VISUALIZER"));
    assert!(stdout.contains("--mode"));
    assert!(stdout.contains("--lora"));
    assert!(stdout.contains("--cl"));
    assert!(stdout.contains("--atp"));
    assert!(stdout.contains("--open"));
}

#[test]
fn test_cli_visualizer_default_export() {
    let temp_dir = std::env::temp_dir().join(format!("cron_cli_viz_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let _ = fs::create_dir_all(&temp_dir);
    let out_html = temp_dir.join("test_dashboard_all.html");

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["visualize", "--mode", "all", "-o", out_html.to_str().unwrap()])
        .output()
        .expect("run cron visualize --mode all");

    assert!(out.status.success(), "visualize command must succeed: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Active Topologies"));
    assert!(stdout.contains("4D Torus NoC"));
    assert!(stdout.contains("Hexagonal ATP Life"));
    assert!(stdout.contains("CL-LoRA Manifold"));
    assert!(stdout.contains("Unified 3D WebGL Visualizer generated successfully!"));

    assert!(out_html.exists(), "Output HTML file must be generated on disk");
    let content = fs::read_to_string(&out_html).expect("Read output HTML");
    assert!(content.contains("<!DOCTYPE html>"));
    assert!(content.contains("three.min.js"));
    assert!(content.contains("4D Torus NoC"));
    assert!(content.contains("Hexagonal ATP"));
    assert!(content.contains("CL-LoRA"));
    assert!(content.contains("432 Hz Resonance"));

    let _ = fs::remove_file(&out_html);
    let _ = fs::remove_dir(&temp_dir);
}

#[test]
fn test_cli_visualizer_with_cl_lora_adapter() {
    let temp_dir = std::env::temp_dir().join(format!("cron_cli_viz_lora_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let _ = fs::create_dir_all(&temp_dir);
    let lora_file = temp_dir.join("discrete_llama.cll");
    let out_html = temp_dir.join("lora_manifold.html");

    // 1. Create a synthetic discrete CL-LoRA adapter (.cll)
    let rank = 4;
    let dim = 32;
    let mut adapter = ClLoraAdapter::new(
        "discrete_llama_vision_lora",
        ClLoraConfig {
            rank,
            alpha: 2.0,
            hidden_dim: dim,
            num_layers: 1,
            target_modules: vec!["q_proj".to_string()],
        },
    );
    let a_trits: Vec<i8> = (0..rank * dim).map(|i| match i % 3 { 0 => 1, 1 => -1, _ => 0 }).collect();
    let b_trits: Vec<i8> = (0..dim * rank).map(|i| match (i + 1) % 3 { 0 => 1, 1 => -1, _ => 0 }).collect();
    let layer = DiscreteLoraLayer::new(0, "q_proj", dim, dim, rank, &a_trits, &b_trits);
    adapter.layers.push(layer);
    adapter.save_to_file(&lora_file).expect("Save test .cll file");

    // 2. Invoke cron visualize with the .cll file
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "visualize",
            "--lora",
            lora_file.to_str().unwrap(),
            "--mode",
            "lora",
            "-o",
            out_html.to_str().unwrap(),
        ])
        .output()
        .expect("run cron visualize with --lora");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CL-LoRA Discrete Latent: 'discrete_llama_vision_lora'"));
    assert!(stdout.contains("Rank: 4, Dim: 32"));
    assert!(stdout.contains("L1 Memory Footprint   : < 4.1 KB"));

    assert!(out_html.exists());
    let content = fs::read_to_string(&out_html).unwrap();
    assert!(content.contains("discrete_llama_vision_lora"));

    let _ = fs::remove_file(&lora_file);
    let _ = fs::remove_file(&out_html);
    let _ = fs::remove_dir(&temp_dir);
}
