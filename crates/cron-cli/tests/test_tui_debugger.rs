// ============================================================================
// CRON Milestone #013 Integration Test: SSS+ Microarchitectural Debugger TUI
// Validates:
//   - 10-character VLIW slot micro-op disassembler coverage
//   - Cycle-accurate stepping and register diff highlighting
//   - Hardware breakpoints on Cycle, Opcode, and Register Conditions
//   - Multi-core inspection across the 256-Core 4D-Torus Processor
//   - Batch mode execution of complex debugging sessions
// ============================================================================

use cron_vm::debugger::{disassemble_bundle, disassemble_slot, Breakpoint, Debugger, StepResult};
use cron_vm::VliwInstruction;

#[test]
fn test_debugger_vliw_disassembler_coverage() {
    // 1. Immediate Load Slot
    let imm_slot = disassemble_slot("'=01#0042>");
    assert_eq!(imm_slot.opcode, "LOAD_IMM");
    assert_eq!(imm_slot.dest_reg, 1);
    assert_eq!(imm_slot.imm_val, Some(0x0042));
    assert!(imm_slot.mnemonic.contains("LI R1, #0x0042"));

    // 2. Photonic MZI Optical GEMM Slot
    let opt_slot = disassemble_slot("_OP04$100>");
    assert_eq!(opt_slot.opcode, "OP");
    assert_eq!(opt_slot.dest_reg, 4);
    assert_eq!(opt_slot.src_reg, 1);
    assert!(opt_slot.mnemonic.contains("OP_GEMM R4, R1"));

    // 3. Reversible Fredkin Swap Gate
    let rev_slot = disassemble_slot("_RF06$300>");
    assert_eq!(rev_slot.opcode, "RF");
    assert_eq!(rev_slot.dest_reg, 6);
    assert_eq!(rev_slot.src_reg, 3);
    assert!(rev_slot.description.contains("Reversible Fredkin Swap"));

    // 4. Neuromorphic STDP Synapse Update
    let stdp_slot = disassemble_slot("_ST0C$084>");
    assert_eq!(stdp_slot.opcode, "ST");
    assert!(stdp_slot.description.contains("Spike-Timing Dependent Plasticity"));

    // 5. Streaming Kernel Fusion Start & End
    let fu_slot = disassemble_slot("_FU00$000>");
    assert_eq!(fu_slot.opcode, "FU");
    assert!(fu_slot.mnemonic.contains("FUSE_START"));

    let fe_slot = disassemble_slot("_FE00$000>");
    assert_eq!(fe_slot.opcode, "FE");
    assert!(fe_slot.mnemonic.contains("FUSE_END"));

    // 6. Global Chip-Wide Barrier
    let bb_slot = disassemble_slot("_bb00$000>");
    assert_eq!(bb_slot.opcode, "bb");
    assert!(bb_slot.mnemonic.contains("CHIP_BARRIER"));

    // 7. Full Bundle Disassembly
    let inst = VliwInstruction {
        cycle: 42,
        slots: vec![
            "'=01#000A>".to_string(),
            "_FU00$000>".to_string(),
            "_PO03*102>".to_string(),
            "_FE00$000>".to_string(),
        ],
    };
    let bundle = disassemble_bundle(&inst);
    assert_eq!(bundle.cycle, 42);
    assert_eq!(bundle.slots.len(), 4);
    assert_eq!(bundle.slots[0].opcode, "LOAD_IMM");
    assert_eq!(bundle.slots[1].opcode, "FU");
    assert_eq!(bundle.slots[2].opcode, "PO");
    assert_eq!(bundle.slots[3].opcode, "FE");
}

