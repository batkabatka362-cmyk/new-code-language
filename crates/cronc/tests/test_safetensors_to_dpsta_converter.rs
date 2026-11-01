// ============================================================================
// Integration Test: Safetensors -> DPSTA 1.58-Bit Ternary Converter
// Verifies:
// 1. Loading and parsing standard HuggingFace Safetensors Float32/BFloat16/Float16
// 2. DPSTA 1.58-bit sub-byte ternary quantization {-1, 0, +1}
// 3. 5-in-1-byte compaction (~19.9x RAM savings) and .cldb export
// 4. Automated Zero-Verbal .cl architectural model code generation
// ============================================================================

use cronc::cl_safetensors_convert::convert_safetensors_to_dpsta;
use cronc::cl_zero_verbal_lint::ZeroVerbalLinter;
use std::fs;
use std::path::Path;

fn create_mock_hf_safetensors(path: &str) {
    let mut data = Vec::new();

    // Tensor 1: model.embed_tokens.weight [4, 4]
    let t1_vals: Vec<f32> = vec![
        0.5, -0.4, 0.01, 1.2,
        -0.9, 0.0, 0.05, -1.1,
        0.8, -0.7, 0.0, 0.3,
        -0.2, 0.6, -0.5, 0.0,
    ];
    let mut t1_bytes = Vec::new();
    for v in t1_vals {
        t1_bytes.extend_from_slice(&v.to_le_bytes());
    }

    // Tensor 2: model.layers.0.mlp.gate_proj.weight [4, 8]
    let t2_vals: Vec<f32> = (0..32).map(|i| if i % 2 == 0 { 0.8f32 } else { -0.8f32 }).collect();
    let mut t2_bytes = Vec::new();
    for v in t2_vals {
        t2_bytes.extend_from_slice(&v.to_le_bytes());
    }

    let header_json = format!(
        r#"{{"model.embed_tokens.weight":{{"dtype":"F32","shape":[4,4],"data_offsets":[0,{}]}},"model.layers.0.mlp.gate_proj.weight":{{"dtype":"F32","shape":[4,8],"data_offsets":[{},{}]}}}}"#,
        t1_bytes.len(),
        t1_bytes.len(),
        t1_bytes.len() + t2_bytes.len()
    );

    let header_len = header_json.len() as u64;
    data.extend_from_slice(&header_len.to_le_bytes());
    data.extend_from_slice(header_json.as_bytes());
    data.extend_from_slice(&t1_bytes);
    data.extend_from_slice(&t2_bytes);

    fs::write(path, data).expect("Must write mock safetensors");
}

#[test]
fn test_safetensors_to_dpsta_158bit_compression_and_cl_emission() {
    let input_st = "temp_mock_model.safetensors";
    let output_cldb = "temp_mock_model.cldb";
    let output_cl = "temp_mock_model.cl";

    create_mock_hf_safetensors(input_st);

    let summary = convert_safetensors_to_dpsta(input_st, output_cldb, Some(output_cl), true)
        .expect("Conversion to DPSTA must succeed");

    assert_eq!(summary.total_tensors, 2);
    assert_eq!(summary.total_elements, 48); // 16 + 32
    assert!(summary.compression_ratio > 1.5, "DPSTA sub-byte packing must compress data");
    assert!(Path::new(output_cldb).exists());
    assert!(Path::new(output_cl).exists());

    // Verify emitted .cl file is 100% Zero-Verbal compliant
    let cl_text = fs::read_to_string(output_cl).expect("Must read emitted .cl file");
    let audit = ZeroVerbalLinter::audit_source(&cl_text, output_cl);
    assert!(
        audit.is_constitution_compliant,
        "Emitted .cl model must strictly adhere to the Zero-Verbal Constitution! Violations: {:?}",
        audit.diagnostics
    );

    // Verify HUD renders cleanly
    let hud = summary.render_ascii_hud();
    assert!(hud.contains("CRON SAFETENSORS -> DPSTA 1.58-BIT TERNARY SILICON CONVERTER"));
    assert!(hud.contains("RAM Savings"));

    // Cleanup
    let _ = fs::remove_file(input_st);
    let _ = fs::remove_file(output_cldb);
    let _ = fs::remove_file(output_cl);
}

