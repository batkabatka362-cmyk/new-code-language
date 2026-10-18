// ============================================================================
// Test Suite: Zero-DRAM 4D-Torus Spatial Flow & Pipe Execution
// Validates end-to-end multi-core execution of `cron_bitnet_7b_foundation.cl`
// across all 7 cores with hardware FIFO flow routing and zero DRAM traffic.
// ============================================================================

use cron_vm::Simulator;
use std::fs;

#[test]
fn test_bitnet_foundation_7b_multi_core_flow() {
    let cl_path = "../../examples/cl/cron_bitnet_7b_foundation.cl";
    // Also try local relative path if running from crate directory or root
    let content = fs::read_to_string(cl_path)
        .or_else(|_| fs::read_to_string("examples/cl/cron_bitnet_7b_foundation.cl"))
        .expect("Failed to read examples/cl/cron_bitnet_7b_foundation.cl");

    let mut sim = Simulator::new();
    sim.load_machine_code(&content);

    // 1. Verify Flow Routes are correctly registered
    assert_eq!(sim.flow_routes.get(&0), Some(&1), "Core 0 must route to Core 1");
    assert_eq!(sim.flow_routes.get(&1), Some(&2), "Core 1 must route to Core 2");
    assert_eq!(sim.flow_routes.get(&2), Some(&3), "Core 2 must route to Core 3");
    assert_eq!(sim.flow_routes.get(&3), Some(&7), "Core 3 must route to Core 7 [3,1,0,0]");
    assert_eq!(sim.flow_routes.get(&7), Some(&11), "Core 7 must route to Core 11 [3,2,0,0]");
    assert_eq!(sim.flow_routes.get(&11), Some(&255), "Core 11 must route to Core 255 [3,3,3,3]");

    // 2. Verify Weights are bound in SRAM across cores
    assert_ne!(sim.cores[0].bank_registers[1][0], 0, "Bank 1 weights must be resident");

    // 3. Step simulation until completion (max 1,000 cycles safeguard)
    let mut cycles = 0;
    while sim.step() {
        cycles += 1;
        if cycles > 1000 {
            break;
        }
    }

    assert!(cycles > 0, "Simulation must execute cycles");
    assert!(cycles < 1000, "Simulation should complete without deadlock (took {} cycles)", cycles);

    // 4. Verify Core Execution across the spatial pipeline
    let core0 = &sim.cores[0];
    let core1 = &sim.cores[1];
    let core2 = &sim.cores[2];
    let core3 = &sim.cores[3];
    let core4 = &sim.cores[7];
    let core5 = &sim.cores[11];
    let core6 = &sim.cores[255];

    assert!(core0.cycle_count > 0, "Core 0 (Prompt Embedding) must execute");
    assert!(core1.cycle_count > 0, "Core 1 (GQA Attention) must execute");
    assert!(core2.cycle_count > 0, "Core 2 (FlashAttention) must execute");
    assert!(core3.cycle_count > 0, "Core 3 (SwiGLU MLP) must execute");
    assert!(core4.cycle_count > 0, "Core 4 (Down Projection) must execute");
    assert!(core5.cycle_count > 0, "Core 5 (Mamba SSM) must execute");
    assert!(core6.cycle_count > 0, "Core 6 (LM Head Unembed) must execute");

    // Final core [3,3,3,3] must have reached halt with committed tokens
    assert!(core6.is_halted, "Core 255 (LM Head) must reach halt");

    // Packets were routed across cores via the Torus NoC without DRAM
    assert!(sim.stats.mesh_packets_routed > 0, "NoC packets must be routed between cores");
    assert_eq!(sim.stats.dram_bandwidth_saved_mb, 0.0);
}
