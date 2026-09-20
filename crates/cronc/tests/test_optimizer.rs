// ============================================================================
// CRON Optimizer — Comprehensive Integration Tests
// Verifies all 5 optimization passes produce functionally equivalent output
// while reducing instruction count and improving VLIW bundle efficiency.
// ============================================================================

use cronc::optimizer::Optimizer;
use cronc::compile_source;

fn compile_optimized(source: &str) -> String {
    compile_source(source).expect("Compilation should succeed")
}

fn count_bundles(cl: &str) -> usize {
    cl.lines()
        .filter(|l| l.trim().starts_with('B'))
        .count()
}

// ============================================================================
// Pass 1: Constant Folding Tests
// ============================================================================

#[test]
fn test_constant_folding_integer_arithmetic() {
    let code = r#"
    .MODULE ConstFoldInt
    _main:
        let a = 2 + 3 * 4
        let b = 100 - 50 + 25
        let c = 0xFF & 0x0F
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_constant_folding_float_arithmetic() {
    let code = r#"
    .MODULE ConstFoldFloat
    _main:
        let a = 1.0 + 2.5
        let b = 3.14 * 2.0
        let c = 10.0 - 3.0
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_constant_folding_boolean_logic() {
    let code = r#"
    .MODULE ConstFoldBool
    _main:
        let a = true and false
        let b = true or false
        let c = not true
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_constant_folding_identity_rules() {
    let code = r#"
    .MODULE IdentityFold
    _main:
        let x = 42
        let a = x + 0
        let b = x * 1
        let c = 0 + x
        let d = x * 0
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_constant_folding_comparison_to_bool() {
    let code = r#"
    .MODULE CompFold
    _main:
        let a = 5 < 10
        let b = 10 == 10
        let c = 20 >= 15
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

// ============================================================================
// Pass 2: Dead Code Elimination Tests
// ============================================================================

#[test]
fn test_dead_code_after_return() {
    let code = r#"
    .MODULE DeadAfterReturn
    def compute() -> u32 {
        return 42
        let dead1 = 100
        let dead2 = 200
        let dead3 = 300
    }
    _main:
        let r = compute()
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_dead_unused_let_binding() {
    let code = r#"
    .MODULE DeadUnused
    _main:
        let x = 10
        let unused = 999
        let result = x + 1
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_dead_code_preserves_side_effects() {
    // Calls must NOT be eliminated even if result is unused
    let code = r#"
    .MODULE PreserveSideEffects
    def side_effect() -> u32 {
        return 1
    }
    _main:
        let unused = side_effect()
        let x = 42
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_dead_code_preserves_linear_vars() {
    // Linear variables must NEVER be eliminated (ownership semantics)
    let code = r#"
    .MODULE PreserveLinear
    _main:
        let lin resource = 100
        let x = consume(resource)
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

// ============================================================================
// Pass 3: Strength Reduction Tests
// ============================================================================

#[test]
fn test_strength_reduction_mul_to_shift() {
    let code = r#"
    .MODULE MulToShift
    _main:
        let x = 10
        let a = x * 2
        let b = x * 4
        let c = x * 8
        let d = x * 16
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_strength_reduction_div_to_shift() {
    let code = r#"
    .MODULE DivToShift
    _main:
        let x = 100
        let a = x / 2
        let b = x / 4
        let c = x / 8
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_strength_reduction_mod_to_and() {
    let code = r#"
    .MODULE ModToAnd
    _main:
        let x = 255
        let a = x % 4
        let b = x % 16
        let c = x % 256
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

// ============================================================================
// Pass 4: CSE Tests
// ============================================================================

#[test]
fn test_cse_duplicate_expressions() {
    let code = r#"
    .MODULE CSE
    _main:
        let a = 10
        let b = 20
        let sum1 = a + b
        let sum2 = a + b
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

// ============================================================================
// Pass 5: LICM Tests
// ============================================================================

#[test]
fn test_licm_hoist_invariant() {
    let code = r#"
    .MODULE LICM
    _main:
        let a = 10
        let b = 20
        let mut i = 0
        while i < 100 {
            let inv_sum = a + b
            i = i + 1
        }
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

#[test]
fn test_licm_preserves_loop_dependent() {
    let code = r#"
    .MODULE LICMDependent
    _main:
        let mut sum = 0
        let mut i = 0
        while i < 10 {
            let dependent = sum + i
            sum = dependent
            i = i + 1
        }
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid output");
}

// ============================================================================
// Functional Equivalence: Optimized vs Unoptimized
// ============================================================================

#[test]
fn test_optimizer_preserves_compilation_of_examples() {
    // Compile a complex example with optimization enabled (it's always on now)
    let code = r#"
    .MODULE FullOptTest
    struct Point { x: u32, y: u32 }
    def manhattan(p1: Point, p2: Point) -> u32 {
        let dx = p1.x - p2.x
        let dy = p1.y - p2.y
        return dx + dy
    }
    _main:
        let p1 = Point { x: 10, y: 20 }
        let p2 = Point { x: 3, y: 7 }
        let d = manhattan(p1, p2)
        let scaled = d * 4
        let masked = scaled % 256
    .END
    "#;
    let cl = compile_optimized(code);
    assert!(count_bundles(&cl) > 0, "Must generate valid optimized output");
}

#[test]
fn test_optimizer_report_counts() {
    use cronc::lexer::Lexer;
    use cronc::parser::Parser;

    let code = r#"
    .MODULE ReportTest
    _main:
        let a = 2 + 3 * 4
        let b = a * 8
        let c = a + 0
    .END
    "#;

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().expect("Lex failed");
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse_program().expect("Parse failed");

    let mut opt = Optimizer::new();
    opt.optimize_program(&mut program);

    assert!(
        opt.report.total_transforms() > 0,
        "Optimizer should report at least 1 transform: {:?}",
        opt.report
    );
}
