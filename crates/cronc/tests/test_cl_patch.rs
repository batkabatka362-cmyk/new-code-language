// ============================================================================
// CRON Hardware Instruction Extension & Microcode Patch Engine Tests
// ============================================================================

use cronc::cl_patch::{
    apply_cl_patch, format_patch_ascii_hud, patch_package_to_json,
    synthesize_patch_controller_verilog, ClPatchPackage, MicrocodePatchEntry,
    PatchAction,
};

#[test]
fn test_patch_package_serialization_roundtrip() {
    let mut pkg = ClPatchPackage::new("TORUS_256_REV_B");

    pkg.add_entry(MicrocodePatchEntry {
        entry_id: 0,
        target_cycle: 4,
        target_core_id: Some(12),
        action: PatchAction::ReplaceBundle,
        replacement_bundle_raw: "_OP01$28F> _NO00#000> _NO00#000> _NO00#000>".to_string(),
        enabled: true,
        comment: "Fix silicon errata #042 on MZI accumulator".to_string(),
    }).expect("Adding entry should succeed");

    pkg.add_entry(MicrocodePatchEntry {
        entry_id: 1,
        target_cycle: 9,
        target_core_id: None, // broadcast
        action: PatchAction::TrapHalt,
        replacement_bundle_raw: "_HL00#000! _NO00#000> _NO00#000> _NO00#000>".to_string(),
        enabled: true,
        comment: "Security boundary isolation trap".to_string(),
    }).expect("Adding entry should succeed");

    assert_eq!(pkg.entries.len(), 2);
    assert!(pkg.crc32_checksum != 0);

    // Binary serialization
    let bytes = pkg.to_bytes();
    assert!(bytes.len() > 32);

    // Binary deserialization
    let restored = ClPatchPackage::from_bytes(&bytes).expect("Deserialization must succeed");
    assert_eq!(restored.target_silicon_rev, "TORUS_256_REV_B");
    assert_eq!(restored.entries.len(), 2);
    assert_eq!(restored.entries[0].target_cycle, 4);
    assert_eq!(restored.entries[0].action, PatchAction::ReplaceBundle);
    assert_eq!(restored.entries[1].action, PatchAction::TrapHalt);
}

#[test]
fn test_apply_cl_patch_override_cycle() {
    let cl_code = "
        B0000: _NO00#000> _NO00#000> _NO00#000> _NO00#000>
        B0001: _MD00#000> _NO00#000> _NO00#000> _NO00#000>
        B0002: _NO00#000> _NO00#000> _NO00#000> _HL00#000!
    ";

    let mut pkg = ClPatchPackage::new("TORUS_256_REV_A");
    pkg.add_entry(MicrocodePatchEntry {
        entry_id: 0,
        target_cycle: 1,
        target_core_id: None,
        action: PatchAction::ReplaceBundle,
        replacement_bundle_raw: "_OP01$28F> _NO00#000> _NO00#000> _NO00#000>".to_string(),
        enabled: true,
        comment: "Patching cycle 1 to optical GEMM".to_string(),
    }).unwrap();

    let report = apply_cl_patch(cl_code, &pkg).expect("Patching should succeed");
    assert_eq!(report.patches_applied, 1);
    assert!(report.patched_cl_code.contains("B0001: _OP01$28F>"));
    assert!(!report.patched_cl_code.contains("B0001: _MD00#000>"));
}

#[test]
fn test_custom_isa_extension_expansion() {
    let cl_code = "
        B0000: _PX00#000> _NO00#000> _NO00#000> _NO00#000>
        B0001: _P000#000> _NO00#000> _NO00#000> _HL00#000!
    ";

    let pkg = ClPatchPackage::new("TORUS_256_REV_C");
    let report = apply_cl_patch(cl_code, &pkg).expect("Applying patch should succeed");

    assert_eq!(report.custom_opcodes_expanded, 2);
    // _PX expands to _OP01$28F>
    assert!(report.patched_cl_code.contains("_OP01$28F>"));
    // _P0 expands to _CD01#100>
    assert!(report.patched_cl_code.contains("_CD01#100>"));
}

#[test]
fn test_patch_ascii_hud_and_json() {
    let mut pkg = ClPatchPackage::new("TORUS_256_REV_B");
    pkg.add_entry(MicrocodePatchEntry {
        entry_id: 0,
        target_cycle: 2,
        target_core_id: Some(1),
        action: PatchAction::InsertNop,
        replacement_bundle_raw: "_NO00#000> _NO00#000> _NO00#000> _NO00#000>".to_string(),
        enabled: true,
        comment: "Inject NOP delay".to_string(),
    }).unwrap();

    let hud = format_patch_ascii_hud(&pkg, None);
    assert!(hud.contains("CRON POST-SILICON HARDWARE MICROCODE PATCH"));
    assert!(hud.contains("MICROCODE PATCH TABLE (MPT)"));
    assert!(hud.contains("TORUS_256_REV_B"));

    let json = patch_package_to_json(&pkg);
    assert!(json.contains("\"silicon_rev\": \"TORUS_256_REV_B\""));
    assert!(json.contains("\"target_cycle\": 2"));
}

#[test]
fn test_patch_controller_verilog_synthesis() {
    let mut pkg = ClPatchPackage::new("TORUS_256_REV_A");
    pkg.add_entry(MicrocodePatchEntry {
        entry_id: 0,
        target_cycle: 5,
        target_core_id: None,
        action: PatchAction::ReplaceBundle,
        replacement_bundle_raw: "_OP01$28F> _NO00#000> _NO00#000> _NO00#000>".to_string(),
        enabled: true,
        comment: "Test CAM match".to_string(),
    }).unwrap();

    let verilog = synthesize_patch_controller_verilog(&pkg);
    assert!(verilog.contains("module microcode_patch_controller"));
    assert!(verilog.contains("current_cycle_pc == 32'd5"));
    assert!(verilog.contains("cam_hit_flag"));
}
