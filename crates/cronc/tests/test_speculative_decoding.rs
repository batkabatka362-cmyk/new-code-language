// ============================================================================
// CRON Hardware-Level Speculative Decoding Verification Test
// Tests multi-token drafting, causal verification, and memory bandwidth speedup.
// ============================================================================

use cronc::speculative_decoding::SpeculativeEngine;

#[test]
fn test_speculative_decoding_acceptance_and_speedup() {
    let mut engine = SpeculativeEngine::new(4);
    let prompt = vec![101, 2054, 2003]; // Initial prompt tokens

    // Target evaluator mock: agrees with first 2 drafted tokens, then diverges on the 3rd
    let generated = engine.run_speculative_decoding(&prompt, 10, |_ctx, drafts| {
        let mut target_preds = Vec::new();
        for (i, &d) in drafts.iter().enumerate() {
            if i < 2 {
                target_preds.push(d); // Accepted
            } else {
                target_preds.push(9999); // Divergence -> target correction
                break;
            }
        }
        target_preds
    });

    assert_eq!(generated.len(), prompt.len() + 10);
    assert!(engine.telemetry.total_accepted_tokens > 0);
    assert!(engine.telemetry.verification_cycles > 0);
    // Speedup factor must be > 1.0 (indicating bandwidth amplification)
    assert!(
        engine.telemetry.effective_speedup >= 1.5,
        "Expected effective speedup >= 1.5x, got {:.2}x",
        engine.telemetry.effective_speedup
    );
    assert!(
        engine.telemetry.bandwidth_savings_pct > 30.0,
        "Expected bandwidth savings > 30%, got {:.1}%",
        engine.telemetry.bandwidth_savings_pct
    );
}

#[test]
fn test_speculative_step_exact_rejection_and_correction() {
    let mut engine = SpeculativeEngine::new(3);
    let drafts = vec![10, 20, 30];
    let target_evals = vec![10, 20, 99]; // Token 30 rejected, replaced by 99

    let accepted = engine.verify_step(&drafts, &target_evals);
    assert_eq!(accepted, vec![10, 20, 99]);
    assert_eq!(engine.telemetry.total_accepted_tokens, 2);
    assert_eq!(engine.telemetry.verification_cycles, 1);
    assert_eq!(engine.telemetry.effective_speedup, 3.0); // 3 tokens emitted in 1 verification pass
}
