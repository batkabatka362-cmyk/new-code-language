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
use cronc::lexer::Lexer;
use cronc::parser::Parser;
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
fn test_mamba2_hybrid_layer_syntax_and_ast() {
    let path = find_path("libcr/neuro/mamba2_hybrid.cr");
    assert!(path.exists(), "Mamba-2 hybrid source file must exist: {:?}", path);

    let content = fs::read_to_string(path).expect("Failed to read mamba2_hybrid.cr");
    let mut lexer = Lexer::new(&content);
    let tokens = lexer.tokenize().expect("Lexing mamba2_hybrid.cr failed");
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing mamba2_hybrid.cr failed");
    
    // Check that mamba2 functions and structs are parsed
    assert!(!program.structs.is_empty(), "Mamba-2 module must declare structs");
    assert!(!program.functions.is_empty(), "Mamba-2 module must declare functions");
    assert_eq!(program.module_name, "cron.neuro.mamba2_hybrid");
}

#[test]
fn test_neuromorphic_cochlea_sensory_module() {
    let path = find_path("libcr/sensory/neuromorphic_cochlea.cr");
    assert!(path.exists(), "Cochlea module must exist: {:?}", path);

    let content = fs::read_to_string(path).expect("Failed to read neuromorphic_cochlea.cr");
    let mut lexer = Lexer::new(&content);
    let tokens = lexer.tokenize().expect("Lexing neuromorphic_cochlea.cr failed");
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing neuromorphic_cochlea.cr failed");
    assert!(!program.structs.is_empty(), "Cochlea module must declare structs");
    assert!(!program.functions.is_empty(), "Cochlea module must declare functions");
    assert_eq!(program.module_name, "cron.sensory.neuromorphic_cochlea");
}

#[test]
fn test_spatial_vision_patch_sensory_module() {
    let path = find_path("libcr/sensory/spatial_vision_patch.cr");
    assert!(path.exists(), "Spatial vision module must exist: {:?}", path);

    let content = fs::read_to_string(path).expect("Failed to read spatial_vision_patch.cr");
    let mut lexer = Lexer::new(&content);
    let tokens = lexer.tokenize().expect("Lexing spatial_vision_patch.cr failed");
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing spatial_vision_patch.cr failed");
    assert!(!program.structs.is_empty(), "Spatial vision module must declare structs");
    assert!(!program.functions.is_empty(), "Spatial vision module must declare functions");
    assert_eq!(program.module_name, "cron.sensory.spatial_vision_patch");
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
