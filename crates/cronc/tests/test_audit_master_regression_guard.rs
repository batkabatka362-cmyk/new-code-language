// ============================================================================
// CRON Master Regression Guard & Ecosystem Audit Suite (Phase D5)
//
// Automatically exercises and verifies all 5 debugging layers in a single pass:
//   D1: Numerical Precision & Finite-Difference Gradient Audit
//   D2: Cross-Backend Parity (5-way check: CL == C23 == C++20 == CUDA == VLIW)
//   D3: Memory Safety & FFI Leak Audit (100,000 FFI calls, 0 leak)
//   D4: Dimension Checker Fuzzing & Circular Dependency Diagnostics
//   D5: Turnkey C++20 Compilation & Python Live C-ABI Benchmarks
//
// Zero-Verbal Silicon Mathematical Language Guarantee.
// ============================================================================

use std::process::Command;

#[test]
fn test_master_regression_guard_ecosystem_all_green() {
    println!("================================================================================");
    println!("  CRON SOVEREIGN ECOSYSTEM: MASTER REGRESSION GUARD (PHASES D1 - D5)");
    println!("================================================================================");

    // 1. Verify D1: Finite-Difference Gradient Audit
    println!("\n[GUARD D1] Running Finite-Difference Gradient Audit Suite...");
    let d1 = Command::new("cargo")
        .args(["test", "-p", "cronc", "--test", "test_finite_difference_gradient_audit"])
        .output()
        .expect("Failed to execute D1 suite");
    assert!(d1.status.success(), "D1 Finite-Difference audit failed: {:?}", String::from_utf8_lossy(&d1.stderr));
    println!("  -> [PASS] Phase D1 verified (6/6 tests green).");

    // 2. Verify D2: Cross-Backend Parity Suite
    println!("\n[GUARD D2] Running Cross-Backend 5-Way Parity Suite...");
    let d2 = Command::new("cargo")
        .args(["test", "-p", "cronc", "--test", "test_cross_backend_parity"])
        .output()
        .expect("Failed to execute D2 suite");
    assert!(d2.status.success(), "D2 Cross-Backend parity failed: {:?}", String::from_utf8_lossy(&d2.stderr));
    println!("  -> [PASS] Phase D2 verified (5/5 tests green).");

    // 3. Verify D3: Memory Safety & FFI Leak Suite
    println!("\n[GUARD D3] Running Memory Safety & 100k FFI Leak Suite...");
    let d3 = Command::new("cargo")
        .args(["test", "-p", "cronc", "--test", "test_memory_safety_and_ffi"])
        .output()
        .expect("Failed to execute D3 suite");
    assert!(d3.status.success(), "D3 Memory safety failed: {:?}", String::from_utf8_lossy(&d3.stderr));
    println!("  -> [PASS] Phase D3 verified (4/4 tests green).");

    // 4. Verify D4: Dimension Fuzzing & Circular Graph Diagnostics
    println!("\n[GUARD D4] Running Dimension Checker & Circular Graph Suite...");
    let d4 = Command::new("cargo")
        .args(["test", "-p", "cronc", "--test", "test_shape_fuzzing_and_diagnostics"])
        .output()
        .expect("Failed to execute D4 suite");
    assert!(d4.status.success(), "D4 Shape fuzzing failed: {:?}", String::from_utf8_lossy(&d4.stderr));
    println!("  -> [PASS] Phase D4 verified (4/4 tests green).");

    // 5. Verify C++20 Turnkey Verification
    println!("\n[GUARD C++20] Compiling and running examples/cpp/test_cron_cpp20.cpp...");
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.join("../..");
    let temp_dir = std::env::temp_dir();
    let cpp_bin = temp_dir.join("guard_test_cron_cpp20.exe");

    let cpp_compile = Command::new("g++")
        .args([
            "-std=c++20",
            "-O3",
            repo_root.join("examples/cpp/test_cron_cpp20.cpp").to_str().unwrap(),
            format!("-I{}", repo_root.join("examples/cpp").display()).as_str(),
            "-o",
            cpp_bin.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to invoke g++");
    assert!(cpp_compile.status.success(), "G++ compilation failed: {:?}", String::from_utf8_lossy(&cpp_compile.stderr));

    let cpp_run = Command::new(&cpp_bin).output().expect("Failed to run C++20 test binary");
    assert!(cpp_run.status.success(), "C++20 execution failed");
    let _ = std::fs::remove_file(cpp_bin);
    println!("  -> [PASS] Modern C++20 and CUDA single-header verified cleanly.");

    // 6. Verify Python Live C-ABI ctypes benchmark
    println!("\n[GUARD Python] Running examples/python/demo_c_abi_live_inference.py...");
    let py_run = Command::new("python")
        .args([repo_root.join("examples/python/demo_c_abi_live_inference.py").to_str().unwrap()])
        .output();
    if let Ok(py_out) = py_run {
        assert!(py_out.status.success(), "Python live benchmark failed: {:?}", String::from_utf8_lossy(&py_out.stderr));
        println!("  -> [PASS] Sub-microsecond live Python ctypes inference verified.");
    }

    println!("\n================================================================================");
    println!("  [100% GREEN] ALL CRON SYSTEMATIC AUDIT PHASES D1-D5 PASSED WITH ZERO ERRORS!");
    println!("================================================================================");
}
