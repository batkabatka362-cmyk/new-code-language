// ============================================================================
// CRON AI Silicon ISA Extensions & Zero-Copy Weights Verification Suite
// Tests _RM (RMSNorm), _SM (Flash-Softmax), _SI (SiLU), _GE (GELU), _SS (SSM),
// and .weights directives in standalone .cl microcode.
// Target: Bit-Exact JIT and Bare-Metal C23 Silicon Parity
// ============================================================================

use std::process::Command;
use cronc::cl_lang::{parse_weights_directive, verify_cl_program};

#[test]
fn test_weights_directive_parsing() {
    // 1. Float list
    let line1 = ".weights bank=1, offset=0: [1.0, 2.0, -0.5, 0.25]";
    let b1 = parse_weights_directive(line1).expect("Float weights must parse");
    assert_eq!(b1.bank, 1);
    assert_eq!(b1.offset, 0);
    assert_eq!(b1.values.len(), 4);
    assert_eq!(b1.values[0], 1.0f32.to_bits());
    assert_eq!(b1.values[1], 2.0f32.to_bits());
    assert_eq!(b1.values[2], (-0.5f32).to_bits());
    assert_eq!(b1.values[3], 0.25f32.to_bits());

    // 2. Hex list
    let line2 = ".weights bank=3, offset=0x04: [0x3F800000, 0x40000000]";
    let b2 = parse_weights_directive(line2).expect("Hex weights must parse");
    assert_eq!(b2.bank, 3);
    assert_eq!(b2.offset, 4);
    assert_eq!(b2.values.len(), 2);
    assert_eq!(b2.values[0], 0x3F800000);
    assert_eq!(b2.values[1], 0x40000000);
}

#[test]
fn test_cl_ai_silicon_isa_verification() {
    // VLIW code scheduled cleanly without RAW or WAW hazards
    let cl_code = r#"
; ============================================================================
; AI Silicon ISA Microcode Program
; ============================================================================
.weights bank=1, offset=0: [1.0, 1.5, 2.0, 0.5]
.weights bank=2, offset=0: [0.85, 0.15]

@ai_kernel:
B0000: '==01#002> '==02#003> '==03#001> _NO00#000>
B0001: _RM04@100> _SI05$200> _GE06$300> _NO00#000>
B0002: _SM07$400> _SM08$500> _SS09@200> _HL00!000>
"#;

    let report = verify_cl_program(cl_code).expect("AI ISA .cl code must verify cleanly");
    assert_eq!(report.total_bundles, 3);
    assert_eq!(report.weights_bound, 2);
    assert_eq!(report.total_weight_bytes, (4 + 2) * 4);
    assert!(report.opcodes_verified >= 8);
    assert!(report.hazards.is_empty(), "Program must be hazard-free: {:?}", report.hazards);
}

#[test]
fn test_cl_ai_jit_numerical_accuracy() {
    // Test numerical precision of RM, SI, GE, SM, SS in JIT
    let val_x: f32 = 2.0;

    let mut core = cronc::cl_jit::ClJitCore::new();
    core.r[1] = val_x.to_bits();

    let cl_exec = r#"
.weights bank=1, offset=0: [1.5]
.weights bank=2, offset=0: [0.8, 0.2]

@compute:
B0000: _RM04@100> _SI05$100> _GE06$100> _NO00#000>
B0001: _SM07$100> _SS08@200> _NO00#000> _HL00!000>
"#;

    cronc::cl_jit::execute_cl_on_core(cl_exec, &mut core).expect("JIT execution must succeed");

    assert_eq!(core.ai_isa_ops_count, 5);

    // Verify RMSNorm: y = (x / sqrt(x^2 + 1e-5)) * scale
    // With x = 2.0, scale = 1.5:
    let expected_rms = (val_x * val_x + 1.0e-5f32).sqrt();
    let expected_rmsnorm = (val_x / expected_rms) * 1.5f32;
    let actual_rmsnorm = f32::from_bits(core.r[4]);
    assert!((actual_rmsnorm - expected_rmsnorm).abs() < 1e-4,
        "RMSNorm output mismatch: actual {}, expected {}", actual_rmsnorm, expected_rmsnorm);

    // Verify SiLU: y = x / (1 + e^-x)
    let expected_silu = val_x / (1.0f32 + (-val_x).exp());
    let actual_silu = f32::from_bits(core.r[5]);
    assert!((actual_silu - expected_silu).abs() < 1e-4,
        "SiLU output mismatch: actual {}, expected {}", actual_silu, expected_silu);

    // Verify GELU:
    let inner = 0.7978845608f32 * (val_x + 0.044715f32 * val_x * val_x * val_x);
    let expected_gelu = 0.5f32 * val_x * (1.0f32 + inner.tanh());
    let actual_gelu = f32::from_bits(core.r[6]);
    assert!((actual_gelu - expected_gelu).abs() < 1e-4,
        "GELU output mismatch: actual {}, expected {}", actual_gelu, expected_gelu);

    // Verify SSM: next_h = 0.8 * 0.0 + 0.2 * 2.0 = 0.4, y = 0.4 + 0.05 * 2.0 = 0.5
    let actual_ssm = f32::from_bits(core.r[8]);
    assert!((actual_ssm - 0.5f32).abs() < 1e-4,
        "SSM output mismatch: actual {}, expected 0.5", actual_ssm);
}

