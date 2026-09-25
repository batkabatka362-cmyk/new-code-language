// ============================================================================
// CRON .cl Ecosystem, Standard Microcode Kernel Library (libcl) & Package Manager Test Suite
// Verifies:
// 1. Standard Microcode Kernels in `libcl/` (math, optical_gemm, hdc_vsa, attention, stdp, crypto)
// 2. `.cl` Package Manifest Creation & Resolution (`cl_pkg`)
// 3. 10-Character Slot Width & CRC-8 ATM Silicon Safety Audit
// 4. In-Memory Microcode Benchmark Suite & Throughput Metrics (`cl_benchmark`)
// ============================================================================

use std::fs;
use std::path::Path;
use cronc::cl_pkg::{init_cl_package, list_standard_kernels, audit_cl_source, generate_standard_libcl_files};
use cronc::cl_benchmark::run_libcl_benchmarks;
use cronc::cl_jit::run_cl_jit;

#[test]
fn test_libcl_standard_kernels_integrity_and_execution() {
    let workspace = Path::new(".");
    let libcl_dir = workspace.join("libcl");
    let _ = generate_standard_libcl_files(&libcl_dir);
    let kernels = list_standard_kernels(workspace);
    assert!(!kernels.is_empty(), "Must find standard kernels in libcl");

    for k in &kernels {
        let path = Path::new(&k.path);
        assert!(path.exists(), "Kernel file must exist: {}", k.path);

        let content = fs::read_to_string(&path).expect("Must read kernel .cl");
        
        // 1. Audit safety
        let audit = audit_cl_source(&content, &k.name);
        assert!(audit.passed, "Kernel {} failed safety audit: {:?}", k.name, audit.diagnostics);
        assert_eq!(audit.invalid_slot_width_count, 0, "All slots must be exactly 10 characters");
        assert_eq!(audit.crc_mismatch_count, 0, "CRC-8 ATM tokens must match 100%");
        assert_eq!(audit.raw_waw_hazard_count, 0, "Must be hazard-free");
        assert_eq!(audit.average_ipc, 4.0, "Must achieve 4.00 nominal IPC");

        // 2. Direct RAM JIT execution
        let core = run_cl_jit(&content).expect("JIT execution must succeed");
        assert!(core.cycle_count > 0, "Core must execute cycles");
        assert!(core.is_halted, "Kernel must halt cleanly");
    }
}

