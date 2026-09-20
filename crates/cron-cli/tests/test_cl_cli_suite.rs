// ============================================================================
// Integration Tests for CRON Standalone .cl CLI Commands
// ============================================================================

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_attention_cl_path() -> String {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let candidate = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");
    if candidate.exists() {
        candidate.to_str().unwrap().to_string()
    } else {
        "examples/cl/mini_transformer_attention.cl".to_string()
    }
}

#[test]
fn test_cli_cl_run_execution() {
    let cl_path = get_attention_cl_path();
    let output = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-run", &cl_path])
        .output()
        .expect("Failed to execute cron cl-run");

    assert!(output.status.success(), "cl-run should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON .cl DIRECT IN-MEMORY JIT MACHINE EXECUTION"));
    assert!(stdout.contains("Total Execution Cycles:       8 cycles"));
    assert!(stdout.contains("Photonic MZI Optical Ops:"));
    assert!(stdout.contains("STATUS: .cl JIT EXECUTED WITH 100% BIT-EXACT SILICON PARITY"));
}

#[test]
fn test_cli_cl_llvm_transpilation() {
    let cl_path = get_attention_cl_path();
    let temp_dir = std::env::temp_dir();
    let out_ll = temp_dir.join("test_cl_transpile.ll");

    let output = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-llvm", &cl_path, "-o", out_ll.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-llvm");

    assert!(output.status.success(), "cl-llvm should succeed");
    assert!(out_ll.exists(), "Output .ll file must exist");
    let content = fs::read_to_string(&out_ll).unwrap();
    assert!(content.contains("define void @cron_execute_bundles"));
    assert!(content.contains("define i32 @main"));

    let _ = fs::remove_file(&out_ll);
}

#[test]
fn test_cli_cl_verilog_synthesis() {
    let cl_path = get_attention_cl_path();
    let temp_dir = std::env::temp_dir();
    let out_v = temp_dir.join("test_cl_core.v");

    let output = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-verilog", &cl_path, "-o", out_v.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-verilog");

    assert!(output.status.success(), "cl-verilog should succeed");
    assert!(out_v.exists(), "Output .v file must exist");
    let content = fs::read_to_string(&out_v).unwrap();
    assert!(content.contains("module mini_transformer_attention"));
    assert!(content.contains("reg [31:0] inst_data;"));

    let _ = fs::remove_file(&out_v);
}

#[test]
fn test_cli_cl_heal_and_opt() {
    let temp_dir = std::env::temp_dir();
    let broken_file = temp_dir.join("broken_ai.cl");
    let healed_file = temp_dir.join("healed_ai.cl");
    let opt_file = temp_dir.join("optimized_ai.cl");

    // Unpadded bundle with missing CRC
    let broken_content = "B0000: '==01#004> _OP01$2?F>\n";
    fs::write(&broken_file, broken_content).unwrap();

    // 1. Run cl-heal
    let heal_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-heal", broken_file.to_str().unwrap(), "-o", healed_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute cl-heal");
    assert!(heal_out.status.success());
    assert!(healed_file.exists());
    let healed_text = fs::read_to_string(&healed_file).unwrap();
    assert!(healed_text.contains("_NO00#000>"), "Must be padded with NOPs");

    // 2. Run cl-opt
    let opt_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-opt", healed_file.to_str().unwrap(), "-o", opt_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute cl-opt");
    assert!(opt_out.status.success());
    assert!(opt_file.exists());

    let _ = fs::remove_file(&broken_file);
    let _ = fs::remove_file(&healed_file);
    let _ = fs::remove_file(&opt_file);
}

#[test]
fn test_cli_cl_direct_verify_and_flash() {
    let cl_path = get_attention_cl_path();

    // Verify directly on .cl
    let verify_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["verify", &cl_path])
        .output()
        .expect("Failed to run verify on .cl");
    assert!(verify_out.status.success());
    let stdout = String::from_utf8_lossy(&verify_out.stdout);
    assert!(stdout.contains("Parsing & Auditing .cl Machine Bundles"));
    assert!(stdout.contains("PROVABLY SAFE & THERMODYNAMICALLY BOUNDED"));

    // Flash directly from .cl
    let temp_dir = std::env::temp_dir();
    let deploy_dir = temp_dir.join("test_cl_deploy_output");
    let flash_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "flash",
            &cl_path,
            "--target",
            "xilinx_u280",
            "--out-dir",
            deploy_dir.to_str().unwrap(),
            "--dry-run",
        ])
        .output()
        .expect("Failed to run flash on .cl");
    assert!(flash_out.status.success());
    assert!(deploy_dir.join("cron_top.v").exists());
    assert!(deploy_dir.join("cron_pcie_dma.h").exists());

    let _ = fs::remove_dir_all(&deploy_dir);
}

#[test]
fn test_cli_vibe_spec_and_schema() {
    let temp_dir = std::env::temp_dir();
    let spec_file = temp_dir.join("test_vibe_spec.txt");
    let schema_file = temp_dir.join("test_schema.json");

    // 1. Test vibe-spec --format prompt
    let spec_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["vibe-spec", "--format", "prompt", "-o", spec_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute vibe-spec");
    assert!(spec_out.status.success());
    assert!(spec_file.exists());
    let spec_content = fs::read_to_string(&spec_file).unwrap();
    assert!(spec_content.contains("SYSTEM ROLE: SSS+ CRON .cl SILICON VIBE-CODING ENGINE"));

    // 2. Test cl-schema
    let schema_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-schema", "-o", schema_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute cl-schema");
    assert!(schema_out.status.success());
    assert!(schema_file.exists());
    let schema_content = fs::read_to_string(&schema_file).unwrap();
    assert!(schema_content.contains("\"CronCognitiveLowLevelLanguageSpec\""));

    let _ = fs::remove_file(&spec_file);
    let _ = fs::remove_file(&schema_file);
}

#[test]
fn test_cli_cl_link_multicore_pipeline() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let pipeline_cl = manifest_dir.join("../../examples/cl/multicore_4d_pipeline.cl");
    assert!(pipeline_cl.exists(), "multicore_4d_pipeline.cl must exist");

    let temp_dir = std::env::temp_dir();
    let out_clpack = temp_dir.join("pipeline.clpack");
    let out_c = temp_dir.join("pipeline_harness.c");
    let out_v = temp_dir.join("pipeline_top.v");

    let link_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-link",
            pipeline_cl.to_str().unwrap(),
            "-o",
            out_clpack.to_str().unwrap(),
            "--emit-c",
            out_c.to_str().unwrap(),
            "--emit-verilog",
            out_v.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cl-link");

    assert!(link_out.status.success(), "cl-link should succeed");
    let stdout = String::from_utf8_lossy(&link_out.stdout);
    assert!(stdout.contains("4D-TORUS SPATIAL LINKING REPORT"));
    assert!(stdout.contains("Active Cores Linked:          2 Cores"));
    assert!(stdout.contains("Inter-Core Channels:          1 NoC Communication Channels"));
    assert!(stdout.contains("Max Torus Hop Distance:       3 Hops"));
    assert!(stdout.contains("DOR Deadlock-Freedom Proof:   PROVEN ACYCLIC"));

    assert!(out_clpack.exists(), "Binary pack must be generated");
    assert!(out_c.exists(), "C23 harness must be generated");
    assert!(out_v.exists(), "Verilog top module must be generated");

    let _ = fs::remove_file(&out_clpack);
    let _ = fs::remove_file(&out_c);
    let _ = fs::remove_file(&out_v);
}

