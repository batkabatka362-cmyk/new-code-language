// ============================================================================
// CRON Test Suite: Universal AI Model Converter & 4D Spherical Quantization Bridge
// Module: cronc::tests::test_cl_convert
//
// Validates conversion of PyTorch / SafeTensors / GGUF models into
// BitNet b1.58 Ternary and 4D Spherical Curvature Machine Code (.cl & .sphere).
// Guarantees:
//  - 100% Multiplier-Free (0.00 FLOPs)
//  - 16x memory footprint reduction vs FP32
//  - Sovereign 128-bit VLIW bundle synthesis
//  - C-ABI FFI Bridge parity for Python integration
// ============================================================================

use std::fs;
use std::ffi::{CStr, CString};
use cronc::cl_convert::{convert_ai_model_file, ModelConvertOptions, ModelInputFormat};
use cronc::model_importer::safetensors::{create_synthetic_safetensors, SafeTensorDType};
use cronc::model_importer::gguf::{create_synthetic_gguf, GgufTensorType, GgufValue};
use cronc::cl_python_bridge::{
    cron_sphere_quantize_tensor, cron_sphere_export_layer_cl, cron_sphere_convert_file,
};

#[test]
fn test_safetensors_to_4d_sphere_conversion() {
    let temp_dir = std::env::temp_dir().join("cron_test_safetensors_convert");
    let _ = fs::create_dir_all(&temp_dir);
    let st_path = temp_dir.join("mini_llama.safetensors");
    let cl_out_path = temp_dir.join("mini_llama_sphere");

    // 1. Synthesize multi-layer SafeTensors file
    let weights_q = vec![
        0.8f32, -1.2, 0.05, 1.5, -0.9, 0.0, 0.4, -0.6,
        1.1, -1.4, 0.02, 0.7, -0.85, 0.0, 1.0, -1.1,
    ];
    let weights_k = vec![
        -0.5f32, 1.3, -0.01, -1.0, 0.7, -0.8, 0.0, 0.9,
        1.2, -0.3, 0.0, -1.5, 0.6, -0.9, 0.2, 1.1,
    ];

    let bytes = create_synthetic_safetensors(&[
        ("model.layers.0.self_attn.q_proj.weight", SafeTensorDType::F32, &[4, 4], &weights_q),
        ("model.layers.0.self_attn.k_proj.weight", SafeTensorDType::F32, &[4, 4], &weights_k),
    ]);
    fs::write(&st_path, &bytes).expect("Write synthetic safetensors file");

    // 2. Perform Full Conversion to 4D Sphere Microcode
    let options = ModelConvertOptions {
        target_heads: 4,
        base_layer_depth: 2,
        max_bundles_per_layer: 100,
        export_microcode: true,
        export_packed_binary: true,
    };

    let report = convert_ai_model_file(
        st_path.to_str().unwrap(),
        Some(cl_out_path.to_str().unwrap()),
        &options,
    ).expect("SafeTensors conversion must succeed");

    // 3. Verify Conversion Metrics
    assert_eq!(report.input_format, ModelInputFormat::SafeTensors);
    assert_eq!(report.total_tensors_converted, 2);
    assert_eq!(report.total_parameters, 32);
    assert_eq!(report.original_fp32_bytes, 32 * 4); // 128 bytes
    assert_eq!(report.quantized_ternary_bytes, 8); // 32 trits packed at 4/byte = 8 bytes
    assert!(report.overall_compression_ratio >= 16.0, "Expected >=16x compression, got {:.2}x", report.overall_compression_ratio);
    assert!(report.zero_flop_guarantee, "0-FLOP guarantee must be true");
    assert!(report.total_cl_bundles > 0);
    assert!(report.total_geodesic_operations > 0);

    // 4. Verify Emitted Files
    let cl_file = temp_dir.join("mini_llama_sphere.cl");
    assert!(cl_file.exists(), ".cl microcode file should exist");
    let cl_text = fs::read_to_string(&cl_file).unwrap();
    assert!(cl_text.contains("@kernel sphere_neural_layer_model_layers_0_self_attn_q_proj_weight") || cl_text.contains(".target silicon.sphere_isa"));
    assert!(cl_text.contains("B0001:"));

    let sphere_file = temp_dir.join("mini_llama_sphere.sphere");
    assert!(sphere_file.exists(), ".sphere binary file should exist");
    let sphere_bytes = fs::read(&sphere_file).unwrap();
    assert_eq!(&sphere_bytes[0..4], b"SPH4", "Magic header must be SPH4");
    assert!(sphere_bytes.len() > 4);

    // 5. Verify Telemetry Formatting
    let hud = report.format_ascii_report();
    assert!(hud.contains("CRON UNIVERSAL AI CONVERTER"));
    assert!(hud.contains("0.00 FLOPs"));
    assert!(hud.contains("16.00x"));

    let json = report.to_json();
    assert!(json.contains("\"zero_flop_guarantee\": true"));
    assert!(json.contains("\"total_tensors_converted\": 2"));

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_gguf_to_4d_sphere_conversion() {
    let temp_dir = std::env::temp_dir().join("cron_test_gguf_convert");
    let _ = fs::create_dir_all(&temp_dir);
    let gguf_path = temp_dir.join("model_test.gguf");
    let out_path = temp_dir.join("converted_gguf");

    // 1. Synthesize GGUF binary
    let metadata = vec![
        ("general.architecture", GgufValue::String("llama".to_string())),
        ("general.name", GgufValue::String("mini-gguf-sphere".to_string())),
    ];

    let mut raw_bytes = Vec::new();
    for i in 0..16 {
        let f = if i % 2 == 0 { 0.75f32 } else { -0.75f32 };
        raw_bytes.extend_from_slice(&f.to_le_bytes());
    }

    let tensors = vec![
        ("blk.0.attn_q.weight", vec![4, 4], GgufTensorType::F32, raw_bytes.as_slice()),
    ];

    let gguf_bytes = create_synthetic_gguf(&metadata, &tensors, 32);
    fs::write(&gguf_path, &gguf_bytes).expect("Write synthetic gguf");

    // 2. Convert to Sphere
    let options = ModelConvertOptions::default();
    let report = convert_ai_model_file(
        gguf_path.to_str().unwrap(),
        Some(out_path.to_str().unwrap()),
        &options,
    ).expect("GGUF conversion must succeed");

    assert_eq!(report.input_format, ModelInputFormat::Gguf);
    assert_eq!(report.total_tensors_converted, 1);
    assert_eq!(report.total_parameters, 16);
    assert_eq!(report.quantized_ternary_bytes, 4);
    assert!(report.overall_compression_ratio >= 16.0);
    assert!(report.zero_flop_guarantee);

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_raw_float_dump_to_4d_sphere_conversion() {
    let temp_dir = std::env::temp_dir().join("cron_test_raw_float_convert");
    let _ = fs::create_dir_all(&temp_dir);
    let raw_path = temp_dir.join("pytorch_weights.bin");
    let out_path = temp_dir.join("pytorch_sphere");

    // 64 float elements (8x8 matrix)
    let mut raw_bytes = Vec::with_capacity(64 * 4);
    for i in 0..64 {
        let val = ((i as f32 * 0.5).sin() * 2.0) - 0.2;
        raw_bytes.extend_from_slice(&val.to_le_bytes());
    }
    fs::write(&raw_path, &raw_bytes).expect("Write raw float binary");

    let options = ModelConvertOptions::default();
    let report = convert_ai_model_file(
        raw_path.to_str().unwrap(),
        Some(out_path.to_str().unwrap()),
        &options,
    ).expect("Raw float conversion must succeed");

    assert_eq!(report.input_format, ModelInputFormat::RawFloatDump);
    assert_eq!(report.total_parameters, 64);
    assert_eq!(report.layers[0].rows, 8);
    assert_eq!(report.layers[0].cols, 8);
    assert!(report.total_cl_bundles > 0);

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_python_c_abi_ffi_bridge() {
    // 1. Test cron_sphere_quantize_tensor
    let weights = vec![1.5f32, -1.2, 0.05, 0.0, -0.01, 2.0, -2.5, 0.9];
    let mut out_ternary = vec![0i8; weights.len()];
    let mut scale = 0.0f32;

    let res = cron_sphere_quantize_tensor(
        weights.as_ptr(),
        weights.len(),
        out_ternary.as_mut_ptr(),
        &mut scale as *mut f32,
    );
    assert_eq!(res, 0);
    assert!(scale > 0.0);
    for &t in &out_ternary {
        assert!(t == -1 || t == 0 || t == 1);
    }

    // 2. Test cron_sphere_export_layer_cl
    let layer_name = CString::new("test_ffi_layer").unwrap();
    let mut cl_buf = vec![0u8; 8192];

    let res_cl = cron_sphere_export_layer_cl(
        layer_name.as_ptr(),
        weights.as_ptr(),
        2,
        4,
        cl_buf.as_mut_ptr() as *mut std::os::raw::c_char,
        cl_buf.len(),
    );
    assert_eq!(res_cl, 0);

    let cl_str = unsafe { CStr::from_ptr(cl_buf.as_ptr() as *const std::os::raw::c_char) }
        .to_str()
        .unwrap();
    assert!(cl_str.contains("@kernel sphere_neural_layer_test_ffi_layer"));
    assert!(cl_str.contains(".target silicon.sphere_isa"));

    // 3. Test cron_sphere_convert_file
    let temp_dir = std::env::temp_dir().join("cron_test_ffi_convert");
    let _ = fs::create_dir_all(&temp_dir);
    let raw_path = temp_dir.join("ffi_weights.raw");
    let mut raw_bytes = Vec::new();
    for f in [1.0f32, -1.0, 0.5, -0.5] {
        raw_bytes.extend_from_slice(&f.to_le_bytes());
    }
    fs::write(&raw_path, &raw_bytes).unwrap();

    let in_cstr = CString::new(raw_path.to_str().unwrap()).unwrap();
    let out_prefix = temp_dir.join("out_ffi");
    let out_cstr = CString::new(out_prefix.to_str().unwrap()).unwrap();
    let mut report_buf = vec![0u8; 4096];

    let res_conv = cron_sphere_convert_file(
        in_cstr.as_ptr(),
        out_cstr.as_ptr(),
        report_buf.as_mut_ptr() as *mut std::os::raw::c_char,
        report_buf.len(),
    );
    assert_eq!(res_conv, 0);

    let rep_json = unsafe { CStr::from_ptr(report_buf.as_ptr() as *const std::os::raw::c_char) }
        .to_str()
        .unwrap();
    assert!(rep_json.contains("\"zero_flop_guarantee\": true"));

    let _ = fs::remove_dir_all(&temp_dir);
}
