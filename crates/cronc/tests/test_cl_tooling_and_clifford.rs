// ============================================================================
// CRON .cl Tooling & Clifford Algebra Verification Suite
// Tests:
//   1. 4D Clifford Algebra Cl(4,0) Golden Kernel (.cl) Synthesis & Verification
//   2. Semantic Directives (.clifford, .stage, .tensor, .fuse, .flow, .layout)
//   3. .cl Source Formatter & Canonicalizer (cl_fmt)
//   4. Source-Level Debugger Metadata (cl_debug)
//   5. Optical Pipeline Latency & 4D-Torus DOR Routing Delay (cl_cosim)
// ============================================================================

use cronc::{
    format_cl_program, generate_clifford_rotate4d, parse_directive,
    synthesize_kernel, verify_cl_program, ClDebugInfo, ClDirective, ClFmtOptions,
    NocRoutingDelay, OpticPipelineStage,
};

#[test]
fn test_clifford_rotate4d_kernel_synthesis_and_verification() {
    let kernel_code = generate_clifford_rotate4d(4);
    assert!(kernel_code.contains(".clifford"));
    assert!(kernel_code.contains(".stage"));
    assert!(kernel_code.contains("@rotor_init:"));
    assert!(kernel_code.contains("@vector_load:"));

    let report = verify_cl_program(&kernel_code).expect("Synthesized Clifford kernel must be valid");
    assert!(report.total_bundles > 0);
    assert_eq!(report.crc_verified, report.total_slots);
    assert_eq!(report.hazards.len(), 0);

    // Verify synthesize_kernel alias
    let alias_code = synthesize_kernel("clifford-rotate4d", 4, 16)
        .expect("synthesize_kernel must support clifford-rotate4d");
    assert!(alias_code.contains("clifford_so4_rot"));
}