#[test]
fn test_cli_vibe_loop_json_and_inline() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let pipeline_cl = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");

    // 1. Test vibe-loop on file with --json output
    let output = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["vibe-loop", pipeline_cl.to_str().unwrap(), "--json"])
        .output()
        .expect("Failed to execute cron vibe-loop --json");

    assert!(output.status.success(), "vibe-loop should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"status\": \"SUCCESS\"") || stdout.contains("\"status\": \"HEALED_AND_EXECUTED\""));
    assert!(stdout.contains("\"execution\""));
    assert!(stdout.contains("\"optical_ops\""));
    assert!(stdout.contains("\"diagnostics\": []"));

    // 2. Test vibe-loop with inline code (--code)
    let inline_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "vibe-loop",
            "--code",
            "B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>\nB0001: _OP01$28F> _NO00#000> _NO00#000> _HL00#000!",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron vibe-loop --code");

    assert!(inline_out.status.success());
    let inline_stdout = String::from_utf8_lossy(&inline_out.stdout);
    assert!(inline_stdout.contains("\"status\": \"SUCCESS\"") || inline_stdout.contains("\"status\": \"HEALED_AND_EXECUTED\""));
    assert!(inline_stdout.contains("\"cycles\": 2"));
    assert!(inline_stdout.contains("\"optical_ops\": 1"));
}

#[test]
fn test_cli_cl_cosim_trace_and_json() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_cl = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");

    // 1. Test cl-cosim with ASCII trace HUD
    let trace_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-cosim", kernel_cl.to_str().unwrap(), "--trace"])
        .output()
        .expect("Failed to execute cron cl-cosim --trace");

    assert!(trace_out.status.success(), "cl-cosim --trace must exit with 0");
    let trace_str = String::from_utf8_lossy(&trace_out.stdout);
    assert!(trace_str.contains("CRON HARDWARE CO-SIMULATION BRIDGE"));
    assert!(trace_str.contains("100% BIT-EXACT HARDWARE PARITY"));
    assert!(trace_str.contains("Total Cycles: 8"));
    assert!(trace_str.contains("Final Hardware Register File Parity:"));

    // 2. Test cl-cosim with pure JSON telemetry
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-cosim", kernel_cl.to_str().unwrap(), "--json"])
        .output()
        .expect("Failed to execute cron cl-cosim --json");

    assert!(json_out.status.success(), "cl-cosim --json must exit with 0");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"total_cycles_simulated\": 8"));
    assert!(json_str.contains("\"is_100pct_parity\": true"));
    assert!(json_str.contains("\"divergence_count\": 0"));
    assert!(json_str.contains("\"bit_exact_match_count\": 8"));
    assert!(json_str.contains("\"final_rtl_regs\": ["));
}

#[test]
fn test_cli_cl_opt_level2_dag() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_cl = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");

    let output = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-opt", kernel_cl.to_str().unwrap(), "--level", "2"])
        .output()
        .expect("Failed to execute cron cl-opt --level 2");

    assert!(output.status.success(), "cl-opt --level 2 must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON VLIW SUPER-OPTIMIZER SPEEDUP REPORT"));
    assert!(stdout.contains("Optimization Level:           Level2"));
    assert!(stdout.contains("Critical Path Depth:"));
    assert!(stdout.contains("IPC After Compaction:"));
}

#[test]
fn test_cli_cl_memcheck_verification() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_cl = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");

    // 1. Test cl-memcheck with ASCII Report
    let ascii_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-memcheck", kernel_cl.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-memcheck");

    assert!(ascii_out.status.success(), "cl-memcheck must succeed on valid kernel");
    let ascii_str = String::from_utf8_lossy(&ascii_out.stdout);
    assert!(ascii_str.contains("CRON 256-CORE 4D-TORUS PGAS 16-BANK MEMORY CONFLICT VERIFIER"));
    assert!(ascii_str.contains("GF(2^4) XOR Permutation"));
    assert!(ascii_str.contains("PROVABLY CONFLICT-FREE"));
    assert!(ascii_str.contains("Total Ops:"));

    // 2. Test cl-memcheck with JSON telemetry
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-memcheck", kernel_cl.to_str().unwrap(), "--json"])
        .output()
        .expect("Failed to execute cron cl-memcheck --json");

    assert!(json_out.status.success(), "cl-memcheck --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"is_provably_conflict_free\": true"));
    assert!(json_str.contains("\"conflicted_cycles\": 0"));
    assert!(json_str.contains("\"total_stall_penalty_cycles\": 0"));
    assert!(json_str.contains("\"swizzling_enabled\": true"));
    assert!(json_str.contains("\"bank_access_histogram\": ["));

    // 3. Test conflict detection and strict mode rejection
    let temp_dir = std::env::temp_dir();
    let conflict_file = temp_dir.join("test_mem_conflict.cl");
    // Bundle with 2 parallel stores to linear bank 0: offset 0x00 and 0x40 (without swizzle, both bank 0)
    let conflict_cl = "B0000: _ST00#000> _ST00#040> _NO00#000> _HL00$008!\n";
    fs::write(&conflict_file, conflict_cl).unwrap();

    let strict_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-memcheck", conflict_file.to_str().unwrap(), "--no-swizzle", "--strict"])
        .output()
        .expect("Failed to execute cron cl-memcheck --strict");

    assert!(!strict_out.status.success(), "cl-memcheck --strict must fail when bank conflicts exist");
    let strict_str = String::from_utf8_lossy(&strict_out.stdout);
    assert!(strict_str.contains("SRAM BANK CONFLICT DETECTED") || strict_str.contains("SRAM BANK CONFLICT BREAKDOWN"));

    let _ = fs::remove_file(&conflict_file);
}

#[test]
fn test_cli_cl_fuzz_resilience() {
    // 1. Run cl-fuzz with ASCII output
    let ascii_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-fuzz", "--iterations", "100", "--seed", "0x12345678"])
        .output()
        .expect("Failed to execute cron cl-fuzz");

    assert!(ascii_out.status.success(), "cl-fuzz must succeed with zero crashes");
    let ascii_str = String::from_utf8_lossy(&ascii_out.stdout);
    assert!(ascii_str.contains("CRON AUTONOMOUS AI VIBE-FUZZ & MUTATION RESILIENCE REPORT"));
    assert!(ascii_str.contains("100% CRASH-RESILIENT"));
    assert!(ascii_str.contains("Total Fuzz Iterations:   100"));

    // 2. Run cl-fuzz with pure JSON output
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-fuzz", "--iterations", "50", "--json"])
        .output()
        .expect("Failed to execute cron cl-fuzz --json");

    assert!(json_out.status.success(), "cl-fuzz --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"total_iterations\": 50"));
    assert!(json_str.contains("\"unhandled_panics\": 0"));
    assert!(json_str.contains("\"is_robust_and_crash_free\": true"));
}

