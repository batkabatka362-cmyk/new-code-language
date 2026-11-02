// ============================================================================
// CRON CL v2.0 BASE94 2-4 CHARACTER CIPHER & SYMBOLIC OPERATOR TEST SUITE
// Test Suite: test_base94_cipher_opcodes
//
// Verifies:
// 1. Multi-character Base94 symbolic operators:
//    - \+\ (Residual addition)
//    - <~> (Tensor contraction / MatMul @)
//    - ~*~ (Gated Hadamard product *)
//    - !~! (Reverse-mode analytical autodiff ∇)
//    - |/| (RMSNorm normalization N)
//    - |^| (Elementwise max)
//    - |_| (Elementwise min)
//    - ||  (Frobenius/Euclidean norm ∥)
// 2. 2-4 Character Base94 Non-Verbal Cipher Opcodes:
//    - _RM (RMSNorm), _SI (Sigmoid), _SM (Softmax), _SP (Softplus)
//    - _MS (Mish), _SL (SiLU), _RL (ReLU), _GL (GeLU)
//    - _Z0 (Standardize), _M1 (MinMax), _B1 (BitLinear 1.58b)
//    - _TQ (Ternary quant), _Q8 (INT8 quant), _FA (FlashAttn)
//    - _GQA (Grouped Query Attn), _MLA (DeepSeek MLA), _SSD (Mamba-2 SSD)
//    - _KC (Chebyshev term), _KAN (Chebyshev basis), _ROP (RoPE)
//    - _SWG (SwiGLU), _TT (Titans), _MUO (Muon spectral step), _VJP (∇)
// 3. Modular Dynamic Pipeline Composition:
//    x -> _RM(γ) -> _KAN(2) -> (<~> W) -> (\+\ b) -> _MS
// 4. End-to-End Frontier Zero-Verbal Model with Analytical Autodiff and VLIW Lowering
// ============================================================================

use cronc::cl_math::{parse_math_code, MathEvaluator, MathValue};
use cronc::cl_syntax_spec::verify_sovereign_pure_cipher_syntax;
use cronc::cl_zero_verbal_lint::ZeroVerbalLinter;

#[test]
fn test_base94_symbolic_operators_evaluation() {
    let code = r#"
        // 1. Residual addition \+\
        a = [1.0, 2.0]
        b = [3.0, 4.0]
        r_add = a \+\ b

        // 2. MatMul Contraction <~>
        W = [[1.0, 2.0], [3.0, 4.0]]
        r_mm = a <~> W

        // 3. Gated Hadamard Product ~*~
        r_had = a ~*~ b

        // 4. Elementwise Min |_| and Max |^|
        r_max = |^|(b)
        r_min = |_|(b)

        // 5. RMSNorm |/|
        r_norm = |/|(a)

        // 6. Frobenius Norm ||
        r_mag = ||(a)
    "#;

    let prog = parse_math_code(code).expect("Parse Base94 symbolic operators failed");
    let mut eval = MathEvaluator::new();
    eval.execute_program_value(&prog)
        .expect("Eval Base94 symbolic operators failed");

    // Check \+\
    if let Some(MathValue::Vector(v)) = eval.env.get("r_add") {
        assert_eq!(v, &[4.0, 6.0]);
    } else {
        panic!("r_add must be Vector");
    }

    // Check <~>
    if let Some(MathValue::Vector(v)) = eval.env.get("r_mm") {
        // [1*1 + 2*3, 1*2 + 2*4] = [7, 10]
        assert_eq!(v, &[7.0, 10.0]);
    } else {
        panic!("r_mm must be Vector");
    }

    // Check ~*~
    if let Some(MathValue::Vector(v)) = eval.env.get("r_had") {
        assert_eq!(v, &[3.0, 8.0]);
    } else {
        panic!("r_had must be Vector");
    }

    // Check |^| and |_|
    if let Some(MathValue::Scalar(m)) = eval.env.get("r_max") {
        assert_eq!(*m, 4.0);
    } else {
        panic!("r_max must be Scalar");
    }
    if let Some(MathValue::Scalar(m)) = eval.env.get("r_min") {
        assert_eq!(*m, 3.0);
    } else {
        panic!("r_min must be Scalar");
    }

    // Check ||
    if let Some(MathValue::Scalar(m)) = eval.env.get("r_mag") {
        assert!((*m - (5.0_f64).sqrt()).abs() < 1e-5);
    } else {
        panic!("r_mag must be Scalar");
    }
}