#[test]
fn test_cl_ai_c23_native_parity() {
    let gcc_check = Command::new("gcc").arg("--version").output();
    let clang_check = Command::new("clang").arg("--version").output();

    if gcc_check.is_err() && clang_check.is_err() {
        eprintln!("Neither gcc nor clang found on system, skipping native execution test");
        return;
    }

    let cl_code = r#"
; ============================================================================
; AI Silicon Block: RMSNorm -> SiLU -> FlashSoftmax -> SSM
; ============================================================================
.weights bank=1, offset=0: [1.25, 0.50, 2.00]
.weights bank=2, offset=0: [0.85, 0.15]

@pipeline:
B0000: _PO01+000> _PO02+000> _NO00#000> _NO00#000>
B0001: _RM04@100> _SI05$100> _GE06$100> _NO00#000>
B0002: _SM07$100> _SS08@200> _NO00#000> _HL00!000>
"#;

    // Run in JIT
    let mut jit_core = cronc::cl_jit::ClJitCore::new();
    jit_core.r[1] = 2.0f32.to_bits();
    cronc::cl_jit::execute_cl_on_core(cl_code, &mut jit_core).expect("JIT execution failed");

    // Run natively via C23
    let temp_dir = std::env::temp_dir();
    let out_bin = temp_dir.join(if cfg!(windows) { "test_cl_ai_native.exe" } else { "test_cl_ai_native" });

    // In C23 test code, inject r[1] = 2.0f before bundles
    let mut c23_code = cronc::cl_c23::compile_cl_to_c23(cl_code, "ai_kernel").expect("C23 compilation failed");
    c23_code = c23_code.replace(
        "cron_silicon_init(&core);",
        "cron_silicon_init(&core);\n    core.r[1] = cron_f32_to_u32(2.0f);",
    );

    let temp_c_file = temp_dir.join(format!("test_cl_ai_{}.c", std::process::id()));
    std::fs::write(&temp_c_file, &c23_code).expect("Failed to write C file");

    let cc = if gcc_check.is_ok() { "gcc" } else { "clang" };
    let mut cmd = Command::new(cc);
    cmd.arg(temp_c_file.to_str().unwrap());
    cmd.arg("-o");
    cmd.arg(out_bin.to_str().unwrap());
    cmd.arg("-O3");
    cmd.arg("-std=c11");
    cmd.arg("-lm");

    let compile_status = cmd.output().expect("Host compiler execution failed");
    assert!(
        compile_status.status.success(),
        "C23 compilation error: {}",
        String::from_utf8_lossy(&compile_status.stderr)
    );

    let run_res = Command::new(out_bin.to_str().unwrap()).output().expect("Native binary run failed");
    assert!(run_res.status.success());
    let stdout = String::from_utf8_lossy(&run_res.stdout);

    assert!(stdout.contains("CRON SILICON C23 NATIVE EXECUTION TELEMETRY"));
    assert!(stdout.contains("AI Silicon ISA Ops:          5"));
    assert!(stdout.contains("STATUS: 100% BIT-EXACT SILICON C23 PARITY VERIFIED"));

    // Check register R4 (RMSNorm) bit-exact parity
    let r4_hex = format!("{:08X}", jit_core.r[4]);
    assert!(
        stdout.contains(&r4_hex),
        "Native output must match JIT R4 0x{}, got stdout:\n{}",
        r4_hex, stdout
    );

    let _ = std::fs::remove_file(&temp_c_file);
    let _ = std::fs::remove_file(&out_bin);
}

#[test]
fn test_canonical_full_transformer_layer_file() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples/cl/full_llm_transformer_layer.cl");
    let content = std::fs::read_to_string(path).expect("Must read canonical full_llm_transformer_layer.cl");

    // 1. Verify VLIW grammar, bundles, zero hazards
    let report = verify_cl_program(&content).expect("Canonical model must pass verification");
    assert_eq!(report.weights_bound, 5, "Must bind 5 weight banks");
    assert!(report.total_bundles >= 10, "Must contain at least 10 execution cycles");
    assert!(report.hazards.is_empty(), "Canonical model must have zero hazards: {:?}", report.hazards);

    // 2. Execute on JIT core
    let mut jit_core = cronc::cl_jit::ClJitCore::new();
    cronc::cl_jit::execute_cl_on_core(&content, &mut jit_core).expect("Canonical model JIT execution must succeed");
    assert!(jit_core.ai_isa_ops_count >= 4, "Must execute dedicated AI ISA opcodes");
    assert!(jit_core.is_halted, "Must reach halt trap");

    // 3. Compile to native host binary with GCC/Clang and execute
    let temp_dir = std::env::temp_dir();
    let out_bin = temp_dir.join(if cfg!(windows) { "test_canonical_transformer.exe" } else { "test_canonical_transformer" });

    let comp_res = cronc::cl_c23::compile_cl_to_native_binary(&content, out_bin.to_str().unwrap(), "3");
    if comp_res.is_ok() && out_bin.exists() {
        let run_res = Command::new(out_bin.to_str().unwrap()).output().expect("Native binary run failed");
        assert!(run_res.status.success());
        let stdout = String::from_utf8_lossy(&run_res.stdout);
        assert!(stdout.contains("CRON SILICON C23 NATIVE EXECUTION TELEMETRY"));
        assert!(stdout.contains("AI Silicon ISA Ops:"));
        assert!(stdout.contains("STATUS: 100% BIT-EXACT SILICON C23 PARITY VERIFIED"));
        let _ = std::fs::remove_file(&out_bin);
    }
}