#[test]
fn test_cli_cl_bench_roofline() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_cl = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");

    // 1. Run cl-bench with ASCII Roofline graph
    let ascii_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-bench", kernel_cl.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-bench");

    assert!(ascii_out.status.success(), "cl-bench must succeed");
    let ascii_str = String::from_utf8_lossy(&ascii_out.stdout);
    assert!(ascii_str.contains("CRON 256-CORE 4D-TORUS HARDWARE ROOFLINE & PERFORMANCE BENCHMARK"));
    assert!(ascii_str.contains("Operating Regime:"));
    assert!(ascii_str.contains("Operational Intensity:"));
    assert!(ascii_str.contains("Silicon Roofline Model Curve:"));
    assert!(ascii_str.contains("80 GFLOPs"));

    // 2. Run cl-bench with JSON telemetry
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-bench", kernel_cl.to_str().unwrap(), "--json"])
        .output()
        .expect("Failed to execute cron cl-bench --json");

    assert!(json_out.status.success(), "cl-bench --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"total_cycles\": 8"));
    assert!(json_str.contains("\"optical_gemm_ops\": 2"));
    assert!(json_str.contains("\"operational_intensity_flop_per_byte\":"));
    assert!(json_str.contains("\"attainable_chip_tflops\":"));
    assert!(json_str.contains("\"silicon_efficiency_pct\":"));
}

#[test]
fn test_cli_cl_kernel_synthesis() {
    // 1. Test cl-kernel list catalog
    let list_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-kernel", "list"])
        .output()
        .expect("Failed to execute cron cl-kernel list");

    assert!(list_out.status.success(), "cl-kernel list must succeed");
    let list_str = String::from_utf8_lossy(&list_out.stdout);
    assert!(list_str.contains("CRON GOLDEN AI SILICON MICRO-KERNEL CATALOG"));
    assert!(list_str.contains("flash-attn"));
    assert!(list_str.contains("bitnet-gemm"));
    assert!(list_str.contains("rmsnorm"));
    assert!(list_str.contains("swiglu"));
    assert!(list_str.contains("rope"));
    assert!(list_str.contains("kv-cache"));

    // 2. Synthesize a golden kernel to file
    let temp_dir = std::env::temp_dir();
    let out_cl = temp_dir.join("test_synth_flash_attn.cl");

    let synth_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-kernel", "flash-attn", "-o", out_cl.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-kernel flash-attn");

    assert!(synth_out.status.success(), "cl-kernel flash-attn must succeed");
    assert!(out_cl.exists(), "Synthesized kernel file must exist");
    let content = fs::read_to_string(&out_cl).unwrap();
    assert!(content.contains("B0000:"));
    assert!(content.contains("_OP"));
    assert!(content.contains("_HL"));

    let _ = fs::remove_file(&out_cl);
}

#[test]
fn test_cli_cl_native_compilation() {
    let gcc_check = Command::new("gcc").arg("--version").output();
    let clang_check = Command::new("clang").arg("--version").output();
    if gcc_check.is_err() && clang_check.is_err() {
        return;
    }

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_cl = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");

    let temp_dir = std::env::temp_dir();
    let out_bin = temp_dir.join(if cfg!(windows) { "test_cli_native_attn.exe" } else { "test_cli_native_attn" });

    let native_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-native", kernel_cl.to_str().unwrap(), "-o", out_bin.to_str().unwrap(), "--run"])
        .output()
        .expect("Failed to execute cron cl-native");

    assert!(native_out.status.success(), "cl-native with --run must succeed");
    let stdout = String::from_utf8_lossy(&native_out.stdout);
    assert!(stdout.contains("CRON .cl STANDALONE NATIVE AOT BINARY COMPILER"));
    assert!(stdout.contains("Native standalone executable built in"));
    assert!(stdout.contains("CRON SILICON C23 NATIVE EXECUTION TELEMETRY"));
    assert!(stdout.contains("STATUS: 100% BIT-EXACT SILICON C23 PARITY VERIFIED"));

    let _ = fs::remove_file(&out_bin);
}

#[test]
fn test_cli_cl_tile_gemm_and_conv() {
    let temp_dir = std::env::temp_dir();
    let out_cl = temp_dir.join("test_cli_tiled_gemm.cl");

    // 1. Run cl-tile gemm
    let tile_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-tile",
            "gemm",
            "--m",
            "64",
            "--n",
            "64",
            "--k",
            "64",
            "-o",
            out_cl.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-tile gemm");

    assert!(tile_out.status.success(), "cl-tile gemm must succeed");
    let stdout = String::from_utf8_lossy(&tile_out.stdout);
    assert!(stdout.contains("CRON POLYHEDRAL VLIW LOOP TILER"));
    assert!(stdout.contains("Optimal Tile Partition"));
    assert!(out_cl.exists(), "Tiled output .cl must exist");

    let _ = fs::remove_file(&out_cl);

    // 2. Run cl-tile conv with --json
    let conv_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-tile",
            "conv",
            "--cin",
            "16",
            "--cout",
            "32",
            "--spatial",
            "8",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-tile conv");

    assert!(conv_out.status.success(), "cl-tile conv must succeed");
    let json_str = String::from_utf8_lossy(&conv_out.stdout);
    assert!(json_str.contains("\"operation_type\":"));
    assert!(json_str.contains("\"is_conflict_free\": true"));

    // 3. Run cl-tile gemm with --systolic and --cr
    let sys_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-tile",
            "gemm",
            "--m",
            "32",
            "--n",
            "32",
            "--k",
            "32",
            "--systolic",
            "--cr",
        ])
        .output()
        .expect("Failed to execute cron cl-tile gemm --systolic --cr");

    assert!(sys_out.status.success(), "cl-tile gemm --systolic --cr must succeed");
    let sys_stdout = String::from_utf8_lossy(&sys_out.stdout);
    assert!(sys_stdout.contains("Systolic Wavefront"));
    assert!(sys_stdout.contains("@systolic"));
    assert!(sys_stdout.contains("flow A -> EAST;"));
}

#[test]
fn test_cli_cl_cluster_topology_and_collective() {
    // 1. Test cl-cluster topology
    let top_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-cluster", "topology", "--chips", "4"])
        .output()
        .expect("Failed to execute cron cl-cluster topology");

    assert!(top_out.status.success(), "cl-cluster topology must succeed");
    let top_str = String::from_utf8_lossy(&top_out.stdout);
    assert!(top_str.contains("CRON MULTI-DIE 5D HIERARCHICAL CLUSTER TOPOLOGY"));
    assert!(top_str.contains("4 Physical Silicon Dies | 1024 Distributed Cores"));

    // 2. Test cl-cluster collective AllReduce
    let temp_dir = std::env::temp_dir();
    let col_cl = temp_dir.join("test_cli_allreduce.cl");

    let col_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-cluster",
            "collective",
            "allreduce",
            "--chips",
            "4",
            "--bytes",
            "512",
            "-o",
            col_cl.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-cluster collective");

    assert!(col_out.status.success(), "cl-cluster collective must succeed");
    let col_str = String::from_utf8_lossy(&col_out.stdout);
    assert!(col_str.contains("CRON MULTI-DIE COLLECTIVE COMMUNICATION SCHEDULE"));
    assert!(col_str.contains("AllReduce"));
    assert!(col_cl.exists(), "Synthesized collective microcode file must exist");

    let _ = fs::remove_file(&col_cl);
}

