use cron_vm::{CoreEngine, Simulator, VliwInstruction};

#[test]
fn test_fiber_lifecycle_in_simulator() {
    let mut sim = Simulator::new();
    assert_eq!(sim.active_fiber_count(), 0);
    assert_eq!(sim.fibers_completed, 0);

    // Spawn fiber 1
    let fid1 = sim.spawn_fiber(0, 10);
    assert_eq!(fid1, 1);
    assert_eq!(sim.active_fiber_count(), 1);

    // Spawn fiber 2
    let fid2 = sim.spawn_fiber(4, 25);
    assert_eq!(fid2, 2);
    assert_eq!(sim.active_fiber_count(), 2);

    // Set mock result in fiber 1's saved registers (R0 convention)
    if let Some(f) = sim.fiber_queue.iter_mut().find(|f| f.fiber_id == fid1) {
        f.saved_registers[0] = 42;
    }

    // Join fiber 1
    let res1 = sim.join_fiber(fid1);
    assert_eq!(res1, Some(42));
    assert_eq!(sim.fibers_completed, 1);
    assert_eq!(sim.active_fiber_count(), 1);

    // Join fiber 2
    let res2 = sim.join_fiber(fid2);
    assert_eq!(res2, Some(0));
    assert_eq!(sim.fibers_completed, 2);
    assert_eq!(sim.active_fiber_count(), 0);
}

#[test]
fn test_fiber_vliw_slot_detection() {
    let mut sim = Simulator::new();

    let bundles = vec![
        VliwInstruction {
            cycle: 1,
            slots: vec![
                "'=00#0A04>".to_string(),
                "_SP00#0000>".to_string(), // _SP fiber spawn
                "_NO00#000>".to_string(),
                "_NO00#000>".to_string(),
            ],
        },
        VliwInstruction {
            cycle: 2,
            slots: vec![
                "'=01#0042>".to_string(),
                "_FJ00#0000>".to_string(), // _FJ fiber join
                "_NO00#000>".to_string(),
                "_NO00#000>".to_string(),
            ],
        },
    ];

    sim.load_program(bundles);
    sim.run();

    // The _SP should have spawned a fiber, and _FJ should have joined it
    assert_eq!(sim.fibers_completed, 1);
    assert_eq!(sim.active_fiber_count(), 0);
}

#[test]
fn test_core_engine_hardware_fiber_registers() {
    let mut core = CoreEngine::new(0);
    assert!(!core.fiber1_active);

    // Execute _SP on CoreEngine
    core.execute_slot("_SP00#0000>");
    assert!(core.fiber1_active);

    // Execute _FJ on CoreEngine
    core.execute_slot("_FJ00#0000>");
    assert!(!core.fiber1_active);
}

#[test]
fn test_fdo_execution_profile_generation() {
    let mut sim = Simulator::new();
    sim.enable_profiling();

    let bundles = vec![
        VliwInstruction {
            cycle: 1,
            slots: vec![
                "'=00#0A04>".to_string(),
                "'=01#0042>".to_string(),
                "~G01#0000>".to_string(),
                "_NO00#000>".to_string(),
            ],
        },
        VliwInstruction {
            cycle: 2,
            slots: vec![
                "'=02#0010>".to_string(),
                "_NO00#000>".to_string(),
                "_NO00#000>".to_string(),
                "_NO00#000>".to_string(),
            ],
        },
    ];

    sim.load_program(bundles);
    sim.run();

    let profile = sim.get_execution_profile();
    assert_eq!(profile.bundle_profiles.len(), 2);
    assert_eq!(profile.total_nops, 4); // 1 in bundle 1, 3 in bundle 2
    assert_eq!(profile.avg_slot_utilization, 0.5); // 4 active / (2 * 4) = 0.5
    assert_eq!(profile.underutilized_bundles, 2); // Both bundles have >= 1 NOP

    // Export .prof text representation
    let prof_str = profile.to_prof_string();
    assert!(prof_str.contains("CRON FDO Execution Profile"));
    assert!(prof_str.contains("Total Bundles: 2"));
    assert!(prof_str.contains("Total NOPs: 4"));
    assert!(prof_str.contains("Avg Slot Utilization: 0.5000"));
    assert!(prof_str.contains("[OPCODE_FREQUENCY]"));
    assert!(prof_str.contains("NO = 4"));
}
