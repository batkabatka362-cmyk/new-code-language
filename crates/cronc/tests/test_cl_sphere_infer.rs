use cronc::cl_lang::parse_slot;
use cronc::cl_sphere_infer::{
    compile_weights_to_sphere_cl, SphereCompileOptions, SphereInferenceEngine, SphereWeightMatrix,
};
use cronc::verilog_backend::generate_verilog_hdl;

#[test]
fn test_sphere_weight_matrix_quantization() {
    let raw_weights = vec![
        1.2, -0.8, 0.05, -1.5,
        0.01, 2.1, -0.02, 0.9,
        -1.1, -0.05, 0.8, -2.0,
        1.5, 0.0, -1.2, 0.7,
    ];

    let mat = SphereWeightMatrix::from_floats("layer1.weight", 4, 4, &raw_weights);
    assert_eq!(mat.rows, 4);
    assert_eq!(mat.cols, 4);
    assert_eq!(mat.ternary_weights.len(), 16);
    // Every weight must be -1, 0, or 1
    for &w in &mat.ternary_weights {
        assert!(w == -1 || w == 0 || w == 1);
    }
    assert!(mat.scale > 0.0);
}

#[test]
fn test_compile_weights_to_sphere_cl() {
    let raw_weights = vec![
        1.5, -1.2, 0.0, 0.8,
        -0.9, 2.0, -1.5, 0.1,
        0.05, -1.8, 1.1, -0.4,
        1.0, 0.0, -0.7, 1.3,
    ];

    let mat = SphereWeightMatrix::from_floats("test_layer", 4, 4, &raw_weights);
    let activations = vec![3, 7, 1, 5];
    let opts = SphereCompileOptions {
        layer_depth: 1,
        max_bundles_limit: 100,
        target_heads: 4,
        enable_reversible_reset: true,
    };

    let cl_code = compile_weights_to_sphere_cl(&mat, &activations, &opts);
    assert!(cl_code.contains("@kernel sphere_neural_layer_test_layer"));
    assert!(cl_code.contains(".target silicon.sphere_isa"));

    // Verify all bundles and slots
    for line in cl_code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('B') && trimmed.contains(':') {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            assert_eq!(parts.len(), 5, "Each bundle must have Bxxxx: and 4 slots");
            for slot_str in &parts[1..5] {
                // Every slot must be valid and 10 characters
                let slot = parse_slot(slot_str).expect("Valid slot parsing");
                assert_eq!(slot.raw.len(), 10);
            }
        }
    }
}

#[test]
fn test_sphere_inference_native_and_similarity() {
    let rows = 8;
    let cols = 8;
    let mut raw_weights = Vec::with_capacity(rows * cols);
    for i in 0..(rows * cols) {
        let val = ((i as f32 * 0.73).sin() * 2.0) - 0.3;
        raw_weights.push(val);
    }

    let mat = SphereWeightMatrix::from_floats("attn_proj", rows, cols, &raw_weights);
    let activations: Vec<i32> = (0..cols).map(|c| (c as i32 * 2 + 1) % 15).collect();

    let result = SphereInferenceEngine::infer_layer_native(&mat, &activations, 0);

    assert_eq!(result.full_layer_output.len(), rows);
    assert_eq!(result.dsp_multipliers_used, 0);
    assert_eq!(result.flop_count, 0);
    assert!(result.total_geodesic_ops > 0);
    assert!(result.compression_ratio > 10.0);
    // Cosine similarity must be exact (~1.0) under signed BitNet b1.58 multiplier-free MAC
    assert!(result.cosine_similarity_vs_float > 0.999, "Cosine similarity must be > 0.999, got {}", result.cosine_similarity_vs_float);
}

#[test]
fn test_sphere_inference_via_cl_jit() {
    let raw_weights = vec![
        1.0, -1.0, 0.5, 0.0,
        -1.5, 1.2, 0.0, -0.8,
        0.0, 1.0, -1.0, 0.7,
        1.1, -0.5, 1.0, -1.0,
    ];

    let mat = SphereWeightMatrix::from_floats("core_mlp", 4, 4, &raw_weights);
    let activations = vec![4, 2, 6, 1];
    let opts = SphereCompileOptions::default();

    let jit_core = SphereInferenceEngine::infer_via_cl_jit(&mat, &activations, &opts)
        .expect("JIT execution of compiled sphere microcode");

    assert!(jit_core.sphere_curvature_ops_count > 0);
    assert!(jit_core.cycle_count > 0);
}

