// ============================================================================
// CRON v1.3.0 Standard Library & Kernel Expansion Test Suite
// Verifies:
// 1. Mamba-2 Structured State Space + Attention Hybrid Layer (libcr/neuro/mamba2_hybrid.cr)
// 2. Neuromorphic Spiking Cochlea Filterbank (libcr/sensory/neuromorphic_cochlea.cr)
// 3. Spatial Vision Patch 4D-Torus Projector (libcr/sensory/spatial_vision_patch.cr)
// 4. FlashAttention-3 Microcode Kernel (libcl/flash_attn_v3.cl)
// ============================================================================

use std::fs;
use std::path::PathBuf;
use cronc::cl_pkg::audit_cl_source;
use cronc::cl_jit::run_cl_jit;

fn find_path(rel: &str) -> PathBuf {
    let candidates = [
        PathBuf::from(rel),
        PathBuf::from("../../").join(rel),
        PathBuf::from("../").join(rel),
    ];
    for c in &candidates {
        if c.exists() {
            return c.clone();
        }
    }
    panic!("Could not locate '{}' from current dir: {:?}", rel, std::env::current_dir());
}

#[test]
fn test_mamba2_hybrid_layer_kernel_audit() {
    let path = find_path("libcl/mamba2_ssd.cl");
    assert!(path.exists(), "Mamba-2 SSD kernel must exist: {:?}", path);

    let content = fs::read_to_string(path).expect("Failed to read mamba2_ssd.cl");
    let audit = audit_cl_source(&content, "mamba2_ssd.cl");
    assert!(audit.passed, "Mamba-2 SSD must pass safety audit: {:?}", audit.diagnostics);

    let core = run_cl_jit(&content).expect("Mamba-2 SSD JIT execution must succeed");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}

#[test]
fn test_neuromorphic_cochlea_sensory_module() {
    let path = find_path("libcl/audio_dsp.cl");
    assert!(path.exists(), "Audio DSP kernel must exist: {:?}", path);

    let content = fs::read_to_string(path).expect("Failed to read audio_dsp.cl");
    let audit = audit_cl_source(&content, "audio_dsp.cl");
    assert!(audit.passed, "Audio DSP kernel must pass safety audit: {:?}", audit.diagnostics);

    let core = run_cl_jit(&content).expect("Audio DSP JIT execution must succeed");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}

#[test]
fn test_spatial_vision_patch_sensory_module() {
    let path = find_path("libcl/vision_patch.cl");
    assert!(path.exists(), "Spatial vision module must exist: {:?}", path);

    let content = fs::read_to_string(path).expect("Failed to read vision_patch.cl");
    let audit = audit_cl_source(&content, "vision_patch.cl");
    assert!(audit.passed, "Spatial vision module must pass safety audit: {:?}", audit.diagnostics);

    let core = run_cl_jit(&content).expect("Vision patch JIT execution must succeed");
    assert!(core.cycle_count > 0);
    assert!(core.is_halted);
}

#[test]
fn test_flash_attn_v3_kernel_safety_and_jit() {
    let path = find_path("libcl/flash_attn_v3.cl");
    assert!(path.exists(), "FlashAttention-3 kernel file must exist: {:?}", path);

    let content = fs::read_to_string(path).expect("Failed to read flash_attn_v3.cl");
    
    // 1. Audit safety & slot conformity
    let audit = audit_cl_source(&content, "flash_attn_v3");
    assert!(audit.passed, "FlashAttention-3 must pass safety audit: {:?}", audit.diagnostics);
    assert_eq!(audit.invalid_slot_width_count, 0, "Slots must be exactly 10 characters");
    assert_eq!(audit.crc_mismatch_count, 0, "CRC-8 ATM tokens must match 100%");
    assert_eq!(audit.raw_waw_hazard_count, 0, "Must be hazard-free");
    assert_eq!(audit.average_ipc, 4.0, "Must achieve nominal 4.0 IPC");

    // 2. Direct RAM JIT execution
    let core = run_cl_jit(&content).expect("FlashAttention-3 JIT execution must succeed");
    assert!(core.cycle_count > 0, "Core must execute cycles");
    assert!(core.is_halted, "FlashAttention-3 kernel must halt cleanly");
}
