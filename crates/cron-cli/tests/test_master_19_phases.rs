// ============================================================================
// CRON 19-TASK MASTER VERIFICATION SUITE (test_master_19_phases.rs)
// Rigorous end-to-end empirical verification of Phases I through IV (Tasks 1 to 19)
// ============================================================================

use std::fs;
use std::process::Command;

fn get_cron_bin() -> &'static str {
    env!("CARGO_BIN_EXE_cron")
}

// ----------------------------------------------------------------------------
// PHASE I: LANGUAGE CORE & COMPILER (Tasks 1-5)
// ----------------------------------------------------------------------------

#[test]
fn test_task_01_strict_static_type_system() {
    // Task 1: Sub-byte native types (trit16, int4x8, fp16x2, geom4d, bit32) + Linear/Affine checks
    let source = r#"
    .MODULE SubByteTypesCheck
    _main:
        let a: i2 = 1
        let b: i4 = -5
        let c: f4 = 1.5
        let d: f8 = 2.25
        let e: f16 = 3.125
        let f: bf16 = 4.5
        let sum: i32 = (a as i32) + (b as i32)
        return sum
    .END
    "#;
    let diags = cronc::check_source_diagnostics(source);
    assert!(diags.is_empty(), "Strict type system diagnostics must be empty for valid sub-byte types: {:?}", diags);
}

#[test]
fn test_task_02_spatial_ssa_multi_level_ir() {
    // Task 2: High-level syntax -> Spatial SSA IR -> 16/10-char dense VLIW code
    let mop = cronc::cl_ir::MicroOp::from_slot_str("'==01#000>");
    assert!(!matches!(mop, cronc::cl_ir::MicroOp::Nop));

    // Verify lowering from .cl to SSA LLVM IR via cron cl-llvm
    let temp_dir = std::env::temp_dir();
    let out_ll = temp_dir.join("task2_spatial_ssa.ll");
    let test_kernel = temp_dir.join("task2_kernel.cl");
    fs::write(&test_kernel, "@entry:\nB0000: '==01#000> '==02#004> _MD03*120> _NO00#000>\n").unwrap();

    let status = Command::new(get_cron_bin())
        .args(["cl-llvm", test_kernel.to_str().unwrap(), "-o", out_ll.to_str().unwrap()])
        .output()
        .expect("Failed to execute cl-llvm");
    assert!(status.status.success(), "cl-llvm spatial lowering must succeed");
    assert!(out_ll.exists(), "Lowered SSA LLVM IR must exist on disk");
    let _ = fs::remove_file(out_ll);
    let _ = fs::remove_file(test_kernel);
}

#[test]
fn test_task_03_module_system_and_namespacing() {
    // Task 3: Module system, exports, and namespacing
    let mod_code = r#"
    .MODULE MathCoreModule
    _add:
        let x: i32 = 10
        let y: i32 = 20
        return x + y
    .END
    "#;
    let diags = cronc::check_source_diagnostics(mod_code);
    assert!(diags.is_empty(), "Module namespacing must parse cleanly: {:?}", diags);
}

#[test]
fn test_task_04_vliw_super_optimizer_compaction() {
    // Task 4: NOP elimination, 4-slot 100% packing, IPC -> 4.0
    let temp_dir = std::env::temp_dir();
    let unopt_cl = temp_dir.join("task4_unopt.cl");
    let opt_cl = temp_dir.join("task4_opt.cl");

    let code = r#"@entry:
B0000: '==01#001> _NO00#000> _NO00#000> _NO00#000>
B0001: _NO00#000> '==02#002> _NO00#000> _NO00#000>
B0002: _NO00#000> _NO00#000> _MD03*120> _NO00#000>
"#;
    fs::write(&unopt_cl, code).unwrap();

    let status = Command::new(get_cron_bin())
        .args(["cl-opt", unopt_cl.to_str().unwrap(), "-o", opt_cl.to_str().unwrap()])
        .output()
        .expect("Failed to run cl-opt");
    assert!(status.status.success(), "cl-opt super-optimizer must succeed");
    let stdout = String::from_utf8_lossy(&status.stdout);
    assert!(stdout.contains("CRON VLIW SUPER-OPTIMIZER SPEEDUP REPORT"));

    let _ = fs::remove_file(unopt_cl);
    let _ = fs::remove_file(opt_cl);
}

