// ============================================================================
// CRON & CL HIGH-LEVEL TO VLIW LOWERING UNIT TESTS
// Verifying Lowering to 128-Bit 4-Slot VLIW Machine Code & 4-Snake Scheduling
// ============================================================================

use cronc::cl_high_lowering::{compile_high_cl_to_cl, HighLoweringEngine, LoweringConfig};
use cronc::cl_ir::ClProgramIR;

#[test]
fn test_lower_basic_high_cl_to_vliw() {
    let source = r#"
        module test_gemm;

        .stage "ComputeTile", tile_x = 16, tile_y = 16, memory = "sram";

        kernel forward(x: Tensor[NVFP4, 16, 16], w: Tensor[NVFP4, 16, 16]) -> Tensor[NVFP4, 16, 16] {
            let out = .op.nvfp4_gemm(x, w);
            return out;
        }
    "#;

    let cl_text = compile_high_cl_to_cl(source).expect("Failed to lower high CL to VLIW");
    assert!(cl_text.contains(".stage \"ComputeTile\""));
    assert!(cl_text.contains("B0000:"));
    assert!(cl_text.contains("; Module: test_gemm"));

    // Verify it can be parsed back into ClProgramIR
    let ir = ClProgramIR::from_cl_text(&cl_text).expect("Lowered .cl must be valid ClProgramIR");
    assert!(!ir.bundles.is_empty());
}

#[test]
fn test_lower_beyond_addition_operators_to_microcode() {
    let source = r#"
        module beyond_addition_lowering;

        kernel test_ops(a: Tensor[FP32, 16, 16], b: Tensor[FP32, 16, 16]) {
            // Interference Coupling (<~>)
            let c = a <~> b;

            // Associative Scan (>>=)
            let s = a >>= b;

            // LogSpace Gating (<|>)
            let g = a <|> b;

            // Periodic Resonance (~*~)
            let w = a ~*~ b;

            // Rotor Reverse (~)
            let r = ~a;
        }
    "#;

    let cl_text = compile_high_cl_to_cl(source).expect("Failed to lower beyond-addition operators");
    // Verify that neuromorphic beyond-addition opcodes are present in the bundles:
    // OP = OpticalGemm, SS = SsmStep, SM = Softmax, WF = WaveResonance, BK = ReversibleInvert
    assert!(cl_text.contains("_OP") || cl_text.contains("_SS") || cl_text.contains("_SM") || cl_text.contains("_WF") || cl_text.contains("_BK"));

    let ir = ClProgramIR::from_cl_text(&cl_text).expect("Generated beyond-addition code must be valid ClProgramIR");
    assert!(ir.bundles.len() >= 2);
}

#[test]
fn test_lower_multi_primitive_pipeline() {
    let source = r#"
        module transformer_hybrid_block;

        arch HybridBlock {
            state kv_cache: RegTile[32, 32, FP16];

            fn forward(x: Tensor[BF16, 1, 512, 4096]) -> Tensor[BF16, 1, 512, 4096] {
                let norm = .op.rmsnorm(x);
                let ssd = .op.mamba2_ssd(norm);
                let attn = .op.flash_attention(norm, ssd);
                let gated = .op.swiglu(attn);
                return gated;
            }
        }
    "#;

    let cl_text = compile_high_cl_to_cl(source).expect("Failed to compile hybrid transformer block");
    assert!(cl_text.contains("B0000:"));

    // Check that RMSNorm (_RM), SSM (_SS), Softmax/Attn (_SM/_OP), SiLU (_SI) were lowered
    assert!(cl_text.contains("_RM"));
    assert!(cl_text.contains("_SS"));
    assert!(cl_text.contains("_SI"));

    let ir = ClProgramIR::from_cl_text(&cl_text).expect("Generated hybrid block code must be valid ClProgramIR");
    assert!(ir.bundles.len() >= 2);
}

#[test]
fn test_4snake_schedule_bundle_structure() {
    let source = r#"
        module snake_test;

        kernel math_kernel(a: FP32, b: FP32, c: FP32, d: FP32) {
            let x = a + b;
            let y = b * c;
            let z = c - d;
            let w = d / a;
            return x;
        }
    "#;

    let engine = HighLoweringEngine::new(LoweringConfig::default());
    let cl_text = engine.compile_high_source_to_cl(source).expect("Failed to compile snake test");
    let lines: Vec<&str> = cl_text.lines().filter(|l| l.starts_with("B")).collect();

    // Each bundle line must have 4 slots (128-bit VLIW)
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // format is "B0000: slot0 slot1 slot2 slot3"
        assert_eq!(parts.len(), 5, "Line should have label and 4 slots: {}", line);
        assert!(parts[0].ends_with(':'));
        assert_eq!(parts[1].len(), 10, "Slot 0 must be 10 characters: {}", parts[1]);
        assert_eq!(parts[2].len(), 10, "Slot 1 must be 10 characters: {}", parts[2]);
        assert_eq!(parts[3].len(), 10, "Slot 2 must be 10 characters: {}", parts[3]);
        assert_eq!(parts[4].len(), 10, "Slot 3 must be 10 characters: {}", parts[4]);
    }
}
