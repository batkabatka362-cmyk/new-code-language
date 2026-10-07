use cronc::docgen::*;
use cronc::lexer::Lexer;
use cronc::parser::Parser;

#[test]
fn test_docgen_ast_extraction_and_markdown() {
    let source = r#"
    .MODULE cron.optical

    brain OpticalCortex {
        let x = 1
    }
    
    struct MziInterferometer {
        theta: i64,
        phi: i64,
    }

    impl MziInterferometer {
        fn rotate_phase(delta: i64) -> i64 {
            delta + 1
        }
    }

    fn calculate_superposition(amp: i64) -> i64 {
        amp * 2
    }
    "#;

    let tokens = Lexer::new(source).tokenize().expect("Lexing failed");
    let program = Parser::new(tokens).parse_program().expect("Parsing failed");

    let doc = extract_program_doc("cron.optical", &program);

    assert_eq!(doc.module_name, "cron.optical");
    assert_eq!(doc.structs.len(), 1);
    assert_eq!(doc.structs[0].name, "MziInterferometer");
    assert_eq!(doc.structs[0].fields.len(), 2);
    assert_eq!(doc.structs[0].methods.len(), 1);
    assert_eq!(doc.functions.len(), 1);
    assert_eq!(doc.functions[0].name, "calculate_superposition");
    assert_eq!(doc.brains.len(), 1);

    let md = doc.to_markdown();
    assert!(md.contains("# Module `cron.optical`"));
    assert!(md.contains("### `struct MziInterferometer`"));
    assert!(md.contains("| `theta` |"));
    assert!(md.contains("### `fn calculate_superposition(amp:"));
    assert!(md.contains("## 🧠 Cognitive Brain Partitions"));
}

const SAGI_SUPERINTELLIGENCE_E2E_SRC: &str = r#"
.MODULE cron.examples.sagi_e2e

fn evaluate_pillar(x: i64) -> i64 {
    let result = x * 2;
    return result;
}

fn main() -> i64 {
    let bcm_syn = evaluate_pillar(100);
    return bcm_syn;
}
"#;

#[test]
fn test_docgen_on_sagi_example() {
    let content = SAGI_SUPERINTELLIGENCE_E2E_SRC;

    let tokens = Lexer::new(content).tokenize().expect("Lexing failed");
    let program = Parser::new(tokens).parse_program().expect("Parsing failed");

    let doc = extract_program_doc("cron.examples.sagi_e2e", &program);
    assert_eq!(doc.module_name, "cron.examples.sagi_e2e");
    assert!(!doc.functions.is_empty());

    let md = doc.to_markdown();
    assert!(md.contains("# Module `cron.examples.sagi_e2e`"));
}
