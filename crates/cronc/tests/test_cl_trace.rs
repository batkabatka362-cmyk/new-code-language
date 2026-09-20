// ============================================================================
// Unit Tests: CRON Dynamic Trace Optimizer & L0 Trace Cache (test_cl_trace.rs)
// ============================================================================

use cronc::cl_trace::{
    is_slot_nop, optimize_cl_trace, parse_cl_bundles, render_ascii_trace_schedule,
    TraceCacheConfig,
};

const SAMPLE_CL: &str = r#"
@attn_init:
B0000: '==01#004> '==02#008> '==03#00C> _NO00#000>
B0001: '==04#010> '==05#020> '==06#001> _NO00#000>

@q_k_projection:
B0002: _OP01$28F> _MD02*3A2> _PO04+600> _SB00#000>
B0003: _FA03$142> _TT05$200> _PO06&100> _bb00#000>

@attention_weights_softmax:
B0004: _MD07.280> _PO08*300> _PO09+600> _NO00#000>
B0005: _PO01/400> _PO02/400> _PO03/400> _bb00#000>

@context_aggregation:
B0006: _OP0A$48F> _MD0B*2A2> _PO0C+100> _SB00#000>
B0007: _FU00#000> _PO00+600> _FE00#000> _HL00!000>
"#;

#[test]
fn test_parse_cl_bundles() {
    let bundles = parse_cl_bundles(SAMPLE_CL).expect("Should parse bundles");
    assert_eq!(bundles.len(), 8);
    assert_eq!(bundles[0].cycle, 0);
    assert_eq!(bundles[7].cycle, 7);
    assert_eq!(bundles[0].slots.len(), 4);
}

#[test]
fn test_is_slot_nop() {
    assert!(is_slot_nop("_NO00#000>"));
    assert!(is_slot_nop("'==00#000>"));
    assert!(is_slot_nop(" _NO00#000> "));
    assert!(!is_slot_nop("_OP01$28F>"));
    assert!(!is_slot_nop("_HL00!000>"));
}

#[test]
fn test_modulo_schedule_computation() {
    let config = TraceCacheConfig::default();
    let result = optimize_cl_trace(SAMPLE_CL, "mini_transformer_attention.cl", &config)
        .expect("Trace optimization should succeed");

    assert_eq!(result.original_bundle_count, 8);
    assert!(result.schedule.res_mii >= 1);
    assert!(result.schedule.mii <= 8);
    assert!(result.schedule.scheduled_kernel_cycles <= result.original_bundle_count);
    assert!(result.schedule.optimized_ipc >= result.schedule.original_ipc);
    assert!(!result.schedule.kernel_bundles.is_empty());
    assert!(result.synthesized_cl.contains("CRON Modulo-Scheduled & Trace-Cached Microcode"));
}

#[test]
fn test_trace_cache_hit_rate_and_energy() {
    let config = TraceCacheConfig {
        cache_size_kb: 16,
        ways: 4,
        line_bundles: 4,
        trip_count: 64,
        unroll_factor: 2,
    };

    let result = optimize_cl_trace(SAMPLE_CL, "test_kernel.cl", &config)
        .expect("Optimization should succeed");

    assert_eq!(result.cache_report.capacity_kb, 16);
    assert_eq!(result.cache_report.ways, 4);
    assert!(result.cache_report.hit_rate_percent > 95.0);
    assert!(result.cache_report.energy_savings_percent > 80.0);
    assert!(result.cache_report.cache_hits > result.cache_report.cache_misses);
}

#[test]
fn test_ascii_trace_schedule_rendering() {
    let config = TraceCacheConfig::default();
    let result = optimize_cl_trace(SAMPLE_CL, "attention.cl", &config).unwrap();
    let ascii = render_ascii_trace_schedule(&result);

    assert!(ascii.contains("CRON HARDWARE DYNAMIC TRACE OPTIMIZER & L0 TRACE CACHE SCOREBOARD"));
    assert!(ascii.contains("Minimum Initiation Int:"));
    assert!(ascii.contains("Modulo Software Pipelining Progression Matrix"));
    assert!(ascii.contains("On-Chip L0 Microcode Trace Cache Telemetry"));
    assert!(ascii.contains("Instruction Power Saved:"));
}

#[test]
fn test_trace_json_serialization() {
    let config = TraceCacheConfig::default();
    let result = optimize_cl_trace(SAMPLE_CL, "attention.cl", &config).unwrap();
    let json = result.to_json();

    assert!(json.contains("\"source_name\": \"attention.cl\""));
    assert!(json.contains("\"modulo_schedule\""));
    assert!(json.contains("\"trace_cache\""));
    assert!(json.contains("\"hit_rate_percent\""));
    assert!(json.contains("\"energy_savings_percent\""));
}
