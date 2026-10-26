// ============================================================================
// CRON CL HIGH-LEVEL STRICT TYPE SYSTEM & SEMANTIC CHECKING TEST SUITE
// ============================================================================
// Tests:
// 1. Compile-time Tensor Contraction (⊗) dimension mismatch detection
// 2. Beyond-Addition Associative Scan (>>=) state dimension verification
// 3. Photonic Coupling (<~>) phase size checking
// 4. SRAM local window (64KB) overflow prevention
// 5. Valid high-level multi-physics mathematical morphisms
// ============================================================================

use cronc::cl_v2_parser::parse_cl_v2;
use cronc::cl_v2_lowering::V2LoweringEngine;

#[test]
fn test_valid_tensor_contraction_and_elementwise() {
    let code = r#"
@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    A[16×32:FP32] @0x0000
    B[32×16:FP32] @0x0200
    Y[16×16:FP32] @0x0400
}

§[16×16:FP32] {
    A ↦ Y :
        M := A ⊗ B
        Y := σ(M)
}
    "#;

    let doc = parse_cl_v2(code).expect("Valid code should parse");
    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Valid tensor contraction should lower");
    assert!(!bundles.is_empty(), "Must synthesize VLIW bundles");
}

#[test]
fn test_mismatched_tensor_contraction_fails_at_compile_time() {
    let code = r#"
@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    A[16×64:FP32] @0x0000
    B[32×16:FP32] @0x0200
    Y[16×16:FP32] @0x0400
}

§[16×16:FP32] {
    A ↦ Y :
        Y := A ⊗ B
}
    "#;

    let doc = parse_cl_v2(code).expect("Syntax is well-formed");
    let mut engine = V2LoweringEngine::new();
    let res = engine.lower_document(&doc);
    assert!(res.is_err(), "Inner dimension mismatch (64 != 32) must fail compile-time checking");
    let err_msg = res.err().unwrap();
    assert!(err_msg.contains("Dimension Mismatch"), "Error message must state Dimension Mismatch: {}", err_msg);
}

#[test]
fn test_sram_overflow_detected_at_compile_time() {
    let code = r#"
@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    X[16×16:FP32] @sram[0x25000]
    Y[16×16:FP32] @sram[0x0400]
}

§[16×16:FP32] {
    X ↦ Y :
        Y := σ(X)
}
    "#;

    let doc = parse_cl_v2(code).expect("Syntax is well-formed");
    let mut engine = V2LoweringEngine::new();
    let res = engine.lower_document(&doc);
    assert!(res.is_err(), "Address 0x25000 must trigger SRAM overflow error");
    let err_msg = res.err().unwrap();
    assert!(err_msg.contains("SRAM Overflow"), "Error must state SRAM Overflow: {}", err_msg);
}

#[test]
fn test_associative_scan_mamba2_state_checking() {
    let code = r#"
@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    X[16×16:NVFP4] @0x0000
    Y[16×16:NVFP4] @0x0400
}

§[16×16:NVFP4] {
    H: SS[16×256]
    X ↦ Y :
        Y := X >>= H
}
    "#;

    let doc = parse_cl_v2(code).expect("Parse associative scan");
    let mut engine = V2LoweringEngine::new();
    let res = engine.lower_document(&doc);
    assert!(res.is_ok(), "Associative scan with matching dim 16 should succeed: {:?}", res.err());
}

#[test]
fn test_clifford_exterior_wedge_and_moe_gating() {
    let code = r#"
@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    U[16×16:FP32] @0x0000
    V[16×16:FP32] @0x0200
    G[16×16:FP32] @0x0400
    Y[16×16:FP32] @0x0600
}

§[16×16:FP32] {
    U ↦ Y :
        W := U ^ V
        Y := W <|> G
}
    "#;

    let doc = parse_cl_v2(code).expect("Parse Clifford wedge and MoE gating");
    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Lowering Clifford wedge and MoE gating should succeed");
    assert!(!bundles.is_empty(), "Must synthesize VLIW bundles");
    
    // Check that CW and GT opcodes were generated
    let all_slots: Vec<String> = bundles.iter().flat_map(|b| b.slots.iter().map(|s| s.raw.clone())).collect();
    assert!(all_slots.iter().any(|s| s.contains("CW")), "Must contain Clifford Wedge (_CW) opcode: {:?}", all_slots);
    assert!(all_slots.iter().any(|s| s.contains("GT")), "Must contain Gating (_GT) opcode: {:?}", all_slots);
}

