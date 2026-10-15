// ============================================================================
// CRON CL-LoRA Sub-Byte Fine-Tuning CLI Integration Tests
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_finetune_plain_text_and_json() {
    let out_cll = "test_finetune_adapter.cll";
    let _ = fs::remove_file(out_cll);

    let corpus = "Монгол хэл нь Алтай язгуурын агглютинатив хэл юм. Үгийн үндэс дээр дагавар залгагдана.";

    // 1. Run cron finetune with --json
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "finetune",
            corpus,
            "-r",
            "2",
            "--steps",
            "50",
            "-o",
            out_cll,
            "--json",
        ])
        .output()
        .expect("Failed to run cron finetune");

    assert!(out.status.success(), "cron finetune must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"adapter_name\": \"cron-cl-lora-adapter\"") || stdout.contains("\"adapter_name\":\"cron-cl-lora-adapter\""));
    assert!(stdout.contains("\"is_l1_bounded\": true") || stdout.contains("\"is_l1_bounded\":true"));
    assert!(stdout.contains("\"loss_reduction_pct\":"));

    // Check that adapter was written to disk and is under 4.1 KB
    let meta = fs::metadata(out_cll).expect("Adapter file must exist");
    assert!(meta.len() < 4196, "Adapter must be strictly bounded in L1 cache (<4.1 KB)");

    let _ = fs::remove_file(out_cll);
}

#[test]
fn test_cli_finetune_jsonl_dataset_and_vliw_compile() {
    let dataset_file = "test_temp_pairs.jsonl";
    let out_cll = "test_jsonl_adapter.cll";
    let out_cl = "test_jsonl_adapter.cl";
    let _ = fs::remove_file(out_cll);
    let _ = fs::remove_file(out_cl);

    let content = r#"{"instruction": "4D Torus NoC Routing", "output": "Dimension Order Routing DOR"}
{"instruction": "Монгол бичиг", "output": "Босоо тигээр бичигддэг эртний соёл"}"#;
    fs::write(dataset_file, content).expect("Failed to write test dataset");

    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "finetune",
            dataset_file,
            "--rank",
            "4",
            "--steps",
            "40",
            "-o",
            out_cll,
            "--compile",
        ])
        .output()
        .expect("Failed to run cron finetune with --compile");

    assert!(out.status.success(), "cron finetune with --compile must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON ON-DEVICE SUB-BYTE CL-LoRA FINE-TUNING ENGINE"));
    assert!(stdout.contains("128-bit VLIW Microcode synthesized"));
    assert!(stdout.contains("PASS"));

    // Verify .cl microcode
    assert!(std::path::Path::new(out_cl).exists());
    let cl_code = fs::read_to_string(out_cl).unwrap();
    assert!(cl_code.contains("CRON CL-LoRA Microcode"));
    assert!(cl_code.contains("B0000:"));

    let _ = fs::remove_file(dataset_file);
    let _ = fs::remove_file(out_cll);
    let _ = fs::remove_file(out_cl);
}
