use std::fs;

#[test]
fn test_end_to_end_cron_ecosystem() {
    let sample_path = "../../examples/complete_agi_agent.cr";
    let source_cr = fs::read_to_string(sample_path)
        .or_else(|_| fs::read_to_string("examples/complete_agi_agent.cr"))
        .expect("Failed to read sample CRON source");

    // Phase 1: Compile .cr to machine-native .cl VLIW (pure code, zero comments)
    let cl_output = cronc::compile_source(&source_cr).expect("CRON compilation failed");
    assert!(cl_output.starts_with("B0001:"), "Expected pure .cl machine bundles without comments");

    // Verify bundle format and 10-char slot width
    let mut bundle_count = 0;
    for line in cl_output.lines() {
        if line.starts_with('B') && line.contains(':') {
            bundle_count += 1;
            let parts: Vec<&str> = line.split_whitespace().collect();
            // parts[0] is B0001:, parts[1..5] are 4 slots
            assert_eq!(parts.len(), 5, "Each VLIW bundle must contain exactly 4 slots");
            for slot in &parts[1..5] {
                assert_eq!(slot.len(), 10, "Each slot must be exactly 10 characters wide: '{}'", slot);
            }
        }
    }
    assert!(bundle_count >= 6, "Expected at least 6 VLIW execution cycles");

    // Phase 2: Execute in 256-Core 4D-Torus Simulator
    let telemetry = cron_vm::run_cl(&cl_output);
    assert_eq!(telemetry.total_cycles, bundle_count);
    assert!(telemetry.optical_gemm_ops > 0, "Photonic GEMM must execute");
    assert!(telemetry.reversible_gate_ops > 0, "Reversible Fredkin ops must execute");
    assert!(telemetry.stdp_synapse_updates > 0, "STDP synaptic updates must execute");
    assert!(telemetry.peak_temperature_c <= 180, "Thermal sentry must prevent overheating");

    // Phase 3: Round-trip Decompilation to .cr Blueprint
    let decompiled_cr = cron_decompile::decompile_cl(&cl_output).expect("Decompilation failed");
    assert!(decompiled_cr.contains(".MODULE DecompiledCognitiveCore"));
    assert!(decompiled_cr.contains("resilient_compute"));
    assert!(decompiled_cr.contains("region TileProcessingArena"));
}
