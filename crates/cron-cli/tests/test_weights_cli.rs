// ============================================================================
// CRON CLI Weights Ingestion & SafeTensors / CL-LoRA Test Suite
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use cronc::cl_lora::ClLoraAdapter;
use cronc::model_importer::safetensors::{create_synthetic_safetensors, SafeTensorDType};
use std::fs;
use std::process::Command;

#[test]
fn test_cli_weights_help() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["weights", "--help"])
        .output()
        .expect("run cron weights --help");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("cron weights <SUBCOMMAND>"));
    assert!(stdout.contains("import"));
    assert!(stdout.contains("inspect"));
}

#[test]
fn test_cli_weights_inspect_and_import() {
    let temp_dir = std::env::temp_dir().join("cron_test_weights_cli");
    let _ = fs::create_dir_all(&temp_dir);
    let st_file = temp_dir.join("mini_llama.safetensors");
    let out_cll = temp_dir.join("mini_llama.cll");
    let out_cl = temp_dir.join("mini_llama.cl");

    // 1. Synthesize SafeTensors weights
    let rank = 4usize;
    let dim = 64usize;
    let a_weights: Vec<f32> = (0..rank * dim).map(|i| ((i as f32) * 0.1).sin()).collect();
    let b_weights: Vec<f32> = (0..dim * rank).map(|i| ((i as f32) * 0.1).cos()).collect();

    let bytes = create_synthetic_safetensors(&[
        (
            "model.layers.0.self_attn.q_proj.lora_A.weight",
            SafeTensorDType::F32,
            &[rank, dim],
            &a_weights,
        ),
        (
            "model.layers.0.self_attn.q_proj.lora_B.weight",
            SafeTensorDType::F32,
            &[dim, rank],
            &b_weights,
        ),
    ]);
    fs::write(&st_file, &bytes).expect("Write synthetic safetensors");

    // 2. Test inspect command
    let inspect_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["weights", "inspect", st_file.to_str().unwrap()])
        .output()
        .expect("run cron weights inspect");

    assert!(inspect_out.status.success(), "Inspect should exit 0");
    let inspect_str = String::from_utf8_lossy(&inspect_out.stdout);
    assert!(inspect_str.contains("CRON SAFETENSORS MODEL INSPECTION"));
    assert!(inspect_str.contains("q_proj.lora_A.weight"));
    assert!(inspect_str.contains("Total Ingested Parameters"));

    // 3. Test import command for CL-LoRA adapter (.cll)
    let import_cll_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "weights",
            "import",
            st_file.to_str().unwrap(),
            "--lora",
            "--rank",
            "4",
            "-o",
            out_cll.to_str().unwrap(),
        ])
        .output()
        .expect("run cron weights import --lora");

    assert!(import_cll_out.status.success(), "Import --lora should exit 0");
    assert!(out_cll.exists(), "Output .cll file must exist");

    // Load and verify adapter
    let adapter = ClLoraAdapter::load_from_file(&out_cll).expect("Load generated .cll adapter");
    assert_eq!(adapter.config.rank, 4);
    assert_eq!(adapter.layers.len(), 1);
    assert!(adapter.size_in_bytes() < 4096);

    // 4. Test import command for 40-char VLIW microcode (.cl)
    let import_cl_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "weights",
            "import",
            st_file.to_str().unwrap(),
            "-o",
            out_cl.to_str().unwrap(),
        ])
        .output()
        .expect("run cron weights import -o out.cl");

    assert!(import_cl_out.status.success(), "Import .cl should exit 0");
    assert!(out_cl.exists(), "Output .cl file must exist");
    let cl_content = fs::read_to_string(&out_cl).expect("Read .cl file");
    assert!(cl_content.contains(".core [0, 0, 0, 0]:"));
    assert!(cl_content.contains("@entry:"));

    // Cleanup
    let _ = fs::remove_file(&st_file);
    let _ = fs::remove_file(&out_cll);
    let _ = fs::remove_file(&out_cl);
    let _ = fs::remove_dir(&temp_dir);
}