#[test]
fn test_cli_cl_sparse() {
    let temp_dir = std::env::temp_dir();
    let out_cl = temp_dir.join("test_cli_sparse_gemm.cl");

    // 1. Synthesize 2:4 sparse kernel
    let synth_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-sparse",
            "kernel",
            "--m",
            "32",
            "--n",
            "32",
            "--k",
            "32",
            "-o",
            out_cl.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-sparse kernel");

    assert!(synth_out.status.success(), "cl-sparse kernel must succeed");
    let stdout = String::from_utf8_lossy(&synth_out.stdout);
    assert!(stdout.contains("Synthesized 2:4 sparse GEMM microcode"));
    assert!(out_cl.exists(), "Sparse output .cl must exist");

    // 2. Test cl-sparse analyze
    let analyze_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-sparse", "analyze", "--json"])
        .output()
        .expect("Failed to execute cron cl-sparse analyze");

    assert!(analyze_out.status.success(), "cl-sparse analyze must succeed");
    let json_str = String::from_utf8_lossy(&analyze_out.stdout);
    assert!(json_str.contains("\"is_2_4_compliant\": true"));

    let _ = fs::remove_file(&out_cl);
}

#[test]
fn test_cli_cl_power() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_cl = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");

    // 1. Run cl-power ASCII report
    let power_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-power",
            kernel_cl.to_str().unwrap(),
            "--temp",
            "300.0",
            "--freq",
            "2.0",
        ])
        .output()
        .expect("Failed to execute cron cl-power");

    assert!(power_out.status.success(), "cl-power must succeed");
    let stdout = String::from_utf8_lossy(&power_out.stdout);
    assert!(stdout.contains("CRON LANDAUER THERMODYNAMIC DVFS"));
    assert!(stdout.contains("Landauer Energy Dissipation"));
    assert!(stdout.contains("Silicon Junction Temperature"));

    // 2. Run cl-power JSON telemetry
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-power",
            kernel_cl.to_str().unwrap(),
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-power --json");

    assert!(json_out.status.success(), "cl-power --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"landauer_energy_joules\":"));
    assert!(json_str.contains("\"total_power_watts\":"));
}

#[test]
fn test_cli_cl_autotune() {
    let temp_dir = std::env::temp_dir();
    let out_cl = temp_dir.join("test_tuned_gemm.cl");

    // 1. Run cl-autotune with file output
    let tune_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-autotune",
            "--m",
            "32",
            "--n",
            "32",
            "--k",
            "32",
            "--metric",
            "latency",
            "--max",
            "16",
            "-o",
            out_cl.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-autotune");

    assert!(tune_out.status.success(), "cl-autotune must succeed");
    let stdout = String::from_utf8_lossy(&tune_out.stdout);
    assert!(stdout.contains("CRON SPEC-TO-SILICON MULTI-OBJECTIVE AUTO-TUNER REPORT"));
    assert!(stdout.contains("Winning Optimal Configuration"));
    assert!(out_cl.exists(), "Auto-tuned .cl file must exist");

    // 2. Run cl-autotune JSON mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-autotune",
            "--m",
            "16",
            "--n",
            "16",
            "--k",
            "16",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-autotune --json");

    assert!(json_out.status.success(), "cl-autotune --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"problem_m\": 16"));
    assert!(json_str.contains("\"best_candidate\":"));

    let _ = fs::remove_file(&out_cl);
}

#[test]
fn test_cli_cl_stream() {
    let temp_dir = std::env::temp_dir();
    let out_cl = temp_dir.join("test_stream_pipeline.cl");

    // 1. Run cl-stream vision
    let stream_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-stream",
            "vision",
            "--dim",
            "128",
            "--patch",
            "16",
            "-o",
            out_cl.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-stream");

    assert!(stream_out.status.success(), "cl-stream must succeed");
    let stdout = String::from_utf8_lossy(&stream_out.stdout);
    assert!(stdout.contains("CRON ZERO-COPY MULTI-MODAL SPATIAL STREAMING PIPELINE MAP"));
    assert!(stdout.contains("Vision Transformer Patches"));
    assert!(out_cl.exists(), "Stream pipeline .cl must exist");

    // 2. Run cl-stream audio JSON mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-stream",
            "audio",
            "--bands",
            "64",
            "--quant",
            "4",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-stream --json");

    assert!(json_out.status.success(), "cl-stream audio --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"tokens_per_frame\": 64"));
    assert!(json_str.contains("\"zero_copy_verified\": true"));

    let _ = fs::remove_file(&out_cl);
}

#[test]
fn test_cli_cl_snn() {
    let temp_dir = std::env::temp_dir();
    let out_cl = temp_dir.join("test_snn_kernel.cl");

    // 1. Run cl-snn synth
    let synth_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-snn",
            "synth",
            "--neurons",
            "16",
            "-o",
            out_cl.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-snn synth");

    assert!(synth_out.status.success(), "cl-snn synth must succeed");
    let stdout = String::from_utf8_lossy(&synth_out.stdout);
    assert!(stdout.contains("Synthesized Brain 4 SNN microcode"));
    assert!(out_cl.exists(), "SNN .cl output must exist");

    // 2. Run cl-snn sim (ASCII raster mode)
    let sim_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-snn",
            "sim",
            "--neurons",
            "8",
            "--cycles",
            "16",
        ])
        .output()
        .expect("Failed to execute cron cl-snn sim");

    assert!(sim_out.status.success(), "cl-snn sim must succeed");
    let sim_str = String::from_utf8_lossy(&sim_out.stdout);
    assert!(sim_str.contains("CRON BRAIN 4 NEUROMORPHIC SPIKE RASTER"));
    assert!(sim_str.contains("Terminal Spike Raster Plot"));

    // 3. Run cl-snn sim JSON mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-snn",
            "sim",
            "--neurons",
            "4",
            "--cycles",
            "8",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-snn sim --json");

    assert!(json_out.status.success(), "cl-snn sim --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"num_neurons\": 4"));
    assert!(json_str.contains("\"total_spikes\":"));

    let _ = fs::remove_file(&out_cl);
}

#[test]
fn test_cli_cl_infer() {
    let temp_dir = std::env::temp_dir();
    let out_cl = temp_dir.join("test_transformer_infer.cl");

    // 1. Run cl-infer synth
    let synth_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-infer",
            "synth",
            "--dim",
            "32",
            "--layers",
            "2",
            "-o",
            out_cl.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-infer synth");

    assert!(synth_out.status.success(), "cl-infer synth must succeed");
    let stdout = String::from_utf8_lossy(&synth_out.stdout);
    assert!(stdout.contains("Synthesized LLM Transformer microcode"));
    assert!(out_cl.exists(), "Transformer .cl output must exist");

    // 2. Run cl-infer prompt (ASCII dashboard mode)
    let prompt_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-infer",
            "prompt",
            "CRON",
            "--tokens",
            "4",
            "--dim",
            "32",
            "--layers",
            "2",
        ])
        .output()
        .expect("Failed to execute cron cl-infer prompt");

    assert!(prompt_out.status.success(), "cl-infer prompt must succeed");
    let dash_str = String::from_utf8_lossy(&prompt_out.stdout);
    assert!(dash_str.contains("CRON 4D-TORUS LLM TRANSFORMER INFERENCE DASHBOARD"));
    assert!(dash_str.contains("Time to First Token"));
    assert!(dash_str.contains("Inference Throughput"));

    // 3. Run cl-infer prompt JSON mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-infer",
            "prompt",
            "AI",
            "--tokens",
            "4",
            "--dim",
            "32",
            "--layers",
            "2",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-infer prompt --json");

    assert!(json_out.status.success(), "cl-infer prompt --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"prompt\": \"AI\""));
    assert!(json_str.contains("\"tokens_per_second\":"));

    let _ = fs::remove_file(&out_cl);
}

