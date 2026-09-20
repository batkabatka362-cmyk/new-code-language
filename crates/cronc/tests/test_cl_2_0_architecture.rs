// ============================================================================
// CRON Cognitive Language 2.0 (CL 2.0) Architecture Verification Test
// Tests:
//   1. Variable-width zero-NOP bundles (1..=4 slots per cycle)
//   2. Native self-documenting semantic AI directives (.stage, .tensor, .fuse, .flow)
//   3. 100.0% 94-character alphabet saturation on foundation models
//   4. Zero hardware hazards (RAW, WAW, structural)
// ============================================================================

use cronc::cl_lang::{audit_alphabet_coverage, verify_cl_program};
use std::fs;
use std::path::Path;

#[test]
fn test_cl_2_0_variable_bundles_and_semantic_directives() {
    let cl_sample = r#"
.stage "test_stage", heads=16, dim=2048, zero_overhead=true
.tensor %Q: [16, 128], %K: [4, 128], %V: [4, 128]
.fuse [RMSNorm -> OpticalGEMM & RoPE]
.flow (Core[0,0,0,0] -> Core[1,0,0,0]) {dor=XYZW}

.core [0, 0, 0, 0]:
@entry:
B0000: '==01#010>
B0001: _OP05$28F> _MD06*2A2>
B0002: _TX05$100> _TX06$100> _PK08`200>
B0003: _SB00#000; _WH09\000> _YD00/005> _HL00#000!
"#;

    let report = verify_cl_program(cl_sample).expect("CL 2.0 program must pass verification");
    assert_eq!(report.total_bundles, 4);
    assert_eq!(report.total_slots, 1 + 2 + 3 + 4); // Variable slots: 1, 2, 3, 4!
    assert_eq!(report.directives_found, 4); // .stage, .tensor, .fuse, .flow
    assert_eq!(report.hazards.len(), 0);
}

#[test]
fn test_cron_bitnet_7b_foundation_cl_2_0_supreme_saturation() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let cl_path = repo_root
        .join("examples")
        .join("cl")
        .join("cron_bitnet_7b_foundation.cl");

    let cl_source = fs::read_to_string(&cl_path)
        .expect("Failed to read cron_bitnet_7b_foundation.cl");

    // 1. Verify VLIW structural verification
    let report = verify_cl_program(&cl_source)
        .expect("cron_bitnet_7b_foundation.cl must verify cleanly under CL 2.0");
    assert_eq!(report.hazards.len(), 0, "Hazard count must be 0: {:?}", report.hazards);
    assert!(report.directives_found >= 4, "Must contain CL 2.0 semantic directives");

    // 2. Verify 100.0% 94-character complete alphabet saturation
    let audit = audit_alphabet_coverage(&cl_source);
    assert_eq!(
        audit.unique_characters_used, 94,
        "Must achieve 100% saturation (all 94 characters). Missing: {:?}",
        audit.missing_characters
    );
    assert_eq!(audit.coverage_percentage, 100.0);
    assert!(audit.missing_characters.is_empty());
    assert!(
        audit.entropy_bits_per_char > 5.0,
        "Entropy must exceed 5.0 bits/char under CL 2.0 (got {:.4})",
        audit.entropy_bits_per_char
    );
}
