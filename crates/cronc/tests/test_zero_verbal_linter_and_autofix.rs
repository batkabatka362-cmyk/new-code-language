// ============================================================================
// Integration Test: Zero-Verbal Constitution Linter & Auto-Fixer
// Verifies:
// 1. Detection of forbidden procedural keywords (let, var, fn, return, etc.)
// 2. Detection of forbidden procedural variable soup (H_0, H_norm, Attn_out, etc.)
// 3. Detection of English mathematical functions (softmax, transpose, outer, sqrt, mean, silu)
// 4. Automated Unicode calculus transpilation via ZeroVerbalLinter::auto_fix_source
// 5. Verification that all foundation models in examples/models/ are 100% compliant
// ============================================================================

use cronc::cl_zero_verbal_lint::ZeroVerbalLinter;
use std::fs;
use std::path::Path;

#[test]
fn test_dirty_prose_detection_and_autofix_roundtrip() {
    let dirty_cl = r#"
// Bad code with procedural prose
attn(q, k, v) = softmax((q @ transpose(k)) * 0.125) @ v
M(k, v) = outer(k, v)
N(x) = sqrt(mean(x^2) + 1e-5)
H_act = silu(x)
let mut H_norm = 1.0
"#;

    // 1. Audit dirty code
    let audit_before = ZeroVerbalLinter::audit_source(dirty_cl, "dirty.cl");
    assert!(!audit_before.is_constitution_compliant);
    assert!(audit_before.violations_count >= 6);

    // 2. Run Auto-Fixer
    let (fixed_cl, fix_count) = ZeroVerbalLinter::auto_fix_source(dirty_cl);
    assert!(fix_count >= 5);

    // 3. Verify mathematical Unicode equivalents are present
    assert!(fixed_cl.contains("ρ((q @ k^T) * 0.125) @ v"), "Must contain ρ and ^T: {}", fixed_cl);
    assert!(fixed_cl.contains("(k ⊗ v)"), "Must contain ⊗: {}", fixed_cl);
    assert!(fixed_cl.contains("√(μ(x^2) + 1e-5)"), "Must contain √ and μ: {}", fixed_cl);
    assert!(fixed_cl.contains("σ(x)"), "Must contain σ: {}", fixed_cl);
}

#[test]
fn test_all_foundation_models_zero_verbal_compliance() {
    let models = [
        "examples/models/llama3_swiglu.cl",
        "examples/models/mamba2_ssd.cl",
        "examples/models/rwkv7_titans.cl",
        "examples/models/clifford_4d_rotor.cl",
        "examples/models/snn_lif_spiking.cl",
        "examples/models/deepseek_mla.cl",
    ];

    for path in models {
        let resolved = if Path::new(path).exists() {
            path.to_string()
        } else if Path::new(&format!("../../{}", path)).exists() {
            format!("../../{}", path)
        } else {
            panic!("Model file '{}' not found", path);
        };

        let content = fs::read_to_string(&resolved).expect("Must read model file");
        let report = ZeroVerbalLinter::audit_source(&content, &resolved);
        assert!(
            report.is_constitution_compliant,
            "Model '{}' must be 100% Zero-Verbal compliant! Found {} violations: {:?}",
            resolved, report.violations_count, report.diagnostics
        );
    }
}