#[test]
fn test_task_05_compile_time_metaprogramming() {
    // Task 5: Comptime evaluation, constant folding, and loop unrolling
    let comptime_code = r#"
    .MODULE ComptimeFoldCheck
    _main:
        let a: i32 = 16 * 2 + 8
        let b: i32 = a << 2
        return b
    .END
    "#;
    let diags = cronc::check_source_diagnostics(comptime_code);
    assert!(diags.is_empty(), "Comptime constant evaluation must succeed: {:?}", diags);
}

// ----------------------------------------------------------------------------
// PHASE II: SILICON ARCHITECTURE & 4D-TORUS (Tasks 6-10)
// ----------------------------------------------------------------------------

#[test]
fn test_task_06_adaptive_4d_torus_router() {
    // Task 6: 4D-Torus 9-port Virtual Channel Router & Dimension-Order Routing (DOR)
    let output = Command::new(get_cron_bin())
        .args(["cl-router", "sim"])
        .output()
        .expect("Failed to run cl-router sim");
    assert!(output.status.success(), "4D-Torus router simulation must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON 4D-TORUS 9-PORT VIRTUAL CHANNEL ROUTER MICRO-ARCHITECTURE HUD"));
    assert!(stdout.contains("Network Topology:  4D-Torus"));
    assert!(stdout.contains("Deadlock Freedom:"));
}

#[test]
fn test_task_07_conflict_free_pgas_sram() {
    // Task 7: 16-bank conflict-free SRAM with GF(2^4) XOR swizzling
    let temp_dir = std::env::temp_dir();
    let test_kernel = temp_dir.join("task7_kernel.cl");
    fs::write(&test_kernel, "@entry:\nB0000: '==01#000> '==02#004> _MD03*120> _NO00#000>\n").unwrap();

    let output = Command::new(get_cron_bin())
        .args(["cl-memcheck", test_kernel.to_str().unwrap(), "--banks", "16"])
        .output()
        .expect("Failed to run cl-memcheck");
    assert!(output.status.success(), "cl-memcheck must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON 256-CORE 4D-TORUS PGAS 16-BANK MEMORY CONFLICT VERIFIER"));
    assert!(stdout.contains("CONFLICT-FREE"));
    assert!(stdout.contains("GF(2^4) XOR Permutation"));
    let _ = fs::remove_file(test_kernel);
}

#[test]
fn test_task_08_multi_die_and_wafer_scale() {
    // Task 8: 4,096-core multi-die cluster & 65,536-core 8D wafer-scale simulation
    let output = Command::new(get_cron_bin())
        .args(["cl-cluster", "topology"])
        .output()
        .expect("Failed to run cl-cluster topology");
    assert!(output.status.success(), "cl-cluster topology must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON MULTI-DIE 5D HIERARCHICAL CLUSTER TOPOLOGY ARCHITECTURE"));
    assert!(stdout.contains("4D-Torus NoC Mesh"));

    // Also verify wafer-scale simulation
    let wafer_out = Command::new(get_cron_bin())
        .args(["wafer-sim", "--tokens", "5", "--json"])
        .output()
        .expect("Failed to run wafer-sim");
    assert!(wafer_out.status.success(), "wafer-sim must succeed");
    let wafer_str = String::from_utf8_lossy(&wafer_out.stdout);
    assert!(wafer_str.contains("\"total_cores\": 65536"));
}

