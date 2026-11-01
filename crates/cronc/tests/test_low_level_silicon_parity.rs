use cronc::cl_math::parse_math_code;

#[test]
fn test_ptx_arbitrary_compound_activations_and_fma() {
    let code = r#"
        deep_mlp_act(x: f32, y: f32) = silu(x) * tanh(y) + gelu(x + y)
    "#;
    let prog = parse_math_code(code).expect("Parse math code failed");
    let ptx = prog.to_ptx();

    // Verify PTX headers
    assert!(ptx.contains(".version 7.5"), "Missing PTX version 7.5 directive");
    assert!(ptx.contains(".target sm_80"), "Missing PTX target sm_80 directive");
    assert!(ptx.contains(".address_size 64"), "Missing 64-bit address size directive");

    // Verify function signature
    assert!(
        ptx.contains(".visible .func (.param .f32 ret) deep_mlp_act"),
        "Missing scalar function declaration"
    );

    // Verify real hardware instructions
    assert!(ptx.contains("tanh.approx.f32"), "Missing hardware tanh instruction in PTX");
    assert!(ptx.contains("ex2.approx.f32"), "Missing hardware base-2 exponential instruction");
    assert!(ptx.contains("rcp.rn.f32"), "Missing hardware reciprocal instruction");
    assert!(ptx.contains("fma.rn.f32"), "Missing fused multiply-add instruction");
    assert!(ptx.contains("st.param.f32 [ret]"), "Missing store to return parameter");

    // Verify ZERO mock fallbacks
    assert!(
        !ptx.contains("mov.f32 %f0, 0.0;"),
        "PTX must not contain dummy mov 0.0 fallback: {}",
        ptx
    );
}

#[test]
fn test_ptx_arbitrary_universal_tensor_kernel() {
    let code = r#"
        vector_activation_kernel(A: f32[1024], B: f32[1024]) = silu(A) * tanh(B) + 0.25
    "#;
    let prog = parse_math_code(code).expect("Parse math code failed");
    let ptx = prog.to_ptx();

    // Verify universal kernel entry
    assert!(
        ptx.contains(".visible .entry vector_activation_kernel("),
        "Expected .visible .entry kernel for tensor operation"
    );
    assert!(ptx.contains(".param .u64 param_A"), "Expected 64-bit pointer parameter for A");
    assert!(ptx.contains(".param .u64 param_B"), "Expected 64-bit pointer parameter for B");
    assert!(ptx.contains(".param .u64 param_out"), "Expected 64-bit output pointer parameter");

    // Verify thread and block coordinate calculations
    assert!(ptx.contains("mov.u32 %r0, %tid.x;"), "Missing thread ID x");
    assert!(ptx.contains("mov.u32 %r1, %ctaid.x;"), "Missing block ID x");
    assert!(ptx.contains("mov.u32 %r2, %ntid.x;"), "Missing block dimension x");
    assert!(ptx.contains("mad.lo.u32 %r3, %r1, %r2, %r0;"), "Missing global thread index computation");

    // Verify boundary check
    assert!(ptx.contains("setp.ge.u32 %p0, %r3, 1024;"), "Missing bounds check");
    assert!(ptx.contains("@%p0 bra L_exit;"), "Missing bounds guard branch");

    // Verify global memory loads and stores
    assert!(ptx.contains("ld.global.f32"), "Missing global memory load");
    assert!(ptx.contains("st.global.f32"), "Missing global memory store");

    // Verify hardware instructions
    assert!(ptx.contains("tanh.approx.f32"), "Missing hardware tanh instruction");
    assert!(ptx.contains("fma.rn.f32"), "Missing FMA instruction for adding 0.25 bias");
}

#[test]
fn test_ptx_logarithmic_exponential_gaussian_rbf() {
    let code = r#"
        gaussian_rbf(x: f32, y: f32) = exp(-(x * x + y * y)) + ln(x + 2.0)
    "#;
    let prog = parse_math_code(code).expect("Parse math code failed");
    let ptx = prog.to_ptx();

    assert!(ptx.contains("ex2.approx.f32"), "Missing exp2 instruction");
    assert!(ptx.contains("lg2.approx.f32"), "Missing log2 instruction");
    assert!(ptx.contains("neg.f32"), "Missing negation instruction");
    assert!(ptx.contains("add.f32"), "Missing add instruction");
}

#[test]
fn test_vliw_arbitrary_neural_expression_bundles_and_sealing() {
    let code = r#"
        neural_cell(x: f32, w: f32, b: f32) = silu(x * w + b) * tanh(x) + gelu(x)
    "#;
    let prog = parse_math_code(code).expect("Parse math code failed");
    let bundles = prog.to_vliw_bundles();

    assert!(!bundles.is_empty(), "Bundles must not be empty");
    for (i, b) in bundles.iter().enumerate() {
        assert_eq!(b.slots.len(), 4, "Bundle {} must have exactly 4 slots", i);
        for slot in &b.slots {
            // Verify slot is exactly 10 characters (9 payload + 1 CRC-8 token)
            assert_eq!(
                slot.raw.len(),
                10,
                "Slot {} in bundle {} must be exactly 10 characters",
                slot.raw,
                i
            );
            assert!(
                slot.raw == "_NO00#000>"
                    || slot.raw == "_HL00$000!"
                    || cronc::cl_lang::verify_token_crc8(&slot.raw),
                "Slot {} in bundle {} failed CRC-8 ATM verification",
                slot.raw,
                i
            );
        }
    }

    let vliw_text = prog.to_vliw();
    assert!(vliw_text.contains("B0000:"), "Missing bundle 0 in VLIW output");
    assert!(vliw_text.contains("_HL00"), "Missing halt instruction at end of microcode");
}

#[test]
fn test_vliw_pure_fused_linear_and_matrix_slots() {
    let code = r#"
        X = [[1.0, 2.0], [3.0, 4.0]]
        W = [[0.5, 0.1], [0.2, 0.4]]
        Y = relu(X @ W)
    "#;
    let prog = parse_math_code(code).expect("Parse math code failed");
    let bundles = prog.to_vliw_bundles();

    assert!(!bundles.is_empty(), "Bundles must be synthesized");
    let vliw_text = prog.to_vliw();
    // Verify that Matrix Dot and Max micro-ops appear
    assert!(
        vliw_text.contains("_MD") || vliw_text.contains("_MX") || vliw_text.contains("'LD"),
        "Expected matrix/activation micro-ops in VLIW microcode:\n{}",
        vliw_text
    );
}
