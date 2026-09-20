// ============================================================================
// Tests for CRON AI Vibe-Coding Schema & Specification Engine (cl_spec)
// ============================================================================

use cronc::cl_spec::{
    generate_cl_ai_system_prompt, generate_cl_ebnf_grammar, generate_cl_json_schema,
    generate_cl_spec, SpecFormat,
};

#[test]
fn test_cl_ebnf_grammar_generation() {
    let ebnf = generate_cl_ebnf_grammar();
    assert!(ebnf.contains("CRON Cognitive Low-Level Machine Language (.cl) Formal EBNF Specification"));
    assert!(ebnf.contains("Bundle           = CycleHeader , Slot , Slot , Slot , Slot ;"));
    assert!(ebnf.contains("Slot             = Prefix , Opcode , DestReg , Mode , SrcReg , CrcParity , ImmParam , Terminator ;"));
    assert!(ebnf.contains("CoreDirective    = \".core\""));
    assert!(ebnf.contains("\"OP\""));
    assert!(ebnf.contains("\"MD\""));
    assert!(ebnf.contains("\"TX\""));
    assert!(ebnf.contains("\"RX\""));
}

#[test]
fn test_cl_json_schema_generation() {
    let json = generate_cl_json_schema();
    assert!(json.contains("\"title\": \"CronCognitiveLowLevelLanguageSpec\""));
    assert!(json.contains("\"vliw_bundle_width_bits\": 128"));
    assert!(json.contains("\"canonical_nop_token\": \"_NO00#000>\""));
    assert!(json.contains("\"registers\": ["));
    assert!(json.contains("\"$rv\""));
    assert!(json.contains("\"$bp\""));
    assert!(json.contains("\"opcode_catalog\": ["));
    assert!(json.contains("\"Photonic GEMM\""));
    assert!(json.contains("\"NoC Wormhole\""));
}

#[test]
fn test_cl_ai_system_prompt_generation() {
    let prompt = generate_cl_ai_system_prompt();
    assert!(prompt.contains("SYSTEM ROLE: SSS+ CRON .cl SILICON VIBE-CODING ENGINE"));
    assert!(prompt.contains("Fixed-Width 10-Character Slot Format"));
    assert!(prompt.contains("PAD with idle NOP slots: `_NO00#000>`"));
    assert!(prompt.contains(".core [x, y, z, w]:"));
    assert!(prompt.contains("FlashAttention kernel"));
    assert!(prompt.contains("Multi-Core 4D-Torus Producer-Consumer Pipeline"));
}

#[test]
fn test_cl_spec_format_selector() {
    let ebnf = generate_cl_spec(SpecFormat::Ebnf);
    assert!(ebnf.contains("Formal EBNF Specification"));

    let json = generate_cl_spec(SpecFormat::Json);
    assert!(json.contains("\"CronCognitiveLowLevelLanguageSpec\""));

    let prompt = generate_cl_spec(SpecFormat::Prompt);
    assert!(prompt.contains("SYSTEM ROLE: SSS+ CRON .cl SILICON VIBE-CODING ENGINE"));

    let all = generate_cl_spec(SpecFormat::All);
    assert!(all.contains("SECTION 1: AI VIBE-CODING SYSTEM PROMPT"));
    assert!(all.contains("SECTION 2: MACHINE-READABLE JSON SCHEMA"));
    assert!(all.contains("SECTION 3: FORMAL EBNF GRAMMAR"));

    assert_eq!(SpecFormat::from_str("ebnf"), Some(SpecFormat::Ebnf));
    assert_eq!(SpecFormat::from_str("json"), Some(SpecFormat::Json));
    assert_eq!(SpecFormat::from_str("schema"), Some(SpecFormat::Json));
    assert_eq!(SpecFormat::from_str("prompt"), Some(SpecFormat::Prompt));
    assert_eq!(SpecFormat::from_str("all"), Some(SpecFormat::All));
    assert_eq!(SpecFormat::from_str("unknown"), None);
}
