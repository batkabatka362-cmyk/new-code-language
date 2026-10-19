// ============================================================================
// CRON CL v2.0 TEST SUITE: Lowering & Formal Co-Verification
// Tests automatic VLIW synthesis from Math Morphism and formal safety audits
// ============================================================================

use cronc::cl_v2_parser::parse_cl_v2;
use cronc::cl_v2_lowering::V2LoweringEngine;
use cronc::cl_jit::{ClJitCore, execute_cl_on_core};

#[test]
fn test_lower_pure_math_to_vliw() {
    let source_without_vliw = r#"
@kernel mamba2_synthesis
.target silicon.4d_torus
.ipc_target 4.0
.cores 256

@io {
    in:  X[16×16:NVFP4]  @sram[0x0000]
    out: Y[16×16:NVFP4]  @sram[0x0400]
}

§Mamba2Block[16×16:NVFP4] {
    H: SS[64×256]
    X ↦ silu((rmsnorm(X) <~> ssd(X, H)) ~*~ ssd(X, H))
}
    "#;

    let mut doc = parse_cl_v2(source_without_vliw).expect("Should parse");
    assert!(doc.vliw_stream.is_empty(), "Document initially has no VLIW stream");

    let mut engine = V2LoweringEngine::new();
    let synthesized_bundles = engine.lower_document(&doc).expect("Lowering must succeed");
    assert!(!synthesized_bundles.is_empty(), "Must synthesize VLIW bundles");
    assert_eq!(synthesized_bundles.len(), 3);

    // Attach synthesized bundles to document
    doc.vliw_stream = synthesized_bundles;

    // Verify canonical code generation
    let canonical = engine.emit_canonical(&doc);
    assert!(canonical.contains("B0000:"));
    assert!(canonical.contains("B0001:"));
    assert!(canonical.contains("B0002:"));
    assert!(canonical.contains("!HL00#000"));

    // Verify JIT execution of synthesized code
    let mut core = ClJitCore::new();
    execute_cl_on_core(&canonical, &mut core).expect("Synthesized microcode must execute cleanly");
    assert!(core.is_halted);
    assert_eq!(core.cycle_count, 3);
}

#[test]
fn test_formal_co_verification_passes() {
    let source = r#"
@kernel verified_kernel
.target silicon.ai_isa
.ipc_target 4.0
.cores 256

§Mamba2Block[16×16:NVFP4] {
    H: SS[64×256]
    X ↦ silu((rmsnorm(X) <~> ssd(X, H)) ~*~ ssd(X, H))
}

B0000: '==01#0A0K '==02#080V _MD03*120F _FM04+320A
B0001: ~RM05M100# _SS06M200' _SI07M6007 _PO08$560n
B0002: _ST09#040. _NO00#000L _NO00#000L !HL00#000(
    "#;

    let doc = parse_cl_v2(source).expect("Should parse");
    let engine = V2LoweringEngine::new();
    let report = engine.co_verify(&doc);

    assert!(report.passed, "Formal co-verification must pass: {:?}", report.diagnostics);
    assert!(report.operations_matched >= 3);
    assert_eq!(report.hazards_detected, 0);
}

#[test]
fn test_formal_co_verification_detects_missing_operator() {
    // Microcode omits silu (_SI), violating the mathematical contract
    let invalid_source = r#"
@kernel invalid_kernel
.target silicon.ai_isa
.ipc_target 4.0
.cores 256

§Mamba2Block[16×16:NVFP4] {
    H: SS[64×256]
    X ↦ silu((rmsnorm(X) <~> ssd(X, H)) ~*~ ssd(X, H))
}

B0000: '==01#0A0K '==02#080V _MD03*120F _FM04+320A
B0001: ~RM05M100# _SS06M200' _PO07M304P _PO08$560n
B0002: _ST09#040. _NO00#000L _NO00#000L !HL00#000(
    "#;

    let doc = parse_cl_v2(invalid_source).expect("Should parse");
    let engine = V2LoweringEngine::new();
    let report = engine.co_verify(&doc);

    assert!(!report.passed, "Must fail verification because 'silu' is missing in VLIW");
    assert!(report.diagnostics.iter().any(|d| d.contains("silu")));
}
