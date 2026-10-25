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

#[test]
fn test_lower_arbitrary_freeform_algorithm_without_templates() {
    // Pure free-form code: No @kernel, No @io, No B0000: template!
    // A Fibonacci sequence and dynamic scalar arithmetic written purely in high-level symbolic code:
    let freeform_source = r#"
§Fibonacci(n) {
    a := 0
    b := 1
    i := 0
    ⟳ (10) {
        c := a + b
        a := b
        b := c
    }
    ⮌ b
}
    "#;

    let doc = parse_cl_v2(freeform_source).expect("Universal parser must parse freeform algorithm");
    assert_eq!(doc.functions.len(), 1);
    assert_eq!(doc.functions[0].name, "Fibonacci");
    assert_eq!(doc.functions[0].params, vec!["n".to_string()]);
    assert!(!doc.functions[0].body.is_empty());

    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Lowering freeform code must synthesize valid microcode");
    assert!(!bundles.is_empty());

    let mut doc_with_vliw = doc.clone();
    doc_with_vliw.vliw_stream = bundles;
    let canonical = engine.emit_canonical(&doc_with_vliw);
    assert!(canonical.contains("B0000:"));
    assert!(canonical.contains("!HL00#000"));

    // Run on real JIT silicon core
    let mut core = ClJitCore::new();
    execute_cl_on_core(&canonical, &mut core).expect("Lowered freeform code must execute cleanly on JIT silicon");
    assert!(core.is_halted);
}

#[test]
fn test_lower_top_level_freeform_statements() {
    let toplevel_source = r#"
x := 10
y := 20
z := (x * y) + 5
#R1 ← z
⟳ (5) {
    z := z - 1
}
    "#;

    let doc = parse_cl_v2(toplevel_source).expect("Universal parser must parse top-level statements");
    assert!(!doc.top_level_statements.is_empty());

    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Lowering top-level statements must succeed");
    assert!(!bundles.is_empty());

    let mut doc_with_vliw = doc.clone();
    doc_with_vliw.vliw_stream = bundles;
    let canonical = engine.emit_canonical(&doc_with_vliw);

    let mut core = ClJitCore::new();
    execute_cl_on_core(&canonical, &mut core).expect("Must execute on JIT core");
    assert!(core.is_halted);
}
#[test]
fn test_all_sovereign_v2_libcl_kernels_lower_and_verify() {
    let libcl_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../libcl");
    let mut checked = 0;

    for entry in std::fs::read_dir(&libcl_dir).expect("libcl dir must exist") {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("cl") {
            continue;
        }
        let src = std::fs::read_to_string(&path).unwrap();
        // Only sovereign v2 documents carry a math morphism block
        if !src.contains('§') {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        let mut doc = parse_cl_v2(&src).unwrap_or_else(|e| panic!("{} must parse: {:?}", name, e));
        let mut engine = V2LoweringEngine::new();
        let bundles = engine.lower_document(&doc).unwrap_or_else(|e| panic!("{} must lower: {:?}", name, e));
        assert!(!bundles.is_empty(), "{} must synthesize bundles", name);
        doc.vliw_stream = bundles;
        let report = engine.co_verify(&doc);
        assert!(report.passed, "{} co-verification failed: {:?}", name, report.diagnostics);

        let canonical = engine.emit_canonical(&doc);
        let mut core = ClJitCore::new();
        execute_cl_on_core(&canonical, &mut core).unwrap_or_else(|e| panic!("{} must execute: {:?}", name, e));
        assert!(core.is_halted, "{} must halt", name);
        checked += 1;
    }

    println!("Successfully verified and co-simulated {} sovereign v2 kernels in libcl!", checked);
    assert!(checked >= 24, "Expected at least 24 sovereign v2 kernels, found {}", checked);
}

#[test]
fn test_lower_nested_hardware_loops_execution() {
    let src = r#"
@kernel test_nested_loops
.target silicon.neural_core
.ipc_target 4.0
.cores 256

@io {
    in: A[16×16:FP32] @sram[0x0000]
    out: B[16×16:FP32] @sram[0x0200]
}

§ControlFlow {
    #R1 ← @sram[0x0000]
    ⟳ (4) {
        ⟳ (8) {
            #R1 := #R1 + #R1
        }
    }
    @sram[0x0200] ← #R1
}

!Δ { |B| < 1e6 }
"#;

    let mut doc = parse_cl_v2(src).expect("Nested loops must parse");
    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Nested loops must lower");
    assert!(!bundles.is_empty());
    doc.vliw_stream = bundles;

    let canonical = engine.emit_canonical(&doc);
    let mut core = ClJitCore::new();
    execute_cl_on_core(&canonical, &mut core).expect("Nested loop canonical code must execute");
    assert!(core.is_halted, "Core must halt after nested loops complete");
}

#[test]
fn test_pure_non_verbal_mathematical_morphism_compilation() {
    let src = r#"
@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    Q[16×16:FP32] @0x0000
    K[16×16:FP32] @0x0200
    Y[16×16:FP32] @0x0400
}

§[16×16:FP32] {
    α: T[16×16:FP32]

    Q ↦ Y :
        Z := Q ⊗ K
        S := Z * α
        Y := σ(∥S∥)
}

⟳ (16) {
    #R1 ← @0x0000
    #R2 ← @0x0200
    #R3 := #R1 * #R2
    @0x0400 ← #R3
}

!Δ { ∥Y∥ < 1e5 }
"#;

    let mut doc = parse_cl_v2(src).expect("Pure non-verbal code must parse");
    let mut engine = V2LoweringEngine::new();
    let bundles = engine.lower_document(&doc).expect("Pure non-verbal code must lower to VLIW");
    assert!(!bundles.is_empty());
    doc.vliw_stream = bundles;

    let report = engine.co_verify(&doc);
    assert!(report.passed, "Co-verification must pass: {:?}", report.diagnostics);

    let canonical = engine.emit_canonical(&doc);
    let mut core = ClJitCore::new();
    execute_cl_on_core(&canonical, &mut core).expect("Pure non-verbal code must execute on JIT core");
    assert!(core.is_halted, "Core must halt after executing pure non-verbal code");
}

