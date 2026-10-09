// ============================================================================
// CRON Matrix 4.1 Cortex Supercomputer End-to-End Pipeline Test
// File: crates/cronc/tests/test_matrix4_cortex_pipeline.rs
// ============================================================================

use cronc::cl_lint::lint_cl_source;
use cronc::cl_topological::{ClTopologicalCompiler, UnifiedTopologicalRuntime};
use cronc::cl_topological_lm::TopologicalBrainTransformer;
use cronc::{SphereInferenceEngine, SphereTorusCluster, SphereWeightMatrix};

fn resolve_file_path(rel_path: &str) -> std::path::PathBuf {
    let p = std::path::Path::new(rel_path);
    if p.exists() {
        return p.to_path_buf();
    }
    let p_up2 = std::path::Path::new("../../").join(rel_path);
    if p_up2.exists() {
        return p_up2;
    }
    let p_up1 = std::path::Path::new("../").join(rel_path);
    if p_up1.exists() {
        return p_up1;
    }
    p.to_path_buf()
}

#[test]
fn test_matrix4_cortex_supercomputer_e2e_pipeline() {
    let cl_file = resolve_file_path("examples/matrix4_cortex_supercomputer/cortex_program.cl");
    let corpus_file = resolve_file_path("examples/matrix4_cortex_supercomputer/knowledge_corpus.txt");

    // 1. Static Linter Audit (Zero Verbal Words & Zero Register Hazards)
    let cl_code = std::fs::read_to_string(&cl_file).expect("cortex_program.cl must exist");
    let lint = lint_cl_source(&cl_code, cl_file.to_str().unwrap_or("cortex_program.cl"));
    assert!(lint.is_clean, "cortex_program.cl must be 100% clean of all hazards and verbal words");
    assert_eq!(lint.error_count, 0);
    assert_eq!(lint.warning_count, 0);

    // 2. Discrete 4D Clifford Kinematics & 0-Anchor Hydraulics
    let bundles = ClTopologicalCompiler::compile_source(&cl_code).expect("Compilation must succeed");
    assert_eq!(bundles.len(), 5, "Must compile exactly 5 VLIW bundles");

    let mut runtime = UnifiedTopologicalRuntime::new();
    let mut total_deflections = 0usize;
    for b in &bundles {
        runtime.load_bundle(b);
        let telem = runtime.step_cycle();
        assert_eq!(telem.hamiltonian, 100, "Hamiltonian energy must strictly be 100% across all bundles");
        total_deflections += telem.deflections;
        assert_eq!(telem.raw_universe_hex.len(), 32, "128-bit universe hex must be 32 chars");
    }
    assert!(total_deflections > 0, "Braid deflector must have intervened to enforce non-collision");

    // 3. Multiplier-Free 256-Core 4D-Torus Matrix Supercomputer (0 FLOPs)
    let weights: Vec<f32> = (0..4096).map(|i| ((i as f32 * 0.41).sin() * 2.0) - 0.2).collect();
    let matrix = SphereWeightMatrix::from_floats("cortex_w4d", 64, 64, &weights);
    let activations: Vec<i32> = (0..64).map(|c| (c as i32 * 5 + 3) % 15).collect();

    let single_res = SphereInferenceEngine::infer_layer_native(&matrix, &activations, 0);
    assert!(single_res.compression_ratio > 15.0, "Must achieve >15x compression vs FP32");

    let mut cluster = SphereTorusCluster::new();
    let cluster_res = cluster.execute_matrix_supercomputer(&matrix, &activations);
    assert_eq!(cluster_res.total_noc_hops, 1024);
    assert!(cluster_res.aggregate_throughput_mops_sec > 10.0);

    // 4. Neocortical STDP Training on Manifold (Zero Backprop)
    let corpus_text = std::fs::read_to_string(&corpus_file).expect("knowledge_corpus.txt must exist");
    let mut brain = TopologicalBrainTransformer::new(4, 512);
    brain.train_sequence_stdp(&corpus_text);
    assert!(brain.total_tokens_processed >= corpus_text.len());

    // 5. Softmax-Free Wave Interference Text Generation & 3-Byte KV-Cache
    let output = brain.generate("CRON Matrix 4.1", 16);
    assert_eq!(output.len(), 16);
    let kv_bytes = brain.kv_cache.memory_bytes();
    let fp16_bytes = brain.kv_cache.len() * 512;
    let savings_pct = (1.0 - (kv_bytes as f64 / fp16_bytes as f64)) * 100.0;
    assert!(savings_pct > 95.0, "3-byte KV cache must achieve >95% savings over FP16");

    // 6. 128-Bit Verilog ROM Hex Export
    for b in &bundles {
        let hex = b.to_universe_128().to_verilog_hex();
        assert_eq!(hex.len(), 32);
    }
}
