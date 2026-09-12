use cronc::compile_source;

#[test]
fn test_reassign_immutable_e0005() {
    let source = r#"
    .MODULE TestImmutable
    _main:
        let x: u32 = 100
        x = 200
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("E0005"), "Expected E0005 error, got: {}", err);
    assert!(err.contains("Cannot assign twice to immutable variable 'x'"));
}

#[test]
fn test_mut_reassign_allowed() {
    let source = r#"
    .MODULE TestMutable
    _main:
        let mut x: u32 = 100
        x = 200
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_ok(), "Expected compile success, got error: {:?}", res.err());
}

#[test]
fn test_linear_leak_e0002() {
    let source = r#"
    .MODULE TestLeak
    _main:
        let lin q: u32 = 55
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("E0002"), "Expected E0002 error, got: {}", err);
    assert!(err.contains("Linear type leak: Linear variable 'q'"));
}

#[test]
fn test_linear_double_consume_e0003() {
    let source = r#"
    .MODULE TestDoubleConsume
    _main:
        let lin q: u32 = 42
        let a: u32 = consume(q)
        let b: u32 = consume(q)
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("E0003"), "Expected E0003 error, got: {}", err);
    assert!(err.contains("already consumed and cannot be reused"));
}

#[test]
fn test_reassign_consumed_linear_e0003() {
    let source = r#"
    .MODULE TestAssignConsumed
    _main:
        let mut lin q: u32 = 42
        let a: u32 = consume(q)
        q = 99
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("E0003"), "Expected E0003 error, got: {}", err);
    assert!(err.contains("already consumed and cannot be reassigned"));
}

#[test]
fn test_region_escape_e0004() {
    let source = r#"
    .MODULE TestRegionEscape
    _main:
        region Arena {
            let r_buf: u32 = 123
        }
        let leaked: u32 = r_buf
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("E0004"), "Expected E0004 error, got: {}", err);
    assert!(err.contains("Region violation: 'r_buf' was declared in a region arena and cannot escape"));
}

#[test]
fn test_region_export_allowed() {
    let source = r#"
    .MODULE TestRegionExport
    _main:
        region Arena {
            let r_buf: u32 = 123
            export r_buf as ext_buf
        }
        let safe_buf: u32 = ext_buf
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_ok(), "Expected compile success, got error: {:?}", res.err());
}

#[test]
fn test_export_unknown_symbol_e0007() {
    let source = r#"
    .MODULE TestExportUnknown
    _main:
        region Arena {
            export ghost_symbol as exported_ghost
        }
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("E0007"), "Expected E0007 error, got: {}", err);
    assert!(err.contains("Cannot export unknown symbol 'ghost_symbol'"));
}

#[test]
fn test_linear_param_leak_in_func_e0002() {
    let source = r#"
    .MODULE TestFuncLinearParam
    def process(lin resource: u32) -> u32 {
        return 0
    }
    _main:
        let dummy = 1
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("E0002"), "Expected E0002 error, got: {}", err);
    assert!(err.contains("Linear variable 'resource' was allocated but never consumed"));
}

#[test]
fn test_linear_param_consumed_in_func_ok() {
    let source = r#"
    .MODULE TestFuncLinearParamConsumed
    def process(lin resource: u32) -> u32 {
        let x: u32 = consume(resource)
        return x
    }
    _main:
        let dummy = 1
    .END
    "#;
    let res = compile_source(source);
    assert!(res.is_ok(), "Expected compile success, got error: {:?}", res.err());
}

#[test]
fn test_span_accuracy_in_diagnostic() {
    let source = ".MODULE TestSpan\n_main:\n    let lin unconsumed_var = 999\n.END";
    let res = compile_source(source);
    assert!(res.is_err());
    let err = res.unwrap_err();
    // Line 3 has let lin unconsumed_var = 999
    assert!(err.contains("line 3") || err.contains("3:"), "Expected error location line 3 in: {}", err);
    assert!(err.contains("unconsumed_var"));
}