#[test]
fn test_compile_time_tensor_shape_mismatch_detection() {
    let src = r#"
@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    A[16×32:FP32] @0x0000
    B[16×64:FP32] @0x0200
}

§[16×64:FP32] {
    A ↦ C :
        C := A ⊗ B
}
"#;

    let doc = parse_cl_v2(src).expect("Document should parse");
    let mut engine = V2LoweringEngine::new();
    let res = engine.lower_document(&doc);
    assert!(res.is_err(), "Lowering must fail on contraction dimension mismatch");
    let err = res.err().unwrap();
    assert!(
        err.contains("Compile-time Dimension Mismatch in Tensor Contraction (⊗)"),
        "Error message must specify tensor contraction dimension mismatch, got: {}",
        err
    );
}

#[test]
fn test_hnsw_and_muon_kernels_synthesis_and_execution() {
    let libcl_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../libcl");
    let hnsw_src = std::fs::read_to_string(libcl_dir.join("hnsw_graph_nav.cl"))
        .expect("Must read libcl/hnsw_graph_nav.cl");
    let mut hnsw_doc = parse_cl_v2(&hnsw_src).expect("HNSW kernel should parse cleanly");
    let mut engine = V2LoweringEngine::new();
    let hnsw_bundles = engine.lower_document(&hnsw_doc).expect("HNSW lowering must succeed");
    hnsw_doc.vliw_stream = hnsw_bundles;
    let hnsw_report = engine.co_verify(&hnsw_doc);
    assert!(hnsw_report.passed, "HNSW co-verify passed: {:?}", hnsw_report.diagnostics);
    let hnsw_canon = engine.emit_canonical(&hnsw_doc);
    let mut core = ClJitCore::new();
    execute_cl_on_core(&hnsw_canon, &mut core).expect("HNSW microcode must execute cleanly");
    assert!(core.is_halted);

    let muon_src = std::fs::read_to_string(libcl_dir.join("muon_polar_opt.cl"))
        .expect("Must read libcl/muon_polar_opt.cl");
    let mut muon_doc = parse_cl_v2(&muon_src).expect("Muon kernel should parse cleanly");
    let muon_bundles = engine.lower_document(&muon_doc).expect("Muon lowering must succeed");
    muon_doc.vliw_stream = muon_bundles;
    let muon_report = engine.co_verify(&muon_doc);
    assert!(muon_report.passed, "Muon co-verify passed: {:?}", muon_report.diagnostics);
    let muon_canon = engine.emit_canonical(&muon_doc);
    let mut core2 = ClJitCore::new();
    execute_cl_on_core(&muon_canon, &mut core2).expect("Muon microcode must execute cleanly");
    assert!(core2.is_halted);
}