#[test]
fn test_cl_package_manager_init_and_audit() {
    let temp_dir = std::env::temp_dir().join("test_cron_cl_pkg_init");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Create temp dir");

    let manifest_path = init_cl_package(&temp_dir, "my_neural_kernel")
        .expect("Package initialization must succeed");
    assert!(manifest_path.exists());

    let kernel_path = temp_dir.join("src").join("kernel.cl");
    assert!(kernel_path.exists());

    let kernel_code = fs::read_to_string(&kernel_path).expect("Read starter kernel");
    let audit = audit_cl_source(&kernel_code, "my_neural_kernel");
    assert!(audit.passed, "Starter kernel must pass audit: {:?}", audit.diagnostics);
    assert_eq!(audit.invalid_slot_width_count, 0);
    assert_eq!(audit.crc_mismatch_count, 0);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_cl_benchmark_suite_execution() {
    let workspace = Path::new(".");
    let libcl_dir = workspace.join("libcl");
    let _ = generate_standard_libcl_files(&libcl_dir);
    let report = run_libcl_benchmarks(workspace, 100).expect("Benchmark suite must execute cleanly");

    assert!(report.total_kernels >= 18, "Must benchmark all 18 standard kernels");
    assert!(report.total_ops_executed > 0, "Must execute microcode operations");
    assert!(report.total_time_millis >= 0.0);

    for r in &report.results {
        assert!(r.throughput_mops > 0.0, "Throughput must be positive for {}", r.kernel_name);
        assert_eq!(r.ipc, 4.0, "IPC must be 4.00");
        assert!(r.energy_pj > 0.0, "Energy consumption must be calculated");
    }
}

#[test]
fn test_cl_pkg_search_functionality() {
    let workspace = Path::new(".");
    let libcl_dir = workspace.join("libcl");
    let _ = generate_standard_libcl_files(&libcl_dir);

    let quantum_matches = cronc::cl_pkg::search_standard_kernels(workspace, "quantum");
    assert!(!quantum_matches.is_empty(), "Must find quantum kernel");
    assert!(quantum_matches.iter().any(|k| k.name == "quantum_mzi"));

    let vision_matches = cronc::cl_pkg::search_standard_kernels(workspace, "vision");
    assert!(!vision_matches.is_empty(), "Must find vision kernel");
    assert!(vision_matches.iter().any(|k| k.name == "vision_patch"));

    let bio_matches = cronc::cl_pkg::search_standard_kernels(workspace, "homeostasis");
    assert!(!bio_matches.is_empty(), "Must find bio homeostasis kernel");
    assert!(bio_matches.iter().any(|k| k.name == "bio_homeostasis"));

    let tree_matches = cronc::cl_pkg::search_standard_kernels(workspace, "tree");
    assert!(!tree_matches.is_empty(), "Must find tree of thought kernel");
    assert!(tree_matches.iter().any(|k| k.name == "tree_of_thought"));
}

#[test]
fn test_cl_docgen_markdown_generation() {
    let workspace = Path::new(".");
    let libcl_dir = workspace.join("libcl");
    let _ = generate_standard_libcl_files(&libcl_dir);
    let temp_docs = std::env::temp_dir().join("test_cl_docs_out");
    let _ = fs::remove_dir_all(&temp_docs);

    let doc_files = cronc::cl_docgen::generate_directory_docs(&libcl_dir, &temp_docs)
        .expect("Doc generation must succeed");
    assert!(doc_files.len() >= 18, "Must generate docs for all 18 kernels");

    let math_doc = fs::read_to_string(temp_docs.join("math.md")).expect("Read math doc");
    assert!(math_doc.contains("# Microcode Kernel: `cordic_sincos_invsqrt`"));
    assert!(math_doc.contains("Nominal IPC"));

    let _ = fs::remove_dir_all(&temp_docs);
}

#[test]
fn test_cl_playground_live_stepper() {
    let cl_code = r#"
    @kernel test_kernel
    .target silicon.4d_torus
    B0000: '==01#0408 '==02#020k _AD03M102: _SB04M102)
    B0001: _ML05M304* _CD06M1025 _EX07M500? _SQ08M700!
    B0002: _FX09M8014 ~RM0AM900s _ST0BM102K !HL00#0000
    "#;

    let mut session = cronc::cl_playground::ClPlaygroundSession::new(cl_code);
    assert_eq!(session.bundles.len(), 3);

    let frames = session.run_all(10);
    assert_eq!(frames.len(), 3);
    assert!(frames[2].is_halted, "Kernel must halt at cycle 2");

    let hud = cronc::cl_playground::ClPlaygroundSession::render_frame(&frames[0]);
    assert!(hud.contains("CRON .cl MICROCODE PLAYGROUND"));
    assert!(hud.contains("HARDWARE REGISTERS (R0..RF)"));
}

#[test]
fn test_cl_spatial_pipeline_composer() {
    let stages = vec![
        cronc::cl_compose::CompositionStage {
            stage_name: "Sensory_Preprocessor".to_string(),
            kernel_name: "vision_patch".to_string(),
            core_coord: (0, 0, 0, 0),
            in_channel: 0,
            out_channel: 1,
            barrier_after: false,
        },
        cronc::cl_compose::CompositionStage {
            stage_name: "Recurrent_Reservoir".to_string(),
            kernel_name: "liquid_state".to_string(),
            core_coord: (1, 0, 0, 0),
            in_channel: 1,
            out_channel: 2,
            barrier_after: true,
        },
        cronc::cl_compose::CompositionStage {
            stage_name: "Neuro_Symbolic_Reasoner".to_string(),
            kernel_name: "neuro_symbolic".to_string(),
            core_coord: (2, 0, 0, 0),
            in_channel: 2,
            out_channel: 3,
            barrier_after: true,
        },
    ];

    let config = cronc::cl_compose::PipelineCompositionConfig {
        pipeline_name: "Autonomous_Living_Perception_Pipeline".to_string(),
        stages,
        insert_noc_wormhole_flits: true,
        auto_barrier_synchronization: true,
    };

    let report = cronc::cl_compose::compose_pipeline(&config).expect("Pipeline composition must succeed");
    assert_eq!(report.total_stages, 3);
    assert_eq!(report.total_allocated_cores, 3);
    assert!(report.total_cycles > 0);
    assert!(report.composed_cl.contains("B0000:"));
    assert!(report.composed_cl.contains("!HL00#000"));

    let hud = cronc::cl_compose::render_composition_ascii_hud(&report);
    assert!(hud.contains("CRON MULTI-KERNEL SPATIAL PIPELINE COMPOSITION"));
    assert!(hud.contains("Sensory_Preprocessor"));
    assert!(hud.contains("Recurrent_Reservoir"));
    assert!(hud.contains("Neuro_Symbolic_Reasoner"));
}

#[test]
fn test_cl_golden_reference_mathematical_verifications() {
    let report = cronc::cl_verify_math::run_mathematical_verifications();
    assert!(report.all_passed, "All golden reference math verifications must pass");
    assert_eq!(report.total_tests, 6);
    assert_eq!(report.passed_tests, 6);
    assert_eq!(report.failed_tests, 0);
    assert!(report.max_residual < 1e-3, "Residual must be within strict numerical tolerance");

    let hud = report.render_ascii_hud();
    assert!(hud.contains("CRON GOLDEN REFERENCE MATHEMATICAL PRECISION"));
    assert!(hud.contains("CORDIC Sine/Cosine Generation"));
    assert!(hud.contains("Photonic MZI Tensor Dot Product"));
    assert!(hud.contains("Softmax Gibbs-Boltzmann Normalization"));
    assert!(hud.contains("Quantum Unitary State Fidelity"));
    assert!(hud.contains("STDP Synaptic Hebbian Decay"));
    assert!(hud.contains("Neural ODE Euler Convergence"));
}

#[test]
fn test_cl_linter_and_static_hazard_analysis() {
    let s0 = cronc::build_valid_slot("'", "==01#040");
    let s1 = cronc::build_valid_slot("'", "==02#020");
    let s2 = cronc::build_valid_slot("'", "==03#010");
    let s3 = cronc::build_valid_slot("'", "==04#005");
    let s4 = cronc::build_valid_slot("_", "AD05M102");
    let s5 = cronc::build_valid_slot("_", "SB06M304");
    let s6 = cronc::build_valid_slot("_", "ML07M103");
    let s7 = cronc::build_valid_slot("!", "HL00#000");

    let clean_code = format!(
        "B0000: {} {} {} {}\nB0001: {} {} {} {}\n",
        s0, s1, s2, s3, s4, s5, s6, s7
    );
    let clean_rep = cronc::lint_cl_source(&clean_code, "clean.cl");
    assert!(clean_rep.is_clean, "Clean code must pass lint with zero errors/warnings: {:?}", clean_rep.issues);
    assert_eq!(clean_rep.total_bundles, 2);

    let buggy_code = format!(
        "B0000: {} {} {} {}\nB0001: {} _NO00#000> _NO00#000> _NO00#000>\nB0002: {} {} {} _NO00#000>\n",
        s0, s0, s2, s3, s7, s4, s5, s6
    );
    let buggy_rep = cronc::lint_cl_source(&buggy_code, "buggy.cl");
    assert!(!buggy_rep.is_clean, "Linter must catch WAW hazard and dead code");
    assert!(buggy_rep.dead_code_bundles > 0, "Must detect dead code bundle after HALT");
    assert!(buggy_rep.issues.iter().any(|i| i.rule_id == "L001_DEAD_CODE"));
    assert!(buggy_rep.issues.iter().any(|i| i.rule_id == "L005_INTRA_BUNDLE_WAW"));

    let hud = buggy_rep.render_ascii_hud();
    assert!(hud.contains("CRON .CL MICROCODE STATIC LINTER"));
}

#[test]
fn test_cl_autonomous_kernel_forge() {
    let prompt = "Synthesize fast attention kernel with FlashSoftmax";
    let cfg = cronc::ForgeConfig::default();
    let report = cronc::forge_kernel(prompt, &cfg).expect("Forge must synthesize kernel");

    assert_eq!(report.detected_domain, cronc::ForgeDomain::TransformerAttention);
    assert_eq!(report.ipc_rating, 4.0);
    assert!(report.verified_safe, "Forged kernel must pass JIT verification");
    assert!(report.cl_source.contains("B0000:"));
    assert!(report.cl_source.contains("!HL00#000"));

    let hud = report.ascii_hud;
    assert!(hud.contains("CRON AUTONOMOUS MICROCODE FORGE"));
}

#[test]
fn test_cl_cross_transpiler_wgsl_ptx_c23() {
    let cl_code = r#"
    B0000: '==01#0408 '==02#020k _AD03M102: _SB04M102)
    B0001: _ML05M304* _ST05M000~ _NO00#000> !HL00#0000
    "#;

    // 1. WebGPU WGSL
    let wgsl_rep = cronc::transpile_cl(cl_code, cronc::TranspileTarget::WebGpuWgsl)
        .expect("WGSL transpile must succeed");
    assert!(wgsl_rep.generated_code.contains("@compute @workgroup_size(256, 1, 1)"));
    assert!(wgsl_rep.generated_code.contains("pgas_memory[core_id * 16u + 0u] = r[5];"));

    // 2. NVIDIA PTX
    let ptx_rep = cronc::transpile_cl(cl_code, cronc::TranspileTarget::NvidiaPtx)
        .expect("PTX transpile must succeed");
    assert!(ptx_rep.generated_code.contains(".version 7.5"));
    assert!(ptx_rep.generated_code.contains(".entry cron_cl_ptx_kernel"));
    assert!(ptx_rep.generated_code.contains("add.u32"));

    // 3. ISO C23 + SIMD
    let c23_rep = cronc::transpile_cl(cl_code, cronc::TranspileTarget::C23Simd)
        .expect("C23 transpile must succeed");
    assert!(c23_rep.generated_code.contains("#include <stdint.h>"));
    assert!(c23_rep.generated_code.contains("void cron_cl_c23_execute("));
}
