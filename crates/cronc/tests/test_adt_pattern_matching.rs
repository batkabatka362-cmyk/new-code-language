// ============================================================================
// Milestone #050: ADT & Exhaustive Pattern Matching — Test Suite
// Tests enum declarations, pattern matching (variant, tuple, or, wildcard,
// literal, guard), and compile-time exhaustiveness verification.
// ============================================================================

use cronc::ast::*;
use cronc::checker::SemanticChecker;
use cronc::token::Span;

fn span() -> Span {
    Span::new(1, 1, 0, 1)
}

/// Helper: build a minimal Program with enums and functions
fn make_program(enums: Vec<EnumDecl>, functions: Vec<FunctionDecl>) -> Program {
    Program {
        module_name: "test".to_string(),
        entry_name: None,
        imports: vec![],
        type_aliases: vec![],
        structs: vec![],
        enums,
        traits: vec![],
        impls: vec![],
        functions,
        schedules: vec![],
        brains: vec![],
        main_statements: vec![],
    }
}

/// Helper: create a test FunctionDecl
fn make_fn(name: &str, params: Vec<Param>, body: Vec<Statement>) -> FunctionDecl {
    FunctionDecl {
        is_async: false,
        is_export: false,
        is_inline: false,
        name: name.to_string(),
        generic_params: vec![],
        params,
        return_type: None,
        body,
    }
}

/// Helper: create an Option-like enum with Some(x) and None
fn option_enum() -> EnumDecl {
    EnumDecl {
        name: "Option".to_string(),
        generic_params: vec![],
        variants: vec![
            EnumVariant { name: "Some".to_string(), payload: Some(vec!["T".to_string()]) },
            EnumVariant { name: "None".to_string(), payload: None },
        ],
    }
}

/// Helper: create a Result enum with Ok(v) and Err(e)
fn result_enum() -> EnumDecl {
    EnumDecl {
        name: "Result".to_string(),
        generic_params: vec![],
        variants: vec![
            EnumVariant { name: "Ok".to_string(), payload: Some(vec!["T".to_string()]) },
            EnumVariant { name: "Err".to_string(), payload: Some(vec!["E".to_string()]) },
        ],
    }
}

/// Helper: create a Color enum with Red, Green, Blue
fn color_enum() -> EnumDecl {
    EnumDecl {
        name: "Color".to_string(),
        generic_params: vec![],
        variants: vec![
            EnumVariant { name: "Red".to_string(), payload: None },
            EnumVariant { name: "Green".to_string(), payload: None },
            EnumVariant { name: "Blue".to_string(), payload: None },
        ],
    }
}

// ─── Test 1: Enum registration populates known_enums ───
#[test]
fn test_enum_registration() {
    let prog = make_program(vec![option_enum(), color_enum()], vec![]);
    let mut checker = SemanticChecker::new();
    let _ = checker.check_program(&prog);

    assert!(checker.known_enums.contains_key("Option"), "Option enum should be registered");
    assert!(checker.known_enums.contains_key("Color"), "Color enum should be registered");

    let option_variants = &checker.known_enums["Option"];
    assert_eq!(option_variants.len(), 2);
    assert!(option_variants.contains(&"Some".to_string()));
    assert!(option_variants.contains(&"None".to_string()));

    let color_variants = &checker.known_enums["Color"];
    assert_eq!(color_variants.len(), 3);
    assert!(color_variants.contains(&"Red".to_string()));
    assert!(color_variants.contains(&"Green".to_string()));
    assert!(color_variants.contains(&"Blue".to_string()));
}

