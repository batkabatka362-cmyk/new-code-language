// ============================================================================
// CRON Integration Test: Silicon Autotuner Engine & Decoupled Hardware Schedules
// Tests:
//   - Multi-candidate autotune directive syntax parsing
//   - 4D-Torus VLIW Microarchitectural Hardware Cost Function
//   - Parameter exploration: tile_sizes, unrolls, vectorize_widths
//   - Diagnostic rejection of invalid autotune configurations [E0012]
//   - C backend schedule emission with automated pragma optimization
// ============================================================================

use cronc::autotune::SiliconAutotuner;
use cronc::lexer::Lexer;
use cronc::parser::Parser;
use cronc::checker::SemanticChecker;
use cronc::c_backend::CBackend;
use cronc::codegen::Codegen;

#[test]
fn test_silicon_autotuner_cost_model_selection() {
    let tile_sizes = vec![(4, 4), (8, 8), (16, 16)];
    let unrolls = vec![1, 2, 4, 8];
    let vectorize_widths = vec![1, 2, 4, 8];

    let decision_latency = SiliconAutotuner::select_optimal_schedule(
        &tile_sizes,
        &unrolls,
        &vectorize_widths,
        "min_latency",
    );

    assert_eq!(decision_latency.evaluated_candidates, 3 * 4 * 4);
    assert!(decision_latency.best_vectorize >= 4, "Optimal vectorize should leverage SIMD lanes");
    assert!(decision_latency.latency_cycles > 0.0);

    let decision_throughput = SiliconAutotuner::select_optimal_schedule(
        &tile_sizes,
        &unrolls,
        &vectorize_widths,
        "max_throughput",
    );
    assert_eq!(decision_throughput.evaluated_candidates, 48);
}

#[test]
fn test_autotune_schedule_syntax_and_semantic_checking() {
    let source = r#"
def systolic_gemm(a: tensor<1, 16, 16, f32>, b: tensor<1, 16, 16, f32>) -> tensor<1, 16, 16, f32> {
    return tensor_matmul(a, b);
}

schedule systolic_gemm for "torus_4d" {
    autotune {
        tile_size: [(4, 4), (8, 8)];
        unroll: [2, 4];
        vectorize: [4, 8];
        metric: "min_latency";
    }
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Failed to parse autotune schedule");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_ok(), "Expected valid autotune schedule to pass, got: {:?}", result.err());
}

#[test]
fn test_autotune_invalid_metric_rejected_e0012() {
    let source = r#"
def systolic_gemm(a: tensor<1, 16, 16, f32>, b: tensor<1, 16, 16, f32>) -> tensor<1, 16, 16, f32> {
    return tensor_matmul(a, b);
}

schedule systolic_gemm for "torus_4d" {
    autotune {
        tile_size: [(4, 4)];
        unroll: [2];
        vectorize: [4];
        metric: "invalid_super_speed";
    }
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parse failed");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Expected invalid metric to be rejected");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0012");
    assert!(err.message.contains("Unknown autotune optimization metric"));
}

#[test]
fn test_autotune_non_power_of_two_vectorize_rejected_e0012() {
    let source = r#"
def systolic_gemm(a: tensor<1, 16, 16, f32>, b: tensor<1, 16, 16, f32>) -> tensor<1, 16, 16, f32> {
    return tensor_matmul(a, b);
}

schedule systolic_gemm for "torus_4d" {
    autotune {
        tile_size: [(4, 4)];
        unroll: [2];
        vectorize: [3];
        metric: "min_latency";
    }
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parse failed");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Expected non-power-of-two vectorize width to be rejected");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0012");
    assert!(err.message.contains("must be a power of two"));
}

#[test]
fn test_autotune_c_backend_emission() {
    let source = r#"
def systolic_kernel(a: tensor<1, 8, 8, f32>, b: tensor<1, 8, 8, f32>) -> tensor<1, 8, 8, f32> {
    return tensor_matmul(a, b);
}

schedule systolic_kernel for "torus_4d" {
    autotune {
        tile_size: [(4, 4), (8, 8)];
        unroll: [2, 4];
        vectorize: [4];
        metric: "min_latency";
    }
}

def main() -> int {
    let x: tensor<1, 8, 8, f32> = tensor_init(1.0);
    let y: tensor<1, 8, 8, f32> = tensor_init(2.0);
    let z = systolic_kernel(x, y);
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parse failed");
    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check failed");

    let mut backend = CBackend::new();
    let c_code = backend.generate(&program);

    assert!(c_code.contains("// [Schedule Directive] Autotune: metric=\"min_latency\""), "C code must include autotune header");
    assert!(c_code.contains("// [Autotune Decision] Optimal TileSize:"), "C code must log Pareto-optimal decision");
    assert!(c_code.contains("#pragma GCC optimize (\"unroll-loops\")"));
    assert!(c_code.contains("#pragma GCC optimize (\"tree-vectorize\")"));
}

#[test]
fn test_autotune_vliw_instruction_emission() {
    let source = r#"
def main() -> int {
    let a: tensor<1, 4, 4, f32> = tensor_init(1.0);
    let b: tensor<1, 4, 4, f32> = tensor_init(2.0);
    let c = tensor_matmul(a, b);
    let d = tensor_transpose(c);
    let e = tensor_add(c, d);
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parse failed");
    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check failed");

    let mut codegen = Codegen::new();
    let assembly = codegen.generate(&program);

    assert!(assembly.contains("_OP"), "Codegen should emit _OP (Optical GEMM) for tensor_matmul");
    assert!(assembly.contains("_TT"), "Codegen should emit _TT (Tensor Tile Swizzle) for tensor_transpose");
    assert!(assembly.contains("_PO"), "Codegen should emit _PO for tensor_add");
}