#[test]
fn test_cli_lsp_server_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["lsp", "--version"])
        .output()
        .expect("Failed to execute cron lsp --version");

    assert!(output.status.success(), "cron lsp --version must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cron-lsp 3.17.0"));
    assert!(stdout.contains("Language Server Protocol"));
}

#[test]
fn test_cli_cl_cordic() {
    let out_cl = PathBuf::from("test_cordic_cli.cl");
    let _ = fs::remove_file(&out_cl);

    // 1. Test circular rotation
    let rot_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-cordic",
            "rot",
            "--angle",
            "0.785398",
            "--iters",
            "16",
        ])
        .output()
        .expect("Failed to execute cron cl-cordic rot");

    assert!(rot_out.status.success(), "cl-cordic rot must succeed");
    let rot_str = String::from_utf8_lossy(&rot_out.stdout);
    assert!(rot_str.contains("CRON CORDIC CIRCULAR ROTATION"));
    assert!(rot_str.contains("Computed cos(θ)"));
    assert!(rot_str.contains("Dynamic Energy"));

    // 2. Test circular rotation JSON mode
    let rot_json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-cordic",
            "rot",
            "--angle",
            "0.785398",
            "--iters",
            "16",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-cordic rot --json");

    assert!(rot_json_out.status.success(), "cl-cordic rot --json must succeed");
    let json_str = String::from_utf8_lossy(&rot_json_out.stdout);
    assert!(json_str.contains("\"mode\": \"Circular Rotation (sin, cos)\""));
    assert!(json_str.contains("\"dynamic_energy_pj\""));

    // 3. Test circular vectoring
    let vec_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-cordic",
            "vec",
            "--x",
            "3.0",
            "--y",
            "4.0",
            "--iters",
            "16",
        ])
        .output()
        .expect("Failed to execute cron cl-cordic vec");

    assert!(vec_out.status.success(), "cl-cordic vec must succeed");
    let vec_str = String::from_utf8_lossy(&vec_out.stdout);
    assert!(vec_str.contains("CRON CORDIC CIRCULAR VECTORING"));
    assert!(vec_str.contains("Magnitude r"));

    // 4. Test microcode synthesis to file
    let synth_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-cordic",
            "synth",
            "--iters",
            "16",
            "-o",
            "test_cordic_cli.cl",
        ])
        .output()
        .expect("Failed to execute cron cl-cordic synth");

    assert!(synth_out.status.success(), "cl-cordic synth must succeed");
    assert!(out_cl.exists(), "Synthesized CORDIC .cl file must exist");
    let synth_str = String::from_utf8_lossy(&synth_out.stdout);
    assert!(synth_str.contains("Synthesized CORDIC microcode written"));

    // 5. Test Bloch sphere rendering
    let sphere_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-cordic",
            "sphere",
            "--theta",
            "1.047",
            "--phi",
            "0.785",
        ])
        .output()
        .expect("Failed to execute cron cl-cordic sphere");

    assert!(sphere_out.status.success(), "cl-cordic sphere must succeed");
    let sphere_str = String::from_utf8_lossy(&sphere_out.stdout);
    assert!(sphere_str.contains("CRON BRAIN 5 QUANTUM & PHOTONIC BLOCH / POINCARÉ SPHERE"));

    let _ = fs::remove_file(&out_cl);
}

#[test]
fn test_cli_cl_vcd() {
    let cl_path = get_attention_cl_path();
    let out_vcd = PathBuf::from("test_pipeline_trace.vcd");
    let _ = fs::remove_file(&out_vcd);

    // 1. Run cl-vcd with ASCII timing diagram and VCD file emission
    let output = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-vcd",
            &cl_path,
            "-o",
            "test_pipeline_trace.vcd",
            "--cycles",
            "16",
        ])
        .output()
        .expect("Failed to execute cron cl-vcd");

    assert!(output.status.success(), "cl-vcd must succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CRON 256-CORE 4D-TORUS CYCLE-ACCURATE PIPELINE TIMING DIAGRAM"));
    assert!(stdout.contains("VCD WAVEFORM TRACE REPORT"));
    assert!(stdout.contains("Simulated Cycles:"));
    assert!(stdout.contains("IEEE 1364-2001 VCD waveform written"));

    assert!(out_vcd.exists(), "VCD trace output file must exist");
    let vcd_str = fs::read_to_string(&out_vcd).expect("Must read generated VCD");
    assert!(vcd_str.contains("$version"));
    assert!(vcd_str.contains("$timescale"));
    assert!(vcd_str.contains("$dumpvars"));

    // 2. Run cl-vcd with JSON mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-vcd",
            &cl_path,
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-vcd --json");

    assert!(json_out.status.success(), "cl-vcd --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"total_cycles\":"));
    assert!(json_str.contains("\"total_signals\":"));
    assert!(json_str.contains("\"timeline\":"));

    let _ = fs::remove_file(&out_vcd);
}

#[test]
fn test_cli_cl_perf() {
    let cl_path = get_attention_cl_path();

    // 1. Profile single kernel
    let single_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-perf",
            &cl_path,
        ])
        .output()
        .expect("Failed to execute cron cl-perf on single kernel");

    assert!(single_out.status.success(), "cl-perf single kernel must succeed");
    let single_str = String::from_utf8_lossy(&single_out.stdout);
    assert!(single_str.contains("CRON 256-CORE 4D-TORUS SILICON MICRO-KERNEL PPA PROFILING REPORT"));
    assert!(single_str.contains("VLIW Issue Rate:"));
    assert!(single_str.contains("TOPS / Watt"));
    assert!(single_str.contains("Energy-Delay Product"));

    // 2. Profile entire golden suite
    let suite_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-perf",
            "--all",
        ])
        .output()
        .expect("Failed to execute cron cl-perf --all");

    assert!(suite_out.status.success(), "cl-perf --all must succeed");
    let suite_str = String::from_utf8_lossy(&suite_out.stdout);
    assert!(suite_str.contains("CRON 256-CORE 4D-TORUS SILICON MICRO-KERNEL PPA PERFORMANCE SCOREBOARD"));
    assert!(suite_str.contains("flash-attn"));
    assert!(suite_str.contains("bitnet-gemm"));
    assert!(suite_str.contains("Silicon Efficiency Summary"));

    // 3. Profile entire golden suite JSON mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-perf",
            "--all",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-perf --all --json");

    assert!(json_out.status.success(), "cl-perf --all --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"total_benchmarked\": 8"));
    assert!(json_str.contains("\"peak_tops_per_watt\":"));
    assert!(json_str.contains("\"kernels\":"));
}

