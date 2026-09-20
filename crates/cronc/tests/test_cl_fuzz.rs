use cronc::cl_fuzz::{run_cl_fuzz, FuzzOptions, XorShift64};

#[test]
fn test_cl_fuzz_prng_determinism() {
    let mut rng1 = XorShift64::new(0x12345678);
    let mut rng2 = XorShift64::new(0x12345678);

    for _ in 0..100 {
        assert_eq!(rng1.next_u64(), rng2.next_u64());
        assert_eq!(rng1.next_usize(50), rng2.next_usize(50));
        assert_eq!(rng1.next_bool(), rng2.next_bool());
    }
}

#[test]
fn test_cl_fuzz_zero_crash_invariant_1000_iterations() {
    let options = FuzzOptions {
        iterations: 1000,
        seed: 0x987654321,
        seed_code: None,
        max_mutations_per_cycle: 3,
        test_jit_execution: true,
        test_optimizer: true,
        test_memcheck: true,
    };

    let report = run_cl_fuzz(&options);

    assert_eq!(report.total_iterations, 1000);
    assert!(report.mutations_applied >= 1000);
    assert_eq!(report.unhandled_panics, 0, "Toolchain must NEVER panic on fuzzed input");
    assert!(report.is_robust_and_crash_free());
    assert!(report.auto_healed_count > 0, "Fuzzer must successfully heal mutated candidates");
    assert_eq!(
        report.auto_healed_count + report.gracefully_rejected_count,
        1000,
        "Every iteration must either be healed or gracefully rejected without panicking"
    );
}

#[test]
fn test_cl_fuzz_seed_reproducibility() {
    let options = FuzzOptions {
        iterations: 200,
        seed: 0xCAFEBABE1234,
        seed_code: None,
        max_mutations_per_cycle: 2,
        test_jit_execution: true,
        test_optimizer: true,
        test_memcheck: true,
    };

    let report1 = run_cl_fuzz(&options);
    let report2 = run_cl_fuzz(&options);

    assert_eq!(report1.total_iterations, report2.total_iterations);
    assert_eq!(report1.mutations_applied, report2.mutations_applied);
    assert_eq!(report1.auto_healed_count, report2.auto_healed_count);
    assert_eq!(report1.gracefully_rejected_count, report2.gracefully_rejected_count);
    assert_eq!(report1.optimized_count, report2.optimized_count);
    assert_eq!(report1.unhandled_panics, 0);
    assert_eq!(report2.unhandled_panics, 0);
}

#[test]
fn test_cl_fuzz_json_and_ascii_reports() {
    let options = FuzzOptions {
        iterations: 50,
        seed: 0x11223344,
        seed_code: None,
        max_mutations_per_cycle: 2,
        test_jit_execution: true,
        test_optimizer: true,
        test_memcheck: true,
    };

    let report = run_cl_fuzz(&options);

    // 1. JSON report
    let json = report.to_json();
    assert!(json.contains("\"total_iterations\": 50"));
    assert!(json.contains("\"unhandled_panics\": 0"));
    assert!(json.contains("\"is_robust_and_crash_free\": true"));

    // 2. ASCII report
    let ascii = report.format_ascii_report();
    assert!(ascii.contains("CRON AUTONOMOUS AI VIBE-FUZZ & MUTATION RESILIENCE REPORT"));
    assert!(ascii.contains("100% CRASH-RESILIENT"));
    assert!(ascii.contains("Unhandled Crashes/Panics:"));
}
