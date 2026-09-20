use cronc::formal_verify::FormalVerifier;
use cronc::lexer::Lexer;
use cronc::parser::Parser;

#[test]
fn test_formal_deadlock_free_and_landauer_audit() {
    let source = r#"
    .MODULE FormalSafeMod
    .ENTRY main

    schedule main for "256-Core-4D-Torus" {
        distribute_4d(axis: X+, cores: 4);
        distribute_4d(axis: Y+, cores: 4);
        distribute_4d(axis: Z+, cores: 4);
        distribute_4d(axis: W+, cores: 4);
    }

    def main() -> i64 {
        let mut x: i64 = 10;
        x = 20;
        x = 30;
        return x;
    }
    .END
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parse failed");

    let verifier = FormalVerifier::new(300.0, 1.0e9);
    let report = verifier.verify_program(&program);

    assert!(report.dor_deadlock_free, "Strict monotonic DOR (X+ -> Y+ -> Z+ -> W+) must be deadlock-free");
    assert!(report.is_provably_safe, "Valid program must be provably safe");
    assert!(report.total_bits_erased > 0, "Variable reassignments must register bit-erasure entropy");
    assert!(report.landauer_power_microwatts > 0.0, "Active operations must have non-zero Landauer dissipation");
    assert!(report.thermal_headroom_pct > 90.0, "Operations must remain comfortably within 350W TDP envelope");
}

#[test]
fn test_dor_routing_cycle_deadlock_detection() {
    let source = r#"
    .MODULE InvertedRoutingMod
    .ENTRY main

    schedule main for "256-Core-4D-Torus" {
        distribute_4d(axis: W+, cores: 4);
        distribute_4d(axis: X+, cores: 4); // Inversion! High rank W (3) -> Low rank X (0)
    }

    def main() -> i64 {
        return 0;
    }
    .END
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parse failed");

    let verifier = FormalVerifier::new(300.0, 1.0e9);
    let report = verifier.verify_program(&program);

    assert!(!report.dor_deadlock_free, "Routing inversion (W+ -> X+) must trigger deadlock violation");
    assert!(!report.is_provably_safe, "Program with routing cycle must fail safety proof");
    assert!(!report.violations.is_empty(), "Violation messages must be generated");
}

#[test]
fn test_pgas_bank_collision_detection() {
    let source = r#"
    .MODULE BankCollisionMod
    .ENTRY main

    def main() -> i64 {
        // Core only has 16 SRAM banks: bank=0..15. Bank 99 is invalid and must be rejected
        let bad_bank_buffer: @sram(bank=99) i32 = 0;
        return 0;
    }
    .END
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parse failed");

    let verifier = FormalVerifier::new(300.0, 1.0e9);
    let report = verifier.verify_program(&program);

    assert!(!report.pgas_bank_conflict_free, "Bank index 99 must trigger PGAS bank collision failure");
    assert!(!report.is_provably_safe);
    assert!(report.violations.iter().any(|v| v.contains("invalid bank index 99")));
}
