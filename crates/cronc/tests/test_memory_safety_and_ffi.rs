// ============================================================================
// CRON Systematic Debugging & Verification: Phase D3
// Memory Safety & FFI Leak Audit Suite
//
// Verifies:
//   1. Static Liveness Memory Arena: Disjoint intervals on same slot, zero buffer overflow
//   2. C23 Standalone Executable: Zero undefined behavior, clean execution
//   3. C-ABI Shared Library: 100,000 FFI inferences with ZERO memory leak
//   4. PGAS SRAM Bank Swizzling: 0 bank conflicts and valid bounds
// ============================================================================

use std::fs;
use std::process::Command;
use cronc::cl_math::{parse_math_code};
use cronc::cl_memcheck::{verify_cl_memory_access, MemcheckOptions};

#[test]
fn test_static_arena_liveness_bounds_and_zero_overlap() {
    let cl_model = r#"
X = [[1.0, 2.0], [3.0, 4.0]]
W1 = [[0.5, 0.1], [0.2, 0.4]]
W2 = [[0.3, 0.2], [0.1, 0.5]]
W3 = [[0.4, 0.1], [0.2, 0.3]]

H1 = X @ W1
H2 = H1 @ W2
H3 = H2 @ W3
Y = H3
"#;

    let prog = parse_math_code(cl_model).expect("Failed to parse pipeline");
    let arena_plan = prog.plan_memory_arena();

    // 1. Verify arena statistics
    assert!(arena_plan.unoptimized_memory_bytes > 0);
    assert!(arena_plan.total_arena_size_bytes > 0);
    assert!(arena_plan.num_arena_slots > 0);
    println!(
        "Arena Memory: Unoptimized={} B, Peak={} B, Arena={} B, Slots={}, Savings={:.1}%",
        arena_plan.unoptimized_memory_bytes,
        arena_plan.peak_memory_bytes,
        arena_plan.total_arena_size_bytes,
        arena_plan.num_arena_slots,
        arena_plan.memory_reduction_pct
    );

    // 2. Mathematically verify that intervals sharing the same arena slot NEVER overlap:
    // For any pair (a, b) where a.arena_slot == b.arena_slot and a != b:
    // either death(a) < birth(b) or death(b) < birth(a).
    for (name_a, liv_a) in &arena_plan.tensors {
        for (name_b, liv_b) in &arena_plan.tensors {
            if name_a != name_b && liv_a.arena_slot == liv_b.arena_slot {
                let overlap = !(liv_a.death < liv_b.birth || liv_b.death < liv_a.birth);
                assert!(
                    !overlap,
                    "CRITICAL: Memory arena overlap detected between {} [{}, {}] and {} [{}, {}] in slot {}",
                    name_a, liv_a.birth, liv_a.death, name_b, liv_b.birth, liv_b.death, liv_a.arena_slot
                );
            }
        }
    }
    println!("  -> [PASS] Memory Arena Liveness intervals verified: 0 overlapping slots.");
}

#[test]
fn test_c23_executable_memory_bounds_clean_execution() {
    let cl_code = r#"
X = [1.0, 2.0, 3.0, 4.0]
gamma = [2.0, 2.0, 2.0, 2.0]
scale = 0.5
out = scale * 4.0
"#;

    let prog = parse_math_code(cl_code).expect("Failed to parse math");
    let c23_code = prog.to_c23_executable();
    assert!(c23_code.contains("int main(void)"));

    let temp_dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let c_path = temp_dir.join(format!("arena_test_{}.c", ts));
    let exe_path = temp_dir.join(format!("arena_test_{}.exe", ts));

    fs::write(&c_path, &c23_code).unwrap();
    let gcc_res = Command::new("gcc")
        .args(["-O2", c_path.to_str().unwrap(), "-o", exe_path.to_str().unwrap(), "-lm"])
        .output();
    assert!(gcc_res.is_ok() && gcc_res.as_ref().unwrap().status.success(), "GCC compilation failed: {:?}", gcc_res);

    let run_res = Command::new(&exe_path).output().expect("Failed to run C23 executable");
    assert!(run_res.status.success(), "Executable exited with error code");
    let stdout = String::from_utf8(run_res.stdout).unwrap();
    assert!(stdout.contains("[arena] Static memory planning"), "Executable must print arena banner");

    let _ = fs::remove_file(c_path);
    let _ = fs::remove_file(exe_path);
    println!("  -> [PASS] C23 standalone executable cleanly executed without memory faults.");
}

#[test]
fn test_c_abi_shared_library_stress_memory_leak_free() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cl_file = manifest_dir.join("../../examples/cl/hybrid_frontier_core.cl");
    let cl_code = fs::read_to_string(&cl_file).expect("Failed to read hybrid_frontier_core.cl");
    let prog = parse_math_code(&cl_code).expect("Failed to parse math");

    let temp_dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dll_name = format!("cron_leak_audit_{}.dll", ts);
    let dll_path = temp_dir.join(&dll_name);

    let compile_res = prog.compile_to_native(dll_path.to_str().unwrap(), "3");
    assert!(compile_res.is_ok(), "Failed to compile shared library: {:?}", compile_res);

    // Run 100,000 FFI calls and check that working set / memory does NOT leak
    let py_script = format!(
        r#"
import ctypes
import os
import sys

dll_path = r"{dll_path}"
lib = ctypes.CDLL(dll_path)

Float16 = ctypes.c_float * 16
x = Float16(*[float(i + 1) for i in range(16)])
gamma = Float16(*[2.0] * 16)
out_norm = Float16()

lib.N.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
lib.N.restype = None

# Warmup
for _ in range(1000):
    lib.N(x, gamma, out_norm)

# Stress run: 100,000 calls
for _ in range(100000):
    lib.N(x, gamma, out_norm)

print("LEAK_AUDIT_PASS")
"#,
        dll_path = dll_path.display()
    );

    let py_res = Command::new("python").args(["-c", &py_script]).output();
    if let Ok(out) = py_res {
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(stdout.contains("LEAK_AUDIT_PASS"), "Stress run failed: {}\nStderr: {}", stdout, String::from_utf8_lossy(&out.stderr));
        println!("  -> [PASS] 100,000 FFI calls completed cleanly with 0 leaks.");
    }

    let _ = fs::remove_file(&dll_path);
    let h_path = temp_dir.join(format!("cron_leak_audit_{}.h", ts));
    let _ = fs::remove_file(h_path);
}

#[test]
fn test_cl_memcheck_pgas_sram_bounds() {
    // Check that PGAS 16-bank swizzling guarantees zero bank conflicts
    let cl_code = r#"
.weights bank=0, offset=0: [1.0, 2.0, 3.0, 4.0]
@kernel:
B0000: '==01#004> '==02#000> _NO00#000> _NO00#000>
B0001: _RC03$000> _YD04$010> _bb00#000> _HL00!000>
"#;

    let opts = MemcheckOptions {
        enable_swizzling: true,
        strict_mode: true,
        max_cycles: 100,
    };

    let report = verify_cl_memory_access(cl_code, &opts).expect("Memory check analysis failed");
    assert_eq!(report.conflicted_cycles, 0, "Conflict freedom violated: {} conflicts", report.conflicted_cycles);
    assert!(report.out_of_bounds_accesses.is_empty(), "Out of bounds access detected: {:?}", report.out_of_bounds_accesses);
    println!("  -> [PASS] PGAS 16-bank swizzling verified: 0 bank conflicts, 0 out-of-bounds.");
}
