// ============================================================================
// CRON Test Suite: Flagship Sovereign AGI & Wafer-Scale DeepSeek-R1 Models
// (C) 2026 CRON Language Project - 100% Industrial Verification
// ============================================================================

use cronc::checker::SemanticChecker;
use cronc::lexer::Lexer;
use cronc::parser::Parser;
use std::fs;
use std::path::PathBuf;

fn find_file(rel: &str) -> PathBuf {
    let candidates = [
        PathBuf::from(rel),
        PathBuf::from("../../").join(rel),
        PathBuf::from("../").join(rel),
    ];
    for c in &candidates {
        if c.exists() {
            return c.clone();
        }
    }
    panic!("Could not locate file: {}", rel);
}

#[test]
fn test_deepseek_r1_cognitive_foundation_compilation_and_checking() {
    let actual_path = find_file("examples/deepseek_r1_cognitive_foundation.cr");
    let source = fs::read_to_string(&actual_path)
        .expect("deepseek_r1_cognitive_foundation.cr must exist");
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().expect("Parsing deepseek_r1_cognitive_foundation.cr failed");
    
    let mut checker = SemanticChecker::new();
    let checked = checker.check_program(&ast);
    assert!(checked.is_ok(), "Checking deepseek_r1_cognitive_foundation.cr failed: {:?}", checked.err());
}

#[test]
fn test_sagi_wafer_deepseek_r1_self_evolution_compilation() {
    let actual_path = find_file("examples/sagi_wafer_deepseek_r1_self_evolution.cr");
    let source = fs::read_to_string(&actual_path)
        .expect("sagi_wafer_deepseek_r1_self_evolution.cr must exist");
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().expect("Parsing sagi_wafer_deepseek_r1_self_evolution.cr failed");
    
    let mut checker = SemanticChecker::new();
    let checked = checker.check_program(&ast);
    assert!(checked.is_ok(), "Checking sagi_wafer_deepseek_r1_self_evolution.cr failed: {:?}", checked.err());
}

#[test]
fn test_sagi_omni_sensory_r1_cortex_compilation() {
    let actual_path = find_file("examples/sagi_omni_sensory_r1_cortex.cr");
    let source = fs::read_to_string(&actual_path)
        .expect("sagi_omni_sensory_r1_cortex.cr must exist");
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().expect("Parsing sagi_omni_sensory_r1_cortex.cr failed");
    
    let mut checker = SemanticChecker::new();
    let checked = checker.check_program(&ast);
    assert!(checked.is_ok(), "Checking sagi_omni_sensory_r1_cortex.cr failed: {:?}", checked.err());
}
