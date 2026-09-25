use cronc::cl_self_rewriter::*;
use cronc::cl_sleep_consolidation::*;
use cronc::cl_ternary_simd::TritWord128;

#[test]
fn test_self_rewriting_engine_hot_patch_and_rollback() {
    let cl_program = r#"
B0001: _TL03$004> _SH00$104> _SP00#008> _ST00$100>
B0002: _NO00#000> _NO00#000> _NO00#000> _NO00#000>
B0003: _RS00#00B> _HL00#000! _NO00#000> _NO00#000>
"#;

    let mut engine = SelfRewritingEngine::new();
    let count = engine.load_program(cl_program).expect("Loading .cl program failed");
    assert_eq!(count, 3);

    // Create a shadow checkpoint before mutation
    engine.create_checkpoint(2, 101).expect("Checkpoint creation failed");

    // Hot-patch B0002 slot 0 to an optical GEMM slot: _OP01$000>
    let status = engine.hot_patch_slot(2, 0, "_OP01$000>".to_string(), true);
    assert_eq!(status.unwrap(), PatchStatus::Applied);

    let bundle = engine.active_icache.get(&2).unwrap();
    assert!(bundle[0].starts_with("_OP01$0"));

    // Anomaly detected -> perform 1-cycle rollback
    let rb_status = engine.rollback_latest_checkpoint().expect("Rollback failed");
    assert!(matches!(rb_status, PatchStatus::RolledBack(_)));

    let restored_bundle = engine.active_icache.get(&2).unwrap();
    assert_eq!(restored_bundle[0], "_NO00#000>");
}

#[test]
fn test_sleep_consolidation_sws_replay_and_rem_pruning() {
    let mut engine = SleepConsolidationEngine::new(64, 0.2, 1);

    // Record awake experiences
    for i in 0..10 {
        let mut context = [0i16; 64];
        context[0] = 100;
        context[1] = -50;
        context[i % 64] = 80;

        let recorded = engine.record_experience(EpisodicTrace {
            step_id: i as u64,
            sensory_context: context,
            outcome_valence: 10,
            novelty_salience: 0.85,
        });
        assert!(recorded);
    }
    assert_eq!(engine.short_term_buffer.len(), 10);

    // Initialize ternary neocortical weights with low/noisy values
    let mut initial_trits = [0i8; 64];
    initial_trits[0] = 1;
    initial_trits[1] = -1;
    initial_trits[2] = 1; // Sub-threshold noise
    initial_trits[3] = 1; // Sub-threshold noise
    let mut weights = vec![TritWord128::pack(&initial_trits)];

    // Execute full biological sleep cycle
    let report = engine.execute_full_sleep_cycle(&mut weights);

    assert_eq!(report.traces_replayed, 10);
    assert_eq!(engine.short_term_buffer.len(), 0, "Short-term buffer must be consolidated and cleared");
    assert!(report.synapses_pruned_to_zero > 0, "REM pruning must eliminate sub-threshold noisy synapses");
    assert!(report.final_sparsity_percentage >= 50.0);
}
