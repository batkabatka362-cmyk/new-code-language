// ============================================================================
// CRON Low-Level Machine Language (.cl) Interactive REPL & Vibe Console
// Allows interactive, cycle-by-cycle execution of .cl VLIW bundles,
// live register/photonic inspection, and on-the-fly AI self-healing.
// ============================================================================

use cronc::{heal_cl_program, optimize_cl_program, ClJitCore};
use std::io::{self, Write};

pub fn start_cl_repl() {
    println!(r#"
============================================================
   CRON .cl INTERACTIVE VIBE-CODING REPL & SILICON CONSOLE
   Target: 256-Core 4D-Torus Hybrid Photonic Neuromorphic
============================================================
 Commands:
   :regs              Display 16 physical registers (R0..R15)
   :state             Display 6-Brain silicon counters & telemetry
   :reset             Reset silicon core state to initial power-on
   :heal <bundle>     Auto-repair CRC-8 ATM and pad slots
   :opt <cl_code>     Run VLIW slot compaction super-optimizer
   :help              Display this help menu
   :exit / :quit      Exit the interactive console

 Type any .cl VLIW bundle (e.g. B0000: '==01#005> '==02#003> _PO01+000> _NO00#000>)
 or individual slot tokens to execute immediately in RAM!
"#);

    let mut core = ClJitCore::new();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("cron-cl [{}]> ", core.cycle_count);
        io::stdout().flush().unwrap();
        input.clear();

        if stdin.read_line(&mut input).is_err() || input.is_empty() {
            println!("\nExiting CRON .cl REPL.");
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == ":exit" || trimmed == ":quit" || trimmed == "exit" || trimmed == "quit" {
            println!("CRON .cl REPL terminated.");
            break;
        }

        if trimmed == ":regs" {
            print_registers(&core);
            continue;
        }

        if trimmed == ":state" {
            print_state(&core);
            continue;
        }

        if trimmed == ":reset" {
            core.reset();
            println!("[RESET] Silicon Core reset to Cycle 0. Registers cleared.");
            continue;
        }

        if trimmed == ":help" {
            println!("Type raw VLIW bundles or commands (:regs, :state, :reset, :heal <bundle>, :opt <bundle>, :exit)");
            continue;
        }

        if let Some(arg) = trimmed.strip_prefix(":heal ") {
            match heal_cl_program(arg) {
                Ok(rep) => {
                    println!("[HEALED] Slots repaired: {}, Bundles padded: {}", rep.slots_repaired_crc, rep.bundles_padded_nops);
                    println!("{}", rep.canonical_code);
                }
                Err(e) => eprintln!("[HEAL ERROR] {}", e),
            }
            continue;
        }

        if let Some(arg) = trimmed.strip_prefix(":opt ") {
            match optimize_cl_program(arg) {
                Ok(rep) => {
                    println!("[OPTIMIZED] Speedup: +{:.1}% | IPC: {:.2} -> {:.2}", rep.speedup_percentage, rep.original_ipc, rep.optimized_ipc);
                    println!("{}", rep.optimized_code);
                }
                Err(e) => eprintln!("[OPT ERROR] {}", e),
            }
            continue;
        }

        // Prepare line for execution: if missing "Bxxxx:", format as next cycle
        let line_to_exec = if trimmed.starts_with('B') && trimmed.contains(':') {
            trimmed.to_string()
        } else if trimmed.starts_with('_') || trimmed.starts_with('\'') {
            // Auto-heal single or multiple slots into a 4-slot bundle
            match heal_cl_program(trimmed) {
                Ok(rep) => rep.canonical_code.lines().find(|l| l.starts_with('B')).unwrap_or("").to_string(),
                Err(_) => format!("B{:04}: {} _NO00#000> _NO00#000> _NO00#000>", core.cycle_count, trimmed),
            }
        } else {
            eprintln!("Unknown command or invalid .cl slot syntax: '{}'. Type :help for commands.", trimmed);
            continue;
        };

        if line_to_exec.is_empty() {
            continue;
        }

        match cronc::cl_jit::execute_cl_on_core(&line_to_exec, &mut core) {
            Ok(()) => {
                println!("  ✓ Executed: {}", line_to_exec);
                print_register_summary(&core);
            }
            Err(e) => {
                eprintln!("  ✗ Execution Error: {}", e);
            }
        }
    }
}

fn print_registers(core: &ClJitCore) {
    println!("+-------------------------------------------------------------+");
    println!("|                   PHYSICAL REGISTER FILE (16 x 32-bit)       |");
    println!("+-------------------------------------------------------------+");
    for i in (0..16).step_by(4) {
        println!(
            "| R{:02}: 0x{:08X} ({:<10}) | R{:02}: 0x{:08X} ({:<10}) |",
            i, core.r[i], core.r[i], i + 1, core.r[i + 1], core.r[i + 1]
        );
        println!(
            "| R{:02}: 0x{:08X} ({:<10}) | R{:02}: 0x{:08X} ({:<10}) |",
            i + 2, core.r[i + 2], core.r[i + 2], i + 3, core.r[i + 3], core.r[i + 3]
        );
    }
    println!("+-------------------------------------------------------------+");
}

fn print_register_summary(core: &ClJitCore) {
    print!("    Active Regs: ");
    let mut non_zero = 0;
    for i in 0..16 {
        if core.r[i] != 0 {
            print!("R{}={:#X} ", i, core.r[i]);
            non_zero += 1;
        }
    }
    if non_zero == 0 {
        print!("(all zero)");
    }
    println!();
}

fn print_state(core: &ClJitCore) {
    println!("+-------------------------------------------------------------+");
    println!("|           256-CORE 4D-TORUS SILICON HARDWARE STATE           |");
    println!("+-------------------------------------------------------------+");
    println!("  Total Cycles:             {}", core.cycle_count);
    println!("  Photonic MZI Optical Ops: {}", core.optical_gemm_count);
    println!("  Reversible Gate Ops:      {}", core.reversible_ops_count);
    println!("  STDP Synapse Updates:     {}", core.stdp_updates_count);
    println!("  Spatial Broadcasts:       {}", core.spatial_broadcast_count);
    println!("  Barrier Synchronizations: {}", core.barrier_count);
    println!("  Fused Streaming Ops:      {}", core.fused_ops_count);
    println!("  DRAM/HBM Bytes Saved:     {} B", core.hbm_bytes_saved);
    println!("  Halted State:             {}", core.is_halted);
    println!("+-------------------------------------------------------------+");
}
