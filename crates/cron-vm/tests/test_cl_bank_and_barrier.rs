use cron_vm::{CoreEngine, Simulator};

#[test]
fn test_positional_bank_a0_vs_0a() {
    let mut core = CoreEngine::new(0);

    // 0A: Bank 0 (Local Register File), Channel 10 ($R10)
    // A0: Bank 10 (4D-Torus Hyperspace Cluster W-axis), Channel 0 ($R0)
    core.execute_slot("'==0A#0042>");
    core.execute_slot("'==A0#0099>");

    // Local registers: R10 should be 0x42, R0 should remain 0 (not overwritten by Bank 10 Channel 0)
    assert_eq!(core.registers[10], 0x42, "0A must target Local Register 10");
    assert_eq!(core.get_bank_register(0, 10), 0x42, "Bank 0 Reg 10 must mirror Local R10");

    assert_eq!(core.registers[0], 0, "Local R0 must be preserved when targeting Bank 10");
    assert_eq!(core.get_bank_register(10, 0), 0x99, "A0 must target Bank 10 Reg 0");

    // Clear distinction: 0A != A0
    assert_ne!(
        core.get_bank_register(0, 10),
        core.get_bank_register(10, 0),
        "Positional permutation invariance: 0A and A0 must address orthogonal hardware domains"
    );
}

#[test]
fn test_homopolymer_aa_auto_accumulation() {
    let mut core = CoreEngine::new(0);

    // Initialize R10 = 5
    core.execute_slot("'==0A#0005>");
    assert_eq!(core.registers[10], 5);

    // AA: Dual-bank SIMD self-broadcast lockstep auto-accumulation (RA <- RA * RA)
    core.execute_slot("_POAA$000>");
    assert_eq!(core.registers[10], 25, "First AA cycle must compute 5 * 5 = 25");
    assert_eq!(core.get_bank_register(10, 10), 25);

    core.execute_slot("_POAA$000>");
    assert_eq!(core.registers[10], 625, "Second AA cycle must compute 25 * 25 = 625");
    assert_eq!(core.get_bank_register(10, 10), 625);

    // Verify hardware CSR burst counter incremented
    assert!(core.read_csr(3) >= 2, "CSR_VEC_BURST_CNT must track SIMD auto-accumulations");
}

#[test]
fn test_homopolymer_bb_global_barrier_synchronization() {
    let mut sim = Simulator::new();
    let cl_code = r#"
    B0001: '=00#0A04> '=01#0005> _SH00$01B4> _SP00#0088>
    B0002: _bb00$000> _bb00$000> _bb00$000> _bb00$000>
    B0003: '=02#0007> _HLT_____8! _NO00$000> _NO00$000>
    "#;

    sim.load_machine_code(cl_code);

    // Cycle 1: execution
    assert!(sim.step());
    for core in &sim.cores {
        assert_eq!(core.barrier_count, 0);
        assert!(!core.in_barrier);
    }

    // Cycle 2: Global 256-Core Chip-Wide Hardware Barrier (bb)
    assert!(sim.step());
    for core in &sim.cores {
        assert_eq!(core.barrier_count, 1, "All 256 cores must synchronize on global bb barrier");
        assert!(!core.in_barrier, "Barrier flag must be released in lockstep after synchronization");
    }

    // Cycle 3: post-barrier execution
    assert!(sim.step());
    assert_eq!(sim.core_dump(0)[2], 7);
}