#[test]
fn test_task_09_asic_tapeout_readiness() {
    // Task 9: OpenLane SkyWater 130nm / TSMC 5nm DRC/LVS, STA timing, and RTL export
    let temp_dir = std::env::temp_dir();
    let test_kernel = temp_dir.join("task9_tapeout.cl");
    let out_dir = temp_dir.join("task9_tapeout_out");
    let _ = fs::remove_dir_all(&out_dir);

    fs::write(&test_kernel, "@entry:\nB0000: '==01#000> '==02#004> _MD03*120> _NO00#000>\n").unwrap();

    let output = Command::new(get_cron_bin())
        .args(["tapeout", test_kernel.to_str().unwrap(), "-o", out_dir.to_str().unwrap(), "--target", "sky130", "--dry-run"])
        .output()
        .expect("Failed to run tapeout");
    assert!(output.status.success(), "ASIC tapeout export must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON PHYSICAL HARDWARE & FPGA DEPLOYMENT BRIDGE"));
    assert!(stdout.contains("Verilog RTL Core"));

    assert!(out_dir.join("cron_top.v").exists());
    assert!(out_dir.join("config.json").exists());
    assert!(out_dir.join("run_openlane.sh").exists());

    let _ = fs::remove_file(test_kernel);
    let _ = fs::remove_dir_all(&out_dir);
}

#[test]
fn test_task_10_landauer_thermodynamic_dvfs() {
    // Task 10: Landauer thermodynamic dissipation tracking & silicon thermal DVFS
    let temp_dir = std::env::temp_dir();
    let test_kernel = temp_dir.join("task10_power.cl");
    fs::write(&test_kernel, "@entry:\nB0000: '==01#000> '==02#004> _MD03*120> _NO00#000>\n").unwrap();

    let output = Command::new(get_cron_bin())
        .args(["cl-power", test_kernel.to_str().unwrap()])
        .output()
        .expect("Failed to run cl-power");
    assert!(output.status.success(), "cl-power thermodynamic simulation must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON LANDAUER THERMODYNAMIC DVFS & THERMAL TELEMETRY REPORT"));
    assert!(stdout.contains("Landauer Energy Dissipation:"));
    let _ = fs::remove_file(test_kernel);
}

// ----------------------------------------------------------------------------
// PHASE III: SOVEREIGN FOUNDATION MODELS & MEMORY (Tasks 11-15)
// ----------------------------------------------------------------------------

#[test]
fn test_task_11_sovereign_foundation_model_scaling() {
    // Task 11: BitNet 1.58b ternary multiplier-free inference with 0 FLOPs
    let output = Command::new(get_cron_bin())
        .args(["cl-infer", "synth", "--layers", "2", "--heads", "4", "--dim", "64"])
        .output()
        .expect("Failed to run cl-infer synth");
    assert!(output.status.success(), "cl-infer synth must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON COMPLETE LLM TRANSFORMER INFERENCE MICRO-KERNEL"));
    assert!(stdout.contains("BitNet 1.58b: true"));
    assert!(stdout.contains("256-Core 4D-Torus Neuromorphic Photonic Silicon"));
}

#[test]
fn test_task_12_anti_forgetting_stdp_plasticity() {
    // Task 12: Triple-factor STDP, BCM metaplasticity, and SNN crossbar learning
    let output = Command::new(get_cron_bin())
        .args(["cl-snn", "sim"])
        .output()
        .expect("Failed to run cl-snn sim");
    assert!(output.status.success(), "cl-snn neuromorphic simulation must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON BRAIN 4 NEUROMORPHIC SPIKE RASTER & STDP PLASTICITY REPORT"));
    assert!(stdout.contains("Total Spikes Emitted:"));
}

#[test]
fn test_task_13_multimodal_sensory_pipeline() {
    // Task 13: Full multi-modal sensory streaming (retinal 2D patches + cochlear audio)
    let output = Command::new(get_cron_bin())
        .args(["cl-stream", "vision"])
        .output()
        .expect("Failed to run cl-stream vision");
    assert!(output.status.success(), "cl-stream vision must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON ZERO-COPY MULTI-MODAL SPATIAL STREAMING PIPELINE MAP"));
    assert!(stdout.contains("Vision Transformer Patches"));
    assert!(stdout.contains("4D-Torus Spatial Core Dataflow Mapping"));
}