#[test]
fn test_cli_cl_balance() {
    // 1. Default ASCII heatmap run
    let default_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-balance",
        ])
        .output()
        .expect("Failed to execute cron cl-balance");

    assert!(default_out.status.success(), "cl-balance default must succeed");
    let default_str = String::from_utf8_lossy(&default_out.stdout);
    assert!(default_str.contains("CRON 256-CORE 4D-TORUS WORKLOAD HEATMAP"));
    assert!(default_str.contains("TP: 8, PP: 4, DP: 8"));
    assert!(default_str.contains("FLOP Imbalance Ratio:"));
    assert!(default_str.contains("Average Torus Hop Count:"));

    // 2. Custom TP, PP, DP strategy with multi-core bundle emission
    let temp_dir = std::env::temp_dir();
    let out_bundle = temp_dir.join("test_cli_balance_bundle.cl");

    let custom_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-balance",
            "--m", "1024",
            "--k", "2048",
            "--n", "4096",
            "--layers", "16",
            "--tp", "8",
            "--pp", "4",
            "--dp", "8",
            "--batches", "16",
            "-o", out_bundle.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-balance with custom strategy");

    assert!(custom_out.status.success(), "cl-balance custom strategy must succeed");
    let custom_str = String::from_utf8_lossy(&custom_out.stdout);
    assert!(custom_str.contains("Multi-Core coordinated bundle written"));
    assert!(out_bundle.exists(), "Synthesized multi-core bundle file must exist");

    let bundle_content = fs::read_to_string(&out_bundle).unwrap();
    assert!(bundle_content.contains("CORE_0000"));
    assert!(bundle_content.contains("_SB00#000!"));
    let _ = fs::remove_file(&out_bundle);

    // 3. JSON serialization mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-balance",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-balance --json");

    assert!(json_out.status.success(), "cl-balance --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"total_cores\": 256"));
    assert!(json_str.contains("\"strategy\""));
    assert!(json_str.contains("\"imbalance_ratio\""));
    assert!(json_str.contains("\"cores\""));
}

#[test]
fn test_cli_cl_trace() {
    let cl_path = get_attention_cl_path();

    // 1. Default ASCII schedule run
    let default_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-trace",
            &cl_path,
        ])
        .output()
        .expect("Failed to execute cron cl-trace");

    assert!(default_out.status.success(), "cl-trace default must succeed");
    let default_str = String::from_utf8_lossy(&default_out.stdout);
    assert!(default_str.contains("CRON HARDWARE DYNAMIC TRACE OPTIMIZER & L0 TRACE CACHE SCOREBOARD"));
    assert!(default_str.contains("Minimum Initiation Int:"));
    assert!(default_str.contains("Trace Cache Hits:"));
    assert!(default_str.contains("Instruction Power Saved:"));

    // 2. Output file synthesis mode
    let temp_dir = std::env::temp_dir();
    let out_cl = temp_dir.join("test_cli_trace_opt.cl");

    let file_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-trace",
            &cl_path,
            "--cache-kb", "32",
            "--trip-count", "128",
            "-o", out_cl.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-trace with -o");

    assert!(file_out.status.success(), "cl-trace with -o must succeed");
    let file_str = String::from_utf8_lossy(&file_out.stdout);
    assert!(file_str.contains("Optimized modulo trace written"));
    assert!(out_cl.exists(), "Synthesized trace .cl must exist");

    let cl_content = fs::read_to_string(&out_cl).unwrap();
    assert!(cl_content.contains("CRON Modulo-Scheduled & Trace-Cached Microcode"));
    let _ = fs::remove_file(&out_cl);

    // 3. JSON serialization mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-trace",
            &cl_path,
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-trace --json");

    assert!(json_out.status.success(), "cl-trace --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"modulo_schedule\":"));
    assert!(json_str.contains("\"trace_cache\":"));
    assert!(json_str.contains("\"hit_rate_percent\":"));
}

#[test]
fn test_cli_cl_router() {
    // 1. Inspect command with HUD rendering
    let inspect_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-router",
            "inspect",
        ])
        .output()
        .expect("Failed to execute cron cl-router inspect");

    assert!(inspect_out.status.success(), "cl-router inspect must succeed");
    let inspect_str = String::from_utf8_lossy(&inspect_out.stdout);
    assert!(inspect_str.contains("CRON 4D-TORUS 9-PORT VIRTUAL CHANNEL ROUTER MICRO-ARCHITECTURE HUD"));
    assert!(inspect_str.contains("9-Port Virtual Channel Buffer Occupancy"));
    assert!(inspect_str.contains("9x9 Crossbar Switch Routing & Arbitration Matrix"));

    // 2. Synthesize Verilog RTL to file
    let temp_dir = std::env::temp_dir();
    let out_v = temp_dir.join("test_cli_noc_router.v");

    let synth_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-router",
            "synth",
            "-o", out_v.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute cron cl-router synth");

    assert!(synth_out.status.success(), "cl-router synth must succeed");
    let synth_str = String::from_utf8_lossy(&synth_out.stdout);
    assert!(synth_str.contains("Synthesizable Verilog router RTL written"));
    assert!(out_v.exists(), "Synthesized router .v must exist");

    let v_content = fs::read_to_string(&out_v).unwrap();
    assert!(v_content.contains("module noc_router_4d"));
    assert!(v_content.contains("credit_out"));
    assert!(v_content.contains("endmodule"));
    let _ = fs::remove_file(&out_v);

    // 3. JSON serialization mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-router",
            "inspect",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron cl-router inspect --json");

    assert!(json_out.status.success(), "cl-router --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"router_coord\":"));
    assert!(json_str.contains("\"num_ports\": 9"));
    assert!(json_str.contains("\"ports\":"));
}

#[test]
fn test_cli_cl_esoteric() {
    // 1. Demo mode HUD
    let demo_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-esoteric", "demo"])
        .output()
        .expect("Failed to execute cron cl-esoteric demo");

    assert!(demo_out.status.success(), "cl-esoteric demo must succeed");
    let demo_str = String::from_utf8_lossy(&demo_out.stdout);
    assert!(demo_str.contains("CRON ESOLANG-INSPIRED AI SILICON COPROCESSOR TELEMETRY HUD"));
    assert!(demo_str.contains("Brainfuck Hardware Tape Pointer"));
    assert!(demo_str.contains("Malbolge Balanced Ternary"));
    assert!(demo_str.contains("Befunge 2D/4D Spatial Systolic"));
    assert!(demo_str.contains("Prolog 1-Cycle Hardware Unifier"));

    // 2. Synthesizable Verilog RTL emission
    let temp_dir = std::env::temp_dir();
    let out_v = temp_dir.join("test_cli_esoteric_coproc.v");
    let synth_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-esoteric", "synth", "-o", out_v.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-esoteric synth");

    assert!(synth_out.status.success(), "cl-esoteric synth must succeed");
    let synth_str = String::from_utf8_lossy(&synth_out.stdout);
    assert!(synth_str.contains("Synthesizable Verilog esoteric coprocessor RTL written"));
    assert!(out_v.exists(), "Synthesized esoteric coprocessor .v must exist");

    let v_content = fs::read_to_string(&out_v).unwrap();
    assert!(v_content.contains("module esoteric_coprocessor"));
    assert!(v_content.contains("trit_mac_result"));
    assert!(v_content.contains("endmodule"));
    let _ = fs::remove_file(&out_v);

    // 3. JSON serialization mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-esoteric", "demo", "--json"])
        .output()
        .expect("Failed to execute cron cl-esoteric demo --json");

    assert!(json_out.status.success(), "cl-esoteric --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"status\": \"SUCCESS\""));
    assert!(json_str.contains("\"total_trit_macs\":"));
    assert!(json_str.contains("\"flags\":"));
}