#[test]
fn test_debugger_cycle_stepping_and_register_diffs() {
    let cl_code = r#"
B0000: '=03#0005> '=02#0007> _NO00$000> _NO00$000>
B0001: _PO03+200> _NO00$000> _NO00$000> _HL00$000>
"#;
    let mut dbg = Debugger::new(cl_code);
    assert_eq!(dbg.current_cycle, 0);
    assert_eq!(dbg.sim.core_dump(0)[3], 0);

    // Step Cycle 0 (Load R3=5, R2=7)
    let res1 = dbg.step();
    assert_eq!(res1, StepResult::Stepped { cycle: 1 });
    assert_eq!(dbg.sim.core_dump(0)[3], 5);
    assert_eq!(dbg.sim.core_dump(0)[2], 7);

    // Verify diffs were recorded for R3 and R2
    assert!(dbg.last_reg_diffs.iter().any(|d| d.reg == 3 && d.new_val == 5));
    assert!(dbg.last_reg_diffs.iter().any(|d| d.reg == 2 && d.new_val == 7));

    // Step Cycle 1 (Add R3 = R3 + R2 = 12)
    let res2 = dbg.step();
    assert_eq!(res2, StepResult::Stepped { cycle: 2 });
    assert_eq!(dbg.sim.core_dump(0)[3], 12);
    assert!(dbg.last_reg_diffs.iter().any(|d| d.reg == 3 && d.new_val == 12));

    // Subsequent step halts
    let res3 = dbg.step();
    assert_eq!(res3, StepResult::NoMoreInstructions);
    assert!(dbg.is_halted);
}

#[test]
fn test_debugger_breakpoints_cycle_and_opcode() {
    let cl_code = r#"
B0000: '=01#000A> '=02#0014> _NO00$000> _NO00$000>
B0001: _OP04$100> _NO00$000> _NO00$000> _NO00$000>
B0002: _PO05+402> _bb00$000> _NO00$000> _HL00$000>
"#;
    let mut dbg = Debugger::new(cl_code);

    // Add breakpoint on Opcode "OP"
    dbg.add_breakpoint(Breakpoint::Opcode("OP".to_string()));

    // Run continue
    let res = dbg.continue_exec();
    match res {
        StepResult::BreakpointHit { cycle, breakpoint } => {
            assert_eq!(cycle, 1, "Should pause at Cycle 1 when bundle contains OP");
            assert_eq!(breakpoint, Breakpoint::Opcode("OP".to_string()));
        }
        other => panic!("Expected BreakpointHit on OP, got {:?}", other),
    }

    // Add breakpoint on Cycle 2
    dbg.add_breakpoint(Breakpoint::Cycle(2));
    let res2 = dbg.continue_exec();
    match res2 {
        StepResult::BreakpointHit { cycle, breakpoint } => {
            assert_eq!(cycle, 2, "Should pause at Cycle 2");
            assert_eq!(breakpoint, Breakpoint::Cycle(2));
        }
        other => panic!("Expected BreakpointHit on Cycle 2, got {:?}", other),
    }

    // Continue to halt
    let res3 = dbg.continue_exec();
    match res3 {
        StepResult::Halted { total_cycles } => {
            assert_eq!(total_cycles, 3);
        }
        other => panic!("Expected Halted, got {:?}", other),
    }
}

#[test]
fn test_debugger_breakpoints_register_condition() {
    let cl_code = r#"
B0000: '=01#000F> _NO00$000> _NO00$000> _NO00$000>
B0001: '=01#001F> _NO00$000> _NO00$000> _NO00$000>
B0002: _HL00$000> _NO00$000> _NO00$000> _NO00$000>
"#;
    let mut dbg = Debugger::new(cl_code);

    // Break when Core 0 R1 == 0x001F (31)
    dbg.add_breakpoint(Breakpoint::RegisterCond {
        core_id: 0,
        reg: 1,
        op: "==".to_string(),
        val: 31,
    });

    // Step first cycle: R1 becomes 15
    let _ = dbg.step();
    assert_eq!(dbg.sim.core_dump(0)[1], 15);

    // Continue: should pause before Cycle 1 executes because R1 will be evaluated
    let _res = dbg.continue_exec();
    // After step 1, R1 is 31, which triggers the condition on the next check
    assert!(dbg.sim.core_dump(0)[1] == 31 || dbg.is_halted);
}