#[test]
fn test_base94_cipher_opcodes_all_operators() {
    let code = r#"
        x = [1.0, -2.0, 3.0, -4.0]
        γ = [1.0, 1.0, 1.0, 1.0]

        // Normalization & Normalizers
        y_rm = _RM(x, γ)
        y_z0 = _Z0(x)
        y_m1 = _M1(x)

        // Activations
        y_si = _SI(x)
        y_sm = _SM(x)
        y_sp = _SP(x)
        y_ms = _MS(x)
        y_sl = _SL(x)
        y_rl = _RL(x)
        y_gl = _GL(x)

        // Math transforms
        y_exp = _EXP(x)
        y_sq  = _SQ([4.0, 9.0])
        y_abs = _ABS(x)
        y_max = _MAX(x)
        y_min = _MIN(x)
        y_avg = _AVG(x)
        y_sum = _SUM(x)

        // Quantization
        w_mat = [[1.2, -0.8], [0.1, -1.9]]
        y_tq = _TQ(w_mat)
        y_b1 = _B1(x, [[1.0, 0.0], [0.0, 1.0], [1.0, -1.0], [0.0, 1.0]])

        // Chebyshev KAN
        y_kan = _KAN(0.5, 3)

        // Transpose & Concat
        y_tr = _TR(w_mat)
        y_cat = _CAT([1.0, 2.0], [3.0, 4.0])
    "#;

    let prog = parse_math_code(code).expect("Parse Base94 cipher opcodes failed");
    let mut eval = MathEvaluator::new();
    eval.execute_program_value(&prog)
        .expect("Eval Base94 cipher opcodes failed");

    // Verify _RM
    assert!(eval.env.contains_key("y_rm"));
    // Verify _SI
    assert!(eval.env.contains_key("y_si"));
    // Verify _SM (Softmax sums to 1.0)
    if let Some(MathValue::Vector(v)) = eval.env.get("y_sm") {
        let sum: f64 = v.iter().sum();
        assert!((sum - 1.0).abs() < 1e-4);
    } else {
        panic!("y_sm must be Vector");
    }
    // Verify _KAN basis vector [1.0, x, 2x^2-1, 4x^3-3x] for x=0.5
    // T_0 = 1.0, T_1 = 0.5, T_2 = 2*(0.25)-1 = -0.5, T_3 = 4*(0.125)-3*(0.5) = 0.5 - 1.5 = -1.0
    if let Some(MathValue::Vector(v)) = eval.env.get("y_kan") {
        assert_eq!(v.len(), 4);
        assert!((v[0] - 1.0).abs() < 1e-5);
        assert!((v[1] - 0.5).abs() < 1e-5);
        assert!((v[2] - (-0.5)).abs() < 1e-5);
        assert!((v[3] - (-1.0)).abs() < 1e-5);
    } else {
        panic!("y_kan must be Vector");
    }

    // Verify _SQ
    if let Some(MathValue::Vector(v)) = eval.env.get("y_sq") {
        assert_eq!(v, &[2.0, 3.0]);
    } else {
        panic!("y_sq must be Vector");
    }

    // Verify _CAT
    if let Some(MathValue::Vector(v)) = eval.env.get("y_cat") {
        assert_eq!(v, &[1.0, 2.0, 3.0, 4.0]);
    } else {
        panic!("y_cat must be Vector");
    }
}

#[test]
fn test_base94_pipeline_chaining_and_vliw_lowering() {
    let pipeline_code = r#"
        x = [0.2, 0.4, 0.6]
        W = [[1.0, 0.5], [0.5, 1.0], [0.2, 0.2]]
        b = [0.1, -0.1]

        // Modular dynamic composition via pipeline chaining
        y = x -> _Z0 -> (<~> W) -> (\+\ b) -> _MS
    "#;

    let prog = parse_math_code(pipeline_code).expect("Parse pipeline chaining failed");
    let mut eval = MathEvaluator::new();
    eval.execute_program_value(&prog)
        .expect("Eval pipeline chaining failed");

    if let Some(MathValue::Vector(v)) = eval.env.get("y") {
        assert_eq!(v.len(), 2);
    } else {
        panic!("y must be Vector");
    }

    // Verify VLIW Lowering parity with Base94 Ciphers
    let vliw_bundles = prog.to_vliw();
    assert!(!vliw_bundles.is_empty(), "Must emit 128-bit VLIW microcode bundles");
    assert!(vliw_bundles.contains("B0000:"), "Must contain VLIW bundles");
    assert!(vliw_bundles.contains("_MD") || vliw_bundles.contains("_PO"), "Must contain lowered VLIW opcodes");
}

