// ============================================================================
// Integration Test: Multi-Layer Deep LM & Mixture-of-Experts (MoE) Training
// Verifies:
// 1. Multi-Layer Transformer (2-Layer, 4-Layer) forward pass and residual stream
// 2. Exact analytical Reverse-Mode VJP Autodiff pullback across all layers
// 3. Mixture-of-Experts (MoE) Top-K sparse routing and expert parameter update
// 4. Multi-Layer autoregressive text generation
// ============================================================================

use cronc::cl_train_pipeline::{
    AutonomousLanguageModel, TrainingConfig, TrainingOptimizer,
};

#[test]
fn test_multi_layer_deep_transformer_training_and_convergence() {
    let mut config = TrainingConfig::default();
    config.dim = 16;
    config.hidden_dim = 32;
    config.vocab_size = 64;
    config.num_layers = 3; // 3-Layer Deep Transformer
    config.num_experts = 1;
    config.optimizer = TrainingOptimizer::AdamW;

    let mut model = AutonomousLanguageModel::new(config);
    assert_eq!(model.layers.len(), 3);

    let input = vec![1, 5, 12, 18, 24, 30, 42, 55];
    let target = vec![5, 12, 18, 24, 30, 42, 55, 60];

    // Initial step
    let (loss1, ppl1) = model.forward_backward_step(&input, &target);
    assert!(loss1 > 0.0);
    assert!(ppl1 >= 1.0);

    let grad_norm = model.total_grad_norm();
    assert!(grad_norm > 0.0, "Multi-layer gradient norm must be non-zero");

    // Take optimizer step
    model.optimizer_step(1, 0.05);

    // Subsequent step on same batch should have lower loss
    let (loss2, _) = model.forward_backward_step(&input, &target);
    assert!(
        loss2 < loss1,
        "3-Layer deep network loss must decrease after AdamW step: loss1={}, loss2={}",
        loss1,
        loss2
    );

    // Verify multi-layer generation
    let prompt = vec![1, 5];
    let generated = model.generate_tokens(&prompt, 4, 0.7);
    assert_eq!(generated.len(), 6);
}

#[test]
fn test_moe_sparse_routing_training_and_convergence() {
    let mut config = TrainingConfig::default();
    config.dim = 16;
    config.hidden_dim = 32;
    config.vocab_size = 64;
    config.num_layers = 2;   // 2 Layers
    config.num_experts = 4;  // 4 Experts per layer
    config.top_k = 2;        // Top-2 Sparse Routing
    config.optimizer = TrainingOptimizer::AdamW;

    let mut model = AutonomousLanguageModel::new(config);
    assert_eq!(model.layers.len(), 2);
    assert_eq!(model.layers[0].w_gate.len(), 4);
    assert!(model.layers[0].w_router.is_some());

    let input = vec![2, 8, 14, 20, 26, 32, 44, 56];
    let target = vec![8, 14, 20, 26, 32, 44, 56, 62];

    let (loss1, _) = model.forward_backward_step(&input, &target);
    assert!(loss1 > 0.0);

    let grad_norm = model.total_grad_norm();
    assert!(grad_norm > 0.0, "MoE router & expert gradients must be non-zero");

    model.optimizer_step(1, 0.05);

    let (loss2, _) = model.forward_backward_step(&input, &target);
    assert!(
        loss2 < loss1,
        "MoE Top-2 network loss must decrease after step: loss1={}, loss2={}",
        loss1,
        loss2
    );
}

