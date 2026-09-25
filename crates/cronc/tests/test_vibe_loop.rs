// ============================================================================
// Tests for CRON Autonomous AI Vibe-Coding Loop (vibe_loop)
// ============================================================================

use cronc::vibe_loop::{run_vibe_loop, VibeLoopConfig, VibeStatus};
use cronc::cl_heal::heal_cl_program;

#[test]
fn test_vibe_loop_clean_canonical_program_success() {
    let source = r#"
    B0000: '==01#00A> '==02#00B> _NO00#000> _NO00#000>
    B0001: _OP01$28F> _NO00#000> _NO00#000> _HL00#000!
    "#;

    // Canonicalize code so all CRC-8 ATM tokens are mathematically exact
    let canonical = heal_cl_program(source).unwrap().canonical_code;

    let config = VibeLoopConfig::default();
    let result = run_vibe_loop(&canonical, &config);

    assert_eq!(result.status, VibeStatus::Success);
    assert!(!result.healed);
    assert_eq!(result.execution_cycles, 2);
    assert_eq!(result.optical_ops, 1);
    assert!(result.diagnostics.is_empty());
}

#[test]
fn test_vibe_loop_broken_ai_code_healed_and_optimized() {
    // Unpadded bundle with missing CRC nibble
    let broken_ai_source = "B0000: '==01#004> _OP01$2?F>\n";

    let config = VibeLoopConfig::default();
    let result = run_vibe_loop(broken_ai_source, &config);

    assert_eq!(result.status, VibeStatus::HealedAndExecuted);
    assert!(result.healed, "Must detect that healing was applied");
    assert!(result.fixed_crc_count > 0 || result.padded_bundles > 0);
    assert!(result.execution_cycles >= 1);
    assert_eq!(result.optical_ops, 1);
    assert_eq!(result.registers[1], 0x00FF0000); // Authentic Optical MZI result
}

#[test]
fn test_vibe_loop_json_serialization_validity() {
    let source = r#"
    B0000: '==01#005> _NO00#000> _NO00#000> _NO00#000>
    B0001: _PO00$100> _NO00#000> _NO00#000> _HL00#000!
    "#;

    let canonical = heal_cl_program(source).unwrap().canonical_code;
    let config = VibeLoopConfig::default();
    let result = run_vibe_loop(&canonical, &config);

    let json_str = result.to_json();
    assert!(json_str.contains("\"status\": \"SUCCESS\""));
    assert!(json_str.contains("\"healing_summary\""));
    assert!(json_str.contains("\"optimization_summary\""));
    assert!(json_str.contains("\"execution\""));
    assert!(json_str.contains("\"r0\": \"0x00000005\""));
    assert!(json_str.contains("\"optical_ops\": 0"));
    assert!(json_str.contains("\"diagnostics\": []"));
}

#[test]
fn test_vibe_loop_syntax_error_diagnostics_and_recommendations() {
    // Unrepairable invalid prefix and unknown opcode
    let invalid_source = "B0000: ^ZZ99$999> _NO00#000> _NO00#000> _NO00#000>\n";

    let config = VibeLoopConfig {
        auto_heal: false, // Disable auto heal to test raw diagnostic reporting
        ..Default::default()
    };

    let result = run_vibe_loop(invalid_source, &config);

    assert_eq!(result.status, VibeStatus::CompilationError);
    assert!(!result.diagnostics.is_empty());
    let diag = &result.diagnostics[0];
    assert_eq!(diag.error_code, "CL_SYNTAX_ERROR");
    assert!(diag.llm_fix_recommendation.contains("slot") || diag.llm_fix_recommendation.contains("prefix") || diag.llm_fix_recommendation.contains("syntax"));
}
