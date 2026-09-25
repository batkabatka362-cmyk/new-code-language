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
