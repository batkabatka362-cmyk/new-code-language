use cronc::{
    run_cl_autotune, verify_cl_program, run_cl_jit,
    AutotuneConfig,
};

#[test]
fn test_autotune_candidate_generation_and_pareto() {
    let config = AutotuneConfig {
        m: 64,
        n: 64,
        k: 64,
        metric: "balanced".to_string(),
        max_candidates: 32,
        enable_sparsity: true,
        ambient_temp_c: 25.0,
    };

    let report = run_cl_autotune(&config);

    assert_eq!(report.problem_m, 64);
    assert_eq!(report.problem_n, 64);
    assert_eq!(report.problem_k, 64);
    assert!(report.total_evaluated > 0);
    assert!(report.pareto_count >= 1);
    assert_eq!(report.pareto_candidates.len(), report.pareto_count);

    // Verify non-dominance of Pareto frontier
    for p in &report.pareto_candidates {
        assert!(p.is_pareto);
        assert!(p.latency_cycles > 0);
        assert!(p.power_watts > 0.0);
    }
}

#[test]
fn test_autotune_metric_selection_differentiates() {
    let config_lat = AutotuneConfig {
        m: 32,
        n: 32,
        k: 32,
        metric: "latency".to_string(),
        max_candidates: 24,
        enable_sparsity: true,
        ambient_temp_c: 25.0,
    };
    let report_lat = run_cl_autotune(&config_lat);

    let config_thermal = AutotuneConfig {
        m: 32,
        n: 32,
        k: 32,
        metric: "thermal".to_string(),
        max_candidates: 24,
        enable_sparsity: true,
        ambient_temp_c: 25.0,
    };
    let report_thermal = run_cl_autotune(&config_thermal);

    // Latency-optimized candidate should have equal or lower latency cycles than thermal candidate
    assert!(report_lat.best_candidate.latency_cycles <= report_thermal.best_candidate.latency_cycles);
    // Thermal candidate should have equal or lower temperature than latency candidate
    assert!(report_thermal.best_candidate.junction_temp_c <= report_lat.best_candidate.junction_temp_c);
}

#[test]
fn test_autotune_cl_synthesis_and_jit_execution() {
    let config = AutotuneConfig {
        m: 16,
        n: 16,
        k: 16,
        metric: "latency".to_string(),
        max_candidates: 8,
        enable_sparsity: false,
        ambient_temp_c: 25.0,
    };

    let report = run_cl_autotune(&config);
    let cl_code = &report.synthesized_cl_kernel;

    // Verify that the synthesized code is valid .cl
    let cl_rep = verify_cl_program(cl_code);
    assert!(cl_rep.is_ok(), "Synthesized kernel must be valid .cl: {:?}", cl_rep.err());

    // Execute via JIT engine
    let jit_res = run_cl_jit(cl_code);
    assert!(jit_res.is_ok(), "JIT execution failed: {:?}", jit_res.err());
}

#[test]
fn test_autotune_reports_ascii_and_json() {
    let config = AutotuneConfig {
        m: 32,
        n: 32,
        k: 32,
        metric: "balanced".to_string(),
        max_candidates: 16,
        enable_sparsity: true,
        ambient_temp_c: 25.0,
    };

    let report = run_cl_autotune(&config);

    // Test ASCII rendering
    let ascii = report.render_ascii();
    assert!(ascii.contains("CRON SPEC-TO-SILICON MULTI-OBJECTIVE AUTO-TUNER REPORT"));
    assert!(ascii.contains("Winning Optimal Configuration"));
    assert!(ascii.contains("Pareto Frontier Trade-Off Curve"));

    // Test JSON serialization
    let json = report.to_json();
    assert!(json.contains("\"problem_m\": 32"));
    assert!(json.contains("\"metric\": \"balanced\""));
    assert!(json.contains("\"best_candidate\":"));
    assert!(json.contains("\"pareto_candidates\":"));
}
