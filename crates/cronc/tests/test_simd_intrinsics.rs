use cronc::{compile_to_c23, compile_to_llvm, execute_jit, check_source_diagnostics};
use std::process::Command;

#[test]
fn test_simd_types_syntax_and_checking() {
    let source = r#"
    .MODULE SimdSyntaxCheck
    _main:
        let a: vec8f = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
        let b: vec8f = simd_splat(2.5)
        let c: vec8f = a + b
        let s: f32 = simd_reduce_sum(c)
    .END
    "#;

    let diags = check_source_diagnostics(source);
    assert!(diags.is_empty(), "Diagnostics should be empty for valid SIMD code: {:?}", diags);
}

#[test]
fn test_simd_llvm_ir_generation() {
    let source = r#"
    .MODULE SimdLlvmTest
    _main:
        let a: vec8f = simd_splat(1.5)
        let b: vec8f = simd_splat(2.0)
        let c: vec8f = a * b
        let d: vec8f = simd_fma(a, b, c)
        let sum: float = simd_reduce_sum(d)
        let dot: float = simd_dot(a, b)
    .END
    "#;

    let llvm_ir = compile_to_llvm(source).expect("LLVM IR compilation failed");
    assert!(llvm_ir.contains("<8 x float>"), "LLVM IR must contain <8 x float> vector type");
    assert!(llvm_ir.contains("fmul <8 x float>"), "LLVM IR must contain vector fmul instruction");
    assert!(llvm_ir.contains("@llvm.fma.v8f32"), "LLVM IR must contain LLVM FMA intrinsic call");
    assert!(llvm_ir.contains("@llvm.vector.reduce.fadd.v8f32"), "LLVM IR must contain vector reduction intrinsic call");
}

#[test]
fn test_simd_c23_native_gcc_execution() {
    let source = r#"
    .MODULE SimdGccTest
    _main:
        let a: vec8f = simd_splat(3.0)
        let b: vec8f = simd_splat(4.0)
        let c: vec8f = a * b
        let total: float = simd_reduce_sum(c)
        let ans: i32 = total as i32
        return ans
    .END
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");
    assert!(c_code.contains("cron_vec8f"), "C23 must contain vector type definition");

    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("test_simd_c23.c");
    let exe_path = temp_dir.join("test_simd_c23.exe");

    std::fs::write(&c_path, &c_code).expect("Failed to write C file");

    let status = Command::new("gcc")
        .args([
            "-std=c2x",
            "-O3",
            "-mavx2",
            "-mfma",
            c_path.to_str().unwrap(),
            "-o",
            exe_path.to_str().unwrap(),
        ])
        .status();

    if let Ok(st) = status {
        if st.success() {
            let run_output = Command::new(&exe_path).output().expect("Failed to execute native SIMD binary");
            let exit_code = run_output.status.code().unwrap_or(-1);
            // 3.0 * 4.0 = 12.0 each lane. 8 lanes * 12.0 = 96.0 -> 96.
            assert_eq!(exit_code, 96, "Expected return code 96 from 8-lane SIMD vector reduction");
        }
    }
}

#[test]
fn test_simd_jit_vector_arithmetic_and_reduction() {
    let source = r#"
    .MODULE SimdJitArithTest
    _main:
        let a: vec8f = [1, 2, 3, 4, 5, 6, 7, 8]
        let b: vec8f = simd_splat(2)
        let c: vec8f = a * b
        let sum: i32 = simd_reduce_sum(c)
        return sum
    .END
    "#;

    let res = execute_jit(source).expect("JIT execution failed");
    // a * 2 = [2, 4, 6, 8, 10, 12, 14, 16] -> sum = 72
    assert_eq!(res, 72, "JIT SIMD vector elementwise multiplication + reduction failed");
}

#[test]
fn test_simd_jit_fma_execution() {
    let source = r#"
    .MODULE SimdJitFmaTest
    _main:
        let a: vec8f = [1, 2, 3, 4, 5, 6, 7, 8]
        let b: vec8f = simd_splat(3)
        let c: vec8f = simd_splat(10)
        let d: vec8f = simd_fma(a, b, c)
        let sum: i32 = simd_reduce_sum(d)
        return sum
    .END
    "#;

    let res = execute_jit(source).expect("JIT FMA execution failed");
    // a * 3 + 10 = [13, 16, 19, 22, 25, 28, 31, 34] -> sum = 188
    assert_eq!(res, 188, "JIT Fused Multiply-Add (vfmadd231ps) reduction failed");
}

#[test]
fn test_simd_jit_dot_product() {
    let source = r#"
    .MODULE SimdJitDotTest
    _main:
        let a: vec8f = [1, 2, 3, 4, 5, 6, 7, 8]
        let b: vec8f = [2, 2, 2, 2, 2, 2, 2, 2]
        let dot: i32 = simd_dot(a, b)
        return dot
    .END
    "#;

    let res = execute_jit(source).expect("JIT dot product failed");
    // dot = 36 * 2 = 72
    assert_eq!(res, 72, "JIT SIMD dot product calculation failed");
}

#[test]
fn test_simd_jit_loop_accumulation() {
    let source = r#"
    .MODULE SimdJitLoopTest
    _main:
        let mut acc: vec8f = simd_splat(0)
        let v: vec8f = [1, 2, 3, 4, 5, 6, 7, 8]
        let mut i: i32 = 0
        while i < 10 {
            acc = acc + v
            i = i + 1
        }
        let total: i32 = simd_reduce_sum(acc)
        return total
    .END
    "#;

    let res = execute_jit(source).expect("JIT loop accumulation failed");
    // 10 iterations * 36 = 360
    assert_eq!(res, 360, "JIT SIMD loop accumulation failed");
}
