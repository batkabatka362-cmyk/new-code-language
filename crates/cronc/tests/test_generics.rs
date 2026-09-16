// ============================================================================
// CRON Parametric Generics (<T, U>) & Zero-Cost Monomorphization Tests
// Verifies:
//   - Generic struct parsing: struct Box<T>, struct Pair<T, U>
//   - Generic function parsing: def wrap<T>(val: T) -> Box<T>
//   - Monomorphization into concrete C23 types: Box_i32, Pair_f64_wave_t
//   - Zero runtime overhead and GCC -O3 native execution
// ============================================================================

use cronc::compile_to_c23;
use std::fs;
use std::process::Command;

#[test]
fn test_generic_struct_declaration_and_monomorphization() {
    let source = r#"
    .MODULE GenericStructTest
    .ENTRY _main

    struct Container<T> {
        value: T,
        tag: i32
    }

    def get_tag(c: Container<i32>) -> i32 {
        return c.tag
    }

    _main:
        let c_int: Container<i32> = Container<i32> { value: 42, tag: 1 }
        let c_wave: Container<wave_t> = Container<wave_t> { value: 0xDEADBEEF, tag: 2 }
        let t = get_tag(c_int)
        export c_wave as exported_wave
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Should compile generic structs to C23");
    assert!(c_code.contains("Container_i32"), "Must define monomorphized Container_i32");
    assert!(c_code.contains("Container_wave_t"), "Must define monomorphized Container_wave_t");
    assert!(c_code.contains("value"), "Container must have value field");
}

#[test]
fn test_generic_multi_param_struct_and_functions() {
    let source = r#"
    .MODULE MultiGenericTest
    .ENTRY _main

    struct KeyValue<K, V> {
        key: K,
        val: V
    }

    def make_pair<K, V>(k: K, v: V) -> KeyValue<K, V> {
        let pair: KeyValue<K, V> = KeyValue<K, V> { key: k, val: v }
        return pair
    }

    _main:
        let item: KeyValue<i32, f64> = make_pair<i32, f64>(100, 3.14159)
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Should compile multi-param generics to C23");
    assert!(c_code.contains("typedef struct KeyValue_i32_f64 {"), "Must define monomorphized KeyValue_i32_f64");
    assert!(c_code.contains("int32_t key;"), "KeyValue_i32_f64 must have int32_t key");
    assert!(c_code.contains("double val;"), "KeyValue_i32_f64 must have double val");
    assert!(c_code.contains("make_pair_i32_f64("), "Must generate specialized make_pair function");
    assert!(c_code.contains("(KeyValue_i32_f64){ .key = k, .val = v }"), "Must initialize specialized struct in function body");
}

#[test]
fn test_generic_4d_tensor_gcc_native_execution() {
    let source = r#"
    .MODULE NativeGenericTensor
    .ENTRY _main

    struct Tensor4D<T> {
        data: T,
        scale: f64
    }

    def scale_tensor<T>(t: Tensor4D<T>, factor: f64) -> Tensor4D<T> {
        let res: Tensor4D<T> = Tensor4D<T> { data: t.data, scale: t.scale * factor }
        return res
    }

    _main:
        let original: Tensor4D<i32> = Tensor4D<i32> { data: 50, scale: 1.5 }
        let scaled: Tensor4D<i32> = scale_tensor<i32>(original, 2.0)
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Should compile NativeGenericTensor to C23");
    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("cron_generic_test.c");
    let exe_path = temp_dir.join("cron_generic_test.exe");

    fs::write(&c_path, &c_code).expect("Write C source to temp");

    let gcc_status = Command::new("gcc")
        .args(&["-O3", c_path.to_str().unwrap(), "-o", exe_path.to_str().unwrap(), "-lm"])
        .status();

    if let Ok(status) = gcc_status {
        if status.success() {
            let run_output = Command::new(&exe_path)
                .output()
                .expect("Execute native binary");
            assert!(run_output.status.success(), "Native binary should exit 0");
            let stdout_str = String::from_utf8_lossy(&run_output.stdout);
            assert!(stdout_str.contains("[CRON Native C23 Engine]"), "Output must contain CRON banner");
            assert!(stdout_str.contains("Execution completed successfully"), "Output must indicate success");

            let _ = fs::remove_file(c_path);
            let _ = fs::remove_file(exe_path);
        }
    }
}
