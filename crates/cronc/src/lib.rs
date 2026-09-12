pub mod ast;
pub mod checker;
pub mod cl_lang;
pub mod codegen;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod token;

use checker::SemanticChecker;
use codegen::Codegen;
use diagnostic::Diagnostic;
use lexer::Lexer;
use parser::Parser;

pub fn compile_source(source: &str) -> Result<String, String> {
    compile_source_with_name(source, None)
}

pub fn compile_source_with_name(source: &str, file_label: Option<&str>) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Check EBNF grammar syntax rules in docs/cron_spec.md");
            return Err(diag.render(file_label));
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            let diag = Diagnostic::new("E0001", &e, 1, 1)
                .with_source(source)
                .with_help("Verify module declaration, matching braces and parentheses");
            return Err(diag.render(file_label));
        }
    };

    let mut checker = SemanticChecker::new();
    if let Err(type_err) = checker.check_program(&program) {
        let span = type_err.span.unwrap_or(crate::token::Span::point(1, 1, 0));
        let mut diag = Diagnostic::new(type_err.code, &type_err.message, span.line, span.col)
            .with_span(span.len)
            .with_source(source);
        if let Some(note) = type_err.note {
            diag = diag.with_note(note);
        }
        if let Some(help) = type_err.help {
            diag = diag.with_help(help);
        }
        return Err(diag.render(file_label));
    }

    let mut codegen = Codegen::new();
    let machine_code = codegen.generate(&program);

    Ok(machine_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_and_parser() {
        let code = r#"
        .MODULE TestMod
        .ENTRY _main
        _main:
            let ext_hbm_base: u32 = 0x1000
            let lin w_seed: wave_t = pack_wave(amp=[1, 2], phase=[3, 4])
            export w_seed as exported_w
        .END
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().expect("Tokenize failed");
        let mut parser = Parser::new(tokens);
        let prog = parser.parse_program().expect("Parse failed");
        assert_eq!(prog.module_name, "TestMod");
        assert_eq!(prog.entry_name, Some("_main".to_string()));
    }

    #[test]
    fn test_linear_type_leak_detected() {
        let code = r#"
        .MODULE LeakMod
        _main:
            let lin unconsumed_var = 123
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Linear variable 'unconsumed_var' was allocated but never consumed"));
        assert!(err.contains("error[E0002]"));
        assert!(err.contains("--> <input>:4:21"));
    }

    #[test]
    fn test_linear_type_double_consume_detected() {
        let code = r#"
        .MODULE DoubleConsumeMod
        _main:
            let lin x = 123
            let a = consume(x)
            let b = consume(x)
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("was already consumed"));
    }

    #[test]
    fn test_operator_precedence_and_expressions() {
        let code = r#"
        .MODULE ExprMod
        _main:
            let a = 2 + 3 * 4
            let b = (2 + 3) * 4
            let c = a > 10 and b <= 20
            let d = not c or false
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile expressions: {:?}", result.err());
    }

    #[test]
    fn test_control_flow_if_while_for() {
        let code = r#"
        .MODULE ControlFlowMod
        _main:
            let mut x = 10
            if x > 5 {
                let y = x + 1
            } else {
                let y = 0
            }

            while x > 0 {
                x = x - 1
            }

            for item in [1, 2, 3] {
                let z = item * 2
            }
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile control flow: {:?}", result.err());
    }

    #[test]
    fn test_reassign_immutable_fails() {
        let code = r#"
        .MODULE ImmMod
        _main:
            let x = 10
            x = 20
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("error[E0005]"));
        assert!(err.contains("Cannot assign twice to immutable variable 'x'"));
        assert!(err.contains("--> <input>:5:13"));
    }

    #[test]
    fn test_assign_to_consumed_linear_rejected() {
        let code = r#"
        .MODULE InvalidAssignMod
        _main:
            let mut lin res = 50
            let x = consume(res)
            res = 100
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_err(), "Expected assigning to consumed linear variable to fail");
        assert!(result.unwrap_err().contains("was already consumed"));
    }

    #[test]
    fn test_struct_declaration_and_init() {
        let code = r#"
        .MODULE StructTest
        struct Waveform {
            amp: u32,
            phase: u32,
        }
        _main:
            let w = Waveform { amp: 10, phase: 20 }
            let a = w.amp
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile struct init: {:?}", result.err());
    }

    #[test]
    fn test_import_and_type_alias() {
        let code = r#"
        .MODULE ImportTest
        import { SpatialTensor, RoutingMatrix } from "core/spatial.cr"
        type Angle = f64
        type Matrix4x4 = [[f64; 4]; 4]
        _main:
            let theta: Angle = 3.14159
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile import and type alias: {:?}", result.err());
    }

    #[test]
    fn test_nested_if_else_chains() {
        let code = r#"
        .MODULE NestedIfMod
        _main:
            let x = 15
            let mut result = 0
            if x > 20 {
                result = 1
            } else if x > 10 {
                result = 2
            } else {
                result = 3
            }
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile nested if: {:?}", result.err());
    }

    #[test]
    fn test_function_multi_params() {
        let code = r#"
        .MODULE FuncMod
        def compute_energy(freq: f64, lin photon: u32, h_bar: f64) -> f64 {
            let p = consume(photon)
            return freq * h_bar
        }
        _main:
            let lin p = 42
            let e = compute_energy(1.5e14, consume(p), 6.626e-34)
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile multi-param function: {:?}", result.err());
    }

    #[test]
    fn test_inline_function_expansion() {
        let code = r#"
        .MODULE InlineMod
        inline def fast_dot(a: u32, b: u32) -> u32 {
            let res = a * b
            return res
        }
        _main:
            let x = fast_dot(3, 4)
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile inline function: {:?}", result.err());
    }

    #[test]
    fn test_bitwise_operators() {
        let code = r#"
        .MODULE BitwiseMod
        _main:
            let a = 0x0F & 0x33
            let b = 0x0F | 0x30
            let c = 0xAA ^ 0x55
            let d = a << 2
            let e = b >> 1
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile bitwise ops: {:?}", result.err());
    }

    #[test]
    fn test_comparison_all_operators() {
        let code = r#"
        .MODULE CmpMod
        _main:
            let a = 10 < 20
            let b = 10 <= 10
            let c = 20 > 10
            let d = 20 >= 20
            let e = 10 == 10
            let f = 10 != 20
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile comparisons: {:?}", result.err());
    }

    #[test]
    fn test_type_cast_expression() {
        let code = r#"
        .MODULE CastMod
        _main:
            let raw: u64 = 0x1234
            let truncated = raw as u32
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile cast: {:?}", result.err());
    }

    #[test]
    fn test_unary_negation_and_inversion() {
        let code = r#"
        .MODULE UnaryMod
        _main:
            let a = -42
            let b = -3.14
            let c = not true
            let d = !false
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile unary: {:?}", result.err());
    }

    #[test]
    fn test_array_repeat_and_empty_tuple() {
        let code = r#"
        .MODULE ArrayTupleMod
        _main:
            let empty_unit = ()
            let zeros = [0; 256]
            let pair = (1, 2)
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile array repeat and tuple: {:?}", result.err());
    }

    #[test]
    fn test_scientific_notation_literals() {
        let code = r#"
        .MODULE SciMod
        _main:
            let a = 1e5
            let b = 1.0e-5
            let c = 2.5e+10
            let d = 3E4
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile scientific notation: {:?}", result.err());
    }

    #[test]
    fn test_comments_and_blank_lines() {
        let code = r#"
        // Line comment 1
        # Python-style comment

        /* Multi-line
           block comment */

        .MODULE CommentMod

        // Comment before entry
        _main:
            ; Semicolon start comment
            let x = 100 // trailing comment
            /* inline block */ let y = 200

        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile with various comments: {:?}", result.err());
    }

    #[test]
    fn test_hex_literals_with_underscores() {
        let code = r#"
        .MODULE HexMod
        _main:
            let addr = 0x000A_0000
            let mask = 0xFFFF_0000
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile hex with underscores: {:?}", result.err());
    }

    #[test]
    fn test_await_and_spawn_expressions() {
        let code = r#"
        .MODULE AsyncMod
        _main:
            let handle = spawn (1 + 2)
            let result = await handle
        .END
        "#;
        let result = compile_source(code);
        assert!(result.is_ok(), "Failed to compile async expressions: {:?}", result.err());
    }
}
