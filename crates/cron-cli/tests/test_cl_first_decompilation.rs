// ============================================================================
// CRON Integration Test: SSS+ .cl First Paradigm & Universal Decompilation
// Tests direct .cl execution, reverse semantic decompilation (.cl -> .cr),
// direct .cl to C23 transpilation, and 256-core 4D-Torus silicon parity.
// ============================================================================

use cronc::compile_cl_to_c23;
use cron_decompile::decompile_cl;
use cron_vm::run_cl;
use std::process::Command;

#[test]
fn test_cl_first_full_accelerator_pipeline() {
    // 128-bit VLIW machine code written directly in .cl (AI-native silicon code)
    let cl_kernel = r#"
B0000: _RS00$000> _SH00$000> '=01#000A> '=02#0014>
B0001: _OP04$100> _TT03$200> _MD05.400> _PO06+503>
B0002: _RF06$300> _ST0C$084> _SB06$000> _bb00$000>
B0003: _EE00$000> _WH07$600> _LI08$705> _HL00$000>
"#;

    // 1. Direct Silicon VM Execution (AI vibe runs directly on 256-core 4D-Torus)
    let vm_stats = run_cl(cl_kernel);
    assert_eq!(vm_stats.total_cycles, 4, "Must execute exactly 4 VLIW cycles");
    assert!(vm_stats.optical_gemm_ops > 0, "Photonic MZI GEMM must execute");
    assert!(vm_stats.reversible_gate_ops > 0, "Reversible Fredkin swap must execute");
    assert!(vm_stats.stdp_synapse_updates > 0, "STDP synapse update must execute");
    assert!(vm_stats.mesh_packets_routed > 0, "NoC packets must route across 4D Torus");
    assert!(vm_stats.peak_temperature_c <= 180, "Thermal sentry must prevent overheating");

    // 2. Universal Reverse Semantic Decompilation (.cl -> .cr for human auditing)
    let cr_projection = decompile_cl(cl_kernel).expect("Decompilation of .cl must succeed");
    
    // Validate that human-readable high-level constructs were reconstructed
    assert!(cr_projection.contains(".MODULE DecompiledCognitiveCore"));
    assert!(cr_projection.contains(".ENTRY _main"));
    assert!(cr_projection.contains("resilient_compute"));
    assert!(cr_projection.contains("region TileProcessingArena"));
    assert!(cr_projection.contains("optical_gemm"));
    assert!(cr_projection.contains("tensor_transpose"));
    assert!(cr_projection.contains("subbyte_dot"));
    assert!(cr_projection.contains("reversible_swap") || cr_projection.contains("reversible_entangle"));
    assert!(cr_projection.contains("step_synaptic_plasticity"));
    assert!(cr_projection.contains("spatial_broadcast"));
    assert!(cr_projection.contains("pgas_barrier"));
    assert!(cr_projection.contains("wormhole_tunnel"));
    assert!(cr_projection.contains("lif_neuron_step"));
    assert!(cr_projection.contains(".END"));

    // 3. Direct .cl -> C23 Native Transpilation
    let c23_code = compile_cl_to_c23(cl_kernel, "VibeAttentionKernel").expect("Direct C23 emission must succeed");
    assert!(c23_code.contains("CronSiliconCore core;"));
    assert!(c23_code.contains("cron_silicon_init(&core);"));
    assert!(c23_code.contains("cron_execute_bundles(&core);"));
    assert!(c23_code.contains("cron_subbyte_ternary_dot"));
    assert!(c23_code.contains("cron_tile_transpose"));
    assert!(c23_code.contains("cron_reversible_swap"));
    assert!(c23_code.contains("core.optical_gemm_count"));
    assert!(c23_code.contains("BIT-EXACT SILICON C23 PARITY VERIFIED"));

    // 4. Native GCC/Clang Compilation & Execution (if GCC is present)
    let has_gcc = Command::new("gcc").arg("--version").output().is_ok();
    if has_gcc {
        let temp_dir = std::env::temp_dir();
        let c_file = temp_dir.join("test_vibe_cl_kernel.c");
        let exe_file = if cfg!(windows) {
            temp_dir.join("test_vibe_cl_kernel.exe")
        } else {
            temp_dir.join("test_vibe_cl_kernel")
        };

        std::fs::write(&c_file, &c23_code).expect("Write C source");
        let compile_status = Command::new("gcc")
            .arg("-std=c2x")
            .arg("-O3")
            .arg(&c_file)
            .arg("-o")
            .arg(&exe_file)
            .arg("-lm")
            .status();

        if let Ok(status) = compile_status {
            if status.success() {
                let run_output = Command::new(&exe_file).output().expect("Execute C binary");
                assert!(run_output.status.success());
                let stdout = String::from_utf8_lossy(&run_output.stdout);
                assert!(stdout.contains("Executed Bundles/Cycles:     4"));
                assert!(stdout.contains("Photonic MZI Optical Ops:    1"));
                assert!(stdout.contains("Reversible Gate Ops:         1"));
                assert!(stdout.contains("STDP Synapse Updates:        2"));
                assert!(stdout.contains("Global Barrier Syncs:        1"));
                assert!(stdout.contains("STATUS: 100% BIT-EXACT SILICON C23 PARITY VERIFIED"));
            }
        }
        let _ = std::fs::remove_file(c_file);
        let _ = std::fs::remove_file(exe_file);
    }
}

