use cronc::{
    profile_cl_kernel, run_cl_perf_suite, render_ascii_ppa_scoreboard,
    ClPerfConfig,
};

#[test]
fn test_single_kernel_ppa_profiling() {
    let cl_code = r#"
    B0000: '==01#00A> '==02#014> '==03#000> '==04#000>
    B0001: _OP01$29F> _MD02*322> _PO04+650> _SB00#060>
    B0002: _bb00#000> _NO00#070> _NO00#070> _HL00$0E8!
    "#;

    let config = ClPerfConfig {
        frequency_ghz: 2.5,
        voltage_v: 0.85,
        active_cores: 256,
        ambient_temp_c: 25.0,
    };

    let metrics = profile_cl_kernel(cl_code, "test-kernel", &config);

    assert_eq!(metrics.kernel_name, "test-kernel");
    assert_eq!(metrics.total_cycles, 3);
    assert!(metrics.ipc > 0.0, "IPC must be positive");
    assert!(metrics.chip_tflops > 0.0, "Chip TFLOPs must be positive");
    assert!(metrics.tops_per_watt > 0.0, "TOPS/Watt must be positive");
    assert!(metrics.dynamic_energy_pj > 0.0, "Energy in pJ must be positive");
    assert!(metrics.energy_delay_product_edp > 0.0, "EDP must be positive");
    assert!(metrics.junction_temp_c >= 25.0, "Junction temp must be above ambient");
    assert!(metrics.thermal_margin_c > 0.0, "Thermal margin must be positive");
    assert!(!metrics.bottleneck_diagnosis.is_empty());
}

#[test]
fn test_golden_suite_profiling() {
    let config = ClPerfConfig::default();
    let suite = run_cl_perf_suite(&config);

    assert_eq!(suite.total_benchmarked, 8, "Suite must benchmark all 8 core AI kernels");
    assert!(suite.peak_tops_per_watt > 0.0);
    assert!(!suite.peak_tops_per_watt_kernel.is_empty());
    assert!(suite.highest_ipc >= 2.0, "Highest IPC across golden suite should be >= 2.0");
    assert!(!suite.highest_ipc_kernel.is_empty());
    assert!(!suite.pareto_optimal_kernel.is_empty());

    // Verify all 8 kernels are present
    let names: Vec<&str> = suite.kernels.iter().map(|k| k.kernel_name.as_str()).collect();
    assert!(names.contains(&"flash-attn"));
    assert!(names.contains(&"bitnet-gemm"));
    assert!(names.contains(&"rmsnorm"));
    assert!(names.contains(&"swiglu"));
    assert!(names.contains(&"rope"));
    assert!(names.contains(&"cordic-rot"));
    assert!(names.contains(&"snn-lif"));
    assert!(names.contains(&"sparse-gemm"));
}

#[test]
fn test_ascii_ppa_scoreboard() {
    let config = ClPerfConfig::default();
    let suite = run_cl_perf_suite(&config);
    let scoreboard = render_ascii_ppa_scoreboard(&suite);

    assert!(scoreboard.contains("CRON 256-CORE 4D-TORUS SILICON MICRO-KERNEL PPA PERFORMANCE SCOREBOARD"));
    assert!(scoreboard.contains("Kernel Name   │ Cycles │  IPC  │  TFLOPs │  FLOP/B │  TOPS/W │   pJ/Op"));
    assert!(scoreboard.contains("flash-attn"));
    assert!(scoreboard.contains("bitnet-gemm"));
    assert!(scoreboard.contains("Silicon Efficiency Summary"));
    assert!(scoreboard.contains("Peak Energy Efficiency:"));
    assert!(scoreboard.contains("Peak VLIW Issue Rate:"));
    assert!(scoreboard.contains("Pareto Optimal Co-Design:"));
}

#[test]
fn test_ppa_json_serialization() {
    let config = ClPerfConfig::default();
    let suite = run_cl_perf_suite(&config);
    let json = suite.to_json();

    assert!(json.contains("\"total_benchmarked\": 8"));
    assert!(json.contains("\"peak_tops_per_watt\":"));
    assert!(json.contains("\"highest_ipc\":"));
    assert!(json.contains("\"pareto_optimal_kernel\":"));
    assert!(json.contains("\"kernels\":"));
}
