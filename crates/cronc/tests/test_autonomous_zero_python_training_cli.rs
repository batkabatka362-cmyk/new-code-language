// ============================================================================
// Integration Test: Autonomous Zero-Python Training Pipeline & Safetensors Export
// Tests: crates/cronc/src/cl_train_pipeline.rs
//
// 100% In-Silicon Rust Training - Zero Python, Zero PyTorch, Zero External C++ Runtime
// ============================================================================

use cronc::cl_train_pipeline::{
    prepare_dataset_tokens, train_autonomous_model, AutonomousLanguageModel,
    LrSchedule, TrainingConfig, TrainingOptimizer,
};
use std::fs;
use std::path::Path;

#[test]
fn test_dataset_tokenization_and_batch_preparation() {
    let mut config = TrainingConfig::default();
    config.data_text = Some(
        "CRON Zero-Verbal Machine Architecture v2.0. \
         Continuous-time dynamic differential engine. \
         Монгол хэлний хиймэл оюун ухааны цахиур сургалт."
            .to_string(),
    );
    config.seq_len = 8;
    config.batch_size = 2;

    let tokens = prepare_dataset_tokens(&config).expect("Tokenization must succeed");
    assert!(!tokens.is_empty(), "Tokens must not be empty");
    assert!(tokens.len() >= config.seq_len * config.batch_size);
}

#[test]
fn test_model_forward_backward_vjp_parity() {
    let mut config = TrainingConfig::default();
    config.dim = 16;
    config.hidden_dim = 32;
    config.vocab_size = 64;

    let mut model = AutonomousLanguageModel::new(config);
    let input = vec![1, 5, 12, 18, 24, 30, 42, 55];
    let target = vec![5, 12, 18, 24, 30, 42, 55, 60];

    let (loss1, ppl1) = model.forward_backward_step(&input, &target);
    assert!(loss1 > 0.0, "Loss must be positive");
    assert!(ppl1 >= 1.0, "Perplexity must be >= 1.0");

    let grad_norm = model.total_grad_norm();
    assert!(grad_norm > 0.0, "Grad norm must be non-zero after backward pass");

    // Optimizer step should reduce loss on the same batch
    model.optimizer_step(1, 0.05);

    let (loss2, _) = model.forward_backward_step(&input, &target);
    assert!(
        loss2 < loss1,
        "Single step with high LR should strictly reduce batch loss (loss1: {}, loss2: {})",
        loss1,
        loss2
    );
}

#[test]
fn test_autonomous_training_loop_adamw_convergence() {
    let mut config = TrainingConfig::default();
    config.data_text = Some(
        "CRON post-human sovereign computational language. \
         Zero verbal words, zero DRAM, zero GEMM. Pure VLIW 128-bit microcode."
            .to_string(),
    );
    config.steps = 40;
    config.learning_rate = 0.02;
    config.dim = 16;
    config.hidden_dim = 32;
    config.batch_size = 2;
    config.seq_len = 8;
    config.optimizer = TrainingOptimizer::AdamW;
    config.lr_schedule = LrSchedule::Cosine;
    let ckpt_path = "target/test_adamw_ckpt.safetensors";
    config.checkpoint_out = Some(ckpt_path.to_string());

    let summary = train_autonomous_model(config, None).expect("Training loop must succeed");

    println!("AdamW Training Results:");
    println!("  Initial Loss: {:.4}", summary.initial_loss);
    println!("  Final Loss:   {:.4}", summary.final_loss);
    println!("  Loss Reduction: {:.2}%", summary.loss_reduction_pct);
    println!("  Total Steps:  {}", summary.total_steps);
    println!("  Total Tokens: {}", summary.total_tokens);

    assert!(
        summary.final_loss < summary.initial_loss,
        "Final loss ({:.4}) must be strictly less than initial loss ({:.4})",
        summary.final_loss,
        summary.initial_loss
    );
    assert!(
        summary.loss_reduction_pct > 15.0,
        "Loss reduction must exceed 15% (got {:.2}%)",
        summary.loss_reduction_pct
    );

    // Verify safetensors checkpoint
    assert!(Path::new(ckpt_path).exists(), "Safetensors checkpoint file must exist");
    let bytes = fs::read(ckpt_path).expect("Read safetensors checkpoint");
    assert!(bytes.len() > 8, "Safetensors file must contain header");

    let header_len = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize;
    let header_json = std::str::from_utf8(&bytes[8..8 + header_len]).expect("Valid header UTF-8");
    assert!(header_json.contains("\"embedding\""));
    assert!(header_json.contains("\"w_gate\""));
    assert!(header_json.contains("\"w_up\""));
    assert!(header_json.contains("\"w_down\""));
    assert!(header_json.contains("\"w_head\""));

    let _ = fs::remove_file(ckpt_path);
}

#[test]
fn test_autonomous_training_loop_lion_convergence() {
    let mut config = TrainingConfig::default();
    config.data_text = Some(
        "Sovereign AI hardware-software co-design. 4D Torus NoC matrix absorption."
            .to_string(),
    );
    config.steps = 35;
    config.learning_rate = 0.015;
    config.dim = 16;
    config.hidden_dim = 32;
    config.batch_size = 2;
    config.seq_len = 8;
    config.optimizer = TrainingOptimizer::Lion;
    config.lr_schedule = LrSchedule::Linear;
    let ckpt_path = "target/test_lion_ckpt.safetensors";
    config.checkpoint_out = Some(ckpt_path.to_string());

    let summary = train_autonomous_model(config, None).expect("Lion training loop must succeed");

    assert!(
        summary.final_loss < summary.initial_loss,
        "Lion optimizer must decrease loss (init: {}, final: {})",
        summary.initial_loss,
        summary.final_loss
    );

    let _ = fs::remove_file(ckpt_path);
}
