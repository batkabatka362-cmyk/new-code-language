use std::fs;

#[test]
fn test_ultra_resilient_cognitive_core_v88_pipeline() {
    let sample_path = "../../examples/ultra_resilient_cognitive_core_v88.cr";
    let source_cr = fs::read_to_string(sample_path)
        .or_else(|_| fs::read_to_string("examples/ultra_resilient_cognitive_core_v88.cr"))
        .expect("Failed to read ultra_resilient_cognitive_core_v88.cr");

    // Phase 1: Compile .cr into pure machine-native .cl VLIW
    let cl_output = cronc::compile_source(&source_cr).expect("CRON compilation failed");
    assert!(cl_output.starts_with("B0001:"), "Expected pure .cl machine bundles");

    // Verify key hardware co-processor slots were generated in VLIW bundles
    assert!(cl_output.contains("_OP"), "Must contain Brain 2 Photonic MZI GEMM slot");
    assert!(cl_output.contains("_BK"), "Must contain Brain 3 Reversible Autodiff Backward slot");
    assert!(cl_output.contains("_ST"), "Must contain Brain 4 Biological STDP synapse update slot");
    assert!(cl_output.contains("_RS"), "Must contain Brain 6 0-cycle Region Arena Reset slot");
    assert!(cl_output.contains("_SB"), "Must contain 4D Torus Spatial Broadcast slot");
    assert!(cl_output.contains("_DW"), "Must contain DMA Sync slot");
    assert!(cl_output.contains("_SP"), "Must contain Fiber 1 Spawn slot");
    assert!(cl_output.contains("_FJ"), "Must contain Fiber Join slot");

    // Verify all slots conform to 10-character width
    let mut bundle_count = 0;
    for line in cl_output.lines() {
        if line.starts_with('B') && line.contains(':') {
            bundle_count += 1;
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(parts.len(), 5, "Each VLIW bundle must contain exactly 4 slots");
            for slot in &parts[1..5] {
                assert_eq!(slot.len(), 10, "Each slot must be exactly 10 characters wide: '{}'", slot);
            }
        }
    }
    assert!(bundle_count >= 6, "Expected at least 6 VLIW execution cycles");

    // Phase 2: Execute in 256-Core 4D-Torus Bare-Metal Simulator
    let telemetry = cron_vm::run_cl(&cl_output);
    assert_eq!(telemetry.total_cycles, bundle_count);
    assert!(telemetry.optical_gemm_ops > 0, "Must execute Photonic MZI optical operations");
    assert!(telemetry.reversible_gate_ops > 0, "Must execute Reversible thermodynamic operations");
    assert!(telemetry.stdp_synapse_updates > 0, "Must execute STDP synaptic updates");
    assert!(telemetry.mesh_packets_routed > 0, "Must route packets across 4D Torus NoC");
    assert!(telemetry.peak_temperature_c < 180, "Thermal level must stay below safety threshold");
}
