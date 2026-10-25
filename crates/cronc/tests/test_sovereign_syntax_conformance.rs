// ============================================================================
// CRON CL v2.0 SOVEREIGN SYNTAX CONFORMANCE & VALIDATION TEST SUITE
// Tests the Zero-Verbal Constitution and Pure Symbolic EBNF Verification
// ============================================================================

use cronc::cl_syntax_spec::{verify_sovereign_pure_cipher_syntax, SOVEREIGN_SYNTAX_EBNF};
use cronc::cl_v2_parser::parse_cl_v2;
use cronc::cl_v2_lowering::V2LoweringEngine;
use cronc::cl_jit::{ClJitCore, execute_cl_on_core};

#[test]
fn test_sovereign_ebnf_grammar_is_defined() {
    assert!(!SOVEREIGN_SYNTAX_EBNF.is_empty());
    assert!(SOVEREIGN_SYNTAX_EBNF.contains("@⌖"));
    assert!(SOVEREIGN_SYNTAX_EBNF.contains("@io"));
    assert!(SOVEREIGN_SYNTAX_EBNF.contains("↦"));
    assert!(SOVEREIGN_SYNTAX_EBNF.contains("!Δ"));
}

#[test]
fn test_verbal_english_keywords_are_rejected_by_conformance_verifier() {
    let verbal_code = r#"
@kernel bad_kernel
.target silicon.generic
.cores 256

@io {
    query: QueryVector[16×16:FP32] @0x0000
    out:   OutputVector[16×16:FP32] @0x0200
}

§NamedBlock[16×16:FP32] {
    QueryVector ↦ OutputVector :
        Delta := QueryVector * 2.0
        OutputVector := Delta + 1.0
}
    "#;

    let res = verify_sovereign_pure_cipher_syntax(verbal_code);
    assert!(res.is_err(), "Verbal code with English keywords must be rejected");
    let report = res.err().unwrap();
    assert!(!report.is_pure_cipher);
    assert!(!report.violations.is_empty());

    let violations_tokens: Vec<String> = report.violations.iter().map(|v| v.token.clone()).collect();
    assert!(violations_tokens.iter().any(|t| t == "kernel"), "Must catch 'kernel'");
    assert!(violations_tokens.iter().any(|t| t == "target"), "Must catch 'target'");
    assert!(violations_tokens.iter().any(|t| t == "query"), "Must catch 'query'");
}

#[test]
fn test_pure_cipher_hnsw_kernel_passes_conformance() {
    let libcl_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../libcl");
    let src = std::fs::read_to_string(libcl_dir.join("hnsw_graph_nav.cl"))
        .expect("Read hnsw_graph_nav.cl");

    let report = verify_sovereign_pure_cipher_syntax(&src)
        .expect("Pure cipher HNSW kernel must have 0 violations");

    assert!(report.is_pure_cipher);
    assert!(report.has_topology);
    assert!(report.has_io_binding);
    assert!(report.has_math_morphism);
    assert!(report.has_control_flow);
    assert!(report.has_invariant);
    assert!(report.violations.is_empty());

    // Lower & JIT execute
    let mut doc = parse_cl_v2(&src).expect("Parse must succeed");
    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Lowering must succeed");
    doc.vliw_stream = bundles;
    let canonical = engine.emit_canonical(&doc);
    let mut core = ClJitCore::new();
    execute_cl_on_core(&canonical, &mut core).expect("Execution must succeed");
    assert!(core.is_halted);
}

#[test]
fn test_pure_cipher_muon_kernel_passes_conformance() {
    let libcl_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../libcl");
    let src = std::fs::read_to_string(libcl_dir.join("muon_polar_opt.cl"))
        .expect("Read muon_polar_opt.cl");

    let report = verify_sovereign_pure_cipher_syntax(&src)
        .expect("Pure cipher Muon kernel must have 0 violations");

    assert!(report.is_pure_cipher);
    assert!(report.has_topology);
    assert!(report.has_io_binding);
    assert!(report.has_math_morphism);
    assert!(report.has_control_flow);
    assert!(report.has_invariant);
    assert!(report.violations.is_empty());

    // Lower & JIT execute
    let mut doc = parse_cl_v2(&src).expect("Parse must succeed");
    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Lowering must succeed");
    doc.vliw_stream = bundles;
    let canonical = engine.emit_canonical(&doc);
    let mut core = ClJitCore::new();
    execute_cl_on_core(&canonical, &mut core).expect("Execution must succeed");
    assert!(core.is_halted);
}
