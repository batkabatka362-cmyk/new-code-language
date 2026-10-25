// ============================================================================
// CRON & CL HIGH-LEVEL AST & PARSER UNIT TESTS
// Verifying High-Level AI-Native Declarative Syntax & Beyond-Addition Operators
// ============================================================================

use cronc::cl_high_ast::*;
use cronc::cl_high_parser::parse_high_cl;

#[test]
fn test_parse_high_cl_basic_module() {
    let source = r#"
        module deepseek_v4_layer;

        import cl.tensor as tensor;
        import cl.recurrent as rnn;

        .stage "ComputeTile", tile_x = 16, tile_y = 16, memory = "sram";

        kernel forward(x: Tensor[NVFP4, Batch, Seq, 4096]) -> Tensor[NVFP4, Batch, Seq, 4096] {
            let h = .op.nvfp4_gemm(x, w_gate);
            return h;
        }
    "#;

    let module = parse_high_cl(source).expect("Failed to parse basic module");
    assert_eq!(module.name, "deepseek_v4_layer");
    assert_eq!(module.imports.len(), 2);
    assert_eq!(module.imports[0].path, "cl.tensor");
    assert_eq!(module.imports[0].alias, Some("tensor".to_string()));
    assert_eq!(module.stages.len(), 1);
    assert_eq!(module.stages[0].name, "ComputeTile");
    assert_eq!(module.kernels.len(), 1);
    assert_eq!(module.kernels[0].name, "forward");
    assert_eq!(module.kernels[0].params.len(), 1);
    assert_eq!(module.kernels[0].params[0].name, "x");
}

#[test]
fn test_parse_beyond_addition_operators() {
    let source = r#"
        module beyond_addition_test;

        kernel test_operators(a: Tensor[FP32, 16, 16], b: Tensor[FP32, 16, 16]) {
            // 1. Interference Coupling (<~>)
            let photonic_out = a <~> b;

            // 2. Continuous Associative Scan (>>=)
            let ssm_state = a >>= b;

            // 3. Log-Space Entropy Reduction (<|>)
            let gating = a <|> b;

            // 4. Ring Reduction (<%>)
            let crypto_mod = a <%> b;

            // 5. Radix Tree Graft (/@)
            let kv_branch = a /@ b;

            // 6. Periodic Resonance (~*~)
            let snake_wave = a ~*~ b;

            // 7. Clifford Exterior Wedge (^) & Rotor Reverse (~)
            let rotor_rev = ~a;
            let wedge_vol = a ^ b;
        }
    "#;

    let module = parse_high_cl(source).expect("Failed to parse beyond-addition operators");
    let kernel = &module.kernels[0];
    assert_eq!(kernel.body.len(), 8);

    // Verify operator types in AST
    match &kernel.body[0] {
        Stmt::Let { expr: Expr::BinaryOp { op, .. }, .. } => {
            assert_eq!(*op, HighBinaryOp::InterferenceCoupling);
        }
        _ => panic!("Expected InterferenceCoupling let statement"),
    }

    match &kernel.body[1] {
        Stmt::Let { expr: Expr::BinaryOp { op, .. }, .. } => {
            assert_eq!(*op, HighBinaryOp::AssociativeScan);
        }
        _ => panic!("Expected AssociativeScan let statement"),
    }

    match &kernel.body[2] {
        Stmt::Let { expr: Expr::BinaryOp { op, .. }, .. } => {
            assert_eq!(*op, HighBinaryOp::LogSpaceGating);
        }
        _ => panic!("Expected LogSpaceGating let statement"),
    }

    match &kernel.body[3] {
        Stmt::Let { expr: Expr::BinaryOp { op, .. }, .. } => {
            assert_eq!(*op, HighBinaryOp::RingReduction);
        }
        _ => panic!("Expected RingReduction let statement"),
    }

    match &kernel.body[4] {
        Stmt::Let { expr: Expr::BinaryOp { op, .. }, .. } => {
            assert_eq!(*op, HighBinaryOp::RadixGraft);
        }
        _ => panic!("Expected RadixGraft let statement"),
    }

    match &kernel.body[5] {
        Stmt::Let { expr: Expr::BinaryOp { op, .. }, .. } => {
            assert_eq!(*op, HighBinaryOp::PeriodicResonance);
        }
        _ => panic!("Expected PeriodicResonance let statement"),
    }

    match &kernel.body[6] {
        Stmt::Let { expr: Expr::UnaryOp { op, .. }, .. } => {
            assert_eq!(*op, HighUnaryOp::RotorReverse);
        }
        _ => panic!("Expected RotorReverse unary let statement"),
    }

    match &kernel.body[7] {
        Stmt::Let { expr: Expr::BinaryOp { op, .. }, .. } => {
            assert_eq!(*op, HighBinaryOp::ExteriorWedge);
        }
        _ => panic!("Expected ExteriorWedge let statement"),
    }
}