// ─── Test 2: Exhaustive match passes ───
#[test]
fn test_exhaustive_match_passes() {
    // match val { Some(x) => ..., None => ... }
    let prog = make_program(vec![option_enum()], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "val".to_string(),
                param_type: "Option".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Match {
                    expr: Expr::Ident("val".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Option".to_string()),
                                variant_name: "Some".to_string(),
                                bindings: vec!["x".to_string()],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Option".to_string()),
                                variant_name: "None".to_string(),
                                bindings: vec![],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_ok(), "Exhaustive match should pass: {:?}", result.err());
}

// ─── Test 3: Non-exhaustive match fails with E0020 ───
#[test]
fn test_non_exhaustive_match_fails() {
    // match val { Some(x) => ... } — missing None
    let prog = make_program(vec![option_enum()], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "val".to_string(),
                param_type: "Option".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Match {
                    expr: Expr::Ident("val".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Option".to_string()),
                                variant_name: "Some".to_string(),
                                bindings: vec!["x".to_string()],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_err(), "Non-exhaustive match should fail");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0020");
    assert!(err.message.contains("None"), "Error should mention missing variant 'None': {}", err.message);
}

// ─── Test 4: Wildcard pattern makes match exhaustive ───
#[test]
fn test_wildcard_makes_exhaustive() {
    // match val { Some(x) => ..., _ => ... }
    let prog = make_program(vec![option_enum()], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "val".to_string(),
                param_type: "Option".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Match {
                    expr: Expr::Ident("val".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Option".to_string()),
                                variant_name: "Some".to_string(),
                                bindings: vec!["x".to_string()],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Wildcard,
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_ok(), "Wildcard should satisfy exhaustiveness: {:?}", result.err());
}

// ─── Test 5: Or-pattern covers multiple variants ───
#[test]
fn test_or_pattern_coverage() {
    // match color { Red | Green => ..., Blue => ... }
    let prog = make_program(vec![color_enum()], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "c".to_string(),
                param_type: "Color".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Match {
                    expr: Expr::Ident("c".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Or(vec![
                                MatchPattern::Variant {
                                    enum_name: Some("Color".to_string()),
                                    variant_name: "Red".to_string(),
                                    bindings: vec![],
                                },
                                MatchPattern::Variant {
                                    enum_name: Some("Color".to_string()),
                                    variant_name: "Green".to_string(),
                                    bindings: vec![],
                                },
                            ]),
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Color".to_string()),
                                variant_name: "Blue".to_string(),
                                bindings: vec![],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_ok(), "Or-pattern should cover Red|Green: {:?}", result.err());
}

// ─── Test 6: Three-variant enum missing one variant ───
#[test]
fn test_three_variant_missing_one() {
    // match c { Red => ..., Green => ... } — missing Blue
    let prog = make_program(vec![color_enum()], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "c".to_string(),
                param_type: "Color".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Match {
                    expr: Expr::Ident("c".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Color".to_string()),
                                variant_name: "Red".to_string(),
                                bindings: vec![],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Color".to_string()),
                                variant_name: "Green".to_string(),
                                bindings: vec![],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_err(), "Missing 'Blue' variant should fail");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0020");
    assert!(err.message.contains("Blue"), "Error should mention 'Blue': {}", err.message);
}

// ─── Test 7: Pattern guard is validated ───
#[test]
fn test_pattern_guard_checked() {
    // match val { Some(x) if x > 0 => ..., _ => ... }
    // Guard expressions should be checked without error for valid vars
    let prog = make_program(vec![option_enum()], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "val".to_string(),
                param_type: "Option".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Match {
                    expr: Expr::Ident("val".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Option".to_string()),
                                variant_name: "Some".to_string(),
                                bindings: vec!["x".to_string()],
                            },
                            guard: Some(Expr::Binary {
                                op: ">".to_string(),
                                left: Box::new(Expr::Ident("x".to_string(), span())),
                                right: Box::new(Expr::LiteralInt(0)),
                            }),
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Wildcard,
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_ok(), "Guard with bound variable should pass: {:?}", result.err());
}

// ─── Test 8: Literal pattern match passes ───
#[test]
fn test_literal_pattern_match() {
    // match x { 0 => ..., 1 => ..., _ => ... }
    let prog = make_program(vec![], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "x".to_string(),
                param_type: "i32".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Match {
                    expr: Expr::Ident("x".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Literal(Expr::LiteralInt(0)),
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Literal(Expr::LiteralInt(1)),
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Wildcard,
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_ok(), "Literal pattern match should pass: {:?}", result.err());
}

// ─── Test 9: Result enum exhaustive match ───
#[test]
fn test_result_exhaustive() {
    let prog = make_program(vec![result_enum()], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "r".to_string(),
                param_type: "Result".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Match {
                    expr: Expr::Ident("r".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Result".to_string()),
                                variant_name: "Ok".to_string(),
                                bindings: vec!["v".to_string()],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Result".to_string()),
                                variant_name: "Err".to_string(),
                                bindings: vec!["e".to_string()],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_ok(), "Result exhaustive match should pass: {:?}", result.err());
}

// ─── Test 10: Multiple enums in same program ───
#[test]
fn test_multiple_enums_registration() {
    let prog = make_program(
        vec![option_enum(), result_enum(), color_enum()],
        vec![],
    );
    let mut checker = SemanticChecker::new();
    let _ = checker.check_program(&prog);

    assert_eq!(checker.known_enums.len(), 3);
    assert!(checker.known_enums.contains_key("Option"));
    assert!(checker.known_enums.contains_key("Result"));
    assert!(checker.known_enums.contains_key("Color"));
}

// ─── Test 11: Reachability check rejects pattern after wildcard ───
#[test]
fn test_unreachable_pattern_after_wildcard_fails() {
    let prog = make_program(vec![option_enum()], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "val".to_string(),
                param_type: "Option".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Match {
                    expr: Expr::Ident("val".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Wildcard,
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Option".to_string()),
                                variant_name: "Some".to_string(),
                                bindings: vec!["x".to_string()],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_err(), "Pattern after wildcard must fail reachability check");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0021");
}

// ─── Test 12: Wildcard with guard allows subsequent arms ───
#[test]
fn test_wildcard_with_guard_allows_subsequent_arms() {
    let prog = make_program(vec![option_enum()], vec![
        make_fn(
            "test_fn",
            vec![
                Param {
                    name: "val".to_string(),
                    param_type: "Option".to_string(),
                    is_lin: false,
                    is_grad: false,
                    span: span(),
                },
                Param {
                    name: "cond".to_string(),
                    param_type: "bool".to_string(),
                    is_lin: false,
                    is_grad: false,
                    span: span(),
                },
            ],
            vec![
                Statement::Match {
                    expr: Expr::Ident("val".to_string(), span()),
                    arms: vec![
                        MatchArm {
                            pattern: MatchPattern::Wildcard,
                            guard: Some(Expr::Ident("cond".to_string(), span())),
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Option".to_string()),
                                variant_name: "Some".to_string(),
                                bindings: vec!["x".to_string()],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                        MatchArm {
                            pattern: MatchPattern::Variant {
                                enum_name: Some("Option".to_string()),
                                variant_name: "None".to_string(),
                                bindings: vec![],
                            },
                            guard: None,
                            body: vec![],
                            span: span(),
                        },
                    ],
                    span: span(),
                },
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_ok(), "Wildcard with guard should not block subsequent arms: {:?}", result.err());
}

// ─── Test 13: Unknown enum variant rejected ───
#[test]
fn test_unknown_variant_rejected() {
    let prog = make_program(vec![option_enum()], vec![
        make_fn(
            "test_fn",
            vec![Param {
                name: "val".to_string(),
                param_type: "Option".to_string(),
                is_lin: false,
                is_grad: false,
                span: span(),
            }],
            vec![
                Statement::Expr(Expr::Ident("Option::BogusVariant".to_string(), span())),
            ],
        ),
    ]);

    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&prog);
    assert!(result.is_err(), "Unknown variant must fail");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0022");
}

// ─── Test 14: End-to-end source parsing with enums, or-patterns, guards ───
#[test]
fn test_parser_and_check_adt_program() {
    let source = r#"
enum Status {
    Idle,
    Running(i64),
    Failed
}

def handle(s: Status) -> i64 {
    match s {
        Status::Running(val) if val > 100 => val * 2,
        Status::Running(val) => val,
        Status::Idle | Status::Failed => 0,
    }
}
"#;
    let mut lexer = cronc::lexer::Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = cronc::parser::Parser::new(tokens);
    let prog = parser.parse_program().expect("Parse failed");
    let mut checker = SemanticChecker::new();
    let res = checker.check_program(&prog);
    assert!(res.is_ok(), "Program should check cleanly: {:?}", res.err());
}

// ─── Test 15: Match expression in let statement ───
#[test]
fn test_match_expression_in_let() {
    let source = r#"
enum Command {
    Start,
    Stop
}

def eval_cmd(c: Command) -> i64 {
    let code: i64 = match c {
        Command::Start => 1,
        Command::Stop => 0,
    };
    return code;
}
"#;
    let mut lexer = cronc::lexer::Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = cronc::parser::Parser::new(tokens);
    let prog = parser.parse_program().expect("Parse failed");
    let mut checker = SemanticChecker::new();
    let res = checker.check_program(&prog);
    assert!(res.is_ok(), "Match expression should check cleanly: {:?}", res.err());
}

// ─── Test 16: Codegen produces valid VLIW bundles with _SW branch slots ───
#[test]
fn test_codegen_produces_branch_slots() {
    let source = r#"
enum State { Active, Suspended }

def check_state(s: State) -> i64 {
    match s {
        State::Active => 1,
        State::Suspended => 0,
    }
}
"#;
    let cl_code = cronc::compile_source(source).expect("Compilation failed");
    assert!(cl_code.contains("_SW"), "Compiled .cl machine code should contain _SW condition slot: {}", cl_code);
}

