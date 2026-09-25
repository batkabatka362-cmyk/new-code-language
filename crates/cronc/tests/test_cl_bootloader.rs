use cronc::cl_bootloader::{CognitiveBootloader, CoreBootState, TrapVector};

#[test]
fn test_bootloader_initialization_and_bist() {
    let mut bootloader = CognitiveBootloader::new();
    assert_eq!(bootloader.core_states.len(), 256);
    assert!(!bootloader.is_bootstrapped);

    let bist = bootloader.run_bist();
    assert!(bist.all_passed);
    assert!(bist.sram_banks_ok);
    assert!(bist.cordic_units_ok);
    assert!(bist.photonic_mzi_ok);
    assert_eq!(bist.active_cores, 256);
}

#[test]
fn test_bootloader_bootstrap_256_cores() {
    let mut bootloader = CognitiveBootloader::new();
    let res = bootloader.bootstrap();
    assert!(res.is_ok());
    assert!(bootloader.is_bootstrapped);

    for core in &bootloader.core_states {
        assert_eq!(core.state, CoreBootState::CognitiveLoopRunning);
    }
}

#[test]
fn test_bootloader_ivt_trap_dispatch() {
    let mut bootloader = CognitiveBootloader::new();
    let _ = bootloader.bootstrap();

    // Dispatch Photonic Sync trap to Core 12
    let addr = bootloader.dispatch_trap(12, TrapVector::PhotonicSync);
    assert_eq!(addr, Ok(0x0000_0040));
    assert_eq!(bootloader.core_states[12].executed_traps, 1);

    // Dispatch Thermal Trip trap to Core 44 (should throttle frequency)
    let initial_freq = bootloader.core_states[44].frequency_mhz;
    let _ = bootloader.dispatch_trap(44, TrapVector::ThermalTrip);
    assert_eq!(
        bootloader.core_states[44].frequency_mhz,
        initial_freq - 200
    );
}

#[test]
fn test_pgas_memory_resolution() {
    let bootloader = CognitiveBootloader::new();
    let addr = bootloader.pgas_map.core_sram_address(5, 0x100).unwrap();
    assert_eq!(addr, 0x2000_0000 + (5 * 65536) + 0x100);

    let region = bootloader.pgas_map.resolve_region(addr);
    assert_eq!(region, "Core Local SRAM (Bank 0-15)");

    let optical_region = bootloader.pgas_map.resolve_region(0x8000_1000);
    assert_eq!(optical_region, "Photonic Shared Optical Crossbar");
}
