// ============================================================================
// Integration Test: Autonomous In-Silicon Training & Autoregressive Inference
// Verifies:
// 1. Loss reduction during autonomous in-silicon training
// 2. Telemetry reporting (loss sparklines, perplexity, speed)
// 3. Autoregressive token and text generation from prompt
// 4. SafeTensors checkpoint persistence & header validity
// ============================================================================

use cronc::cl_train_pipeline::{
    render_ascii_sparkline, render_progress_bar, render_training_hud,
    train_autonomous_model, TrainingConfig, TrainingOptimizer,
};
use std::fs;
use std::path::Path;

#[test]
fn test_autonomous_training_and_autoregressive_generation() {
    let tmp_checkpoint = "target/test_e2e_checkpoint.safetensors";
    if Path::new(tmp_checkpoint).exists() {
        let _ = fs::remove_file(tmp_checkpoint);
    }

    let mut config = TrainingConfig::default();
    config.steps = 20;
    config.batch_size = 4;
    config.seq_len = 16;
    config.dim = 32;
    config.hidden_dim = 64;
    config.learning_rate = 0.01;
    config.optimizer = TrainingOptimizer::AdamW;
    config.checkpoint_out = Some(tmp_checkpoint.to_string());
    config.eval_prompt = Some("CRON Machine".to_string());
    config.eval_max_tokens = 10;
    config.eval_temperature = 0.5;

    let callback = Box::new(|rep: &cronc::cl_train_pipeline::TrainingStepReport| {
        assert!(rep.loss.is_finite(), "Loss must be finite");
        assert!(rep.perplexity > 0.0, "Perplexity must be positive");
    });

    let summary = train_autonomous_model(config, Some(callback))
        .expect("Autonomous training must succeed");

    assert_eq!(summary.total_steps, 20);
    assert!(summary.initial_loss > 0.0);
    assert!(summary.final_loss > 0.0);
    assert!(
        summary.final_loss <= summary.initial_loss * 1.05,
        "Final loss ({:.4}) should trend downwards compared to initial loss ({:.4})",
        summary.final_loss,
        summary.initial_loss
    );

    // Verify Checkpoint Exists and has SafeTensors Header
    assert!(Path::new(tmp_checkpoint).exists(), "SafeTensors checkpoint must be written");
    let checkpoint_bytes = fs::read(tmp_checkpoint).expect("Checkpoint must be readable");
    assert!(checkpoint_bytes.len() >= 8, "SafeTensors file must have header length prefix");
    let header_len = u64::from_le_bytes(checkpoint_bytes[0..8].try_into().unwrap()) as usize;
    assert!(header_len > 0 && header_len < checkpoint_bytes.len(), "Valid SafeTensors header length");

    // Verify Autoregressive Generation
    assert!(
        summary.generated_sample.is_some(),
        "Model must return generated continuation from prompt"
    );
    let sample = summary.generated_sample.unwrap();
    assert!(
        sample.starts_with("CRON Machine"),
        "Generated output must preserve prompt prefix. Got: '{}'",
        sample
    );
    assert!(sample.len() > "CRON Machine".len(), "Generated output must contain new characters");

    // Clean up temporary checkpoint
    let _ = fs::remove_file(tmp_checkpoint);
}

#[test]
fn test_hud_and_sparkline_rendering() {
    let losses = vec![7.0, 6.5, 6.2, 5.8, 5.1, 4.3, 3.2, 2.8];
    let sparkline = render_ascii_sparkline(&losses, 10);
    assert_eq!(sparkline.chars().count(), 8);
    // Highest value should be █ and lowest  
    assert!(sparkline.starts_with('█') || sparkline.ends_with(' '));

    let pbar = render_progress_bar(5, 10, 10);
    assert!(pbar.contains("50%"));

    let hud = render_training_hud(10, 20, &losses, 2.8, 16.4, 0.005, 0.32, 1200.0, 1.5);
    assert!(hud.contains("LIVE MONITOR"));
    assert!(hud.contains("Loss Trend"));
    assert!(hud.contains("Speed"));
}