#[test]
fn test_sphere_neural_to_verilog_synthesis() {
    let raw_weights = vec![
        1.0, -1.0, 0.0, 1.0,
        -1.0, 1.0, -1.0, 0.0,
        0.0, -1.0, 1.0, 1.0,
        1.0, 0.0, -1.0, -1.0,
    ];

    let mat = SphereWeightMatrix::from_floats("synth_layer", 4, 4, &raw_weights);
    let activations = vec![2, 5, 1, 3];
    let opts = SphereCompileOptions::default();

    let cl_code = compile_weights_to_sphere_cl(&mat, &activations, &opts);

    let verilog = generate_verilog_hdl(&cl_code, "neural_sphere_layer_rtl")
        .expect("Synthesis to Verilog HDL");

    assert!(verilog.contains("module neural_sphere_layer_rtl ("));
    assert!(verilog.contains("6'h38: begin // 4D Sphere Geodesic ALU"));
    assert!(verilog.contains("sphere_acc[sph_head] <="));
}

#[test]
fn test_sphere_torus_256_core_cluster() {
    use cronc::cl_sphere_infer::SphereTorusCluster;

    let mut cluster = SphereTorusCluster::new();
    assert_eq!(cluster.cores.len(), 256);

    // Large test matrix (64x64 = 4096 parameters)
    let rows = 64;
    let cols = 64;
    let mut raw_weights = Vec::with_capacity(rows * cols);
    for i in 0..(rows * cols) {
        let w = if i % 3 == 0 { 1.5 } else if i % 3 == 1 { -1.2 } else { 0.0 };
        raw_weights.push(w);
    }

    let mat = SphereWeightMatrix::from_floats("torus_large_layer", rows, cols, &raw_weights);
    let activations: Vec<i32> = (0..cols).map(|c| (c as i32 * 3 + 2) % 15).collect();

    let report = cluster.execute_matrix_supercomputer(&mat, &activations);

    assert_eq!(report.total_cores, 256);
    assert_eq!(report.active_cores, 256);
    assert!(report.total_cluster_geodesic_ops > 0);
    assert_eq!(report.dsp_multipliers_used, 0);
    assert_eq!(report.flop_count, 0);
    assert!(report.total_noc_hops > 0);
    assert!(report.landauer_zero_entropy_verified);
    assert!(report.aggregate_throughput_mops_sec > 0.0);
}

#[test]
fn test_sphere_transformer_model_forward_step() {
    use cronc::cl_sphere_infer::{SphereKVCache, SphereTransformerConfig, SphereTransformerModel};

    let config = SphereTransformerConfig {
        vocab_size: 256,
        hidden_dim: 32,
        num_layers: 2,
        num_heads: 4,
        head_dim: 8,
        intermediate_dim: 64,
        max_seq_len: 64,
    };

    let model = SphereTransformerModel::new_synthetic(config.clone());
    assert!(model.total_parameters > 0);

    let mut kv_cache = SphereKVCache::new(config.num_layers, config.max_seq_len, config.hidden_dim);

    let (logits, ops) = model.forward_step(b'A', 0, &mut kv_cache);
    assert_eq!(logits.len(), 256);
    assert!(ops > 0);
    assert_eq!(kv_cache.k.len(), 2);
}

#[test]
fn test_sphere_transformer_generate_text() {
    use cronc::cl_sphere_infer::{SphereTransformerConfig, SphereTransformerModel};

    let config = SphereTransformerConfig {
        vocab_size: 256,
        hidden_dim: 32,
        num_layers: 2,
        num_heads: 4,
        head_dim: 8,
        intermediate_dim: 64,
        max_seq_len: 64,
    };

    let model = SphereTransformerModel::new_synthetic(config);
    let result = model.generate_text("Hi", 6, 0.0);

    assert_eq!(result.tokens_generated, 6);
    assert_eq!(result.dsp_multipliers_used, 0);
    assert_eq!(result.flop_count, 0);
    assert!(result.total_geodesic_ops > 0);
    assert!(result.tokens_per_sec > 0.0);
    assert!(result.memory_reduction_percent > 90.0);
}

