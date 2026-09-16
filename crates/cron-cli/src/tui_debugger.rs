// ============================================================================
// CRON SSS+ Microarchitectural Hardware Debugger TUI
// Interactive Terminal User Interface & Batch Script Runner
// Target: 256-Core 4D-Torus Photonic Neuromorphic Silicon Architecture
// ============================================================================

use cron_vm::debugger::{Breakpoint, Debugger, StepResult};
use std::io::{self, BufRead, Write};

// ANSI Terminal Styling Codes
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_DIM: &str = "\x1b[2m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_MAGENTA: &str = "\x1b[35m";
const ANSI_CYAN: &str = "\x1b[36m";
const ANSI_WHITE: &str = "\x1b[37m";
const ANSI_HIGHLIGHT: &str = "\x1b[1;92m"; // Bright Green for modified values

pub struct TuiDebugger {
    pub debugger: Debugger,
    pub filename: String,
    pub cl_source: String,
    pub quiet: bool,
}

impl TuiDebugger {
    pub fn new(filename: &str, cl_code: &str) -> Self {
        let debugger = Debugger::new(cl_code);
        Self {
            debugger,
            filename: filename.to_string(),
            cl_source: cl_code.to_string(),
            quiet: false,
        }
    }

    /// Renders graphical ASCII energy bar (0..255)
    pub fn render_energy_bar(val: u8, width: usize) -> String {
        let filled = ((val as usize) * width) / 255;
        let empty = width.saturating_sub(filled);
        format!(
            "{}{}{}{}{}",
            ANSI_CYAN,
            "█".repeat(filled),
            ANSI_DIM,
            "░".repeat(empty),
            ANSI_RESET
        )
    }

    /// Renders thermal temperature gradient bar (25°C .. 180°C threshold)
    pub fn render_thermal_bar(temp: u32, width: usize) -> String {
        let clamped = temp.clamp(25, 180);
        let fraction = (clamped - 25) as f64 / (180 - 25) as f64;
        let filled = (fraction * (width as f64)).round() as usize;
        let empty = width.saturating_sub(filled);

        let color = if temp >= 150 {
            ANSI_RED
        } else if temp >= 80 {
            ANSI_YELLOW
        } else {
            ANSI_GREEN
        };

        format!(
            "{}{}{}{}{} {}°C{}",
            color,
            "█".repeat(filled),
            ANSI_DIM,
            "░".repeat(empty),
            ANSI_RESET,
            temp,
            if temp >= 180 { " [SENTRY TRIP]" } else { "" }
        )
    }

    /// Renders the complete Debugger HUD
    pub fn render_hud(&self) {
        let core_id = self.debugger.inspected_core;
        let coord = self.debugger.get_core_coord();
        let current_cycle = self.debugger.current_cycle;
        let total_insts = self.debugger.disassembly.len();
        let temp = self.debugger.get_core_temp();

        println!();
        println!(
            "{}╔════════════════════════════════════════════════════════════════════════════════════════════════════╗{}",
            ANSI_CYAN, ANSI_RESET
        );
        println!(
            "{}║{}  {}CRON SSS+ MICROARCHITECTURAL DEBUGGER TUI{} | 256-Core 4D-Torus Neuromorphic Processor         {}║{}",
            ANSI_CYAN, ANSI_RESET, ANSI_BOLD, ANSI_RESET, ANSI_CYAN, ANSI_RESET
        );
        println!(
            "{}╠════════════════════════════════════════════════════════════════════════════════════════════════════╣{}",
            ANSI_CYAN, ANSI_RESET
        );
        println!(
            "{}║{} File: {}{:<22}{} | Cycle: {}{:>3}/{:<3}{} | Core: {}{:>3}{} [X:{} Y:{} Z:{} W:{}] | Temp: {:<12} {}║{}",
            ANSI_CYAN,
            ANSI_RESET,
            ANSI_BOLD,
            self.filename,
            ANSI_RESET,
            ANSI_YELLOW,
            current_cycle,
            total_insts,
            ANSI_RESET,
            ANSI_MAGENTA,
            core_id,
            ANSI_RESET,
            coord.x,
            coord.y,
            coord.z,
            coord.w,
            Self::render_thermal_bar(temp, 8),
            ANSI_CYAN,
            ANSI_RESET
        );
        println!(
            "{}╚════════════════════════════════════════════════════════════════════════════════════════════════════╝{}",
            ANSI_CYAN, ANSI_RESET
        );

        // Panel 1: Steppable VLIW Instruction Stream & Disassembler
        self.render_disassembly_panel();

        // Panel 2: Silicon Register HUD with Diff Highlights
        self.render_register_panel();

        // Panel 3: Photonic Waveform & Neuromorphic STDP HUD
        self.render_photonic_and_synapse_panel();

        // Panel 4: 4D Torus NoC Mesh & Routing HUD
        self.render_noc_mesh_panel();

        if let Some(ref bp) = self.debugger.hit_breakpoint {
            println!(
                "\n{}>>> BREAKPOINT HIT: {} <<<{}\n",
                ANSI_HIGHLIGHT,
                bp.description(),
                ANSI_RESET
            );
        } else if self.debugger.is_halted {
            println!(
                "\n{}[SILICON CORE #{} QUIESCENT HALTED - Simulation Complete]{}\n",
                ANSI_YELLOW, core_id, ANSI_RESET
            );
        }
    }

    /// Renders Panel 1: Disassembly window with active PC cursor and slot breakdown
    pub fn render_disassembly_panel(&self) {
        println!(
            "\n{}--- [VLIW INSTRUCTION STREAM & DISASSEMBLER] ----------------------------------------------------{}",
            ANSI_BOLD, ANSI_RESET
        );

        let current_step = self.debugger.sim.step_index;
        let total = self.debugger.disassembly.len();

        let window_start = current_step.saturating_sub(2);
        let window_end = (current_step + 3).min(total);

        for i in window_start..window_end {
            let bundle = &self.debugger.disassembly[i];
            let is_current = i == current_step;

            // Check if any breakpoint matches this bundle cycle
            let has_bp = self.debugger.breakpoints.iter().any(|bp| match bp {
                Breakpoint::Cycle(c) => *c == bundle.cycle,
                Breakpoint::Opcode(op) => bundle.slots.iter().any(|s| s.opcode == *op || s.raw.contains(op)),
                _ => false,
            });

            let bp_marker = if has_bp {
                format!("{}[*B*]{}", ANSI_RED, ANSI_RESET)
            } else {
                "     ".to_string()
            };

            let pointer = if is_current {
                format!("{}{}==>{}", ANSI_BOLD, ANSI_YELLOW, ANSI_RESET)
            } else {
                "   ".to_string()
            };

            let line_color = if is_current { ANSI_BOLD } else { ANSI_DIM };
            println!(
                "  {} {} {}B{:04}: {}{}",
                pointer, bp_marker, line_color, bundle.cycle, bundle.raw, ANSI_RESET
            );

            // If this is the current bundle, show microarchitectural slot breakdown
            if is_current {
                for (s_idx, slot) in bundle.slots.iter().enumerate() {
                    let slot_color = match slot.opcode.as_str() {
                        "OP" | "WD" | "11" => ANSI_CYAN,
                        "RF" | "TO" | "BK" => ANSI_MAGENTA,
                        "ST" | "LI" | "LF" => ANSI_GREEN,
                        "TX" | "RX" | "SB" | "WH" | "bb" => ANSI_BLUE,
                        "FU" | "FE" => ANSI_YELLOW,
                        "LOAD_IMM" => ANSI_WHITE,
                        "HL" => ANSI_RED,
                        _ => ANSI_RESET,
                    };

                    println!(
                        "       {}│ Slot {}:{} {:<11} ─ {} {}{}",
                        ANSI_DIM,
                        s_idx,
                        ANSI_RESET,
                        slot.raw,
                        slot_color,
                        slot.description,
                        ANSI_RESET
                    );
                }
            }
        }
    }

    /// Renders Panel 2: Register HUD with ABI names and Diff Highlights
    pub fn render_register_panel(&self) {
        let core_id = self.debugger.inspected_core;
        let regs = self.debugger.sim.core_dump(core_id);
        let abi_names = [
            "$rv ", "$a0 ", "$a1 ", "$a2 ", "$t0 ", "$t1 ", "$t2 ", "$s0 ",
            "$s1 ", "$s2 ", "$im0", "$im1", "$im2", "$ar0", "$ar1", "$bp ",
        ];

        println!(
            "\n{}--- [CORE #{} REGISTERS (CRON STACKLESS ABI)] ----------------------------------------------------{}",
            ANSI_BOLD, core_id, ANSI_RESET
        );

        for row in 0..4 {
            let mut row_str = String::new();
            for col in 0..4 {
                let idx = row * 4 + col;
                let is_diff = self.debugger.last_reg_diffs.iter().any(|d| d.reg == idx);

                let val_str = if is_diff {
                    format!(
                        "{}{}: 0x{:08X} ({:>6}) *MOD*{}",
                        ANSI_HIGHLIGHT, abi_names[idx], regs[idx], regs[idx], ANSI_RESET
                    )
                } else {
                    format!(
                        "{}{}{}: 0x{:08X} ({:>6})      {}",
                        ANSI_DIM, abi_names[idx], ANSI_RESET, regs[idx], regs[idx], ANSI_RESET
                    )
                };

                row_str.push_str(&format!("  {:<32}", val_str));
            }
            println!("{}", row_str);
        }
    }

    /// Renders Panel 3: Photonic Waveform & Neuromorphic STDP HUD
    pub fn render_photonic_and_synapse_panel(&self) {
        let wave = self.debugger.get_core_wave();
        let weights = self.debugger.get_core_stdp_weights();
        let rev_depth = self.debugger.get_core_rev_depth();

        println!(
            "\n{}--- [PHOTONIC MZI WAVEFORM & NEUROMORPHIC SYNAPSES] ---------------------------------------------{}",
            ANSI_BOLD, ANSI_RESET
        );

        // Photonic MZI Optical Channels
        println!("  {}Photonic MZI Optical Wave (Brain 2):{}", ANSI_CYAN, ANSI_RESET);
        for ch in 0..4 {
            let amp = wave.amplitudes[ch];
            let phase_deg = ((wave.phases[ch] as f64 / 255.0) * 360.0).round() as u32;
            let bar = Self::render_energy_bar(amp, 12);
            println!(
                "    Channel {}: {} {:>3}/255  | Phase: {:>3}° | Amp Bar: {}",
                ch, bar, amp, phase_deg, bar
            );
        }

        // Neuromorphic STDP Synapse Weights & Reversible Stack Depth
        let mut synapse_str = String::new();
        for (i, w) in weights.iter().take(8).enumerate() {
            let w_color = if *w > 0 { ANSI_GREEN } else if *w < 0 { ANSI_RED } else { ANSI_DIM };
            synapse_str.push_str(&format!("S{}:{}{:>3}{} ", i, w_color, w, ANSI_RESET));
        }
        println!("  {}STDP Synapses (Brain 4):{} {}", ANSI_GREEN, ANSI_RESET, synapse_str);
        println!(
            "  {}Reversible Thermodynamics (Brain 3):{} Stack Depth = {} | Saved DRAM Bandwidth = {:.3} KB",
            ANSI_MAGENTA, ANSI_RESET, rev_depth, (self.debugger.sim.stats.reversible_gate_ops * 128) as f64 / 1024.0
        );
    }

    /// Renders Panel 4: 4D Torus NoC Mesh & Spatial Interconnect
    pub fn render_noc_mesh_panel(&self) {
        let core_id = self.debugger.inspected_core;
        let coord = self.debugger.get_core_coord();

        println!(
            "\n{}--- [4D-TORUS NoC SPATIAL INTERCONNECT] --------------------------------------------------------{}",
            ANSI_BOLD, ANSI_RESET
        );

        let axes = ["X+", "X-", "Y+", "Y-", "Z+", "Z-", "W+", "W-"];
        let mut neighbors_str = String::new();
        for axis in &axes {
            let n_coord = coord.neighbor(axis);
            let n_id = n_coord.to_core_id();
            neighbors_str.push_str(&format!("{}: Core #{:<3} ", axis, n_id));
        }

        let pending_rx = self.debugger.sim.mesh.packet_queues[core_id].len();
        println!("  Spatial 8-Neighbors: {}", neighbors_str);
        println!(
            "  Mesh Packet Routed (Chip-Wide): {} pkts | Core #{:<3} Inbound Queue: {} pkts",
            self.debugger.sim.stats.mesh_packets_routed, core_id, pending_rx
        );
    }

    /// Disassembles full program listing
    pub fn print_full_disassembly(&self) {
        println!(
            "\n{}=== COMPLETE DISASSEMBLY LISTING ({}) ==={}",
            ANSI_BOLD, self.filename, ANSI_RESET
        );
        for bundle in &self.debugger.disassembly {
            println!("  B{:04}: {}", bundle.cycle, bundle.raw);
            for (idx, slot) in bundle.slots.iter().enumerate() {
                println!("    Slot {}: {:<12} -> {}", idx, slot.raw, slot.description);
            }
        }
        println!();
    }

    /// Disassembles and displays active breakpoints
    pub fn print_breakpoints(&self) {
        println!("\n{}Active Hardware Breakpoints:{}", ANSI_BOLD, ANSI_RESET);
        if self.debugger.breakpoints.is_empty() {
            println!("  (no breakpoints set)");
        } else {
            for (idx, bp) in self.debugger.breakpoints.iter().enumerate() {
                println!("  [{}] {}", idx, bp.description());
            }
        }
        println!();
    }

    /// Executes a single command string in the debugger
    pub fn execute_command(&mut self, cmd_line: &str) -> bool {
        let parts: Vec<&str> = cmd_line.split_whitespace().collect();
        if parts.is_empty() {
            return true;
        }

        match parts[0] {
            "step" | "s" => {
                let count = if parts.len() > 1 {
                    parts[1].parse::<usize>().unwrap_or(1)
                } else {
                    1
                };
                for _ in 0..count {
                    let res = self.debugger.step();
                    match res {
                        StepResult::BreakpointHit { .. } | StepResult::Halted { .. } | StepResult::NoMoreInstructions => {
                            break;
                        }
                        StepResult::Stepped { .. } => {}
                    }
                }
                if !self.quiet {
                    self.render_hud();
                }
            }
            "next" | "n" => {
                let _ = self.debugger.step();
                if !self.quiet {
                    self.render_hud();
                }
            }
            "continue" | "c" => {
                let res = self.debugger.continue_exec();
                if !self.quiet {
                    self.render_hud();
                }
                match res {
                    StepResult::BreakpointHit { cycle, breakpoint } => {
                        println!("Paused at Cycle {} due to Breakpoint: {}", cycle, breakpoint.description());
                    }
                    StepResult::Halted { total_cycles } => {
                        println!("Simulation Halted after {} cycles.", total_cycles);
                    }
                    _ => {}
                }
            }
            "break" | "b" => {
                if parts.len() < 2 {
                    println!("Usage: break <cycle N | op OP | reg R == VAL | trap | temp N>");
                    return true;
                }
                match parts[1] {
                    "cycle" if parts.len() >= 3 => {
                        if let Ok(c) = parts[2].parse::<usize>() {
                            self.debugger.add_breakpoint(Breakpoint::Cycle(c));
                            println!("Breakpoint added: Cycle == {}", c);
                        }
                    }
                    "op" if parts.len() >= 3 => {
                        let op = parts[2].to_uppercase();
                        self.debugger.add_breakpoint(Breakpoint::Opcode(op.clone()));
                        println!("Breakpoint added: Opcode == '{}'", op);
                    }
                    "trap" => {
                        self.debugger.add_breakpoint(Breakpoint::Trap);
                        println!("Breakpoint added on Hardware Exception Trap");
                    }
                    "temp" if parts.len() >= 3 => {
                        if let Ok(t) = parts[2].parse::<u32>() {
                            self.debugger.add_breakpoint(Breakpoint::Thermal(t));
                            println!("Breakpoint added: Temperature >= {}°C", t);
                        }
                    }
                    s if s.starts_with('r') && parts.len() >= 4 => {
                        // e.g. b r0 == 31
                        let reg_num = s.trim_start_matches('r').parse::<usize>().unwrap_or(0);
                        let op = parts[2].to_string();
                        let val = if parts[3].starts_with("0x") || parts[3].starts_with("0X") {
                            u32::from_str_radix(&parts[3][2..], 16).unwrap_or(0)
                        } else {
                            parts[3].parse::<u32>().unwrap_or(0)
                        };
                        self.debugger.add_breakpoint(Breakpoint::RegisterCond {
                            core_id: self.debugger.inspected_core,
                            reg: reg_num,
                            op: op.clone(),
                            val,
                        });
                        println!("Breakpoint added: Core {} R{} {} 0x{:08X}", self.debugger.inspected_core, reg_num, op, val);
                    }
                    _ => {
                        if let Ok(c) = parts[1].parse::<usize>() {
                            self.debugger.add_breakpoint(Breakpoint::Cycle(c));
                            println!("Breakpoint added: Cycle == {}", c);
                        } else {
                            println!("Unknown breakpoint target: '{}'", parts[1]);
                        }
                    }
                }
            }
            "delete" | "d" => {
                if parts.len() >= 2 {
                    if let Ok(idx) = parts[1].parse::<usize>() {
                        if let Some(removed) = self.debugger.remove_breakpoint(idx) {
                            println!("Removed breakpoint: {}", removed.description());
                        } else {
                            println!("Invalid breakpoint index: {}", idx);
                        }
                    }
                } else {
                    self.debugger.clear_breakpoints();
                    println!("Cleared all breakpoints.");
                }
            }
            "info" | "i" => {
                if parts.len() >= 2 && (parts[1] == "break" || parts[1] == "b") {
                    self.print_breakpoints();
                } else {
                    self.render_hud();
                }
            }
            "core" | "k" => {
                if parts.len() >= 2 {
                    if let Ok(id) = parts[1].parse::<usize>() {
                        if id < 256 {
                            self.debugger.set_inspected_core(id);
                            println!("Switched to Core #{}.", id);
                            if !self.quiet {
                                self.render_hud();
                            }
                        } else {
                            println!("Core ID out of range (0..255).");
                        }
                    }
                }
            }
            "reset" | "r" => {
                self.debugger.restart(&self.cl_source);
                println!("Simulator rewound to Cycle 0.");
                if !self.quiet {
                    self.render_hud();
                }
            }
            "regs" => {
                self.render_register_panel();
            }
            "wave" => {
                self.render_photonic_and_synapse_panel();
            }
            "mesh" => {
                self.render_noc_mesh_panel();
            }
            "disasm" => {
                self.print_full_disassembly();
            }
            "help" | "h" => {
                self.print_help();
            }
            "quit" | "q" | "exit" => {
                return false;
            }
            _ => {
                println!("Unknown command: '{}'. Type 'help' for command reference.", parts[0]);
            }
        }
        true
    }

    /// Prints help menu
    pub fn print_help(&self) {
        println!("\n{}CRON Hardware Debugger Command Reference:{}", ANSI_BOLD, ANSI_RESET);
        println!("  {}step [N]{} / {}s [N]{}       - Step N cycles (default 1)", ANSI_YELLOW, ANSI_RESET, ANSI_YELLOW, ANSI_RESET);
        println!("  {}next{} / {}n{}               - Step single cycle forward", ANSI_YELLOW, ANSI_RESET, ANSI_YELLOW, ANSI_RESET);
        println!("  {}continue{} / {}c{}           - Execute until breakpoint or halt", ANSI_YELLOW, ANSI_RESET, ANSI_YELLOW, ANSI_RESET);
        println!("  {}break cycle <N>{} / {}b <N>{}  - Break at specific cycle", ANSI_YELLOW, ANSI_RESET, ANSI_YELLOW, ANSI_RESET);
        println!("  {}break op <OP>{}             - Break on opcode (e.g. OP, FU, ST, bb)", ANSI_YELLOW, ANSI_RESET);
        println!("  {}break rX == <val>{}         - Break on register value (e.g. r0 == 31)", ANSI_YELLOW, ANSI_RESET);
        println!("  {}break trap{}                - Break on hardware exception trap", ANSI_YELLOW, ANSI_RESET);
        println!("  {}break temp <T>{}            - Break when temperature >= T°C", ANSI_YELLOW, ANSI_RESET);
        println!("  {}info break{} / {}ib{}         - List active breakpoints", ANSI_YELLOW, ANSI_RESET, ANSI_YELLOW, ANSI_RESET);
        println!("  {}delete [idx]{} / {}d [idx]{}   - Delete breakpoint by index (or all)", ANSI_YELLOW, ANSI_RESET, ANSI_YELLOW, ANSI_RESET);
        println!("  {}core <0..255>{} / {}k <ID>{}  - Switch inspected core in 4D Torus", ANSI_YELLOW, ANSI_RESET, ANSI_YELLOW, ANSI_RESET);
        println!("  {}reset{} / {}r{}              - Rewind simulation back to Cycle 0", ANSI_YELLOW, ANSI_RESET, ANSI_YELLOW, ANSI_RESET);
        println!("  {}regs{}                       - Display 16 registers with diffs", ANSI_YELLOW, ANSI_RESET);
        println!("  {}wave{}                       - Display photonic MZI amplitudes & phases", ANSI_YELLOW, ANSI_RESET);
        println!("  {}mesh{}                       - Display 4D-Torus NoC interconnect status", ANSI_YELLOW, ANSI_RESET);
        println!("  {}disasm{}                     - Print full program disassembly", ANSI_YELLOW, ANSI_RESET);
        println!("  {}quit{} / {}q{}               - Exit debugger", ANSI_YELLOW, ANSI_RESET, ANSI_YELLOW, ANSI_RESET);
        println!();
    }

    /// Runs interactive TUI session with terminal input loop
    pub fn run_interactive(&mut self) {
        self.render_hud();
        println!("Type 'help' for command reference, 'step' to step, 'continue' to run.\n");

        let stdin = io::stdin();
        let mut reader = stdin.lock();

        loop {
            print!("{}(cron-dbg) {}", ANSI_BOLD, ANSI_RESET);
            let _ = io::stdout().flush();

            let mut line = String::new();
            if reader.read_line(&mut line).is_err() || line.is_empty() {
                break;
            }

            let keep_running = self.execute_command(&line);
            if !keep_running {
                break;
            }
        }
        println!("Exiting CRON Debugger.");
    }

    /// Runs non-interactive batch script (semi-colon separated commands)
    pub fn run_batch(&mut self, batch_script: &str) {
        self.quiet = false;
        println!("[CRON DEBUGGER] Executing batch script: \"{}\"", batch_script);
        for cmd in batch_script.split(';') {
            let trimmed = cmd.trim();
            if trimmed.is_empty() {
                continue;
            }
            println!("{}>>> DEBUG CMD: {}{}", ANSI_CYAN, trimmed, ANSI_RESET);
            let keep_running = self.execute_command(trimmed);
            if !keep_running {
                break;
            }
        }
    }
}
