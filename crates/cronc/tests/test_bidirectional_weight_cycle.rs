// ============================================================================
// CRON TEST SUITE: Bidirectional AI Model Weight Cycle & SafeTensors Merge
// Verifies closed-loop roundtrip between sub-byte discrete CL-LoRA and
// full-scale FP16/FP32 SafeTensors models.
// ============================================================================

use cronc::cl_lora::{ClLoraAdapter, ClLoraConfig, DiscreteLoraLayer};
use cronc::model_importer::safetensors::{
    create_synthetic_safetensors, merge_safetensors_file_with_cl_lora, parse_safetensors,
    SafeTensorDType,
};

#[test]
fn test_discrete_lora_dense_delta_mathematical_identity() {
    let in_dim = 4;
    let out_dim = 3;
    let rank = 2;
    let scale = 1.0;

    // A is rank x in_dim (2 x 4)
    // Row 0: [ 1,  0, -1,  1]
    // Row 1: [-1,  1,  0,  1]
    let a_trits: Vec<i8> = vec![
        1, 0, -1, 1,
        -1, 1, 0, 1,
    ];

    // B is out_dim x rank (3 x 2)
    // Row 0: [ 1, -1]
    // Row 1: [ 0,  1]
    // Row 2: [-1, -1]
    let b_trits: Vec<i8> = vec![
        1, -1,
        0, 1,
        -1, -1,
    ];

    let layer = DiscreteLoraLayer::new(0, "q_proj", in_dim, out_dim, rank, &a_trits, &b_trits);
    let delta_w = layer.reconstruct_dense_delta(scale);
    assert_eq!(delta_w.len(), out_dim * in_dim);

    // Expected Delta W = B x A:
    // Row 0: [1*(1) + (-1)*(-1), 1*(0) + (-1)*(1), 1*(-1) + (-1)*(0), 1*(1) + (-1)*(1)] = [2, -1, -1, 0]
    // Row 1: [0*(1) +  1*(-1),   0*(0) +  1*(1),   0*(-1) +  1*(0),   0*(1) +  1*(1)]   = [-1, 1, 0, 1]
    // Row 2: [-1*(1) + (-1)*(-1),-1*(0) + (-1)*(1),-1*(-1) + (-1)*(0),-1*(1) + (-1)*(1)] = [0, -1, 1, -2]
    let expected: Vec<f64> = vec![
        2.0, -1.0, -1.0, 0.0,
        -1.0, 1.0, 0.0, 1.0,
        0.0, -1.0, 1.0, -2.0,
    ];

    for (idx, (&actual, &exp)) in delta_w.iter().zip(expected.iter()).enumerate() {
        assert_eq!(actual, exp, "Mismatch at index {}: got {}, expected {}", idx, actual, exp);
    }
}

#[test]
fn test_forward_delta_vs_dense_matrix_multiplication_parity() {
    let in_dim = 16;
    let out_dim = 32;
    let rank = 4;
    let scale = 0.5;

    let mut a_trits = Vec::with_capacity(rank * in_dim);
    let mut b_trits = Vec::with_capacity(out_dim * rank);

    for i in 0..(rank * in_dim) {
        a_trits.push(match i % 3 { 0 => 1, 1 => -1, _ => 0 });
    }
    for i in 0..(out_dim * rank) {
        b_trits.push(match (i + 1) % 3 { 0 => 1, 1 => -1, _ => 0 });
    }

    let layer = DiscreteLoraLayer::new(0, "v_proj", in_dim, out_dim, rank, &a_trits, &b_trits);
    let delta_w = layer.reconstruct_dense_delta(scale);

    let test_x: Vec<f64> = (0..in_dim).map(|i| (i as f64) * 0.25 - 2.0).collect();

    // 1. Compute via Discrete 0-FLOP Forward Delta
    let y_discrete = layer.forward_delta(&test_x, scale);

    // 2. Compute via Dense Matrix-Vector Multiplication: y_dense = Delta W * x
    let mut y_dense = vec![0.0f64; out_dim];
    for o in 0..out_dim {
        let mut acc = 0.0f64;
        for i in 0..in_dim {
            acc += delta_w[o * in_dim + i] * test_x[i];
        }
        y_dense[o] = acc;
    }

    // Assert exact numerical equality (< 1e-12)
    for o in 0..out_dim {
        let diff = (y_discrete[o] - y_dense[o]).abs();
        assert!(
            diff < 1e-12,
            "Parity divergence at row {}: discrete={}, dense={}, diff={}",
            o, y_discrete[o], y_dense[o], diff
        );
    }
}

