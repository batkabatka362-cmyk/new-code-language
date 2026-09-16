use cron_vm::Simulator;

#[test]
fn test_homopolymer_cc_cache_invalidation() {
    let mut sim = Simulator::new();
    let cl_code = r#"
    B0001: '=00#0A04> '=01#0005> _SH00$01B4> _SP00#0088>
    B0002: _CC00$000> _CC00$000> _CC00$000> _CC00$000>
    B0003: '=02#0007> _HLT_____8! _NO00$000> _NO00$000>
    "#;

    sim.load_machine_code(cl_code);
    assert!(sim.step()); // B0001
    assert!(sim.step()); // B0002: CC cache invalidation

    for core in &sim.cores {
        assert!(
            core.cache_invalidations >= 1,
            "All 256 cores must invalidate I/D cache on homopolymer CC"
        );
        assert_eq!(core.csr_stall_cnt, 0, "Stall counters must be flushed");
    }
}

#[test]
fn test_homopolymer_dd_direct_dma_burst() {
    let mut sim = Simulator::new();
    let cl_code = r#"
    B0001: '=04#1234> _NO00$000> _NO00$000> _NO00$000>
    B0002: _DDA0$040> _NO00$000> _NO00$000> _NO00$000>
    B0003: _HLT_____8! _NO00$000> _NO00$000> _NO00$000>
    "#;

    sim.load_machine_code(cl_code);
    sim.run();

    assert!(
        sim.get_core_dma_transfers(0) >= 1,
        "Core 0 must record DMA transfer"
    );
    assert!(
        sim.stats.mesh_packets_routed >= 32,
        "Mesh must route DMA packet bursts"
    );
}

#[test]
fn test_homopolymer_ee_dvfs_energy_scaling() {
    let mut sim = Simulator::new();
    // Intentionally raise thermal level on core 0
    sim.cores[0].thermal_level = 85;

    let cl_code = r#"
    B0001: _EE00$000> _NO00$000> _NO00$000> _NO00$000>
    B0002: _HLT_____8! _NO00$000> _NO00$000> _NO00$000>
    "#;

    sim.load_machine_code(cl_code);
    sim.step(); // B0001: EE

    assert_eq!(
        sim.cores[0].thermal_level, 25,
        "EE must dissipate thermal throttle back to 25°C baseline"
    );
    assert_eq!(
        sim.get_core_dvfs_state(0),
        1,
        "EE must activate Eco DVFS power mode"
    );
    assert!(
        sim.get_core_energy_saved(0) >= 450,
        "EE must record micro-Watt energy savings"
    );
}

#[test]
fn test_homopolymer_11_photonic_pump_and_88_arena_reset() {
    let mut sim = Simulator::new();
    // Push values to reversible stack
    sim.cores[0].reversible_stack.push(42);
    sim.cores[0].reversible_stack.push(99);

    let cl_code = r#"
    B0001: _1100$000> _8800$000> _NO00$000> _NO00$000>
    B0002: _HLT_____8! _NO00$000> _NO00$000> _NO00$000>
    "#;

    sim.load_machine_code(cl_code);
    sim.step(); // B0001: 11 (pump) + 88 (arena reset)

    assert!(
        sim.get_core_photonic_pumps(0) >= 1,
        "Core 0 must register optical laser pump strobe"
    );
    assert_eq!(
        sim.cores[0].wave_reg.amplitudes,
        [255, 255, 255, 255],
        "Photonic amplitudes must be saturated to 255"
    );

    assert!(
        sim.get_core_arena_resets(0) >= 1,
        "Core 0 must register 0-cycle arena reset"
    );
    assert!(
        sim.cores[0].reversible_stack.is_empty(),
        "Reversible stack must be instantaneously cleared by 88"
    );
}