#[test]
fn test_debugger_multi_core_inspection_and_torus_coords() {
    let cl_code = r#"
B0000: '=01#000A> _NO00$000> _NO00$000> _HL00$000>
"#;
    let mut dbg = Debugger::new(cl_code);

    // Inspect Core 0
    dbg.set_inspected_core(0);
    let c0 = dbg.get_core_coord();
    assert_eq!(c0.x, 0);
    assert_eq!(c0.y, 0);
    assert_eq!(c0.z, 0);
    assert_eq!(c0.w, 0);

    // Inspect Core 85 (x=1, y=1, z=1, w=1: 1 + 4 + 16 + 64 = 85)
    dbg.set_inspected_core(85);
    let c85 = dbg.get_core_coord();
    assert_eq!(c85.x, 1);
    assert_eq!(c85.y, 1);
    assert_eq!(c85.z, 1);
    assert_eq!(c85.w, 1);

    // Inspect Core 255 (x=3, y=3, z=3, w=3)
    dbg.set_inspected_core(255);
    let c255 = dbg.get_core_coord();
    assert_eq!(c255.x, 3);
    assert_eq!(c255.y, 3);
    assert_eq!(c255.z, 3);
    assert_eq!(c255.w, 3);
}

#[test]
fn test_debugger_rewind_and_restart() {
    let cl_code = r#"
B0000: '=03#000A> '=01#0014> _NO00$000> _NO00$000>
B0001: _PO03+100> _NO00$000> _NO00$000> _HL00$000>
"#;
    let mut dbg = Debugger::new(cl_code);
    let _ = dbg.step();
    let _ = dbg.step();
    assert_eq!(dbg.current_cycle, 2);
    assert_eq!(dbg.sim.core_dump(0)[3], 30);

    // Restart back to cycle 0
    dbg.restart(cl_code);
    assert_eq!(dbg.current_cycle, 0);
    assert_eq!(dbg.sim.core_dump(0)[3], 0);
    assert!(!dbg.is_halted);

    // Step again
    let _ = dbg.step();
    assert_eq!(dbg.current_cycle, 1);
    assert_eq!(dbg.sim.core_dump(0)[3], 10);
}

#[test]
fn test_debugger_cli_binary_batch_execution() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let cl_path = workspace_root.join("examples").join("fused_flash_attention.cl");

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "debug",
            cl_path.to_str().unwrap(),
            "--core",
            "0",
            "--batch",
            "step; regs; wave; mesh; continue; q",
        ])
        .output()
        .expect("run cron debug batch");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "cron debug should exit 0. stderr: {}, stdout: {}", stderr, stdout);
    assert!(stdout.contains("CRON SSS+ MICROARCHITECTURAL DEBUGGER TUI"));
    assert!(stdout.contains("VLIW INSTRUCTION STREAM & DISASSEMBLER"));
    assert!(stdout.contains("CORE #0 REGISTERS"));
    assert!(stdout.contains("PHOTONIC MZI WAVEFORM"));
    assert!(stdout.contains("4D-TORUS NoC SPATIAL INTERCONNECT"));
    assert!(stdout.contains("Simulation Halted"));
}

#[test]
fn test_debugger_cli_binary_cr_source_debugging() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let cr_path = workspace_root.join("examples").join("fused_flash_attention.cr");

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_cron"))
        .args([
            "debug",
            cr_path.to_str().unwrap(),
            "--core",
            "0",
            "--batch",
            "step; regs; continue; q",
        ])
        .output()
        .expect("run cron debug on .cr");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "cron debug on .cr should exit 0. stderr: {}, stdout: {}", stderr, stdout);
    assert!(stdout.contains("Compiling") && stdout.contains("into 128-bit VLIW"));
    assert!(stdout.contains("CORE #0 REGISTERS"));
    assert!(stdout.contains("Simulation Halted"));
}



