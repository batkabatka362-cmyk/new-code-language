use cronc::{
    DoubleBufferedLayerStaging, ModelAccelerationPipeline, NeuralDraftPredictor,
    SparseMoeRouter,
};

#[test]
fn test_sparse_moe_router_top_k() {
    let num_experts = 16;
    let top_k = 2;
    let hidden_dim = 64;
    let router = SparseMoeRouter::new(num_experts, top_k, hidden_dim);

    let dummy_hidden: Vec<i32> = (0..hidden_dim).map(|i| (i as i32 * 2) - 30).collect();
    let routed = router.route_token(&dummy_hidden);

    assert_eq!(routed.len(), top_k, "Must select exactly Top-2 active experts");
    let reduction = router.bandwidth_reduction_pct();
    assert!(
        (reduction - 87.5).abs() < 1e-4,
        "Top-2 out of 16 experts must achieve 87.5% memory bandwidth reduction"
    );
}

#[test]
fn test_double_buffered_layer_staging() {
    let layer_size = 1024;
    let mut staging = DoubleBufferedLayerStaging::new(layer_size);

    assert!(staging.active_is_a);
    let buf1 = staging.active_compute_buffer();
    assert_eq!(buf1.len(), layer_size);

    // Prefetch next layer into inactive buffer B
    staging.prefetch_next_layer(1);
    assert_eq!(staging.hits, 1);

    // Flip buffer
    staging.flip_buffers();
    assert!(!staging.active_is_a, "Now buffer B is active");

    // Prefetch into inactive buffer A
    staging.prefetch_next_layer(2);
    assert_eq!(staging.hits, 2);

    staging.flip_buffers();
    assert!(staging.active_is_a, "Now buffer A is active again");
}

#[test]
fn test_neural_draft_predictor_generation() {
    let predictor = NeuralDraftPredictor::new(4);
    let context = vec![1, 2, 3, 42];
    let drafts = predictor.propose_draft_candidates(&context);

    assert_eq!(drafts.len(), 4, "Must generate exactly K=4 draft candidates");
    assert!(drafts.iter().all(|&t| t < 32000), "Candidates must be valid vocab IDs");
}

#[test]
fn test_end_to_end_accelerated_generation_speedup() {
    let mut pipeline = ModelAccelerationPipeline::new(4, 16, 2, 64, 2048);
    let prompt = vec![10, 20, 30];

    // Target verifier: accepts draft candidates with 75% probability
    let sequence = pipeline.run_accelerated_generation(&prompt, 16, |_ctx, drafts| {
        let mut evals = Vec::new();
        for (i, &d) in drafts.iter().enumerate() {
            if i % 4 != 3 {
                evals.push(d); // Accept
            } else {
                evals.push(d + 1); // Reject & correct
            }
        }
        evals.push(999); // Next token
        evals
    });

    assert!(sequence.len() >= prompt.len() + 16, "Must generate requested tokens");
    assert!(pipeline.telemetry.speculative_speedup > 1.0, "Speculative decoding must provide speedup");
    assert!(pipeline.telemetry.net_speedup_factor > 1.5, "Net acceleration must exceed 1.5x");
    assert!(pipeline.telemetry.total_energy_pj > 0.0, "Energy dissipation must be tracked");

    // Verify ASCII HUD
    let hud = pipeline.render_ascii_acceleration_hud();
    assert!(hud.contains("CRON HIGH-THROUGHPUT HARDWARE ACCELERATION"));
    assert!(hud.contains("Speculative Boost"));
    assert!(hud.contains("Sparse MoE Gating"));
    assert!(hud.contains("FASTER"));
}