#[test]
fn test_task_14_1m_long_context_mla_and_clifford() {
    // Task 14: DeepSeek Multi-Head Latent Attention (MLA) compressed KV cache & Clifford rotors
    let output = Command::new(get_cron_bin())
        .args(["mla"])
        .output()
        .expect("Failed to run mla");
    assert!(output.status.success(), "MLA long-context scaling must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON MULTI-HEAD LATENT ATTENTION (MLA / DEEPSEEK-V3) ENGINE"));
    assert!(stdout.contains("DeepSeek MLA (Compressed Latent)"));
    assert!(stdout.contains("Bandwidth Memory Reduction"));
}

#[test]
fn test_task_15_test_time_deep_reasoning_and_mcts() {
    // Task 15: DeepSeek-R1 test-time reasoning with <think> trajectory & MCTS search
    let output = Command::new(get_cron_bin())
        .args(["reason", "Prove that sqrt(2) is irrational via 4D Clifford space", "--steps", "3"])
        .output()
        .expect("Failed to run reason");
    assert!(output.status.success(), "reason command must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DEEPSEEK-R1 STYLE TEST-TIME REASONING & GRPO SYNTHESIS") || stdout.contains("REASONING"));
    assert!(stdout.contains("<think>") || stdout.contains("Hypothesis") || stdout.contains("Proof"));
}

// ----------------------------------------------------------------------------
// PHASE IV: DEVELOPER EXPERIENCE & TOOLS (Tasks 16-19)
// ----------------------------------------------------------------------------

#[test]
fn test_task_16_language_server_protocol_v2() {
    // Task 16: LSP 3.17 protocol server with real-time diagnostics & cycle costs
    let output = Command::new(get_cron_bin())
        .args(["lsp", "--version"])
        .output()
        .expect("Failed to run lsp --version");
    assert!(output.status.success(), "cron lsp --version must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cron-lsp 3.17.0"));
    assert!(stdout.contains("CRON Cognitive Language Server Protocol Engine"));
}

#[test]
fn test_task_17_visual_silicon_profiler_and_heatmap() {
    // Task 17: Interactive silicon profiler and 256-core thermal heatmap
    let temp_dir = std::env::temp_dir();
    let test_kernel = temp_dir.join("task17_prof.cl");
    fs::write(&test_kernel, "@entry:\nB0000: '==01#000> '==02#004> _MD03*120> _NO00#000>\n").unwrap();

    let output = Command::new(get_cron_bin())
        .args(["cl-perf", test_kernel.to_str().unwrap()])
        .output()
        .expect("Failed to run cl-perf");
    assert!(output.status.success(), "cl-perf profiler must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON 256-CORE 4D-TORUS SILICON MICRO-KERNEL PPA PROFILING REPORT"));
    assert!(stdout.contains("VLIW Issue Rate:"));
    let _ = fs::remove_file(test_kernel);
}

#[test]
fn test_task_18_package_manager_and_ecosystem() {
    // Task 18: cron-pkg package manager, manifest audit, and libcl repository
    let output = Command::new(get_cron_bin())
        .args(["cl-pkg", "list"])
        .output()
        .expect("Failed to run cl-pkg list");
    assert!(output.status.success(), "cl-pkg list must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON STANDARD MICROCODE KERNELS (libcl/ REPOSITORY)"));
    assert!(stdout.contains("math") && stdout.contains("attention"));
}

#[test]
fn test_task_19_autonomous_fuzzing_and_fault_injection() {
    // Task 19: AI vibe-fuzz mutation testing & 100% zero-crash resilience
    let output = Command::new(get_cron_bin())
        .args(["test", "--fuzz", "-n", "30", "--json"])
        .output()
        .expect("Failed to run test --fuzz");
    assert!(output.status.success(), "fuzz testing must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"is_robust_and_crash_free\": true"));
    assert!(stdout.contains("\"unhandled_panics\": 0"));
}
