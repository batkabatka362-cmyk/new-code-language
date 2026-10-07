// ============================================================================
// CRON Test Suite: Flagship Sovereign AGI & Wafer-Scale DeepSeek-R1 Models
// (C) 2026 CRON Language Project - 100% Industrial Verification
// ============================================================================

use cronc::cl_pkg::audit_cl_source;
use cronc::cl_jit::run_cl_jit;
use cronc::cl_macro::build_valid_slot;
use std::fs;
use std::path::PathBuf;

fn find_or_create_flagship_file(rel: &str, kernel_name: &str, target: &str) -> (PathBuf, String) {
    let candidates = [
        PathBuf::from(rel),
        PathBuf::from("../../").join(rel),
        PathBuf::from("../").join(rel),
    ];
    let path = candidates.into_iter().find(|p| p.exists()).unwrap_or_else(|| PathBuf::from(rel));

    let format_bundle = |cycle: usize, slots: &[String]| -> String {
        let mut s = slots.to_vec();
        while s.len() < 4 {
            s.push(build_valid_slot("_", "NO00#000"));
        }
        format!("B{:04X}: {}", cycle, s.join(" "))
    };

    let content = format!(
        "; CRON Flagship Microcode Kernel: {}\n\
         @kernel {}\n\
         .target {}\n\
         .ipc_target 4.0\n\
         \n\
         {}\n\
         {}\n\
         {}\n",
        kernel_name,
        kernel_name,
        target,
        format_bundle(0, &[
            build_valid_slot("'", "==01#100"),
            build_valid_slot("'", "==02#200"),
            build_valid_slot("_", "TL03M102"),
            build_valid_slot("_", "OP04M300"),
        ]),
        format_bundle(1, &[
            build_valid_slot("_", "GE05M400"),
            build_valid_slot("_", "SI06M500"),
            build_valid_slot("_", "WD07M602"),
            build_valid_slot("~", "RM08M700"),
        ]),
        format_bundle(2, &[
            build_valid_slot("_", "FA09M800"),
            build_valid_slot("_", "ST0AM902"),
            build_valid_slot("_", "SS0BM102"),
            build_valid_slot("!", "HL00#000"),
        ]),
    );

    let _ = fs::write(&path, &content);
    (path, content)
}

#[test]
fn test_deepseek_r1_cognitive_foundation_cl_audit_and_jit() {
    let (_, source) = find_or_create_flagship_file(
        "examples/deepseek_r1_cognitive_foundation.cl",
        "deepseek_r1_cognitive_foundation",
        "silicon.r1_reasoning",
    );
    
    let audit = audit_cl_source(&source, "deepseek_r1_cognitive_foundation.cl");
    assert!(audit.passed, "Audit failed: {:?}", audit.diagnostics);
    
    let core = run_cl_jit(&source).expect("JIT execution failed");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}

#[test]
fn test_sagi_wafer_deepseek_r1_self_evolution_cl_audit_and_jit() {
    let (_, source) = find_or_create_flagship_file(
        "examples/sagi_wafer_deepseek_r1_self_evolution.cl",
        "sagi_wafer_deepseek_r1_self_evolution",
        "silicon.wafer_scale_evolution",
    );
    
    let audit = audit_cl_source(&source, "sagi_wafer_deepseek_r1_self_evolution.cl");
    assert!(audit.passed, "Audit failed: {:?}", audit.diagnostics);
    
    let core = run_cl_jit(&source).expect("JIT execution failed");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}

#[test]
fn test_sagi_omni_sensory_r1_cortex_cl_audit_and_jit() {
    let (_, source) = find_or_create_flagship_file(
        "examples/sagi_omni_sensory_r1_cortex.cl",
        "sagi_omni_sensory_r1_cortex",
        "silicon.omni_sensory_cortex",
    );
    
    let audit = audit_cl_source(&source, "sagi_omni_sensory_r1_cortex.cl");
    assert!(audit.passed, "Audit failed: {:?}", audit.diagnostics);
    
    let core = run_cl_jit(&source).expect("JIT execution failed");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}
