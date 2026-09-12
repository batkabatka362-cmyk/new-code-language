use cron_vm::Simulator;

#[test]
fn test_step_execution() {
    let mut sim = Simulator::new();
    let cl_code = r#"
    B0001: '=00#0A04> '=10#040B> _SH00$01B4> _SP00#0088>
    B0002: _OP041$0E> _FA054$20> _BK095$01> _HLT_____8!
    "#;
    sim.load_machine_code(cl_code);

    // Step cycle 1
    let stepped_1 = sim.step();
    assert!(stepped_1, "Expected first step to succeed");
    assert_eq!(sim.stats.total_cycles, 1);

    // Step cycle 2
    let stepped_2 = sim.step();
    assert!(stepped_2, "Expected second step to succeed");
    assert_eq!(sim.stats.total_cycles, 2);

    // Step cycle 3 (EOF)
    let stepped_3 = sim.step();
    assert!(!stepped_3, "Expected step at EOF to return false");
}

#[test]
fn test_core_dump_inspection() {
    let mut sim = Simulator::new();
    let cl_code = r#"
    B0001: '=00#0A04> '=10#040B> _SH00$01B4> _SP00#0088>
    "#;
    sim.load_machine_code(cl_code);
    sim.run();

    let dump_core0 = sim.core_dump(0);
    assert_eq!(dump_core0.len(), 16);
    // Core 0 register 0 was loaded with 0x0A04
    assert_eq!(dump_core0[0], 0x0A04);

    // Out of bounds core returns zeroed registers
    let dump_oob = sim.core_dump(999);
    assert_eq!(dump_oob, [0u32; 16]);
}

#[test]
fn test_trace_mode_enabled() {
    let mut sim = Simulator::new();
    sim.set_trace(true);
    let cl_code = r#"
    B0001: '=00#002A> _HLT_____8!
    "#;
    sim.load_machine_code(cl_code);
    let stats = sim.run();
    assert_eq!(stats.total_cycles, 1);
}

#[test]
fn test_alu_greater_equal_mode() {
    let mut sim = Simulator::new();
    // Load R6 = 100 (0x64), R4 = 50 (0x32), then run PO with mode 'G'
    // Format: '=06#0064> loads R6=0x64, '=04#0032> loads R4=0x32
    // PO slot: '_PO06G400>' dest=6, mode='G', src=4, imm=0 -> R6 = (R6 >= R4 ? 1 : 0)
    let cl_code = r#"
    B0001: '=06#0064> '=04#0032> _PO06G400> _HLT_____8!
    "#;
    sim.load_machine_code(cl_code);
    sim.run();

    let dump = sim.core_dump(0);
    assert_eq!(dump[6], 1, "Expected R6 >= R4 (100 >= 50) to evaluate to 1");

    // Test reverse case: R6 = 20 (0x14), R4 = 50 (0x32)
    let mut sim2 = Simulator::new();
    let cl_code_false = r#"
    B0001: '=06#0014> '=04#0032> _PO06G400> _HLT_____8!
    "#;
    sim2.load_machine_code(cl_code_false);
    sim2.run();

    let dump2 = sim2.core_dump(0);
    assert_eq!(dump2[6], 0, "Expected R6 >= R4 (20 >= 50) to evaluate to 0");
}

#[test]
fn test_alu_multiply_and_divide() {
    let mut sim = Simulator::new();
    // Load R6 = 12 (0xC), R4 = 3, then PO with imm=3 (multiply) -> 12 * 3 = 36 (0x24)
    let cl_code = r#"
    B0001: '=06#000C> '=04#0003> _PO06$403> _HLT_____8!
    "#;
    sim.load_machine_code(cl_code);
    sim.run();
    let dump = sim.core_dump(0);
    assert_eq!(dump[6], 36, "Expected 12 * 3 = 36");

    // Now test division: R6 = 36 (0x24), R4 = 4 -> PO with imm=4 (divide) -> 36 / 4 = 9
    let mut sim2 = Simulator::new();
    let cl_code2 = r#"
    B0001: '=06#0024> '=04#0004> _PO06$404> _HLT_____8!
    "#;
    sim2.load_machine_code(cl_code2);
    sim2.run();
    let dump2 = sim2.core_dump(0);
    assert_eq!(dump2[6], 9, "Expected 36 / 4 = 9");
}
