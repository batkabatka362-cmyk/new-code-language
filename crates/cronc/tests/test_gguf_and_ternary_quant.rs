// ============================================================================
// CRON Compiler: GGUF Parser & BitNet 1.58-Bit Ternary Quantization Tests
// Module: cronc::tests::test_gguf_and_ternary_quant
// ============================================================================

use cronc::model_importer::gguf::{
    create_synthetic_gguf, parse_gguf_header, GgufTensorType, GgufValue,
};
use cronc::model_importer::ternary::{
    pack_ternary_2bit, quantize_to_ternary, ternary_dot_product, ternary_gemv, unpack_ternary_2bit,
};
use cronc::model_importer::{lower_gguf_model, LoweringOptions, QuantizationMode};

#[test]
fn test_gguf_binary_parsing_and_metadata() {
    let metadata = vec![
        ("general.architecture", GgufValue::String("bitnet".to_string())),
        ("general.name", GgufValue::String("cron-bitnet-7b".to_string())),
        ("bitnet.block_count", GgufValue::Uint32(32)),
        ("bitnet.embedding_length", GgufValue::Uint32(4096)),
        ("bitnet.feed_forward_length", GgufValue::Uint32(11008)),
        ("general.alignment", GgufValue::Uint32(32)),
    ];

    let raw_w1 = vec![0.5f32; 16];
    let mut raw_bytes1 = Vec::new();
    for &f in &raw_w1 {
        raw_bytes1.extend_from_slice(&f.to_bits().to_le_bytes());
    }

    let raw_w2 = vec![1.2f32; 32];
    let mut raw_bytes2 = Vec::new();
    for &f in &raw_w2 {
        raw_bytes2.extend_from_slice(&f.to_bits().to_le_bytes());
    }

    let tensors = vec![
        ("blk.0.attn_q.weight", vec![4, 4], GgufTensorType::F32, raw_bytes1.as_slice()),
        ("blk.0.attn_k.weight", vec![8, 4], GgufTensorType::F32, raw_bytes2.as_slice()),
    ];

    let gguf_bytes = create_synthetic_gguf(&metadata, &tensors, 32);

    let model = parse_gguf_header(&gguf_bytes).expect("GGUF header should parse cleanly");

    assert_eq!(model.tensor_count, 2);
    assert_eq!(model.alignment, 32);
    assert!(model.data_offset >= 32);

    // Verify metadata
    match model.metadata.get("general.architecture") {
        Some(GgufValue::String(s)) => assert_eq!(s, "bitnet"),
        other => panic!("Expected bitnet string architecture, got {:?}", other),
    }

    // Verify tensor descriptors
    let q_info = model.tensors.get("blk.0.attn_q.weight").expect("attn_q tensor present");
    assert_eq!(q_info.dimensions, vec![4, 4]);
    assert_eq!(q_info.tensor_type, GgufTensorType::F32);
}

#[test]
fn test_bitnet_158_ternary_quantization_and_packing() {
    let weights = vec![
        -1.5, -0.8, -0.2, 0.0, 0.1, 0.7, 1.4, 2.0,
        -0.05, 0.05, -1.0, 1.0, -0.9, 0.85, -0.01, 0.02,
    ];

    let (ternary, params) = quantize_to_ternary(&weights);

    assert_eq!(ternary.len(), weights.len());
    assert!(params.scale > 0.0);
    assert_eq!(params.original_elements, 16);

    // Verify every element is in {-1, 0, 1}
    for &t in &ternary {
        assert!(t == -1 || t == 0 || t == 1, "Ternary value must be -1, 0, or 1: {}", t);
    }

    // Test 2-bit packing
    let packed = pack_ternary_2bit(&ternary);
    assert_eq!(packed.len(), 4); // 16 elements / 4 = 4 bytes (75% memory compression!)

    // Test 2-bit unpacking round-trip
    let unpacked = unpack_ternary_2bit(&packed, weights.len());
    assert_eq!(unpacked, ternary, "Bit-exact round trip must match");
}

#[test]
fn test_multiplication_free_ternary_dot_product() {
    let weights = vec![1.0, -1.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0];
    let activations = vec![2.0, 3.0, 5.0, 1.5, 4.0, 2.5, 7.0, 1.0];

    let (ternary, params) = quantize_to_ternary(&weights);
    let packed = pack_ternary_2bit(&ternary);

    // Compute via multiplication-free kernel
    let result = ternary_dot_product(&packed, &activations, params.scale);

    // Compute reference
    let mut expected = 0.0f32;
    for i in 0..weights.len() {
        expected += (ternary[i] as f32) * activations[i];
    }
    expected *= params.scale;

    assert!((result - expected).abs() < 1e-5, "Result {} should match expected {}", result, expected);
}

#[test]
fn test_ternary_gemv_matrix_vector() {
    // 2 rows, 4 columns
    let weights_row0 = vec![1.0, -1.0, 0.0, 1.0];
    let weights_row1 = vec![-1.0, 1.0, 1.0, 0.0];

    let (t0, p0) = quantize_to_ternary(&weights_row0);
    let (t1, p1) = quantize_to_ternary(&weights_row1);

    let mut packed_matrix = Vec::new();
    packed_matrix.extend_from_slice(&pack_ternary_2bit(&t0));
    packed_matrix.extend_from_slice(&pack_ternary_2bit(&t1));

    let activations = vec![3.0, 1.0, 2.0, 4.0];
    let scales = vec![p0.scale, p1.scale];
    let mut out = vec![0.0f32; 2];

    ternary_gemv(&packed_matrix, &activations, &scales, &mut out, 4, 2);

    // Verify row 0: (1*3 - 1*1 + 0*2 + 1*4) * p0.scale = (3 - 1 + 4) * p0.scale = 6 * p0.scale
    let expected0 = 6.0 * p0.scale;
    assert!((out[0] - expected0).abs() < 1e-4);

    // Verify row 1: (-1*3 + 1*1 + 1*2 + 0*4) * p1.scale = (-3 + 1 + 2) * p1.scale = 0
    let expected1 = 0.0 * p1.scale;
    assert!((out[1] - expected1).abs() < 1e-4);
}

#[test]
fn test_lower_gguf_to_cron_code() {
    let metadata = vec![
        ("general.architecture", GgufValue::String("bitnet".to_string())),
    ];
    let dummy_data = vec![0u8; 64];
    let tensors = vec![
        ("model.layers.0.weight", vec![4, 4], GgufTensorType::F32, dummy_data.as_slice()),
    ];
    let gguf_bytes = create_synthetic_gguf(&metadata, &tensors, 32);
    let model = parse_gguf_header(&gguf_bytes).unwrap();

    let options = LoweringOptions {
        module_name: "BitNetGgufCore".to_string(),
        quantize_mode: QuantizationMode::Ternary158b,
        tile_size: (4, 4),
        enable_kernel_fusion: true,
        target_cores: 256,
    };

    let lowered = lower_gguf_model(&model, &options).expect("Lowering should succeed");

    assert!(lowered.cr_source.contains(".MODULE BitNetGgufCore"));
    assert!(lowered.cr_source.contains("simd_ternary_dot"));
    assert!(lowered.memory_reduction_percent > 70.0);
}
