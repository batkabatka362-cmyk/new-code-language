// ============================================================================
// CRON Cognitive Interactive REPL (Read-Eval-Print Loop)
// Target: 256-Core 4D-Torus Neuromorphic Hardware Simulator
// Provides interactive expression evaluation, register inspection, and mesh HUD.
// ============================================================================

use cron_vm::Simulator;
use std::io::{self, BufRead, Write};

pub struct ReplSession {
    pub declarations: Vec<String>,
    pub statements: Vec<String>,
    pub sim: Simulator,
    pub last_cl: Option<String>,
    pub step_count: usize,
}

impl ReplSession {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
            statements: Vec::new(),
            sim: Simulator::new(),
            last_cl: None,
            step_count: 0,
        }
    }

    pub fn print_help(&self) {
        println!("\x1b[1;36mCRON Interactive REPL Commands:\x1b[0m");
        println!("  \x1b[1;33m:help\x1b[0m, \x1b[1;33m:h\x1b[0m       Show this help message");
        println!("  \x1b[1;33m:regs\x1b[0m, \x1b[1;33m:r\x1b[0m       Inspect Core 0 registers ($rv, $a0..$a2, $t0..$t2, etc.)");
        println!("  \x1b[1;33m:mesh\x1b[0m, \x1b[1;33m:cores\x1b[0m   Display 4D-Torus hardware mesh status & telemetry");
        println!("  \x1b[1;33m:cl\x1b[0m             Display 128-bit VLIW machine bundles from last evaluation");
        println!("  \x1b[1;33m:reset\x1b[0m          Clear all definitions and reset 4D-Torus simulator");
        println!("  \x1b[1;33m:history\x1b[0m        List all inputs evaluated in this session");
        println!("  \x1b[1;33m:quit\x1b[0m, \x1b[1;33m:exit\x1b[0m   Exit the REPL session");
        println!();
        println!("Expressions and statements evaluate immediately on the 4D-Torus simulator.");
        println!("Definitions ('fn', 'struct', 'trait', 'brain') persist across lines.");
    }

    pub fn print_regs(&self) {
        let regs = self.sim.cores[0].registers;
        println!("\x1b[1;35mCore 0 Registers (CRON Stackless ABI):\x1b[0m");
        println!("  $rv  (R0):  0x{:08X} ({:>10})  |  $a0  (R1):  0x{:08X} ({:>10})", regs[0], regs[0], regs[1], regs[1]);
        println!("  $a1  (R2):  0x{:08X} ({:>10})  |  $a2  (R3):  0x{:08X} ({:>10})", regs[2], regs[2], regs[3], regs[3]);
        println!("  $t0  (R4):  0x{:08X} ({:>10})  |  $t1  (R5):  0x{:08X} ({:>10})", regs[4], regs[4], regs[5], regs[5]);
        println!("  $t2  (R6):  0x{:08X} ({:>10})  |  $s0  (R7):  0x{:08X} ({:>10})", regs[6], regs[6], regs[7], regs[7]);
        println!("  $s1  (R8):  0x{:08X} ({:>10})  |  $s2  (R9):  0x{:08X} ({:>10})", regs[8], regs[8], regs[9], regs[9]);
        println!("  $im0 (R10): 0x{:08X} ({:>10})  |  $im1 (R11): 0x{:08X} ({:>10})", regs[10], regs[10], regs[11], regs[11]);
        println!("  $im2 (R12): 0x{:08X} ({:>10})  |  $ar0 (R13): 0x{:08X} ({:>10})", regs[12], regs[12], regs[13], regs[13]);
        println!("  $ar1 (R14): 0x{:08X} ({:>10})  |  $bp  (R15): 0x{:08X} ({:>10})", regs[14], regs[14], regs[15], regs[15]);
    }

    pub fn print_mesh(&self) {
        let core0 = &self.sim.cores[0];
        println!("\x1b[1;32m4D-Torus Neuromorphic Hardware Status:\x1b[0m");
        println!("  Mesh Geometry:       4x4x4x4 (256 Cores Total)");
        println!("  Core 0 Coords:       [X:0, Y:0, Z:0, W:0]");
        println!("  Active Fibers:       {}", self.sim.active_fiber_count());
        println!("  Total Cycles Run:    {}", self.sim.stats.total_cycles);
        println!("  Packets Routed:      {}", self.sim.stats.mesh_packets_routed);
        println!("  Optical GEMM Ops:    {}", self.sim.stats.optical_gemm_ops);
        println!("  Reversible Gate Ops: {}", self.sim.stats.reversible_gate_ops);
        println!("  STDP Updates:        {}", self.sim.stats.stdp_synapse_updates);
        println!("  Thermal Threshold:   {}°C", core0.thermal_threshold);
    }

    pub fn print_last_cl(&self) {
        if let Some(ref cl) = self.last_cl {
            println!("\x1b[1;36mMachine-Native Low-Level .cl VLIW Code:\x1b[0m");
            for line in cl.lines() {
                println!("  {}", line);
            }
        } else {
            println!("\x1b[33mNo previous machine code generated yet.\x1b[0m");
        }
    }

    pub fn reset(&mut self) {
        self.declarations.clear();
        self.statements.clear();
        self.sim = Simulator::new();
        self.last_cl = None;
        self.step_count = 0;
        println!("\x1b[1;32m[RESET] Session cleared. 4D-Torus Simulator reset to initial state.\x1b[0m");
    }

    pub fn history(&self) {
        println!("\x1b[1;36mSession History:\x1b[0m");
        if self.declarations.is_empty() && self.statements.is_empty() {
            println!("  (empty)");
            return;
        }
        for (i, d) in self.declarations.iter().enumerate() {
            println!("  decl #{}: {}", i + 1, d.replace('\n', " "));
        }
        for (i, s) in self.statements.iter().enumerate() {
            println!("  stmt #{}: {}", i + 1, s.replace('\n', " "));
        }
    }

    pub fn eval_line(&mut self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            return true;
        }

        // Colon commands
        if let Some(cmd) = trimmed.strip_prefix(':') {
            match cmd {
                "help" | "h" | "?" => self.print_help(),
                "regs" | "r" => self.print_regs(),
                "mesh" | "cores" => self.print_mesh(),
                "cl" => self.print_last_cl(),
                "reset" => self.reset(),
                "history" => self.history(),
                "quit" | "q" | "exit" => return false,
                _ => {
                    println!("\x1b[31mUnknown command ':{cmd}'. Type ':help' for available commands.\x1b[0m");
                }
            }
            return true;
        }

        // Low-level VLIW direct injection (B0001: ...)
        if trimmed.starts_with('B') && trimmed.contains(':') {
            self.sim.load_machine_code(trimmed);
            let mut cycles = 0;
            while self.sim.step() {
                cycles += 1;
            }
            let r0 = self.sim.cores[0].registers[0];
            println!("\x1b[1;32m=> 0x{:08X} ({})\x1b[0m \x1b[90m[Direct VLIW, {} cycles]\x1b[0m", r0, r0, cycles);
            return true;
        }

        // Check if line is a top-level declaration
        if trimmed.starts_with("fn ")
            || trimmed.starts_with("def ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("trait ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("brain ")
            || trimmed.starts_with("type ")
            || trimmed.starts_with("import ")
        {
            // Verify compilation of candidate declaration
            let test_src = format!(
                ".MODULE ReplDecl\n.ENTRY _main\n{}\n_main:\n    let _unused = 0\n.END\n",
                trimmed
            );
            match cronc::compile_source(&test_src) {
                Ok(_) => {
                    self.declarations.push(trimmed.to_string());
                    let header = trimmed.lines().next().unwrap_or(trimmed).trim();
                    println!("\x1b[1;32m[Declared]\x1b[0m \x1b[1m{}\x1b[0m", header);
                }
                Err(e) => {
                    eprintln!("\x1b[31mDeclaration error:\x1b[0m\n{}", e);
                }
            }
            return true;
        }

        // Full program input (.MODULE ... .END)
        if trimmed.starts_with(".MODULE") {
            match cronc::compile_source(trimmed) {
                Ok(cl_code) => {
                    self.last_cl = Some(cl_code.clone());
                    self.sim = Simulator::new();
                    self.sim.load_machine_code(&cl_code);
                    let mut cycles = 0;
                    while self.sim.step() {
                        cycles += 1;
                    }
                    let r0 = self.sim.cores[0].registers[0];
                    println!("\x1b[1;32m=> 0x{:08X} ({})\x1b[0m \x1b[90m[Full Program, {} cycles]\x1b[0m", r0, r0, cycles);
                }
                Err(e) => {
                    eprintln!("\x1b[31mCompilation error:\x1b[0m\n{}", e);
                }
            }
            return true;
        }

        // Evaluate expression or statement
        let is_let = trimmed.starts_with("let ");
        let mut full_source = String::new();
        full_source.push_str(".MODULE ReplInteractive\n.ENTRY _main\n\n");
        for decl in &self.declarations {
            full_source.push_str(decl);
            full_source.push('\n');
        }
        full_source.push_str("\n_main:\n");
        for stmt in &self.statements {
            full_source.push_str("    ");
            full_source.push_str(stmt);
            full_source.push('\n');
        }
        full_source.push_str("    ");
        full_source.push_str(trimmed);
        full_source.push('\n');
        full_source.push_str(".END\n");

        match cronc::compile_source(&full_source) {
            Ok(cl_code) => {
                self.last_cl = Some(cl_code.clone());
                // Reset VM and run
                self.sim = Simulator::new();
                self.sim.load_machine_code(&cl_code);
                let mut cycles = 0;
                while self.sim.step() {
                    cycles += 1;
                }
                self.step_count += 1;
                let r0 = self.sim.cores[0].registers[0];

                if is_let {
                    self.statements.push(trimmed.to_string());
                    let var_name = trimmed.split_whitespace().nth(1).unwrap_or("var").trim_matches(':');
                    println!("\x1b[1;32m[Bound: {}]\x1b[0m \x1b[90m({} cycles)\x1b[0m", var_name, cycles);
                } else {
                    println!("\x1b[1;32m=> 0x{:08X} ({})\x1b[0m \x1b[90m[core 0, {} cycles, R0]\x1b[0m", r0, r0, cycles);
                }
            }
            Err(e) => {
                eprintln!("\x1b[31mCompilation error:\x1b[0m\n{}", e);
            }
        }

        true
    }
}