#[test]
fn test_cli_cl_optic() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_cl = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");

    // 1. ASCII HUD mode
    let hud_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-optic", kernel_cl.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-optic");

    assert!(hud_out.status.success(), "cl-optic must succeed");
    let hud_str = String::from_utf8_lossy(&hud_out.stdout);
    assert!(hud_str.contains("CRON HETEROGENEOUS OPTICAL WDM LASER POWER"));
    assert!(hud_str.contains("Total Insertion Loss:"));
    assert!(hud_str.contains("WDM C-BAND 100 GHz LAMBDA CHANNELS ALLOCATION"));

    // 2. JSON Mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-optic", kernel_cl.to_str().unwrap(), "--json"])
        .output()
        .expect("Failed to execute cron cl-optic --json");

    assert!(json_out.status.success(), "cl-optic --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"status\": \"COMPLIANT\""));
    assert!(json_str.contains("\"insertion_loss_db\":"));
    assert!(json_str.contains("\"required_laser_power_mw\":"));
    assert!(json_str.contains("\"wdm_channels\":"));

    // 3. Verilog RTL Synthesis Mode
    let temp_dir = std::env::temp_dir();
    let out_v = temp_dir.join("test_optical_controller.v");

    let synth_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-optic", kernel_cl.to_str().unwrap(), "--synth-verilog", "-o", out_v.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-optic --synth-verilog");

    assert!(synth_out.status.success(), "cl-optic --synth-verilog must succeed");
    assert!(out_v.exists(), "Synthesized optical controller .v must exist");
    let v_content = fs::read_to_string(&out_v).unwrap();
    assert!(v_content.contains("module optical_laser_power_controller"));
    assert!(v_content.contains("laser_diode_enable"));
    let _ = fs::remove_file(&out_v);
}

#[test]
fn test_cli_cl_patch() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_cl = manifest_dir.join("../../examples/cl/mini_transformer_attention.cl");

    let temp_dir = std::env::temp_dir();
    let patch_file = temp_dir.join("cli_test_patch.clpatch");
    let patched_cl = temp_dir.join("cli_patched_output.cl");

    // 1. Create a patch package
    let create_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-patch", "create", kernel_cl.to_str().unwrap(),
            "--cycle", "1",
            "--bundle", "_OP01$28F> _NO00#000> _NO00#000> _NO00#000>",
            "-o", patch_file.to_str().unwrap(),
            "--comment", "CLI Test Patch"
        ])
        .output()
        .expect("Failed to execute cron cl-patch create");

    assert!(create_out.status.success(), "cl-patch create must succeed");
    assert!(patch_file.exists(), "Patch file must be generated");

    // 2. Inspect the patch package
    let inspect_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-patch", "inspect", patch_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-patch inspect");

    assert!(inspect_out.status.success(), "cl-patch inspect must succeed");
    let inspect_str = String::from_utf8_lossy(&inspect_out.stdout);
    assert!(inspect_str.contains("CRON POST-SILICON HARDWARE MICROCODE PATCH"));
    assert!(inspect_str.contains("CLI Test Patch"));

    // 3. Apply the patch package to .cl file
    let apply_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "cl-patch", "apply", kernel_cl.to_str().unwrap(),
            patch_file.to_str().unwrap(),
            "-o", patched_cl.to_str().unwrap()
        ])
        .output()
        .expect("Failed to execute cron cl-patch apply");

    assert!(apply_out.status.success(), "cl-patch apply must succeed");
    assert!(patched_cl.exists(), "Patched .cl output file must exist");
    let patched_content = fs::read_to_string(&patched_cl).unwrap();
    assert!(patched_content.contains("B0001: _OP01$28F>"));

    // Clean up
    let _ = fs::remove_file(&patch_file);
    let _ = fs::remove_file(&patched_cl);
}

#[test]
fn test_cli_cl_compare() {
    // 1. Run cl-compare in text HUD mode
    let hud_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-compare", "all"])
        .output()
        .expect("Failed to execute cron cl-compare all");

    assert!(hud_out.status.success(), "cl-compare all must succeed");
    let hud_str = String::from_utf8_lossy(&hud_out.stdout);
    assert!(hud_str.contains("CRON vs PYTORCH/CUDA vs MOJO: REAL-WORLD PERFORMANCE"));
    assert!(hud_str.contains("BitNet b1.58 Ternary GEMM"));
    assert!(hud_str.contains("RingTape Streaming FlashAttn"));
    assert!(hud_str.contains("GEOMETRIC MEAN ADVANTAGE"));

    // 2. Run cl-compare with JSON mode
    let json_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-compare", "bitnet", "--json"])
        .output()
        .expect("Failed to execute cron cl-compare bitnet --json");

    assert!(json_out.status.success(), "cl-compare --json must succeed");
    let json_str = String::from_utf8_lossy(&json_out.stdout);
    assert!(json_str.contains("\"mean_speedup_vs_pytorch\":"));
    assert!(json_str.contains("\"workloads\":"));
    assert!(json_str.contains("\"cron\":"));
    assert!(json_str.contains("\"pytorch_h100\":"));

    // 3. Run cl-compare with --whitepaper mode to file
    let temp_dir = std::env::temp_dir();
    let whitepaper_path = temp_dir.join("cli_test_whitepaper.md");

    let wp_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["cl-compare", "all", "--whitepaper", "-o", whitepaper_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute cron cl-compare --whitepaper");

    assert!(wp_out.status.success(), "cl-compare --whitepaper must succeed");
    assert!(whitepaper_path.exists(), "Whitepaper markdown file must exist");
    let wp_content = fs::read_to_string(&whitepaper_path).unwrap();
    assert!(wp_content.contains("# CRON: A Spatial-Cognitive Architecture & Language for Extreme-Efficiency AI"));
    assert!(wp_content.contains("## 3. Mathematical Foundations of CRON Superiority"));
    let _ = fs::remove_file(&whitepaper_path);

    // 4. Test alias `cron bench`
    let bench_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["bench", "gemm"])
        .output()
        .expect("Failed to execute cron bench gemm");
    assert!(bench_out.status.success(), "cron bench alias must succeed");
    let bench_str = String::from_utf8_lossy(&bench_out.stdout);
    assert!(bench_str.contains("BitNet b1.58 Ternary GEMM"));
}

#[test]
fn test_cli_swarm_256_and_cluster_4096() {
    // 1. Single-chip 256 core swarm
    let s256_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["swarm", "--task", "Test 256-core consensus", "--json"])
        .output()
        .expect("Failed to execute cron swarm --json");
    assert!(s256_out.status.success(), "cron swarm --json must exit 0");
    let s256_str = String::from_utf8_lossy(&s256_out.stdout);
    assert!(s256_str.contains("\"consensus_achieved\": true"));
    assert!(s256_str.contains("\"status\": \"quorum_reached\""));

    // 2. 16-chip 4,096 core multi-chip swarm cluster
    let s4096_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["swarm", "--cluster", "--task", "Test 4096-core cluster consensus", "--json"])
        .output()
        .expect("Failed to execute cron swarm --cluster --json");
    assert!(s4096_out.status.success(), "cron swarm --cluster --json must exit 0");
    let s4096_str = String::from_utf8_lossy(&s4096_out.stdout);
    assert!(s4096_str.contains("\"consensus_achieved\": true"));
    assert!(s4096_str.contains("\"total_chips\": 16"));
    assert!(s4096_str.contains("\"total_cores\": 4096"));
    assert!(s4096_str.contains("\"status\": \"global_quorum_reached\""));
}

