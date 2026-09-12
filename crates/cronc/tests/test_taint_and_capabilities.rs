use cronc::compile_source;

#[test]
fn test_tainted_variable_rejected_without_sanitize_e0009() {
    let source = r#"
.MODULE TestTaintSecurity
.ENTRY _main

_main:
    let tainted ext_spike : tainted u32 = 0x00FF
    let cap_token = acquire_capability()
    secure_patch_icache(target_slot=2, patch_val=ext_spike, cap=cap_token)
.END
"#;

    let result = compile_source(source);
    assert!(result.is_err(), "Must reject tainted variable passed to secure_patch_icache");
    let err = result.unwrap_err();
    assert!(
        err.contains("E0009"),
        "Expected E0009 taint violation error, got: {}",
        err
    );
    assert!(
        err.contains("ext_spike"),
        "Error should cite the tainted variable name: {}",
        err
    );
}

#[test]
fn test_missing_capability_token_rejected_e0010() {
    let source = r#"
.MODULE TestCapabilitySecurity
.ENTRY _main

_main:
    let ext_data : u32 = 0x0042
    let clean_data = sanitize(ext_data)
    secure_patch_icache(target_slot=2, patch_val=clean_data)
.END
"#;

    let result = compile_source(source);
    assert!(result.is_err(), "Must reject patch without acquired capability token");
    let err = result.unwrap_err();
    assert!(
        err.contains("E0010"),
        "Expected E0010 capability violation error, got: {}",
        err
    );
}

#[test]
fn test_sanitized_with_capability_compiles_cleanly() {
    let source = r#"
.MODULE TestCleanSecurity
.ENTRY _main

_main:
    let tainted ext_spike : tainted u32 = 0x00FF
    let cap_token = acquire_capability()
    let clean_spike = sanitize(ext_spike)
    secure_patch_icache(target_slot=2, patch_val=clean_spike, cap=cap_token)
.END
"#;

    let result = compile_source(source);
    assert!(result.is_ok(), "Must compile cleanly when sanitized with capability");
    let vliw_asm = result.unwrap();
    assert!(vliw_asm.contains("_AC"), "Must contain _AC Acquire Capability slot");
    assert!(vliw_asm.contains("_SN"), "Must contain _SN Sanitize Bounds slot");
    assert!(vliw_asm.contains("_SC"), "Must contain _SC Secure Patch slot");
}
