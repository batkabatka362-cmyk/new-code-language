// ============================================================================
// Integration Test: Foundation Models Library (examples/models/*.cl)
// Verifies:
// 1. llama3_swiglu.cl (RMSNorm + SwiGLU Gated Feed-Forward)
// 2. mamba2_ssd.cl (State-Space Duality Linear Scan Recurrence)
// 3. rwkv7_titans.cl (Associative Long-Term Memory Step)
// 4. clifford_4d_rotor.cl (4D Orthogonal Rotor Manifold)
// 5. snn_lif_spiking.cl (Neuromorphic Leaky Integrate-and-Fire Step)
// ============================================================================

use cronc::cl_math::parse_math_file;
use std::path::Path;

#[test]
fn test_foundation_models_compilation_and_lowering() {
    let models = [
        "examples/models/llama3_swiglu.cl",
        "examples/models/mamba2_ssd.cl",
        "examples/models/rwkv7_titans.cl",
        "examples/models/clifford_4d_rotor.cl",
        "examples/models/snn_lif_spiking.cl",
        "examples/models/deepseek_mla.cl",
    ];

    for model_path in models {
        let resolved_path = if Path::new(model_path).exists() {
            model_path.to_string()
        } else if Path::new(&format!("../../{}", model_path)).exists() {
            format!("../../{}", model_path)
        } else {
            panic!("Model file '{}' must exist", model_path);
        };

        let prog = parse_math_file(&resolved_path).expect(&format!("Failed to parse '{}'", resolved_path));
        assert!(!prog.items.is_empty(), "Model '{}' must contain mathematical definitions", resolved_path);

        // Verify C23 lowering
        let c23_code = prog.to_c23_executable();
        assert!(c23_code.contains("#include <math.h>"), "Generated C23 must include math.h");
        assert!(c23_code.contains("int main("), "Generated C23 must contain main harness");

        // Verify PTX kernel export
        let ptx_code = prog.to_ptx();
        assert!(ptx_code.contains(".version 7.5"), "Generated PTX must be valid CUDA PTX");
    }
}
