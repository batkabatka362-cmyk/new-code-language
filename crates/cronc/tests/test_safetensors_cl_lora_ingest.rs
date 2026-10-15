// ============================================================================
// CRON Test Suite: SafeTensors HuggingFace Direct Ingestion into CL-LoRA & .cl
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use cronc::cl_lora::ClLoraAdapter;
use cronc::model_importer::safetensors::{
    create_synthetic_safetensors, parse_safetensors, safetensors_to_cl_lora,
    safetensors_to_cl_machine_code, SafeTensorDType,
};

#[test]
fn test_safetensors_to_cl_lora_direct_conversion() {
    let rank = 4usize;
    let hidden_dim = 64usize;

    // Create synthetic weights for A (rank x hidden_dim = 256 floats) and B (hidden_dim x rank = 256 floats)
    let a_weights: Vec<f32> = (0..rank * hidden_dim)
        .map(|i| (((i % 7) as f32) - 3.0) * 0.42)
        .collect();
    let b_weights: Vec<f32> = (0..hidden_dim * rank)
        .map(|i| (((i % 5) as f32) - 2.0) * 0.58)
        .collect();

    let bytes = create_synthetic_safetensors(&[
        (
            "model.layers.0.self_attn.q_proj.lora_A.weight",
            SafeTensorDType::F32,
            &[rank, hidden_dim],
            &a_weights,
        ),
        (
            "model.layers.0.self_attn.q_proj.lora_B.weight",
            SafeTensorDType::F32,
            &[hidden_dim, rank],
            &b_weights,
        ),
        (
            "model.layers.0.self_attn.v_proj.lora_A.weight",
            SafeTensorDType::F32,
            &[rank, hidden_dim],
            &a_weights,
        ),
        (
            "model.layers.0.self_attn.v_proj.lora_B.weight",
            SafeTensorDType::F32,
            &[hidden_dim, rank],
            &b_weights,
        ),
        (
            "model.layers.1.self_attn.q_proj.lora_A.weight",
            SafeTensorDType::F32,
            &[rank, hidden_dim],
            &a_weights,
        ),
        (
            "model.layers.1.self_attn.q_proj.lora_B.weight",
            SafeTensorDType::F32,
            &[hidden_dim, rank],
            &b_weights,
        ),
    ]);

    // 1. Parse SafeTensors container
    let model = parse_safetensors(&bytes).expect("Should parse synthetic SafeTensors");
    assert_eq!(model.tensors.len(), 6);

    // 2. Convert to CL-LoRA Discrete Adapter
    let adapter = safetensors_to_cl_lora(&model, "hf_llama_lora", rank, 2.0)
        .expect("Should convert SafeTensors to CL-LoRA");

    assert_eq!(adapter.name, "hf_llama_lora");
    assert_eq!(adapter.config.rank, rank);
    assert_eq!(adapter.config.hidden_dim, hidden_dim);
    assert_eq!(adapter.config.num_layers, 2);
    assert!(adapter.config.target_modules.contains(&"q_proj".to_string()));
    assert!(adapter.config.target_modules.contains(&"v_proj".to_string()));
    assert_eq!(adapter.layers.len(), 3); // 2 in layer 0, 1 in layer 1

    // 3. Verify ternary trits in {-1, 0, 1}
    for layer in &adapter.layers {
        let a_trits = layer.unpack_a();
        let b_trits = layer.unpack_b();
        assert_eq!(a_trits.len(), rank * hidden_dim);
        assert_eq!(b_trits.len(), hidden_dim * rank);

        for &t in &a_trits {
            assert!(t == -1 || t == 0 || t == 1);
        }
        for &t in &b_trits {
            assert!(t == -1 || t == 0 || t == 1);
        }
    }

    // 4. Verify sub-byte binary serialization and roundtrip (< 4.1 KB L1 bound)
    let binary_bytes = adapter.save_to_bytes();
    assert!(
        binary_bytes.len() < 4096,
        "Footprint must fit in L1 cache: actual {} bytes",
        binary_bytes.len()
    );

    let loaded = ClLoraAdapter::load_from_bytes(&binary_bytes).expect("Should reload adapter");
    assert_eq!(loaded.name, adapter.name);
    assert_eq!(loaded.config, adapter.config);
    assert_eq!(loaded.layers.len(), adapter.layers.len());
}

#[test]
fn test_safetensors_cl_lora_forward_delta_zero_flop() {
    let rank = 2usize;
    let hidden_dim = 64usize;
    let a_weights = vec![0.8f32; rank * hidden_dim];
    let b_weights = vec![-0.7f32; hidden_dim * rank];

    let bytes = create_synthetic_safetensors(&[
        (
            "base_model.model.model.layers.0.self_attn.q_proj.lora_A.weight",
            SafeTensorDType::F32,
            &[rank, hidden_dim],
            &a_weights,
        ),
        (
            "base_model.model.model.layers.0.self_attn.q_proj.lora_B.weight",
            SafeTensorDType::F32,
            &[hidden_dim, rank],
            &b_weights,
        ),
    ]);

    let model = parse_safetensors(&bytes).expect("Should parse synthetic SafeTensors");
    let adapter = safetensors_to_cl_lora(&model, "test_forward", rank, 1.0).unwrap();

    let input = vec![1.0f64; hidden_dim];
    let delta = adapter
        .apply_delta(0, "q_proj", &input)
        .expect("Should find layer 0 q_proj");
    assert_eq!(delta.len(), hidden_dim);

    // Delta must be non-zero and purely deterministic additions/subtractions
    let sum_abs: f64 = delta.iter().map(|v| v.abs()).sum();
    assert!(sum_abs > 0.0);
}

#[test]
fn test_safetensors_to_cl_vliw_machine_code() {
    let weights = vec![0.5f32; 16];
    let bytes = create_synthetic_safetensors(&[(
        "model.layers.0.self_attn.q_proj.weight",
        SafeTensorDType::F32,
        &[4, 4],
        &weights,
    )]);

    let model = parse_safetensors(&bytes).expect("Should parse synthetic SafeTensors");
    let cl_code = safetensors_to_cl_machine_code(&model, "MiniAttention").expect("Should lower to .cl");

    assert!(cl_code.contains(".core [0, 0, 0, 0]:"));
    assert!(cl_code.contains("@entry:"));
    assert!(cl_code.contains("B0000:"));
    // Verify 40-char VLIW microcode bundles format (4 slots of 9 chars each separated by single space: 9+1+9+1+9+1+9 = 39 chars + BXXXX: prefix)
    assert!(cl_code.lines().any(|l| l.starts_with("B0000: _MD05")));
}
