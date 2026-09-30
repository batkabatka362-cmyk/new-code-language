use cron_vm::simulator::Simulator;

#[test]
fn test_4d_torus_fourier_thermal_diffusion() {
    let mut sim = Simulator::new();
    // Create a thermal hotspot at Core (0,0,0,0) -> index 0
    sim.cores[0].thermal_level = 105; // Hot core (105°C)

    // Initial check: neighbors are at baseline 25°C
    // Neighbors of (0,0,0,0): (3,0,0,0)->3, (1,0,0,0)->1, (0,3,0,0)->12, (0,1,0,0)->4, (0,0,3,0)->48, (0,0,1,0)->16, (0,0,0,3)->192, (0,0,0,1)->64
    assert_eq!(sim.cores[1].thermal_level, 25);
    assert_eq!(sim.cores[4].thermal_level, 25);

    let cl_code = r#"
B0001: _NO00#000> _NO00#000> _NO00#000> _NO00#000>
"#;
    sim.load_machine_code(cl_code);
    sim.step();

    // Hot core should dissipate heat, and neighbors should absorb heat via Fourier conduction
    assert!(sim.cores[0].thermal_level < 105, "Hot core must cool down via diffusion (was {}, got {})", 105, sim.cores[0].thermal_level);
    assert!(sim.cores[1].thermal_level > 25, "Neighbor core (1,0,0,0) must absorb heat (was 25, got {})", sim.cores[1].thermal_level);
    assert!(sim.cores[4].thermal_level > 25, "Neighbor core (0,1,0,0) must absorb heat (was 25, got {})", sim.cores[4].thermal_level);
    assert!(sim.cores[16].thermal_level > 25, "Neighbor core (0,0,1,0) must absorb heat (was 25, got {})", sim.cores[16].thermal_level);
    assert!(sim.cores[64].thermal_level > 25, "Neighbor core (0,0,0,1) must absorb heat (was 25, got {})", sim.cores[64].thermal_level);
}

#[test]
fn test_adaptive_thermal_deflection_routing_around_hotspot() {
    let mut sim = Simulator::new();
    // Source: Core (0,0,0,0) = 0
    // Target: Core (2,0,0,0) = 2
    // Natural DOR step from (0,0,0,0) along X+ is Core (1,0,0,0) = 1.
    // Set Core 1 as a throttled thermal hotspot (125°C >= 105°C ceiling)
    sim.set_hotspot(1, 125);
    assert!(sim.thermal_throttled_cores[1]);

    // Route packet with resilient thermal deflection
    let (hops, deflected) = sim.route_packet_resilient(0, 2, 0xCAFE_BABE);

    // Must successfully deflect around the hot core
    assert!(deflected, "Packet routing must detect hot core 1 and deflect along cooler orthogonal axis");
    assert!(hops >= 2, "Deflection route took {} hops", hops);

    // Verify delivery at destination core 2
    let delivered = sim.mesh.deliver_packets(2);
    assert_eq!(delivered.len(), 1, "Packet must be delivered to target core without packet drop");
    assert_eq!(delivered[0].payload, 0xCAFE_BABE);
    assert!(delivered[0].was_deflected);
    assert_eq!(delivered[0].virtual_channel, 2, "Deflected packets must use VC2_Detour");
    assert!(sim.stats.thermal_deflection_hops > 0);
    assert!(sim.stats.zero_collision_verified);
}

#[test]
fn test_thermal_throttling_dvfs_cooling() {
    let mut sim = Simulator::new();
    // Inject extreme thermal hotspot
    sim.set_hotspot(42, 130);
    assert!(sim.thermal_throttled_cores[42]);

    let cl_code = r#"
B0001: _NO00#000> _NO00#000> _NO00#000> _NO00#000>
B0002: _NO00#000> _NO00#000> _NO00#000> _NO00#000>
"#;
    sim.load_machine_code(cl_code);
    sim.run();

    // Hot core must have undergone active DVFS step-down and Fourier dissipation
    assert!(sim.cores[42].thermal_level < 130);
    assert!(sim.stats.zero_collision_verified);
}

