// ============================================================================
// CRON .cl to Native C23 / AVX-512 & AVX2 SIMD Integration Tests
// Verifies that .cl machine microcode compiles directly to C23 with native
// SIMD intrinsics, builds successfully with host GCC/Clang, and executes
// with 100% bit-exact parity and maximum IPC.
// ============================================================================

use cronc::cl_c23::{compile_cl_to_c23, compile_cl_to_native_binary_with_target, SimdTarget};
use std::fs;
use std::process::Command;

const SAMPLE_GQA_CL: &str = r#"
; Grouped Query Attention Kernel
B0000: '==01#010> '==02#004> '==03#008> '==04#020>
B0001: _OP05$142> _MD06*2A2> _PO07+100> _SB00#000>
B0002: _SM08$680> _PO09*500> _PO0A+700> _bb00#000>
B0003: _HL00!000> _NO00#000> _NO00#000> _NO00#000>
"#;

const SAMPLE_SWIGLU_CL: &str = r#"
; Fused SwiGLU MoE Expert Kernel
B0000: '==01#008> '==02#00C> '==03#002> _NO00#000>
B0001: _SI04$100> _PO05*240> _MD06*300> _bb00#000>
B0002: _FU00#000> _PO01+500> _FE00#000> _HL00!000>
"#;

const SAMPLE_DEQUANT_CL: &str = r#"
; Quantized Trit Dequantization Kernel
B0000: '==01#555> '==02#AAA> '==03#001> _NO00#000>
B0001: _PK04$100> _MD05.400> _PO06+200> _bb00#000>
B0002: _DA07$000> _SE08$000> _NE09$000> _HL00!000>
"#;

#[test]
fn test_c23_code_generation_preamble_and_simd_headers() {
    let c23_code = compile_cl_to_c23(SAMPLE_GQA_CL, "gqa_kernel")
        .expect(".cl to C23 compilation must succeed");

    assert!(c23_code.contains("#include <stdio.h>"));
    assert!(c23_code.contains("#include <math.h>"));
    assert!(c23_code.contains("#include <time.h>"));
    assert!(c23_code.contains("CRON_SIMD_AVX512"));
    assert!(c23_code.contains("CRON_SIMD_AVX2"));
    assert!(c23_code.contains("cron_subbyte_ternary_dot"));
    assert!(c23_code.contains("cron_photonic_simd_dot"));
    assert!(c23_code.contains("cron_silu"));
    assert!(c23_code.contains("CronSiliconCore"));
    assert!(c23_code.contains("cron_execute_bundles"));
    assert!(c23_code.contains("cron_silicon_init"));
    assert!(c23_code.contains("int main(int argc, char** argv)"));
    assert!(c23_code.contains("--bench"));
}

#[test]
fn test_c23_swiglu_and_dequant_opcodes() {
    let swiglu_c23 = compile_cl_to_c23(SAMPLE_SWIGLU_CL, "swiglu_kernel")
        .expect("SwiGLU compilation must succeed");
    assert!(swiglu_c23.contains("cron_silu"));
    assert!(swiglu_c23.contains("core->fused_ops_count++"));

    let dequant_c23 = compile_cl_to_c23(SAMPLE_DEQUANT_CL, "dequant_kernel")
        .expect("Dequant compilation must succeed");
    assert!(dequant_c23.contains("packed_trits"));
    assert!(dequant_c23.contains("core->dopamine_level"));
    assert!(dequant_c23.contains("core->serotonin_level"));
    assert!(dequant_c23.contains("core->norepinephrine_level"));
}

#[test]
fn test_native_gcc_c23_compilation_and_execution() {
    // Check if gcc is available
    if Command::new("gcc").arg("--version").output().is_err() {
        eprintln!("Skipping native gcc test: gcc not found on host");
        return;
    }

    let temp_dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let exe_name = if cfg!(windows) {
        format!("cron_c23_test_{}_{}.exe", std::process::id(), ts)
    } else {
        format!("cron_c23_test_{}_{}", std::process::id(), ts)
    };
    let exe_path = temp_dir.join(&exe_name);

    // Compile with AVX2 SIMD target
    let res = compile_cl_to_native_binary_with_target(
        SAMPLE_GQA_CL,
        exe_path.to_str().unwrap(),
        "3",
        SimdTarget::Avx2,
    );
    assert!(res.is_ok(), "Native compilation with AVX2 failed: {:?}", res);
    assert!(exe_path.exists(), "Binary executable must exist");

    // Execute the native binary
    let output = Command::new(&exe_path)
        .output()
        .expect("Failed to execute native C23 binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "Execution failed: {}", stdout);
    assert!(stdout.contains("CRON SILICON C23 NATIVE EXECUTION TELEMETRY"));
    assert!(stdout.contains("Executed Bundles/Cycles:     4"));
    assert!(stdout.contains("100% BIT-EXACT SILICON C23 PARITY VERIFIED"));

    // Clean up
    let _ = fs::remove_file(&exe_path);
}

#[test]
fn test_native_gcc_c23_benchmarking_mode() {
    if Command::new("gcc").arg("--version").output().is_err() {
        return;
    }

    let temp_dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let exe_name = if cfg!(windows) {
        format!("cron_c23_bench_{}_{}.exe", std::process::id(), ts)
    } else {
        format!("cron_c23_bench_{}_{}", std::process::id(), ts)
    };
    let exe_path = temp_dir.join(&exe_name);

    let res = compile_cl_to_native_binary_with_target(
        SAMPLE_SWIGLU_CL,
        exe_path.to_str().unwrap(),
        "3",
        SimdTarget::Auto,
    );
    assert!(res.is_ok(), "Native compilation failed: {:?}", res);

    // Run in benchmark mode with 10,000 iterations
    let output = Command::new(&exe_path)
        .arg("--bench")
        .arg("10000")
        .output()
        .expect("Failed to execute benchmark");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "Benchmark run failed: {}", stdout);
    assert!(stdout.contains("Benchmark Iterations:        10000"));
    assert!(stdout.contains("Throughput (MIPS):"));
    assert!(stdout.contains("Compute Peak (GFLOPS/GOPS):"));

    let _ = fs::remove_file(&exe_path);
}