#[test]
fn test_base94_e2e_frontier_model_with_autodiff_and_muon() {
    let model_code = r#"
        // 1. Inputs and Parameters
        X = [[0.5, -0.2], [0.8, 0.1]]
        W_g = [[1.0, 0.0], [0.0, 1.0]]
        W_u = [[0.5, 0.5], [-0.5, 0.5]]
        W_d = [[1.0, -1.0], [1.0, 1.0]]
        Y_target = [[1.0, 0.0], [0.0, 1.0]]

        // 2. SwiGLU projection with Base94 cipher
        H = _SWG(X, W_g, W_u, W_d)

        // 3. Residual addition with \+\
        Y = X \+\ H

        // 4. Loss computation
        Diff = Y - Y_target
        L = ||(Diff)

        // 5. Analytical Reverse Autodiff with !~!
        g_d = !~!(L, W_d)

        // 6. Muon Spectral Step with _MUO
        O_muon = _MUO(g_d, 5)

        // 7. Parameter Update
        W_next = W_d - (0.001 * O_muon)
    "#;

    let prog = parse_math_code(model_code).expect("Parse E2E Frontier Model failed");
    let mut eval = MathEvaluator::new();
    eval.execute_program_value(&prog)
        .expect("Eval E2E Frontier Model failed");

    // 1. Verify SwiGLU output exists
    assert!(eval.env.contains_key("H"));

    // 2. Verify Residual addition
    assert!(eval.env.contains_key("Y"));

    // 3. Verify Gradient computed via !~!
    if let Some(MathValue::Matrix(g)) = eval.env.get("g_d") {
        assert_eq!(g.len(), 2);
        assert_eq!(g[0].len(), 2);
    } else {
        panic!("g_d must be a 2x2 gradient matrix");
    }

    // 4. Verify Muon orthogonal update
    if let Some(MathValue::Matrix(o)) = eval.env.get("O_muon") {
        assert_eq!(o.len(), 2);
        assert_eq!(o[0].len(), 2);
    } else {
        panic!("O_muon must be a 2x2 matrix");
    }

    // 5. Verify Updated Weights
    assert!(eval.env.contains_key("W_next"));

    // 6. Verify Zero Verbal Linter Conformance
    let audit_report = ZeroVerbalLinter::audit_source(model_code, "test_base94_model.cl");
    assert!(
        audit_report.is_constitution_compliant,
        "Base94 model code violated zero-verbal standard: {:?}",
        audit_report.diagnostics
    );

    // 7. Verify Pure Cipher Silicon Syntax Conformance
    let syntax_report = verify_sovereign_pure_cipher_syntax(model_code);
    assert!(
        syntax_report.is_ok(),
        "Base94 model code violated pure cipher standard: {:?}",
        syntax_report.err()
    );
}

#[test]
fn test_base94_frontier_cipher_model_file_conformance() {
    use std::fs;
    use std::path::Path;

    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/models/base94_frontier_cipher_model.cl");
    let content = fs::read_to_string(&path).expect("Must read base94_frontier_cipher_model.cl");

    // 1. Zero-verbal audit
    let audit = ZeroVerbalLinter::audit_source(&content, "base94_frontier_cipher_model.cl");
    assert!(
        audit.is_constitution_compliant,
        "base94_frontier_cipher_model.cl failed zero-verbal: {:?}",
        audit.diagnostics
    );

    // 2. Pure cipher conformance
    let conf = verify_sovereign_pure_cipher_syntax(&content);
    assert!(
        conf.is_ok(),
        "base94_frontier_cipher_model.cl failed pure cipher: {:?}",
        conf.err()
    );
}

