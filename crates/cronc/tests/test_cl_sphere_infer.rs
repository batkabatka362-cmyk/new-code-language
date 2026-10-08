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
    // Cosine similarity should be positive (meaningful alignment with float direction)
    assert!(result.cosine_similarity_vs_float > 0.0);
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