#[test]
fn test_semantic_directives_parsing() {
    // 1. .clifford
    let d1 = parse_directive(r#".clifford rotor=Rotor4D, vector=Vector4D, algebra="Cl(4,0)""#).unwrap();
    if let ClDirective::Clifford { rotor, vector, algebra } = d1 {
        assert_eq!(rotor, "Rotor4D");
        assert_eq!(vector, "Vector4D");
        assert_eq!(algebra, "Cl(4,0)");
    } else {
        panic!("Expected ClDirective::Clifford");
    }

    // 2. .stage
    let d2 = parse_directive(r#".stage "test_stage", params="16", precision="f32", d_model=128, heads=8, kv_heads=4, intermediate=256, zero_overhead=true"#).unwrap();
    if let ClDirective::Stage { id, d_model, heads, kv_heads, intermediate, zero_overhead, .. } = d2 {
        assert_eq!(id, "test_stage");
        assert_eq!(d_model, 128);
        assert_eq!(heads, 8);
        assert_eq!(kv_heads, 4);
        assert_eq!(intermediate, 256);
        assert!(zero_overhead);
    } else {
        panic!("Expected ClDirective::Stage");
    }

    // 3. .tensor
    let d3 = parse_directive(r#".tensor %Q: [16, 64], %K: [16, 64]"#).unwrap();
    if let ClDirective::Tensor { name, dims } = d3 {
        assert_eq!(name, "%Q");
        assert_eq!(dims, vec![16, 64]);
    } else {
        panic!("Expected ClDirective::Tensor");
    }

    // 4. .flow
    let d4 = parse_directive(r#".flow (Core[0,0,0,0] -> Core[0,0,1,0]) {dor=XYZW}"#).unwrap();
    if let ClDirective::Flow { dor, .. } = d4 {
        assert_eq!(dor, "XYZW");
    } else {
        panic!("Expected ClDirective::Flow");
    }

    // 5. .layout
    let d5 = parse_directive(r#".layout {TP=2, EP=1, CP=1, PP=1, dim="4x4x4x4", chip="256_core_torus"}"#).unwrap();
    if let ClDirective::Layout { tp, dim, chip, .. } = d5 {
        assert_eq!(tp, 2);
        assert_eq!(dim, "4x4x4x4");
        assert_eq!(chip, "256_core_torus");
    } else {
        panic!("Expected ClDirective::Layout");
    }
}

#[test]
fn test_cl_formatter_canonicalization() {
    let unformatted = r#"
; Test unformatted kernel
@entry:
B0000: '==01#000>
B0001: _MD0E*220> _MD0D*110>
"#;
    let opts = ClFmtOptions {
        pad_nops: true,
        heal_crc: true,
        align_comments: true,
        comment_column: 56,
    };
    let formatted = format_cl_program(unformatted, &opts).expect("Formatter should succeed");
    assert!(formatted.contains("B0000:"));
    assert!(formatted.contains("B0001:"));
    // With pad_nops, bundle 0 should have 4 slots (1 active + 3 NOPs)
    let b0_line = formatted.lines().find(|l| l.starts_with("B0000:")).unwrap();
    let slots: Vec<&str> = b0_line.split_whitespace().collect();
    assert_eq!(slots.len(), 5); // "B0000:" + 4 slots

    // Idempotent check
    let re_formatted = format_cl_program(&formatted, &opts).expect("Re-formatting should succeed");
    assert_eq!(formatted, re_formatted);
}

#[test]
fn test_cl_debug_metadata_and_stepping() {
    let cl_sample = r#"
.stage "debug_test", params="4", precision="f32", d_model=4, heads=1, kv_heads=1, intermediate=4, zero_overhead=true
@rotor_stage:
B0000: '==01#000> '==02#004> _NO00#000> _NO00#000>
B0001: _MD03*120> _PO04+310> _NO00#000> _HL00$008!
"#;
    let mut debug = ClDebugInfo::extract_from_cl(cl_sample);
    assert_eq!(debug.bundle_meta.len(), 2);
    assert!(debug.bundle_meta.contains_key(&0));
    assert!(debug.bundle_meta.contains_key(&1));

    // Symbol binding
    debug.bind_symbol(1, "rotor.s", "f32");
    debug.bind_symbol(2, "rotor.e12", "f32");
    assert_eq!(debug.get_symbol_name(1), Some("rotor.s"));
    assert_eq!(debug.get_symbol_name(2), Some("rotor.e12"));

    // Register read/write
    debug.write_reg(1, 0x3F800000); // 1.0f
    assert_eq!(debug.read_reg(1), 0x3F800000);

    // Breakpoint
    debug.set_breakpoint(1);
    assert!(!debug.is_breakpoint(0));
    assert!(debug.is_breakpoint(1));
    debug.clear_breakpoint(1);
    assert!(!debug.is_breakpoint(1));

    // Render table
    let table = debug.render_register_table();
    assert!(table.contains("rotor.s"));
    assert!(table.contains("0x3F800000"));
}

#[test]
fn test_optical_pipeline_and_noc_routing_delay() {
    // 1. OpticPipelineStage
    let mut pipe = OpticPipelineStage::new(3);
    assert_eq!(pipe.active_in_flight, 0);
    pipe.issue_op();
    assert_eq!(pipe.active_in_flight, 1);
    assert_eq!(pipe.tick(), 1);
    assert_eq!(pipe.active_in_flight, 0);
    assert_eq!(pipe.completed_ops, 1);

    // 2. NocRoutingDelay
    let src = [0, 0, 0, 0];
    let dst = [2, 1, 0, 3]; // wrapped distance: x=2, y=1, z=0, w=1 -> total hops = 4
    let hops = NocRoutingDelay::dor_hop_count(src, dst, 4);
    assert_eq!(hops, 4);
    let latency = NocRoutingDelay::packet_latency_cycles(src, dst);
    assert_eq!(latency, 8); // 4 hops * 2 cycles

    // Loopback
    assert_eq!(NocRoutingDelay::packet_latency_cycles(src, src), 1);
}
