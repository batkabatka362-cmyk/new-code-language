use cron_vm::CoreEngine;

#[test]
fn test_extended_mode_delimiters_in_vm() {
    let mut core = CoreEngine::new(0);

    // Initialize R4 = 10, R6 = 20
    core.execute_slot("'==04#000A>");
    core.execute_slot("'==06#0014>");
    assert_eq!(core.registers[4], 10);
    assert_eq!(core.registers[6], 20);

    // Mode '+': Addition ALU bypass: R6 = R6 + R4 = 20 + 10 = 30
    core.execute_slot("_PO06+400>");
    assert_eq!(core.registers[6], 30);

    // Mode '-': Subtraction ALU bypass: R6 = R6 - R4 = 30 - 10 = 20
    core.execute_slot("_PO06-400>");
    assert_eq!(core.registers[6], 20);

    // Mode '*': Multiplication ALU bypass: R6 = R6 * R4 = 20 * 10 = 200
    core.execute_slot("_PO06*400>");
    assert_eq!(core.registers[6], 200);

    // Mode '/': Division ALU bypass: R6 = R6 / R4 = 200 / 10 = 20
    core.execute_slot("_PO06/400>");
    assert_eq!(core.registers[6], 20);

    // Mode '%': Modulo ALU bypass: R6 = R6 % R4 = 20 % 10 = 0
    core.execute_slot("_PO06%400>");
    assert_eq!(core.registers[6], 0);

    // Reset R6 = 0xFF00, R4 = 0x0F0F
    core.execute_slot("'==06#FF00>");
    core.execute_slot("'==04#0F0F>");

    // Mode '&': Bitwise AND: R6 = 0xFF00 & 0x0F0F = 0x0F00
    core.execute_slot("_PO06&400>");
    assert_eq!(core.registers[6], 0x0F00);

    // Mode '|': Bitwise OR: R6 = 0x0F00 | 0x0F0F = 0x0F0F
    core.execute_slot("_PO06|400>");
    assert_eq!(core.registers[6], 0x0F0F);

    // Mode '^': Bitwise XOR: R6 = 0x0F0F ^ 0x0F0F = 0
    core.execute_slot("_PO06^400>");
    assert_eq!(core.registers[6], 0);

    // Mode '~': Bitwise NOT of R4 (0x0F0F)
    core.execute_slot("_PO06~400>");
    assert_eq!(core.registers[6], !0x0F0F);

    // Mode '=': Equality check
    core.execute_slot("'==06#000A>");
    core.execute_slot("'==04#000A>");
    core.execute_slot("_PO06=400>");
    assert_eq!(core.registers[6], 1);

    // Mode '<': Less than check
    core.execute_slot("'==06#0005>");
    core.execute_slot("'==04#000A>");
    core.execute_slot("_PO06<400>");
    assert_eq!(core.registers[6], 1);

    // Mode '>': Greater than check
    core.execute_slot("'==06#000F>");
    core.execute_slot("'==04#000A>");
    core.execute_slot("_PO06>400>");
    assert_eq!(core.registers[6], 1);

    // Mode '?': Predicate gate
    core.execute_slot("'==06#1234>");
    core.execute_slot("'==04#0001>");
    core.execute_slot("_PO06?400>");
    assert_eq!(core.registers[6], 0x1234);

    core.execute_slot("'==04#0000>");
    core.execute_slot("_PO06?400>");
    assert_eq!(core.registers[6], 0);
}

#[test]
fn test_tx_rx_and_lif_opcodes_in_vm() {
    let mut core = CoreEngine::new(0);

    // LF: Neuromorphic Spike Generator
    core.execute_slot("'==00#0080>"); // R0 = 128 > 100
    core.execute_slot("_LF0C$000>");
    assert_eq!(core.registers[12], 1); // Spike emitted!

    core.execute_slot("'==00#0010>"); // R0 = 16 <= 100
    core.execute_slot("_LF0C$000>");
    assert_eq!(core.registers[12], 0); // No spike

    // RX / TX FIFO
    core.noc_rx_fifo.push(0xDEAD_BEEF);
    core.execute_slot("_RX01$000>");
    assert_eq!(core.registers[1], 0xDEAD_BEEF);

    let initial_broadcast = core.spatial_broadcast_count;
    core.execute_slot("_TX01$000>");
    assert_eq!(core.spatial_broadcast_count, initial_broadcast + 1);
}
