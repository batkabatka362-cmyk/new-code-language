// ============================================================================
// CRON Test Suite: Universal AI Model Importer & Silicon Graph Lowering (Phase 14)
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use cronc::model_importer::onnx::{create_synthetic_onnx_model, parse_onnx_model};
use cronc::model_importer::quantize::{quantize_int4, quantize_ternary_158b, QuantizationMode};
use cronc::model_importer::safetensors::{
    create_synthetic_safetensors, parse_safetensors, SafeTensorDType,
};
use cronc::{
    compile_source, compile_to_c23, import_model_file, lower_onnx_graph, lower_safetensors_model,
    ImportOptions, LoweringOptions,
};
use std::fs;
use std::path::Path;

#[test]
fn test_safetensors_parser_and_quantization() {
    // 1. Create synthetic SafeTensors file with 16 float weights and a bias
    let weights = vec![
        1.2f32, -0.9, 0.05, 1.4, -1.1, 0.0, 0.8, -0.7,
        1.5, -1.3, 0.1, 0.9, -0.85, 0.0, 1.1, -1.0,
    ];
    let bias = vec![0.5f32];

    let bytes = create_synthetic_safetensors(&[
        ("model.layer.0.q_proj.weight", SafeTensorDType::F32, &[4, 4], &weights),
        ("model.layer.0.q_proj.bias", SafeTensorDType::F32, &[1], &bias),
    ]);

    // 2. Parse SafeTensors
    let model = parse_safetensors(&bytes).expect("Should parse synthetic SafeTensors buffer");
    assert_eq!(model.tensors.len(), 2);
    let q_weight = &model.tensors["model.layer.0.q_proj.weight"];
    assert_eq!(q_weight.shape, vec![4, 4]);
    assert_eq!(q_weight.values.len(), 16);

    // 3. Test BitNet 1.58b Ternary Quantization
    let q_ternary = quantize_ternary_158b("q_proj", &[4, 4], &q_weight.values);
    assert_eq!(q_ternary.mode, QuantizationMode::Ternary158b);
    assert_eq!(q_ternary.packed_words.len(), 1); // 16 ternary values fit in one 32-bit register
    assert!(q_ternary.report.memory_reduction_percent >= 75.0);
    assert!(q_ternary.scales[0] > 0.0);

    // 4. Test INT4 Quantization
    let q_int4 = quantize_int4("q_proj", &[4, 4], &q_weight.values);
    assert_eq!(q_int4.mode, QuantizationMode::Int4);
    assert_eq!(q_int4.packed_words.len(), 2); // 16 nibbles = 2 x 32-bit words
    assert!(q_int4.report.memory_reduction_percent >= 75.0);
}

#[test]
fn test_onnx_protobuf_wire_decoder() {
    // 1. Synthesize a miniature ONNX model: Linear -> Relu -> Add
    let weights = vec![1.0f32; 16];
    let bias = vec![0.25f32; 4];

    let onnx_bytes = create_synthetic_onnx_model(
        "MiniTransformerMLP",
        &[
            ("MatMul", "linear_1", &["input_act", "w_dense"], &["hidden_1"]),
            ("Relu", "relu_1", &["hidden_1"], &["hidden_act"]),
            ("Add", "bias_add", &["hidden_act", "b_dense"], &["output_act"]),
        ],
        &[
            ("w_dense", &[4, 4], &weights),
            ("b_dense", &[4], &bias),
        ],
        &[("input_act", &[1, 4])],
        &[("output_act", &[1, 4])],
    );

    // 2. Decode via pure-Rust ONNX decoder
    let onnx_model = parse_onnx_model(&onnx_bytes).expect("Should parse synthetic ONNX protobuf");
    assert_eq!(onnx_model.graph.name, "MiniTransformerMLP");
    assert_eq!(onnx_model.graph.nodes.len(), 3);
    assert_eq!(onnx_model.graph.initializers.len(), 2);

    let w_tensor = &onnx_model.graph.initializers["w_dense"];
    assert_eq!(w_tensor.dims, vec![4, 4]);
    assert_eq!(w_tensor.float_values.len(), 16);
    assert_eq!(w_tensor.float_values[0], 1.0);
}

#[test]
fn test_graph_lowering_to_cr_syntax() {
    // 1. Create synthetic SafeTensors model
    let weights = vec![1.5f32; 16];
    let bytes = create_synthetic_safetensors(&[
        ("attn.q_proj.weight", SafeTensorDType::F32, &[4, 4], &weights),
    ]);
    let st_model = parse_safetensors(&bytes).expect("Parsed SafeTensors");

    // 2. Lower to CRON .cr with Kernel Fusion enabled
    let opts = LoweringOptions {
        module_name: "ImportedTestCell".to_string(),
        quantize_mode: QuantizationMode::Ternary158b,
        tile_size: (4, 4),
        enable_kernel_fusion: true,
        target_cores: 256,
    };

    let lowered = lower_safetensors_model(&st_model, &opts).expect("Should lower to .cr");
    assert!(lowered.cr_source.contains(".MODULE ImportedTestCell"));
    assert!(lowered.cr_source.contains("@sram(bank=0)"));
    assert!(lowered.cr_source.contains("simd_ternary_dot"));
    assert!(lowered.cr_source.contains("fuse [tile=(4, 4), stream=SRAM]"));
    assert!(lowered.fused_kernel_count >= 1);

    // 3. Verify that the synthesized CRON code compiles with zero errors into 128-bit VLIW!
    let vliw_res = compile_source(&lowered.cr_source);
    assert!(
        vliw_res.is_ok(),
        "Synthesized .cr code must pass compile_source(): {:?}",
        vliw_res.err()
    );
}

