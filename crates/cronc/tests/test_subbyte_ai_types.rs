use cronc::{compile_to_c23, compile_to_llvm, check_source_diagnostics};
use std::process::Command;

#[test]
fn test_subbyte_types_parsing_and_checking() {
    let source = r#"
    .MODULE SubByteTypesCheck
    _main:
        let a: i2 = 1
        let b: i4 = -5
        let c: f4 = 1.5
        let d: f8 = 2.25
        let e: f16 = 3.125
        let f: bf16 = 4.5
        let sum: i32 = (a as i32) + (b as i32)
        return sum
    .END
    "#;

    let diags = check_source_diagnostics(source);
    assert!(diags.is_empty(), "Diagnostics should be empty for valid sub-byte types: {:?}", diags);
}

#[test]
fn test_subbyte_intrinsics_checking() {
    let source = r#"
    .MODULE SubByteIntrinsicsCheck
    _main:
        let raw_weights: [i32; 16] = [1, -1, 1, 0, 1, -1, 0, 0, 1, 1, -1, -1, 1, 0, 0, 1]
        let packed_w: u32 = pack_ternary(raw_weights)
        let unp_0: i8 = unpack_ternary(packed_w, 0)
        let unp_1: i8 = unpack_ternary(packed_w, 1)

        let raw_i4: [i32; 8] = [7, -8, 3, -2, 0, 1, -5, 4]
        let packed_i4: u32 = pack_i4(raw_i4)
        let unp_i4: i8 = unpack_i4(packed_i4, 1)

        let dot: i32 = simd_ternary_dot(packed_w, packed_w)
        let gemm_f4: f32 = simd_f4_gemm(packed_i4, packed_i4, 0.5)
        return dot
    .END
    "#;

    let diags = check_source_diagnostics(source);
    assert!(diags.is_empty(), "Diagnostics should be empty for sub-byte intrinsics: {:?}", diags);
}

#[test]
fn test_subbyte_c23_translation() {
    let source = r#"
    .MODULE SubByteC23Test
    _main:
        let a: i2 = 1
        let b: i4 = -3
        let c: f4 = 0.5
        let d: f8 = 1.25
        let dot: i32 = simd_ternary_dot(0x55555555, 0x55555555)
        let total: i32 = (a as i32) + (b as i32) + (c as i32) + (d as i32) + dot
        return total
    .END
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");
    assert!(c_code.contains("int8_t a = 1;"), "C23 must map i2 to int8_t: {}", c_code);
    assert!(c_code.contains("int8_t b = -3;"), "C23 must map i4 to int8_t: {}", c_code);
    assert!(c_code.contains("float c = 0.5;"), "C23 must map f4 to float: {}", c_code);
    assert!(c_code.contains("float d = 1.25;"), "C23 must map f8 to float: {}", c_code);
    assert!(c_code.contains("simd_ternary_dot"), "C23 must invoke simd_ternary_dot: {}", c_code);
}

#[test]
fn test_subbyte_llvm_ir_translation() {
    let source = r#"
    .MODULE SubByteLlvmTest
    _main:
        let a: i2 = 1
        let b: i4 = 3
        let c: f4 = 2.0
        let total: i32 = (a as i32) + (b as i32) + (c as i32)
        return total
    .END
    "#;

    let llvm_ir = compile_to_llvm(source).expect("LLVM IR compilation failed");
    assert!(llvm_ir.contains("alloca i2"), "LLVM IR must contain alloca i2: {}", llvm_ir);
    assert!(llvm_ir.contains("alloca i4"), "LLVM IR must contain alloca i4: {}", llvm_ir);
    assert!(llvm_ir.contains("alloca float"), "LLVM IR must contain alloca float: {}", llvm_ir);
}

#[test]
fn test_bitnet_ternary_dot_gcc_c23_native_execution() {
    let source = r#"
    .MODULE BitNetExecutionTest
    _main:
        // 0x55555555 encodes 16 elements of +1 (since 01 in 2-bit is +1)
        let all_ones: u32 = 0x55555555
        let dot_16: i32 = simd_ternary_dot(all_ones, all_ones)

        // 0xAAAAAAAA encodes 16 elements of -1 (since 10 in 2-bit is -1)
        let all_neg: u32 = 0xAAAAAAAA
        let dot_neg_16: i32 = simd_ternary_dot(all_ones, all_neg)

        // Orthogonal cancellation test:
        // w has (+1, -1) in first 2 slots, rest 0
        // (01 in slot 0, 10 in slot 1 = 1 | (2 << 2) = 1 | 8 = 9)
        let w: u32 = 9
        // x has (+1, +1) in first 2 slots, rest 0
        // (01 in slot 0, 01 in slot 1 = 1 | (1 << 2) = 1 | 4 = 5)
        let x: u32 = 5
        let dot_cancel: i32 = simd_ternary_dot(w, x)

        // Assertions via return code
        if dot_16 != 16 {
            return 101
        }
        if dot_neg_16 != -16 {
            return 102
        }
        if dot_cancel != 0 {
            return 103
        }

        // Return 42 on complete success
        return 42
    .END
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");

    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("test_bitnet_ternary.c");
    let exe_path = temp_dir.join("test_bitnet_ternary.exe");

    std::fs::write(&c_path, &c_code).expect("Failed to write C file");

    let gcc_status = Command::new("gcc")
        .args(&[
            "-std=c2x",
            "-O3",
            c_path.to_str().unwrap(),
            "-o",
            exe_path.to_str().unwrap(),
        ])
        .status()
        .expect("GCC execution failed");

    assert!(gcc_status.success(), "GCC compilation must succeed without error");

    let run_output = Command::new(&exe_path)
        .output()
        .expect("Failed to execute native BitNet binary");
    let exit_code = run_output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 42, "BitNet ternary dot products must evaluate correctly to return 42, got {}", exit_code);
}

#[test]
fn test_subbyte_pack_and_unpack_c23_execution() {
    let source = r#"
    .MODULE PackUnpackExecutionTest
    _main:
        let weights: [i32; 16] = [1, -1, 0, 1, 0, 0, -1, 1, 1, 0, -1, 0, 1, -1, 1, 0]
        let packed: u32 = pack_ternary(weights)

        let w0: i8 = unpack_ternary(packed, 0)
        let w1: i8 = unpack_ternary(packed, 1)
        let w2: i8 = unpack_ternary(packed, 2)
        let w3: i8 = unpack_ternary(packed, 3)

        if (w0 as i32) != 1 {
            return 1
        }
        if (w1 as i32) != -1 {
            return 2
        }
        if (w2 as i32) != 0 {
            return 3
        }
        if (w3 as i32) != 1 {
            return 4
        }

        return 77
    .END
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");

    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("test_pack_unpack.c");
    let exe_path = temp_dir.join("test_pack_unpack.exe");

    std::fs::write(&c_path, &c_code).expect("Failed to write C file");

    let gcc_status = Command::new("gcc")
        .args(&[
            "-std=c2x",
            "-O3",
            c_path.to_str().unwrap(),
            "-o",
            exe_path.to_str().unwrap(),
        ])
        .status()
        .expect("GCC execution failed");

    assert!(gcc_status.success(), "GCC compilation must succeed without error");

    let run_output = Command::new(&exe_path)
        .output()
        .expect("Failed to execute pack/unpack binary");
    let exit_code = run_output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 77, "Pack and unpack must recover ternary weights exactly, got {}", exit_code);
}
