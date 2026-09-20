use cronc::cl_bench::{analyze_cl_roofline, RooflineRegime, ROOFLINE_KNEE_OI};

#[test]
fn test_cl_bench_flash_attention_kernel() {
    let kernel = r#"
    @attn_init:
    B0000: '==01#004> '==02#008> '==03#00C> _NO00#000>
    B0001: '==04#010> '==05#020> '==06#001> _NO00#000>
    B0002: _OP01$28F> _MD02*3A2> _PO04+600> _SB00#000>
    B0003: _FA03$142> _TT05$200> _PO06&100> _bb00#000>
    B0004: _MD07.280> _PO08*300> _PO09+600> _NO00#000>
    B0005: _PO01/400> _PO02/400> _PO03/400> _bb00#000>
    B0006: _OP0A$48F> _MD0B*2A2> _PO0C+100> _SB00#000>
    B0007: _FU00#000> _PO00+600> _FE00#000> _HL00$008!
    "#;

    let rep = analyze_cl_roofline(kernel).expect("Benchmarking FlashAttention kernel must succeed");

    assert_eq!(rep.total_cycles, 8);
    assert_eq!(rep.optical_gemm_ops, 2, "Must identify 2 Optical GEMMs (B0002, B0006)");
    assert_eq!(rep.subbyte_mac_ops, 3, "Must identify 3 Sub-byte MACs (B0002, B0004, B0006)");
    assert!(rep.simd_alu_ops >= 6, "Must identify SIMD Softmax/ALU operations");
    assert!(rep.total_flops >= 100.0, "Effective compute must reflect Optical + SIMD operations");
    assert!(rep.operational_intensity > 0.0);
    assert!(rep.attainable_core_gflops > 0.0);
    assert!(rep.attainable_chip_tflops > 0.0);
    assert!(rep.silicon_efficiency_pct > 0.0 && rep.silicon_efficiency_pct <= 100.0);
    assert!(rep.theoretical_ipc >= 2.0, "IPC must reflect high VLIW bundle packing");
}

#[test]
fn test_cl_bench_memory_bound_strided_kernel() {
    // Kernel with high memory traffic (stores/transfers) and zero optical compute
    let kernel = r#"
    B0000: _ST01#004> _ST02#008> _ST03#00C> _ST04#010>
    B0001: _ST05#014> _ST06#018> _ST07#01C> _ST08#020>
    B0002: _TT01$100> _TT02$200> _NO00#000> _HL00$008!
    "#;

    let rep = analyze_cl_roofline(kernel).expect("Benchmark must succeed");

    assert_eq!(rep.total_cycles, 3);
    assert_eq!(rep.optical_gemm_ops, 0);
    assert!(rep.operational_intensity < ROOFLINE_KNEE_OI, "Strided memory operations must be below knee point");
    assert_eq!(rep.regime, RooflineRegime::MemoryBound);
    assert!(rep.ai_optimization_guidance.contains("MEMORY-BOUND"));
}

#[test]
fn test_cl_bench_ascii_roofline_visualization() {
    let kernel = r#"
    B0000: _OP01$28F> _MD02*3A2> _PO04+600> _SB00#000>
    B0001: _OP0A$48F> _MD0B*2A2> _PO0C+100> _HL00$008!
    "#;

    let rep = analyze_cl_roofline(kernel).unwrap();
    let ascii = rep.format_ascii_report("test_kernel.cl");

    assert!(ascii.contains("CRON 256-CORE 4D-TORUS HARDWARE ROOFLINE & PERFORMANCE BENCHMARK"));
    assert!(ascii.contains("Target Source:            test_kernel.cl"));
    assert!(ascii.contains("80 GFLOPs"));
    assert!(ascii.contains("Peak Silicon Ceiling"));
    assert!(ascii.contains("Operational Intensity:"));
    assert!(ascii.contains("Operating Point"));
}

#[test]
fn test_cl_bench_json_telemetry_schema() {
    let kernel = r#"
    B0000: _OP01$28F> _NO00#000> _NO00#000> _HL00$008!
    "#;

    let rep = analyze_cl_roofline(kernel).unwrap();
    let json = rep.to_json();

    assert!(json.contains("\"total_cycles\": 1"));
    assert!(json.contains("\"optical_gemm_ops\": 1"));
    assert!(json.contains("\"operational_intensity_flop_per_byte\":"));
    assert!(json.contains("\"roofline_knee_oi\": 0.50"));
    assert!(json.contains("\"regime\": \"COMPUTE_BOUND\""));
    assert!(json.contains("\"attainable_core_gflops\":"));
    assert!(json.contains("\"attainable_chip_tflops\":"));
    assert!(json.contains("\"ai_optimization_guidance\":"));
}