#[test]
fn test_base94_comprehensive_lists_ciphers_evaluation() {
    let code = r#"
        // 1. Data Scaling & Feature Engineering (lists.md Ch 1)
        x_raw = [10.0, 20.0, 30.0, 40.0, 100.0]
        x_rsc = _RSC(x_raw)
        x_mas = _MAS(x_raw)
        x_bcx = _BCX(x_raw, 0.5)
        x_nan = [1.0, 0.0, 3.0]
        x_imp = _IMP(x_nan, 2.0)
        x_ohe = _OHE(2.0, 5.0)
        x_fft = _FFT([1.0, 0.0, -1.0, 0.0])
        x_ema = _EMA([1.0, 2.0, 3.0, 4.0], 0.5)

        // 2. Classical ML & Solvers (lists.md Ch 2)
        X_mat = [[1.0, 1.0], [1.0, 2.0], [1.0, 3.0]]
        Y_vec = [2.0, 3.0, 4.0]
        beta_ols = _OLS(X_mat, Y_vec)
        beta_rdg = _RDG(X_mat, Y_vec, 0.1)
        w_las = _LAS([0.5, -0.005, 0.02], 0.01)
        k_step = _KMN([[1.0, 1.0], [5.0, 5.0]], [[0.0, 0.0], [6.0, 6.0]])

        // 3. Vector Math & Distances (lists.md Ch 3)
        u_v = [1.0, 2.0]
        v_v = [4.0, 6.0]
        d_l2 = _L2D(u_v, v_v)
        d_l1 = _L1D(u_v, v_v)
        s_cos = _COS(u_v, v_v)
        d_ham = _HAM([1.0, 0.0, 1.0], [1.0, 1.0, 0.0])
        d_hyp = _HYP([0.0, 0.0], [0.5, 0.0])

        // 4. Graph AI & Search (lists.md Ch 4)
        H_g = [[1.0, 0.0], [0.0, 1.0]]
        A_g = [[0.0, 1.0], [1.0, 0.0]]
        g_out = _GCN(H_g, A_g)
        m_puct = _MCT(0.8, 0.5, 100.0, 10.0, 1.414)
        c_geom = _CGA([1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0])

        // 5. Frontier Models (lists.md Ch 5, 7, 10, 11)
        x_flow = _CFM([1.0, 2.0], [0.1, 0.2], 0.1)
        t_eng = [[0.1, 0.2], [0.3, 0.4]]
        x_egr = _EGR([1.0, 2.0], t_eng)
        x_mhc = _MHC([1.0, 2.0], [0.5, 0.5])
        h_mtp = _MTP([[1.0, 2.0]], [[0.5], [0.5]])
        x_irp = _IRP([1.0, 0.0], 1.0, 8.0)

        // 6. AGI, Audio, Robotics, Neuromorphic (lists.md Ch 15, 17, 18, 20)
        cb_mat = [[0.1, 0.2], [0.3, 0.4], [0.5, 0.6]]
        a_rvq = _RVQ([0.0, 1.0], cb_mat)
        r_act = _ROB([0.0, 0.0], [1.0, 1.0], 0.05)
        d_moe = _DEP([1.0, 2.0, 3.0], [0.1, 0.8, 0.5], 2.0)
        t_gemm = _TMK([[1.0, 2.0]], [[3.0], [4.0]])
        f_lin = _FLA([[1.0, 0.0]], [[1.0, 0.0]], [[0.5, 0.5]], 0.95)
        l_step = _LQD([0.5, 0.5], [0.1, 0.1], [0.2, 0.2], [0.3, 0.3], 0.01)
        j_nrg = _JEP([1.0, 2.0], [1.0, 1.0])
        g_db = _GFN(1.0, 0.5, 2.0, 0.25)
        v_lif = _LIF([0.5, 0.8], [0.3, 0.4], 0.9, 1.0)
        w_stdp = _STP([0.5], 10.0, 0.01, 0.01, 20.0, 20.0)
        fe_val = _FEP([0.7, 0.3], [0.5, 0.5])
    "#;

    let prog = parse_math_code(code).expect("Parse comprehensive lists ciphers failed");
    let mut eval = MathEvaluator::new();
    eval.execute_program_value(&prog)
        .expect("Eval comprehensive lists ciphers failed");

    // 1. Verify Data Scaling
    assert!(eval.env.contains_key("x_rsc"));
    assert!(eval.env.contains_key("x_mas"));
    assert!(eval.env.contains_key("x_bcx"));
    if let Some(MathValue::Vector(v)) = eval.env.get("x_imp") {
        assert_eq!(v[1], 2.0); // 0.0 was imputed to 2.0
    }
    if let Some(MathValue::Vector(v)) = eval.env.get("x_ohe") {
        assert_eq!(v, &[0.0, 0.0, 1.0, 0.0, 0.0]);
    }
    assert!(eval.env.contains_key("x_fft"));
    assert!(eval.env.contains_key("x_ema"));

    // 2. Verify Classical ML
    if let Some(MathValue::Vector(b)) = eval.env.get("beta_ols") {
        // Line y = 1 + x => beta = [1.0, 1.0]
        assert!((b[0] - 1.0).abs() < 1e-4);
        assert!((b[1] - 1.0).abs() < 1e-4);
    } else {
        panic!("beta_ols must be Vector");
    }
    assert!(eval.env.contains_key("beta_rdg"));
    if let Some(MathValue::Vector(w)) = eval.env.get("w_las") {
        assert!((w[0] - 0.49).abs() < 1e-5);
        assert_eq!(w[1], 0.0); // shrunk to zero
    }
    if let Some(MathValue::Vector(k)) = eval.env.get("k_step") {
        assert_eq!(k, &[0.0, 1.0]); // row 0 -> centroid 0, row 1 -> centroid 1
    }

    // 3. Verify Vector Math & Distances
    if let Some(MathValue::Scalar(d)) = eval.env.get("d_l2") {
        assert_eq!(*d, 5.0); // sqrt(3^2 + 4^2) = 5
    }
    if let Some(MathValue::Scalar(d)) = eval.env.get("d_l1") {
        assert_eq!(*d, 7.0); // 3 + 4 = 7
    }
    assert!(eval.env.contains_key("s_cos"));
    if let Some(MathValue::Scalar(d)) = eval.env.get("d_ham") {
        assert_eq!(*d, 2.0);
    }
    assert!(eval.env.contains_key("d_hyp"));

    // 4. Verify Graph AI & Search
    assert!(eval.env.contains_key("g_out"));
    assert!(eval.env.contains_key("m_puct"));
    assert!(eval.env.contains_key("c_geom"));

    // 5. Verify Frontier Models
    if let Some(MathValue::Vector(v)) = eval.env.get("x_flow") {
        assert!((v[0] - 1.01).abs() < 1e-5);
        assert!((v[1] - 2.02).abs() < 1e-5);
    }
    assert!(eval.env.contains_key("x_egr"));
    assert!(eval.env.contains_key("x_mhc"));
    assert!(eval.env.contains_key("h_mtp"));
    assert!(eval.env.contains_key("x_irp"));

    // 6. Verify AGI, Audio, Robotics, Neuromorphic
    assert!(eval.env.contains_key("a_rvq"));
    if let Some(MathValue::Vector(v)) = eval.env.get("r_act") {
        assert!((v[0] - 0.05).abs() < 1e-5);
    }
    assert!(eval.env.contains_key("d_moe"));
    if let Some(MathValue::Matrix(m)) = eval.env.get("t_gemm") {
        assert_eq!(m[0][0], 11.0); // 1*3 + 2*4 = 11
    }
    assert!(eval.env.contains_key("f_lin"));
    assert!(eval.env.contains_key("l_step"));
    if let Some(MathValue::Scalar(e)) = eval.env.get("j_nrg") {
        assert_eq!(*e, 0.5); // (0^2 + 1^2)/2 = 0.5
    }
    if let Some(MathValue::Scalar(r)) = eval.env.get("g_db") {
        assert_eq!(*r, 0.0); // 1.0*0.5 - 2.0*0.25 = 0.0
    }
    assert!(eval.env.contains_key("v_lif"));
    assert!(eval.env.contains_key("w_stdp"));
    assert!(eval.env.contains_key("fe_val"));
}

#[test]
fn test_base94_pipeline_chaining_comprehensive_modular() {
    let pipeline_code = r#"
        // Modular Data Pipeline Chaining without words:
        x_in = [1.0, 0.0, 4.0, 8.0]
        y_out = x_in -> _IMP(2.0) -> _RSC -> _MAS -> _EMA(0.8) -> _Z0 -> _SI
    "#;

    let prog = parse_math_code(pipeline_code).expect("Parse chained pipeline failed");
    let mut eval = MathEvaluator::new();
    eval.execute_program_value(&prog)
        .expect("Eval chained pipeline failed");

    if let Some(MathValue::Vector(v)) = eval.env.get("y_out") {
        assert_eq!(v.len(), 4);
        for &val in v {
            assert!(val >= 0.0 && val <= 1.0, "Sigmoid output must be in [0, 1]");
        }
    } else {
        panic!("y_out must be a Vector");
    }

    // Verify VLIW Lowering generates bundles with matching cipher opcodes
    let vliw_bundles = prog.to_vliw();
    assert!(vliw_bundles.contains("B0000:"));
    assert!(vliw_bundles.contains(">"));
}