#[test]
fn test_fused_transformer_c23_execution() {
    // 1. Synthesize ONNX model
    let weights = vec![1.0f32; 16];
    let onnx_bytes = create_synthetic_onnx_model(
        "FusedCell",
        &[("MatMul", "dense", &["in", "w"], &["out"])],
        &[("w", &[4, 4], &weights)],
        &[("in", &[1, 4])],
        &[("out", &[1, 4])],
    );

    let onnx_model = parse_onnx_model(&onnx_bytes).expect("ONNX parse");
    let opts = LoweringOptions {
        module_name: "FusedCellModule".to_string(),
        quantize_mode: QuantizationMode::Ternary158b,
        tile_size: (4, 4),
        enable_kernel_fusion: true,
        target_cores: 256,
    };

    let lowered = lower_onnx_graph(&onnx_model.graph, &opts).expect("Graph lowering");

    // 2. Compile to ISO C23
    let c23_code = compile_to_c23(&lowered.cr_source).expect("Should transpile to C23");
    assert!(c23_code.contains("CRON Native C23 Engine"));
    assert!(c23_code.contains("simd_ternary_dot"));

    // 3. If GCC is present on host, compile and run native binary
    let gcc_check = std::process::Command::new("gcc")
        .arg("--version")
        .output();

    if let Ok(out) = gcc_check {
        if out.status.success() {
            let temp_dir = std::env::temp_dir();
            let c_file = temp_dir.join("test_fused_imported.c");
            let bin_file = temp_dir.join("test_fused_imported.exe");

            fs::write(&c_file, &c23_code).expect("Write temp C file");

            let compile_status = std::process::Command::new("gcc")
                .args(&[
                    c_file.to_str().unwrap(),
                    "-O2",
                    "-o",
                    bin_file.to_str().unwrap(),
                ])
                .status()
                .expect("Run GCC");

            assert!(compile_status.success(), "GCC compilation must succeed");

            let run_status = std::process::Command::new(&bin_file)
                .status()
                .expect("Execute binary");

            assert!(run_status.success(), "Native execution must exit with 0");

            let _ = fs::remove_file(c_file);
            let _ = fs::remove_file(bin_file);
        }
    }
}

#[test]
fn test_cli_import_file_workflow() {
    let temp_dir = std::env::temp_dir();
    let st_path = temp_dir.join("test_pipeline_weights.safetensors");

    let weights = vec![0.75f32; 16];
    let bytes = create_synthetic_safetensors(&[
        ("layer1.weight", SafeTensorDType::F32, &[4, 4], &weights),
    ]);
    fs::write(&st_path, &bytes).expect("Write temp safetensors file");

    let opts = ImportOptions {
        output_path: None,
        quantize_mode: QuantizationMode::Ternary158b,
        tile_size: (4, 4),
        enable_kernel_fusion: true,
        target_cores: 256,
    };

    let imported = import_model_file(&st_path, &opts).expect("Import model file");
    assert_eq!(imported.source_format, "SafeTensors");
    assert_eq!(imported.lowered_code.total_parameters, 16);
    assert!(imported.lowered_code.cr_source.contains(".MODULE Imported_test_pipeline_weights"));

    let _ = fs::remove_file(st_path);
}

#[test]
fn test_generate_example_imported_model() {
    let weights = vec![1.2f32; 16];
    let onnx_bytes = create_synthetic_onnx_model(
        "BitNetTransformerAttentionLayer",
        &[
            ("MatMul", "q_proj", &["x_in", "w_q"], &["q_out"]),
            ("MatMul", "k_proj", &["x_in", "w_k"], &["k_out"]),
            ("Relu", "act_proj", &["q_out"], &["q_act"]),
        ],
        &[
            ("w_q", &[4, 4], &weights),
            ("w_k", &[4, 4], &weights),
        ],
        &[("x_in", &[1, 4])],
        &[("q_act", &[1, 4])],
    );

    // Attempt writing to examples if running from workspace root or crate root
    let (target_onnx, target_cr) = if Path::new("examples").exists() {
        (Path::new("examples/sample_transformer.onnx").to_path_buf(), Path::new("examples/imported_transformer_cell.cr").to_path_buf())
    } else {
        (Path::new("../../examples/sample_transformer.onnx").to_path_buf(), Path::new("../../examples/imported_transformer_cell.cr").to_path_buf())
    };

    let _ = fs::write(&target_onnx, &onnx_bytes);

    let opts = ImportOptions {
        output_path: Some(target_cr.to_str().unwrap().to_string()),
        quantize_mode: QuantizationMode::Ternary158b,
        tile_size: (4, 4),
        enable_kernel_fusion: true,
        target_cores: 256,
    };

    let imported = import_model_file(&target_onnx, &opts).expect("Should import sample onnx");
    fs::write(&target_cr, &imported.lowered_code.cr_source).expect("Write imported .cr");

    // Verify the generated .cr compiles to VLIW with zero errors
    let vliw = compile_source(&imported.lowered_code.cr_source).expect("Synthesized example must compile cleanly to VLIW");
    assert!(!vliw.is_empty());
}

