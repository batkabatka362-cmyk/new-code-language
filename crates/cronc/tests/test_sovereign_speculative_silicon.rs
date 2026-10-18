// ============================================================================
// Test Suite: CRON Sovereign Dual-Model Speculative Silicon Acceleration
// Pairs Sovereign-1B (Draft) with Sovereign-3B (Target) across 256-Core 4D-Torus
// ============================================================================

use cronc::speculative_decoding::SpatialSiliconSpeculativeSession;

#[test]
fn test_sovereign_1b_3b_speculative_silicon_acceleration() {
    let draft_cl = std::fs::read_to_string("models/sovereign_1b/main.cl")
        .or_else(|_| std::fs::read_to_string("../../models/sovereign_1b/main.cl"))
        .expect("Must read Sovereign-1B main.cl");

    let target_cl = std::fs::read_to_string("models/sovereign_3b/main.cl")
        .or_else(|_| std::fs::read_to_string("../../models/sovereign_3b/main.cl"))
        .expect("Must read Sovereign-3B main.cl");

    let session = SpatialSiliconSpeculativeSession::new(4, &draft_cl, &target_cl);
    let prompt = vec![101, 2054, 2003, 1037]; // "what is a"

    let report = session.generate(&prompt, 16)
        .expect("Speculative generation across Sovereign-1B and 3B must succeed");

    assert_eq!(report.generated_tokens.len(), 16);
    assert_eq!(report.prompt_len, 4);
    assert!(report.total_drafted > 0);
    assert!(report.total_accepted > 0);
    assert!(
        report.acceptance_rate_pct >= 60.0,
        "Acceptance rate should be >= 60%, got {:.1}%",
        report.acceptance_rate_pct
    );
    assert!(
        report.effective_speedup >= 1.8,
        "Effective speedup must be >= 1.8x, got {:.2}x",
        report.effective_speedup
    );
    assert!(
        report.bandwidth_savings_pct >= 40.0,
        "Bandwidth savings must be >= 40%, got {:.1}%",
        report.bandwidth_savings_pct
    );
    assert!(report.verification_passes > 0);
}
