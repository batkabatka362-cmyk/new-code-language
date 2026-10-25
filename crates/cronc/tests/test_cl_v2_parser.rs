// ============================================================================
// CRON CL v2.0 TEST SUITE: 5-Pillar Parser
// Tests Silicon Target, I/O Binding, Math Morphism, Control Flow, and VLIW Cipher
// ============================================================================

use cronc::cl_v2_parser::parse_cl_v2;
use cronc::cl_v2_ast::*;

#[test]
fn test_parse_complete_5_pillar_document() {
    let source = r#"
// ============================================================================
// [1] SILICON TARGET CONTRACT
// ============================================================================
@kernel cluster_allreduce_mamba2
.target silicon.distributed_swarm
.ipc_target 4.0
.cores 256
@mesh [4×4×4×4] : ⮑ (x+1, y, z, w)

// ============================================================================
// [2] I/O MEMORY BINDING
// ============================================================================
@io {
    in:  X[16×16:NVFP4]  @sram[0x0000]
    out: Y[16×16:NVFP4]  @sram[0x0400]
    wt:  W[16×16:NVFP4]  @dram[0x8000]
}

// ============================================================================
// [3] MATHEMATICAL TENSOR MORPHISM
// ============================================================================
§Mamba2Block[16×16:NVFP4] {
    H: SS[64×256], W: T[16×16:NVFP4]
    
    X ↦ silu( (rmsnorm(X) <~> ssd(X, H)) ~*~ ssd(X, H) )
}

// ============================================================================
// [4] CIPHERED CONTROL-FLOW
// ============================================================================
§ControlFlow {
    ⟳ (100) {
        #R1 ← @dmem[0x1000]
        ? (#R1 > 0x3C00) ⮞ {
            #R2 := #R1 * 0.95
            @dmem[0x1000] ← #R2
        } : {
            ⮌
        }
    }
}
!Δ { |Y| < 1e4, latency < 15ns }

// ============================================================================
// [5] 10/16-CHAR VLIW SILICON CIPHER
// ============================================================================
B0000: '==01#0A0K '==02#080V _MD03*120F _FM04+320A
B0001: _DV05/310P _RC06!500Z _TG07*AA0M _SB08-760U
B0002: _MZ09~450X _PO0A$980L ~RM0B^0A0C !HL00#000(
    "#;

    let doc = parse_cl_v2(source).expect("Should parse 5-pillar document cleanly");

    // 1. Verify Target Contract
    let target = doc.target_contract.expect("Must have target contract");
    assert_eq!(target.kernel_name, "cluster_allreduce_mamba2");
    assert_eq!(target.target, "silicon.distributed_swarm");
    assert_eq!(target.ipc_target, 4.0);
    assert_eq!(target.cores, 256);
    let mesh = target.mesh_routing.expect("Must have mesh routing");
    assert_eq!(mesh.dimensions, vec![4, 4, 4, 4]);
    assert_eq!(mesh.vector_formula, "(x+1, y, z, w)");

    // 2. Verify I/O Binding
    let io = doc.io_binding.expect("Must have I/O binding");
    assert_eq!(io.items.len(), 3);
    assert_eq!(io.items[0].name, "X");
    assert_eq!(io.items[0].kind, IoKind::Input);
    assert_eq!(io.items[0].tier, MemoryTier::Sram);
    assert_eq!(io.items[0].address, 0x0000);

    assert_eq!(io.items[1].name, "Y");
    assert_eq!(io.items[1].kind, IoKind::Output);
    assert_eq!(io.items[1].address, 0x0400);

    assert_eq!(io.items[2].name, "W");
    assert_eq!(io.items[2].kind, IoKind::Weight);
    assert_eq!(io.items[2].tier, MemoryTier::Dram);
    assert_eq!(io.items[2].address, 0x8000);

    // 3. Verify Math Morphism
    let math = doc.math_morphism.expect("Must have math morphism");
    assert_eq!(math.name, "Mamba2Block");
    assert_eq!(math.tile_dims, vec![16, 16]);
    assert_eq!(math.dtype, "NVFP4");
    assert_eq!(math.states.len(), 2);
    assert_eq!(math.states[0].name, "H");
    assert_eq!(math.states[1].name, "W");
    assert!(math.equation_expr.contains("silu"));
    assert!(math.equation_expr.contains("<~>"));
    assert!(math.equation_expr.contains("~*~"));

    // 4. Verify Control Flow
    let cf = doc.control_flow.expect("Must have control flow");
    assert!(!cf.statements.is_empty());
    let watchdog = cf.watchdog.expect("Must have watchdog SLA");
    assert_eq!(watchdog.max_magnitude, Some(10000.0));
    assert_eq!(watchdog.max_latency_ns, Some(15.0));

    // 5. Verify VLIW Stream
    assert_eq!(doc.vliw_stream.len(), 3);
    assert_eq!(doc.vliw_stream[0].cycle, 0);
    assert_eq!(doc.vliw_stream[0].slots.len(), 4);
    assert_eq!(doc.vliw_stream[0].slots[2].opcode, "MD");
    assert_eq!(doc.vliw_stream[0].slots[3].opcode, "FM");
    assert_eq!(doc.vliw_stream[1].slots[0].opcode, "DV");
    assert_eq!(doc.vliw_stream[1].slots[2].opcode, "TG");
}

