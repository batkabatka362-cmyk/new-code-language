use cronc::compile_to_llvm;

#[test]
fn test_llvm_basic_module_and_metadata() {
    let code = r#"
    .MODULE LlvmMetaTest
    def add_vals(a: i32, b: i32) -> i32 {
        let sum: i32 = a + b
        return sum
    }
    _main:
        let result = add_vals(42, 58)
    .END
    "#;

    let llvm_ir = compile_to_llvm(code).expect("Compilation to LLVM failed");

    assert!(llvm_ir.contains("source_filename = \"LlvmMetaTest.cr\""));
    assert!(llvm_ir.contains("target datalayout ="));
    assert!(llvm_ir.contains("target triple ="));
    assert!(llvm_ir.contains("declare i32 @printf(ptr nocapture readonly, ...)"));
    assert!(llvm_ir.contains("define i32 @main(i32 %argc, ptr %argv)"));
    assert!(llvm_ir.contains("alloca i32"));
    assert!(llvm_ir.contains("add i32"));
    assert!(llvm_ir.contains("ret i32 0"));
}

#[test]
fn test_llvm_struct_and_generics_monomorphization() {
    let code = r#"
    .MODULE LlvmStructTest
    struct Point<T> {
        x: T,
        y: T,
    }

    def get_x(p: Point<i32>) -> i32 {
        return p.x
    }

    _main:
        let p = Point<i32> { x: 10, y: 20 }
        let q = Point<f64> { x: 1.5, y: 2.5 }
        let val = get_x(p)
        export q as exported_q
    .END
    "#;

    let llvm_ir = compile_to_llvm(code).expect("Compilation to LLVM with generic structs failed");

    assert!(llvm_ir.contains("Point_i32"), "Must contain monomorphized Point_i32");
    assert!(llvm_ir.contains("Point_f64"), "Must contain monomorphized Point_f64");
}

#[test]
fn test_llvm_arithmetic_control_flow() {
    let code = r#"
    .MODULE LlvmControlFlow
    _main:
        let mut x: i32 = 10
        let mut count: i32 = 0
        if x > 5 {
            count = count + 1
        } else {
            count = count - 1
        }

        while count < 10 {
            count = count + 2
        }
    .END
    "#;

    let llvm_ir = compile_to_llvm(code).expect("Compilation to LLVM control flow failed");

    assert!(llvm_ir.contains("icmp sgt i32"));
    assert!(llvm_ir.contains("br i1"));
    assert!(llvm_ir.contains("then_"));
    assert!(llvm_ir.contains("else_"));
    assert!(llvm_ir.contains("if_merge_"));
    assert!(llvm_ir.contains("while_cond_"));
    assert!(llvm_ir.contains("while_body_"));
    assert!(llvm_ir.contains("while_end_"));
}

#[test]
fn test_llvm_functions_and_calls() {
    let code = r#"
    .MODULE LlvmFuncTest
    def compute_photonic_flux(amp: f64, phase: f64) -> f64 {
        let res: f64 = amp * 2.0 + phase
        return res
    }

    _main:
        let f: f64 = compute_photonic_flux(1.25, 0.75)
    .END
    "#;

    let llvm_ir = compile_to_llvm(code).expect("Compilation to LLVM functions failed");

    assert!(llvm_ir.contains("define double @compute_photonic_flux(double %arg.amp, double %arg.phase)"));
    assert!(llvm_ir.contains("fmul double"));
    assert!(llvm_ir.contains("fadd double"));
    assert!(llvm_ir.contains("ret double"));
    assert!(llvm_ir.contains("call double @compute_photonic_flux(double"));
}