#[test]
fn test_sphere_transformer_from_safetensors_fixture() {
    use cronc::cl_sphere_infer::SphereTransformerModel;
    use std::path::Path;

    let path = Path::new("tests/fixtures/pytorch_transformer_layer.safetensors");
    if !path.exists() {
        // Fallback relative path for workspace root
        let root_path = Path::new("../../tests/fixtures/pytorch_transformer_layer.safetensors");
        if !root_path.exists() {
            return;
        }
    }

    let actual_path = if path.exists() { path } else { Path::new("../../tests/fixtures/pytorch_transformer_layer.safetensors") };
    let model = SphereTransformerModel::from_safetensors_file(actual_path)
        .expect("Loading real safetensors model weights");

    assert!(model.total_parameters > 0);
    assert_eq!(model.config.num_heads, 4);

    let res = model.generate_text("CRON", 4, 0.7);
    assert_eq!(res.tokens_generated, 4);
    assert_eq!(res.flop_count, 0);
    assert_eq!(res.dsp_multipliers_used, 0);
    assert!(res.total_geodesic_ops > 0);
}

#[test]
fn test_sphere_transformer_compile_to_sphere_cl() {
    use cronc::cl_sphere_infer::SphereTransformerModel;
    use cronc::cl_lang::parse_slot;
    use std::path::Path;

    let path = Path::new("tests/fixtures/pytorch_transformer_layer.safetensors");
    let actual_path = if path.exists() {
        path
    } else {
        Path::new("../../tests/fixtures/pytorch_transformer_layer.safetensors")
    };
    if !actual_path.exists() {
        return;
    }

    let model = SphereTransformerModel::from_safetensors_file(actual_path)
        .expect("Loading model for CL compilation");
    let cl_code = model.compile_to_sphere_cl("CRON");

    assert!(cl_code.contains("@kernel sphere_transformer_llm_flow"));
    assert!(cl_code.contains(".target silicon.sphere_isa"));

    let mut bundle_count = 0;
    for line in cl_code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('B') && trimmed.contains(':') {
            bundle_count += 1;
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            assert_eq!(parts.len(), 5);
            for slot_str in &parts[1..5] {
                let slot = parse_slot(slot_str).expect("Valid slot in LLM microcode");
                assert_eq!(slot.raw.len(), 10);
            }
        }
    }
    assert!(bundle_count > 0, "Generated microcode must contain VLIW bundles");
}

#[test]
fn test_4d_hypersphere_riemannian_geodesic_and_hopf_fibration() {
    use cronc::cl_sphere_infer::HyperspherePoint4D;
    use std::f32::consts::PI;

    // 1. Great circle distance on unit S^3
    let north_pole = HyperspherePoint4D::new(1.0, 0.0, 0.0, 0.0);
    let equator_y = HyperspherePoint4D::new(0.0, 1.0, 0.0, 0.0);
    let south_pole = HyperspherePoint4D::new(-1.0, 0.0, 0.0, 0.0);

    let d_equator = north_pole.geodesic_distance(&equator_y);
    assert!((d_equator - (PI / 2.0)).abs() < 1e-5, "Geodesic to equator must be PI/2, got {}", d_equator);

    let d_antipodal = north_pole.geodesic_distance(&south_pole);
    assert!((d_antipodal - PI).abs() < 1e-5, "Geodesic to antipodal point must be PI, got {}", d_antipodal);

    // 2. Hopf fibration projection pi: S^3 -> S^2
    // For any unit vector in S^3, the projected vector in R^3 must lie on unit sphere S^2 (norm == 1.0)
    let p_arb = HyperspherePoint4D::from_angles(0.7, 1.2, 2.1);
    let [xi1, xi2, xi3] = p_arb.hopf_projection();
    let s2_norm = (xi1 * xi1 + xi2 * xi2 + xi3 * xi3).sqrt();
    assert!((s2_norm - 1.0).abs() < 1e-5, "Hopf projection must lie on unit S^2, norm was {}", s2_norm);
}