#[test]
fn test_parse_universal_machine_primitives() {
    let source = r#"
        module primitive_invocations;

        kernel compute_state(input: Tensor[BF16, 1, 2048, 4096]) {
            let ssd = .op.mamba2_ssd(input, d_state = 128, chunk_size = 64);
            let wgmma = .op.wgmma_async_16x16(input, weights, acc = true);
            let clebsch = .op.nequip_clebsch_gordan(l1 = 1, l2 = 2);
        }
    "#;

    let module = parse_high_cl(source).expect("Failed to parse universal primitives");
    let kernel = &module.kernels[0];
    assert_eq!(kernel.body.len(), 3);

    match &kernel.body[0] {
        Stmt::Let { name, expr: Expr::PrimitiveCall { op_name, positional_args, named_args }, .. } => {
            assert_eq!(name, "ssd");
            assert_eq!(op_name, "mamba2_ssd");
            assert_eq!(positional_args.len(), 1);
            assert_eq!(named_args.len(), 2);
            assert_eq!(named_args[0].0, "d_state");
            assert_eq!(named_args[1].0, "chunk_size");
        }
        _ => panic!("Expected mamba2_ssd primitive call"),
    }

    match &kernel.body[1] {
        Stmt::Let { name, expr: Expr::PrimitiveCall { op_name, positional_args, named_args }, .. } => {
            assert_eq!(name, "wgmma");
            assert_eq!(op_name, "wgmma_async_16x16");
            assert_eq!(positional_args.len(), 2);
            assert_eq!(named_args.len(), 1);
            assert_eq!(named_args[0].0, "acc");
        }
        _ => panic!("Expected wgmma_async_16x16 primitive call"),
    }

    match &kernel.body[2] {
        Stmt::Let { name, expr: Expr::PrimitiveCall { op_name, positional_args, named_args }, .. } => {
            assert_eq!(name, "clebsch");
            assert_eq!(op_name, "nequip_clebsch_gordan");
            assert_eq!(positional_args.len(), 0);
            assert_eq!(named_args.len(), 2);
        }
        _ => panic!("Expected nequip_clebsch_gordan primitive call"),
    }
}

#[test]
fn test_parse_architecture_and_attributes() {
    let source = r#"
        module full_arch_test;

        arch LiquidTimeConstantCell {
            state hidden: StateSpace[64, 256];
            state memory_tile: RegTile[32, 32, FP16];

            @realtime(clock_hz = 1000000000, max_jitter_ns = 5)
            fn step(dt: FP32, x: Tensor[FP32, 64]) -> Tensor[FP32, 64] {
                let d_state = .op.ode_dormand_prince(x, dt = dt);
                return d_state;
            }
        }
    "#;

    let module = parse_high_cl(source).expect("Failed to parse full architecture definition");
    assert_eq!(module.architectures.len(), 1);
    let arch = &module.architectures[0];
    assert_eq!(arch.name, "LiquidTimeConstantCell");
    assert_eq!(arch.states.len(), 2);
    assert_eq!(arch.states[0].name, "hidden");
    assert_eq!(arch.states[1].name, "memory_tile");

    assert_eq!(arch.methods.len(), 1);
    let method = &arch.methods[0];
    assert_eq!(method.name, "step");
    assert_eq!(method.attributes.len(), 1);
    assert_eq!(method.attributes[0].name, "realtime");
    assert_eq!(method.attributes[0].args.get("clock_hz").unwrap(), "1000000000");
    assert_eq!(method.attributes[0].args.get("max_jitter_ns").unwrap(), "5");
}

#[test]
fn test_parse_sovereign_cl_v2_via_high_parser() {
    let sovereign_source = r#"
        @kernel tensor_morphism_engine
        .target silicon.neuro_vliw
        .ipc_target 4.0
        .cores 256
        @mesh [4×4×4×4] : ⮑ (x+1, y, z, w)

        @io {
            in:  InputTensor[16×16:FP32]  @sram[0x0000]
            out: OutputTensor[16×16:FP32] @sram[0x0400]
        }

        §MorphismBlock[16×16:FP32] {
            InputTensor ↦ OutputTensor :
                Normed := rmsnorm(InputTensor)
                OutputTensor := silu(Normed)
        }

        §ControlFlow {
            ⟳ (256) {
                #R1 ← @sram[0x0000]
                @sram[0x0400] ← #R1
            }
        }

        !Δ { |OutputTensor| < 1e4, latency < 10ns }
    "#;

    let module = parse_high_cl(sovereign_source).expect("High-level parser must parse Sovereign CL v2.0 non-verbal code");
    assert_eq!(module.name, "tensor_morphism_engine");
    assert_eq!(module.stages.len(), 1);
    assert_eq!(module.stages[0].name, "SiliconTarget");
    assert_eq!(module.kernels.len(), 1);
    assert_eq!(module.kernels[0].name, "MorphismBlock");
    assert_eq!(module.kernels[0].body.len(), 2);
}