#[test]
fn test_safetensors_adapter_merge_and_roundtrip() {
    let dim = 8;
    let rank = 2;
    let scale = 1.0;

    // 1. Create a base model SafeTensors with 1 layer
    let initial_weights: Vec<f32> = (0..(dim * dim)).map(|i| (i as f32) * 0.1).collect();
    let synthetic_bytes = create_synthetic_safetensors(&[(
        "model.layers.0.self_attn.q_proj.weight",
        SafeTensorDType::F32,
        &[dim, dim],
        &initial_weights,
    )]);

    let mut model = parse_safetensors(&synthetic_bytes).expect("Valid SafeTensors parse");

    // 2. Create and train a discrete CL-LoRA adapter
    let config = ClLoraConfig {
        rank,
        alpha: scale * (rank as f64),
        hidden_dim: dim,
        num_layers: 1,
        target_modules: vec!["q_proj".to_string()],
    };

    let mut adapter = ClLoraAdapter::new("test_adapter", config);
    let a_trits = vec![1i8; rank * dim];
    let b_trits = vec![1i8; dim * rank];
    adapter.layers.push(DiscreteLoraLayer::new(0, "q_proj", dim, dim, rank, &a_trits, &b_trits));

    // Expected Delta W for all-1s: Delta W = B x A where each element is rank * 1 * 1 = rank * 1 = 2.0
    let delta = adapter.reconstruct_dense_delta(0, "q_proj").expect("Layer exists");
    for &d in &delta {
        assert_eq!(d, 2.0);
    }

    // 3. Merge adapter into base SafeTensors model
    let merged_count = model.merge_cl_lora(&adapter).expect("Merge succeeds");
    assert_eq!(merged_count, 1, "Expected exactly 1 tensor merged");

    // 4. Verify in-place updated values: W_new = W_base + 2.0
    let updated_tensor = &model.tensors["model.layers.0.self_attn.q_proj.weight"];
    for i in 0..(dim * dim) {
        let expected_val = initial_weights[i] + 2.0;
        let diff = (updated_tensor.values[i] - expected_val).abs();
        assert!(diff < 1e-6, "Value mismatch at {}: expected {}, got {}", i, expected_val, updated_tensor.values[i]);
    }

    // 5. Serialize model back to SafeTensors bytes and re-parse
    let serialized_bytes = model.serialize_to_bytes();
    assert!(serialized_bytes.len() > synthetic_bytes.len() / 2);

    let reloaded_model = parse_safetensors(&serialized_bytes).expect("Re-parse re-serialized model");
    let reloaded_tensor = &reloaded_model.tensors["model.layers.0.self_attn.q_proj.weight"];
    assert_eq!(reloaded_tensor.values.len(), dim * dim);
    assert_eq!(reloaded_tensor.values, updated_tensor.values);
}

#[test]
fn test_file_based_merge_safetensors_with_cl_lora() {
    let temp_dir = std::env::temp_dir();
    let base_path = temp_dir.join("test_cycle_base_model.safetensors");
    let adapter_path = temp_dir.join("test_cycle_adapter.cll");
    let merged_path = temp_dir.join("test_cycle_merged_model.safetensors");

    let dim = 4;
    let base_vals = vec![10.0f32; dim * dim];
    let base_bytes = create_synthetic_safetensors(&[(
        "q_proj",
        SafeTensorDType::F32,
        &[dim, dim],
        &base_vals,
    )]);
    std::fs::write(&base_path, base_bytes).expect("Write base model");

    let config = ClLoraConfig {
        rank: 1,
        alpha: 1.0,
        hidden_dim: dim,
        num_layers: 1,
        target_modules: vec!["q_proj".to_string()],
    };
    let mut adapter = ClLoraAdapter::new("cycle_adapter", config);
    adapter.layers.push(DiscreteLoraLayer::new(0, "q_proj", dim, dim, 1, &[1; 4], &[1; 4]));
    adapter.save_to_file(&adapter_path).expect("Save adapter");

    let merged_count = merge_safetensors_file_with_cl_lora(&base_path, &adapter_path, &merged_path)
        .expect("File merge execution");
    assert_eq!(merged_count, 1);

    let merged_bytes = std::fs::read(&merged_path).expect("Read merged model");
    let merged_model = parse_safetensors(&merged_bytes).expect("Parse merged");

    // All elements should now be 10.0 + 1.0 = 11.0
    for &v in &merged_model.tensors["q_proj"].values {
        assert_eq!(v, 11.0f32);
    }

    // Clean up temporary test files
    let _ = std::fs::remove_file(base_path);
    let _ = std::fs::remove_file(adapter_path);
    let _ = std::fs::remove_file(merged_path);
}