#[test]
fn test_parse_sequential_math_steps() {
    let source = r#"
§DeepSeekEngram[32×32:FP16] {
    H: SS[128×512]
    
    X ↦ Y :
        N := rmsnorm(X)
        S := ssd(N, H)
        Y := silu((N <~> S) ~*~ S)
}
    "#;

    let doc = parse_cl_v2(source).expect("Should parse sequential math steps");
    let math = doc.math_morphism.expect("Must have math morphism");
    assert_eq!(math.name, "DeepSeekEngram");
    assert_eq!(math.input_symbol, "X");
    assert_eq!(math.output_symbol, Some("Y".to_string()));
    assert_eq!(math.steps.len(), 3);
    assert_eq!(math.steps[0].target_var, "N");
    assert_eq!(math.steps[1].target_var, "S");
    assert_eq!(math.steps[2].target_var, "Y");

    // Verify AST parsing
    assert_eq!(
        math.steps[0].expr_ast,
        Some(MathExpr::Norm(Box::new(MathExpr::Variable("X".to_string()))))
    );
    assert_eq!(
        math.steps[1].expr_ast,
        Some(MathExpr::Primitive {
            name: "ssd".to_string(),
            args: vec![
                MathExpr::Variable("N".to_string()),
                MathExpr::Variable("H".to_string()),
            ],
        })
    );
    assert_eq!(
        math.steps[2].expr_ast,
        Some(MathExpr::Activation {
            kind: ActivationKind::Silu,
            operand: Box::new(MathExpr::Binary {
                op: MathBinaryOp::PeriodicResonance,
                lhs: Box::new(MathExpr::Binary {
                    op: MathBinaryOp::InterferenceCoupling,
                    lhs: Box::new(MathExpr::Variable("N".to_string())),
                    rhs: Box::new(MathExpr::Variable("S".to_string())),
                }),
                rhs: Box::new(MathExpr::Variable("S".to_string())),
            }),
        })
    );
}

#[test]
fn test_recursive_math_expression_ast_parser() {
    use cronc::cl_v2_parser::parse_math_expr_str;

    // 1. Pure non-verbal tensor contraction & activation
    let e1 = parse_math_expr_str("σ(∥S∥)").expect("Should parse σ(∥S∥)");
    assert_eq!(
        e1,
        MathExpr::Activation {
            kind: ActivationKind::Silu,
            operand: Box::new(MathExpr::Norm(Box::new(MathExpr::Variable("S".to_string())))),
        }
    );

    // 2. Binary tensor multiplication
    let e2 = parse_math_expr_str("Q ⊗ K").expect("Should parse Q ⊗ K");
    assert_eq!(
        e2,
        MathExpr::Binary {
            op: MathBinaryOp::TensorMatMul,
            lhs: Box::new(MathExpr::Variable("Q".to_string())),
            rhs: Box::new(MathExpr::Variable("K".to_string())),
        }
    );

    // 3. Precedence: (x * y) + 5
    let e3 = parse_math_expr_str("(x * y) + 5").expect("Should parse (x * y) + 5");
    assert_eq!(
        e3,
        MathExpr::Binary {
            op: MathBinaryOp::Add,
            lhs: Box::new(MathExpr::Binary {
                op: MathBinaryOp::Mul,
                lhs: Box::new(MathExpr::Variable("x".to_string())),
                rhs: Box::new(MathExpr::Variable("y".to_string())),
            }),
            rhs: Box::new(MathExpr::Immediate(5.0)),
        }
    );
}

#[test]
fn test_parse_and_verify_all_flagship_v2_examples() {
    use std::path::Path;
    use cronc::cl_jit::{ClJitCore, execute_cl_on_core};

    let files = [
        "examples/v2/mamba2_ssd.cl",
        "examples/v2/cluster_allreduce.cl",
        "examples/v2/deepseek_moe.cl",
    ];

    for f in &files {
        let p = if Path::new(f).exists() {
            std::path::PathBuf::from(f)
        } else {
            std::path::PathBuf::from("../../").join(f)
        };
        assert!(p.exists(), "Flagship file must exist: {:?}", p);

        let content = std::fs::read_to_string(&p).expect("Must read file");
        let doc = parse_cl_v2(&content).expect("Must parse cleanly");
        assert!(doc.target_contract.is_some(), "Must have target contract: {}", f);
        assert!(doc.io_binding.is_some(), "Must have I/O binding: {}", f);
        assert!(doc.math_morphism.is_some(), "Must have math morphism: {}", f);
        assert!(doc.control_flow.is_some(), "Must have control flow: {}", f);
        assert!(!doc.vliw_stream.is_empty(), "Must have VLIW stream: {}", f);

        let mut core = ClJitCore::new();
        execute_cl_on_core(&content, &mut core).expect("Must execute on JIT core");
        assert!(core.is_halted, "Must halt cleanly: {}", f);
    }
}
