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
    assert!(synthesized_bundles.len() >= 2);

    // Attach synthesized bundles to document
    doc.vliw_stream = synthesized_bundles;

    // Verify canonical code generation
    let canonical = engine.emit_canonical(&doc);
    assert!(canonical.contains("B0000:"));
    assert!(canonical.contains("B0001:"));
    assert!(canonical.contains("!HL00#000"));

    // Verify JIT execution of synthesized code
    let mut core = ClJitCore::new();
    execute_cl_on_core(&canonical, &mut core).expect("Synthesized microcode must execute cleanly");
    assert!(core.is_halted);
    assert!(core.cycle_count >= 2);
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

#[test]
fn test_lower_control_flow_and_math_pipeline() {
    let source = r#"
@kernel adaptive_pipeline
.target silicon.4d_torus
.ipc_target 4.0
.cores 64

@io {
    in:  X[16×16:FP8]  @sram[0x0000]
    out: Y[16×16:FP8]  @sram[0x0400]
}

§AdaptiveLayer[16×16:FP8] {
    H: SS[64×256]
    X ↦ silu(rmsnorm(X) * ssd(X, H))
}

§ControlFlow {
    #R1 ← @sram[0x0000]
    ⟳ (4) {
        #R2 ← @sram[0x0040]
        #R3 := #R1 * #R2
        ? (#R3 > 0x0020) ⮞ {
            @sram[0x0400] ← #R3
        } : {
            @sram[0x0400] ← #R1
        }
    }
    !Δ { |Y| < 1000, latency < 20ns }
}
    "#;

    let mut doc = parse_cl_v2(source).expect("Source must parse without error");
    assert!(doc.control_flow.is_some(), "Control flow block must be parsed");
    assert!(doc.math_morphism.is_some(), "Math morphism block must be parsed");

    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Lowering must succeed");
    assert!(!bundles.is_empty(), "Must synthesize bundles");

    doc.vliw_stream = bundles;
    let canonical = engine.emit_canonical(&doc);
    assert!(canonical.contains("B0000:"));
    assert!(canonical.contains("!HL00#000"));

    // Execute through JIT
    let mut core = ClJitCore::new();
    execute_cl_on_core(&canonical, &mut core).expect("JIT execution must succeed");
    assert!(core.is_halted);
    assert!(core.cycle_count > 0);
}

#[test]
fn test_v2_tensor_runtime_real_execution() {
    use cronc::cl_v2_lowering::V2TensorRuntime;

    let source = r#"
@kernel mamba2_real_tensor
.target silicon.4d_torus
.ipc_target 4.0
.cores 64

@io {
    in:  X[16×16:FP32]  @sram[0x0000]
    out: Y[16×16:FP32]  @sram[0x0400]
}

§Mamba2Compute[16×16:FP32] {
    H: SS[64×256]
    X ↦ Y :
        N := rmsnorm(X)
        S := ssd(N, H)
        Y := silu(S)
}
    "#;

    let doc = parse_cl_v2(source).expect("Parse must succeed");
    let mut runtime = V2TensorRuntime::new();
    runtime.init_from_doc(&doc, 42);

    assert!(runtime.get_buffer("X").is_some(), "Input buffer X must be allocated");
    assert!(runtime.get_buffer("Y").is_some(), "Output buffer Y must be allocated");
    assert!(runtime.get_buffer("H").is_some(), "State buffer H must be allocated");

    // Execute real tensor calculations
    runtime.execute_morphism(&doc).expect("Tensor morphism execution must succeed");

    let y_tensor = runtime.get_buffer("Y").expect("Y buffer must exist");
    let y_vals = y_tensor.to_f32_vec();
    assert_eq!(y_vals.len(), 256, "16x16 tensor must have 256 elements");
    
    // Validate that values are computed non-zero floats
    let non_zero_count = y_vals.iter().filter(|&&v| v.abs() > 1e-6).count();
    assert!(non_zero_count > 200, "Calculated tensor values must be real non-zero numbers: found {}", non_zero_count);
}