pub fn run_repl() {
    println!(r#"
  ██████╗██████╗  ██████╗ ███╗   ██╗
 ██╔════╝██╔══██╗██╔═══██╗████╗  ██║
 ██║     ██████╔╝██║   ██║██╔██╗ ██║
 ██║     ██╔══██╗██║   ██║██║╚██╗██║
 ╚██████╗██║  ██║╚██████╔╝██║ ╚████║
  ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
  CRON Cognitive Interactive REPL v1.0
  Target: 256-Core 4D-Torus Neuromorphic Hardware
  Type ':help' for commands, or enter CRON expressions/statements.
"#);

    let mut session = ReplSession::new();
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut multiline_buf = String::new();
    let mut brace_depth: i32 = 0;

    loop {
        if brace_depth > 0 {
            print!("\x1b[90m... \x1b[0m");
        } else {
            print!("\x1b[1;34mcron>\x1b[0m ");
        }
        let _ = io::stdout().flush();

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                // Count brace balance for multiline input
                for ch in line.chars() {
                    if ch == '{' {
                        brace_depth += 1;
                    } else if ch == '}' {
                        brace_depth -= 1;
                    }
                }

                multiline_buf.push_str(&line);

                if brace_depth <= 0 {
                    let to_eval = multiline_buf.clone();
                    multiline_buf.clear();
                    brace_depth = 0;

                    if !session.eval_line(&to_eval) {
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }

    println!("\x1b[1;36m[CRON REPL] Session terminated. Goodbye!\x1b[0m");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_session_let_and_eval() {
        let mut session = ReplSession::new();
        assert!(session.eval_line("let x = 10"));
        assert_eq!(session.statements.len(), 1);
        assert!(session.eval_line("let y = 20"));
        assert_eq!(session.statements.len(), 2);
        assert!(session.eval_line("x + y"));
        assert!(session.last_cl.is_some());
    }

    #[test]
    fn test_repl_colon_commands() {
        let mut session = ReplSession::new();
        assert!(session.eval_line(":help"));
        assert!(session.eval_line(":regs"));
        assert!(session.eval_line(":mesh"));
        assert!(session.eval_line(":cl"));
        assert!(session.eval_line(":history"));
        assert!(session.eval_line("let a = 5"));
        assert_eq!(session.statements.len(), 1);
        assert!(session.eval_line(":reset"));
        assert_eq!(session.statements.len(), 0);
        assert!(!session.eval_line(":quit"));
    }

    #[test]
    fn test_repl_declaration_persistence() {
        let mut session = ReplSession::new();
        assert!(session.eval_line("fn add(a: Int, b: Int) -> Int { a + b }"));
        assert_eq!(session.declarations.len(), 1);
        assert!(session.eval_line("add(3, 4)"));
    }
}
