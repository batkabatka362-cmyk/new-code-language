use cronc::cl_math::parse_math_code;
use std::fs;
use std::process::Command;

#[test]
fn test_hybrid_model_c_abi_dll_export_and_ctypes_inference() {
    // 1. Verify host GCC compiler exists
    if Command::new("gcc").arg("--version").output().is_err() {
        eprintln!("Skipping test: host gcc compiler not available");
        return;
    }

    // 2. Read Flagship Hybrid Core Model
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cl_file = manifest_dir.join("../../examples/cl/hybrid_frontier_core.cl");
    let cl_code = fs::read_to_string(&cl_file).expect("Failed to read hybrid_frontier_core.cl");

    // 3. Parse zero-verbal mathematical program
    let prog = parse_math_code(&cl_code).expect("Failed to parse hybrid_frontier_core.cl");

    // 4. Compile to native shared library (.dll / .so)
    let temp_dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dll_name = if cfg!(windows) {
        format!("cron_hybrid_model_{}_{}.dll", std::process::id(), ts)
    } else {
        format!("libcron_hybrid_model_{}_{}.so", std::process::id(), ts)
    };
    let dll_path = temp_dir.join(&dll_name);
    let h_path = temp_dir.join(if cfg!(windows) {
        format!("cron_hybrid_model_{}_{}.h", std::process::id(), ts)
    } else {
        format!("libcron_hybrid_model_{}_{}.h", std::process::id(), ts)
    });

    let compile_res = prog.compile_to_native(dll_path.to_str().unwrap(), "3");
    assert!(
        compile_res.is_ok(),
        "Failed to compile shared library with GCC: {:?}",
        compile_res
    );
    assert!(dll_path.exists(), "Compiled DLL must exist at {:?}", dll_path);
    assert!(h_path.exists(), "Generated C header must exist at {:?}", h_path);

    // 5. Verify C header contents
    let header_content = fs::read_to_string(&h_path).expect("Failed to read header");
    assert!(header_content.contains("CRON_API"));
    assert!(header_content.contains("void F("));
    assert!(header_content.contains("void N("));
    assert!(header_content.contains("void P("));

    // 6. Execute pure C ABI verification via Python ctypes
    let python_script = format!(
        r#"
import ctypes
import sys

dll_path = r"{dll_path}"
lib = ctypes.CDLL(dll_path)

# 1. Test N(x[16], gamma[16], out[16])
Float16 = ctypes.c_float * 16
x = Float16(*[float(i + 1) for i in range(16)])
gamma = Float16(*[2.0] * 16)
out_norm = Float16()

lib.N.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
lib.N.restype = None
lib.N(x, gamma, out_norm)

for i in range(16):
    expected = (float(i + 1)) * 2.0
    actual = out_norm[i]
    if abs(actual - expected) > 1e-4:
        sys.exit(f"N mismatch at index {{i}}: expected {{expected}}, got {{actual}}")

# 2. Test F(W[16, 16], x[16], out[16])
Float256 = ctypes.c_float * 256
w_data = [0.0] * 256
for i in range(16):
    w_data[i * 16 + i] = 3.0
W = Float256(*w_data)
out_gemm = Float16()

lib.F.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
lib.F.restype = None
lib.F(W, x, out_gemm)

for i in range(16):
    expected = (float(i + 1)) * 3.0
    actual = out_gemm[i]
    if abs(actual - expected) > 1e-4:
        sys.exit(f"F mismatch at index {{i}}: expected {{expected}}, got {{actual}}")

print("SUCCESS")
"#,
        dll_path = dll_path.display()
    );

    let py_res = Command::new("python")
        .arg("-c")
        .arg(&python_script)
        .output();

    if let Ok(output) = py_res {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success(),
            "Python ctypes execution failed!\nStdout: {}\nStderr: {}",
            stdout,
            stderr
        );
        assert!(stdout.contains("SUCCESS"));
    }

    // Cleanup artifacts
    let _ = fs::remove_file(&dll_path);
    let _ = fs::remove_file(&h_path);
}