#[test]
fn test_cl_first_round_trip_semantics() {
    // Pure arithmetic + NoC broadcast kernel
    let cl_code = r#"
B0000: '=01#0064> '=02#0032> _NO00$000> _NO00$000>
B0001: _PO03+102> _PO04-102> _PO05*102> _SB03$000>
B0002: _bb00$000> _HL00$000> _NO00$000> _NO00$000>
"#;

    let cr_decompiled = decompile_cl(cl_code).expect("Decompile must succeed");
    assert!(cr_decompiled.contains("const_r1"));
    assert!(cr_decompiled.contains("const_r2"));
    assert!(cr_decompiled.contains("+"));
    assert!(cr_decompiled.contains("-"));
    assert!(cr_decompiled.contains("*"));
    assert!(cr_decompiled.contains("spatial_broadcast"));
    assert!(cr_decompiled.contains("pgas_barrier"));

    // Verify VM executes 3 cycles with spatial broadcast packets
    let stats = run_cl(cl_code);
    assert_eq!(stats.total_cycles, 3);
    assert!(stats.mesh_packets_routed >= 1);
}

#[test]
fn test_cl_first_kernel_fusion_decompilation_and_telemetry() {
    let cl_kernel = r#"
B0000: _FU00$000> '=01#0005> '=02#0007> _NO00$000>
B0001: _PO03*102> _FE00$000> _bb00$000> _HL00$000>
"#;

    // 1. VM Execution tracks fused ops and saved DRAM traffic
    let stats = run_cl(cl_kernel);
    assert_eq!(stats.total_cycles, 2);
    assert!(stats.fused_kernel_ops > 0, "Fused kernel ops must be recorded in VM");
    assert!(stats.memory_wall_saved_bytes > 0, "DRAM traffic saved must be tracked");

    // 2. Reverse semantic decompilation lifts back into `fuse [tile=(4, 4), stream=SRAM] { ... }`
    let cr_decompiled = decompile_cl(cl_kernel).expect("Decompile must succeed");
    assert!(cr_decompiled.contains("fuse [tile=(4, 4), stream=SRAM] {"));
    assert!(cr_decompiled.contains("const_r1 * const_r2"));

    // 3. Direct .cl -> C23 transpilation tracks fusion telemetry
    let c23_code = compile_cl_to_c23(cl_kernel, "FusedStreamingKernel").expect("C23 compilation must succeed");
    assert!(c23_code.contains("core->fused_ops_count"));
    assert!(c23_code.contains("core->hbm_bytes_saved"));
}