#[test]
fn test_cli_swarm_synthesize() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&[
            "swarm-synthesize",
            "--prompt",
            "Synthesize FlashAttention-2 forward tile",
            "--json",
        ])
        .output()
        .expect("Failed to execute cron swarm-synthesize --json");

    assert!(out.status.success(), "cron swarm-synthesize must exit 0");
    let out_str = String::from_utf8_lossy(&out.stdout);
    assert!(out_str.contains("\"consensus_achieved\": true"));
    assert!(out_str.contains("\"detected_operators\": [\"flash-attn\"]"));
    assert!(out_str.contains("\"status\": \"synthesis_complete\""));
}

#[test]
fn test_cli_swarm_tui_modes() {
    // 1. Torus Plane mode snapshot
    let out_plane = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["swarm-tui", "--snapshot", "--mode", "plane", "--no-color", "--ticks", "5"])
        .output()
        .expect("Failed to execute cron swarm-tui --mode plane");
    assert!(out_plane.status.success(), "cron swarm-tui --mode plane must exit 0");
    let s_plane = String::from_utf8_lossy(&out_plane.stdout);
    assert!(s_plane.contains("4D-TORUS 2D PLANE SLICE") || s_plane.contains("TORUS"));

    // 2. Cluster Macro mode snapshot
    let out_cluster = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["swarm-tui", "--snapshot", "--mode", "cluster", "--no-color", "--ticks", "3"])
        .output()
        .expect("Failed to execute cron swarm-tui --mode cluster");
    assert!(out_cluster.status.success(), "cron swarm-tui --mode cluster must exit 0");
    let s_cluster = String::from_utf8_lossy(&out_cluster.stdout);
    assert!(s_cluster.contains("16-CHIP CLUSTER MACRO TOPOLOGY") || s_cluster.contains("CLUSTER"));

    // 3. Router Heatmap mode snapshot
    let out_router = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["swarm-tui", "--snapshot", "--mode", "router", "--no-color", "--ticks", "3"])
        .output()
        .expect("Failed to execute cron swarm-tui --mode router");
    assert!(out_router.status.success(), "cron swarm-tui --mode router must exit 0");
    let s_router = String::from_utf8_lossy(&out_router.stdout);
    assert!(s_router.contains("ROUTER & VC VIRTUAL CHANNEL CONGESTION") || s_router.contains("ROUTER"));

    // 4. Swarm Telemetry mode snapshot
    let out_telemetry = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["swarm-tui", "--snapshot", "--mode", "telemetry", "--no-color", "--ticks", "3"])
        .output()
        .expect("Failed to execute cron swarm-tui --mode telemetry");
    assert!(out_telemetry.status.success(), "cron swarm-tui --mode telemetry must exit 0");
    let s_telemetry = String::from_utf8_lossy(&out_telemetry.stdout);
    assert!(s_telemetry.contains("AUTONOMOUS SWARM SELF-SYNTHESIS") || s_telemetry.contains("VIBE-HEALING"));
}

#[test]
fn test_cli_monitor_alias_and_swarm_flag() {
    // Test 'cron monitor --snapshot' alias
    let out_mon = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["monitor", "--snapshot", "--no-color", "--ticks", "2"])
        .output()
        .expect("Failed to execute cron monitor --snapshot");
    assert!(out_mon.status.success(), "cron monitor must exit 0");
    let s_mon = String::from_utf8_lossy(&out_mon.stdout);
    assert!(!s_mon.is_empty());

    // Test 'cron swarm --tui --snapshot' flag
    let out_swarm_tui = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["swarm", "--tui", "--snapshot", "--no-color", "--ticks", "2"])
        .output()
        .expect("Failed to execute cron swarm --tui --snapshot");
    assert!(out_swarm_tui.status.success(), "cron swarm --tui must exit 0");
    let s_swarm_tui = String::from_utf8_lossy(&out_swarm_tui.stdout);
    assert!(!s_swarm_tui.is_empty());
}

#[test]
fn test_cli_swarm_tui_json() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["swarm-tui", "--json", "--ticks", "4"])
        .output()
        .expect("Failed to execute cron swarm-tui --json");
    assert!(out.status.success(), "cron swarm-tui --json must exit 0");
    let out_str = String::from_utf8_lossy(&out.stdout);
    assert!(out_str.contains("\"tick_count\": 4"));
    assert!(out_str.contains("\"simulated_ipc\":"));
    assert!(out_str.contains("\"chip_traffic_gbps\":"));
}

#[test]
fn test_cli_mcts_synthesize_hud_and_json() {
    // 1. Terminal HUD mode
    let out_hud = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["mcts-synthesize", "--prompt", "FlashAttention-2 forward tile", "--sims", "40", "--depth", "8"])
        .output()
        .expect("Failed to execute cron mcts-synthesize");
    assert!(out_hud.status.success(), "cron mcts-synthesize must exit 0");
    let s_hud = String::from_utf8_lossy(&out_hud.stdout);
    assert!(s_hud.contains("CRON NEURO-SYMBOLIC MCTS KERNEL SYNTHESIS ENGINE"));
    assert!(s_hud.contains("MCTS INSTRUCTION SCHEDULING & SLOT PACKING METRICS"));
    assert!(s_hud.contains("STATUS: MCTS INSTRUCTION SCHEDULING CERTIFIED OPTIMAL"));

    // 2. JSON mode
    let out_json = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["mcts-synthesize", "--prompt", "BitNet Ternary GEMM", "--sims", "30", "--json"])
        .output()
        .expect("Failed to execute cron mcts-synthesize --json");
    assert!(out_json.status.success(), "cron mcts-synthesize --json must exit 0");
    let s_json = String::from_utf8_lossy(&out_json.stdout);
    assert!(s_json.contains("\"prompt\": \"BitNet Ternary GEMM\""));
    assert!(s_json.contains("\"optimized_ipc\":"));
    assert!(s_json.contains("\"total_cycles\":"));
}

#[test]
fn test_cli_verify_proof_badge_and_json() {
    // 1. ASCII proof badge mode
    let out_badge = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["verify-proof", "--workload", "Attention-Head-0"])
        .output()
        .expect("Failed to execute cron verify-proof");
    assert!(out_badge.status.success(), "cron verify-proof must exit 0");
    let s_badge = String::from_utf8_lossy(&out_badge.stdout);
    assert!(s_badge.contains("CRON FORMAL MATHEMATICAL PROOF CERTIFICATE"));
    assert!(s_badge.contains("MATHEMATICALLY PROVEN & TAPE-OUT CERTIFIED"));
    assert!(s_badge.contains("LEMMA-DOR-01"));
    assert!(s_badge.contains("LEMMA-INV-02"));

    // 2. JSON certificate mode
    let out_json = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(&["verify-proof", "--workload", "Attention-Head-0", "--json"])
        .output()
        .expect("Failed to execute cron verify-proof --json");
    assert!(out_json.status.success(), "cron verify-proof --json must exit 0");
    let s_json = String::from_utf8_lossy(&out_json.stdout);
    assert!(s_json.contains("\"certificate_id\":"));
    assert!(s_json.contains("\"is_certified\": true"));
    assert!(s_json.contains("\"total_lemmas\": 6"));
}





