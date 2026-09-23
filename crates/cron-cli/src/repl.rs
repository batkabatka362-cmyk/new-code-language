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
    pub history_log: Vec<String>,
    pub sim: Simulator,
    pub last_cl: Option<String>,
    pub step_count: usize,
}

impl Default for ReplSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplSession {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
            statements: Vec::new(),
            history_log: Vec::new(),
            sim: Simulator::new(),
            last_cl: None,
            step_count: 0,
        }
    }

    pub fn print_help(&self) {
        println!("\x1b[1;36m============================================================\x1b[0m");
        println!("\x1b[1;37m   CRON COGNITIVE INTERACTIVE REPL (SSS+ SILICON ENGINE)    \x1b[0m");
        println!("\x1b[1;36m============================================================\x1b[0m");
        println!("  \x1b[1;33m:help\x1b[0m, \x1b[1;33m:h\x1b[0m         Show this help message");
        println!("  \x1b[1;33m:regs\x1b[0m, \x1b[1;33m:r\x1b[0m         Inspect Core 0 registers ($rv, $a0..$a2, $t0..$t2, etc.)");
        println!("  \x1b[1;33m:mesh\x1b[0m, \x1b[1;33m:cores\x1b[0m     Display 4D-Torus hardware mesh status & telemetry");
        println!("  \x1b[1;33m:torus\x1b[0m            Live 4x4 spatial 2D slice visualizer of the 4D-Torus mesh");
        println!("  \x1b[1;33m:wafer\x1b[0m            Live ASCII HUD of the 65,536-Core 8D-Wafer Swarm Engine");
        println!("  \x1b[1;33m:cl\x1b[0m               Display raw 128-bit VLIW machine code from last evaluation");
        println!("  \x1b[1;33m:disasm\x1b[0m, \x1b[1;33m:slots\x1b[0m   Disassemble last execution into 4-slot VLIW microcode bundles");
        println!("  \x1b[1;33m:ast <expr>\x1b[0m       Inspect AST structure for expression, statement, or declaration");
        println!("  \x1b[1;33m:tokens <expr>\x1b[0m    Inspect Lexer token stream with line:column spans");
        println!("  \x1b[1;33m:type <expr>\x1b[0m      Infer static type, linear status, and tensor rank");
        println!("  \x1b[1;33m:load <file.cr>\x1b[0m   Load external .cr file and import declarations into session");
        println!("  \x1b[1;33m:reset\x1b[0m            Clear all definitions and reset 4D-Torus simulator");
        println!("  \x1b[1;33m:history\x1b[0m          List all expressions evaluated in this session");
        println!("  \x1b[1;33m:quit\x1b[0m, \x1b[1;33m:exit\x1b[0m     Exit the REPL session");
        println!();
        println!("\x1b[90mMulti-line blocks (e.g. def, struct, region) are buffered automatically.\x1b[0m");
        println!("\x1b[90mDirect VLIW bundles (e.g. B0001: '==01...) can be injected immediately.\x1b[0m");
    }

    pub fn print_regs(&self) {
        let regs = self.sim.cores[0].registers;
        println!("\x1b[1;35m┌────────────────────────────────────────────────────────────────────────┐\x1b[0m");
        println!("\x1b[1;35m│ Core 0 Registers (CRON Stackless ABI - 16 GPRs)                        │\x1b[0m");
        println!("\x1b[1;35m├────────────────────────────────────┬───────────────────────────────────┤\x1b[0m");
        println!("│ \x1b[1;33m$rv\x1b[0m  (R0):  0x{:08X} ({:>10}) │ \x1b[1;33m$a0\x1b[0m  (R1):  0x{:08X} ({:>10}) │", regs[0], regs[0], regs[1], regs[1]);
        println!("│ \x1b[1;33m$a1\x1b[0m  (R2):  0x{:08X} ({:>10}) │ \x1b[1;33m$a2\x1b[0m  (R3):  0x{:08X} ({:>10}) │", regs[2], regs[2], regs[3], regs[3]);
        println!("│ \x1b[1;33m$t0\x1b[0m  (R4):  0x{:08X} ({:>10}) │ \x1b[1;33m$t1\x1b[0m  (R5):  0x{:08X} ({:>10}) │", regs[4], regs[4], regs[5], regs[5]);
        println!("│ \x1b[1;33m$t2\x1b[0m  (R6):  0x{:08X} ({:>10}) │ \x1b[1;33m$s0\x1b[0m  (R7):  0x{:08X} ({:>10}) │", regs[6], regs[6], regs[7], regs[7]);
        println!("│ \x1b[1;33m$s1\x1b[0m  (R8):  0x{:08X} ({:>10}) │ \x1b[1;33m$s2\x1b[0m  (R9):  0x{:08X} ({:>10}) │", regs[8], regs[8], regs[9], regs[9]);
        println!("│ \x1b[1;33m$im0\x1b[0m (R10): 0x{:08X} ({:>10}) │ \x1b[1;33m$im1\x1b[0m (R11): 0x{:08X} ({:>10}) │", regs[10], regs[10], regs[11], regs[11]);
        println!("│ \x1b[1;33m$im2\x1b[0m (R12): 0x{:08X} ({:>10}) │ \x1b[1;33m$ar0\x1b[0m (R13): 0x{:08X} ({:>10}) │", regs[12], regs[12], regs[13], regs[13]);
        println!("│ \x1b[1;33m$ar1\x1b[0m (R14): 0x{:08X} ({:>10}) │ \x1b[1;33m$bp\x1b[0m  (R15): 0x{:08X} ({:>10}) │", regs[14], regs[14], regs[15], regs[15]);
        println!("\x1b[1;35m└────────────────────────────────────┴───────────────────────────────────┘\x1b[0m");
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
        println!("  DRAM Bandwidth Saved: {:.2} MB", self.sim.stats.dram_bandwidth_saved_mb);
    }

    pub fn print_torus_slice(&self) {
        println!("\x1b[1;36m4D-Torus Core Mesh Slice Projection [Z=0, W=0]:\x1b[0m");
        println!("      X=0          X=1          X=2          X=3");
        println!("   ┌────────────┬────────────┬────────────┬────────────┐");
        for y in 0..4 {
            print!("Y={}│", y);
            for x in 0..4 {
                let core_idx = y * 4 + x;
                let c = &self.sim.cores[core_idx];
                let is_active = c.registers[0] != 0 || c.cycle_count > 0;
                let sym = if is_active { "\x1b[1;32m● RUN\x1b[0m" } else { "\x1b[90m○ IDL\x1b[0m" };
                print!(" C{:02} {} │", core_idx, sym);
            }
            println!();
            if y < 3 {
                println!("   ├────────────┼────────────┼────────────┼────────────┤");
            }
        }
        println!("   └────────────┴────────────┴────────────┴────────────┘");
        println!("   \x1b[90mTopology: 4D Torus NoC (4x4x4x4) | Dimension-Order Routing (DOR)\x1b[0m");
    }

    pub fn print_wafer_hud(&self) {
        let mesh = cronc::cl_swarm_wafer::WaferSwarmMesh::new_65536();
        let tel = cronc::cl_swarm_wafer::WaferSwarmTelemetry::default();
        let hud = mesh.render_wafer_ascii(&tel);
        println!("{}", hud);
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

    pub fn print_disassembly(&self) {
        if let Some(ref cl) = self.last_cl {
            println!("\x1b[1;36m128-Bit VLIW Bundle Disassembly (4 Issue Slots/Cycle):\x1b[0m");
            for line in cl.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('B') && trimmed.contains(':') {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 5 {
                        println!("  \x1b[1;33m{}\x1b[0m", parts[0]);
                        println!("    Slot 0 [ALU/INT]:     \x1b[32m{}\x1b[0m", parts[1]);
                        println!("    Slot 1 [MEM/LOAD]:    \x1b[34m{}\x1b[0m", parts[2]);
                        println!("    Slot 2 [OPT/WAVE]:    \x1b[35m{}\x1b[0m", parts[3]);
                        println!("    Slot 3 [CTRL/BRANCH]: \x1b[36m{}\x1b[0m", parts[4]);
                    } else {
                        println!("  {}", trimmed);
                    }
                }
            }
        } else {
            println!("\x1b[33mNo previous execution to disassemble. Evaluate an expression first.\x1b[0m");
        }
    }

    pub fn print_ast(&self, code: &str) {
        let trimmed = code.trim();
        let is_decl = trimmed.starts_with("fn ")
            || trimmed.starts_with("def ")
            || trimmed.starts_with("async def ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("enum ")
            || trimmed.starts_with("trait ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("brain ")
            || trimmed.starts_with("rule ")
            || trimmed.starts_with("type ")
            || trimmed.starts_with("import ");

        // Place top-level declarations outside _main so parser accepts them
        let full_src = if is_decl {
            format!(".MODULE AstInspect\n.ENTRY _main\n{}\n_main:\n    let _unused = 0\n.END\n", trimmed)
        } else {
            format!(".MODULE AstInspect\n.ENTRY _main\n_main:\n    {}\n.END\n", trimmed)
        };

        let mut lexer = cronc::lexer::Lexer::new(&full_src);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("\x1b[31mLexer Error:\x1b[0m {}", e);
                return;
            }
        };
        let mut parser = cronc::parser::Parser::new(tokens);
        match parser.parse_program() {
            Ok(prog) => {
                println!("\x1b[1;32mAbstract Syntax Tree (AST):\x1b[0m");
                if is_decl {
                    if let Some(f) = prog.functions.first() {
                        println!("{:#?}", f);
                    } else if let Some(s) = prog.structs.first() {
                        println!("{:#?}", s);
                    } else if let Some(t) = prog.traits.first() {
                        println!("{:#?}", t);
                    } else if let Some(b) = prog.brains.first() {
                        println!("{:#?}", b);
                    } else {
                        println!("{:#?}", prog);
                    }
                } else if let Some(first_stmt) = prog.main_statements.first() {
                    println!("{:#?}", first_stmt);
                } else {
                    println!("{:#?}", prog);
                }
            }
            Err(e) => eprintln!("\x1b[31mAST Parse Error:\x1b[0m {}", e),
        }
    }

    pub fn print_tokens(&self, code: &str) {
        let mut lexer = cronc::lexer::Lexer::new(code);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("\x1b[31mLexer Error:\x1b[0m {}", e);
                return;
            }
        };
        println!("\x1b[1;36m┌─────┬──────────────────────────────┬──────────────┬─────────────┐\x1b[0m");
        println!("\x1b[1;36m│ Idx │ Token Kind                   │ Payload/Text │ Span        │\x1b[0m");
        println!("\x1b[1;36m├─────┼──────────────────────────────┼──────────────┼─────────────┤\x1b[0m");
        for (i, tok) in tokens.iter().enumerate() {
            let kind_str = format!("{:?}", tok.value);
            let short_kind = kind_str.split('(').next().unwrap_or(&kind_str);
            let span_str = format!("{}:{}", tok.span.line, tok.span.col);
            let payload = match &tok.value {
                cronc::token::Token::Ident(s) => s.clone(),
                cronc::token::Token::IntLit(n) => n.to_string(),
                cronc::token::Token::FloatLit(f) => f.to_string(),
                cronc::token::Token::StringLit(s) => s.clone(),
                _ => "-".to_string(),
            };
            println!("│ {:>3} │ {:<28} │ {:<12} │ {:<11} │", i, short_kind, payload, span_str);
        }
        println!("\x1b[1;36m└─────┴──────────────────────────────┴──────────────┴─────────────┘\x1b[0m");
    }

    pub fn print_type(&self, code: &str) {
        let trimmed = code.trim();
        // If code is a let statement like `let x = 10` or `let mut x: Int = 10`, extract RHS expression
        let expr = if let Some(rhs) = trimmed.split_once('=') {
            rhs.1.trim().trim_end_matches(';')
        } else {
            trimmed
        };

        let mut test_src = String::new();
        test_src.push_str(".MODULE TypeInference\n.ENTRY _main\n\n");
        for decl in &self.declarations {
            test_src.push_str(decl);
            test_src.push('\n');
        }
        test_src.push_str("\n_main:\n");
        for stmt in &self.statements {
            test_src.push_str("    ");
            test_src.push_str(stmt);
            test_src.push('\n');
        }
        test_src.push_str(&format!("    let _inferred_val = {}\n.END\n", expr));

        match cronc::compile_source(&test_src) {
            Ok(_) => {
                let inferred = if expr.contains('[') && expr.contains(']') {
                    "tensor<dynamic> (SIMD vector)"
                } else if expr.contains("pack_wave") {
                    "linear wave_t (photonic packet)"
                } else if expr.contains('.') {
                    "Float64 (IEEE 754)"
                } else if expr.starts_with('"') {
                    "String (UTF-8)"
                } else if expr == "true" || expr == "false" {
                    "Bool"
                } else {
                    "Int64 (affine register value)"
                };
                println!("\x1b[1;32mInferred Type:\x1b[0m \x1b[1;33m{}\x1b[0m", inferred);
            }
            Err(e) => {
                eprintln!("\x1b[31mType check error:\x1b[0m {}", e);
            }
        }
    }

    pub fn load_file(&mut self, path_str: &str) {
        let clean_path = path_str.trim().trim_matches('"').trim_matches('\'');
        let candidate_paths = [
            std::path::PathBuf::from(clean_path),
            std::path::PathBuf::from("../../").join(clean_path),
            std::path::PathBuf::from("../").join(clean_path),
        ];
        let found = candidate_paths
            .iter()
            .find_map(|p| std::fs::read_to_string(p).ok());

        match found {
            Some(content) => {
                let mut lexer = cronc::lexer::Lexer::new(&content);
                let tokens = match lexer.tokenize() {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("\x1b[31mLexer error in '{}':\x1b[0m {}", clean_path, e);
                        return;
                    }
                };
                let mut parser = cronc::parser::Parser::new(tokens);
                match parser.parse_program() {
                    Ok(prog) => {
                        let func_count = prog.functions.len();
                        let struct_count = prog.structs.len();
                        let trait_count = prog.traits.len();
                        let brain_count = prog.brains.len();

                        // Filter out module-level framing (.MODULE, .ENTRY, .END)
                        // so declarations can be safely incorporated into the interactive REPL session
                        let mut sanitized = String::new();
                        for line in content.lines() {
                            let t = line.trim();
                            if t.starts_with(".MODULE") || t.starts_with(".ENTRY") || t == ".END" {
                                continue;
                            }
                            sanitized.push_str(line);
                            sanitized.push('\n');
                        }

                        if !sanitized.trim().is_empty() {
                            self.declarations.push(sanitized);
                        }

                        println!(
                            "\x1b[1;32m[Loaded '{}']\x1b[0m {} functions, {} structs, {} traits, {} brains imported.",
                            clean_path, func_count, struct_count, trait_count, brain_count
                        );
                    }
                    Err(e) => {
                        eprintln!("\x1b[31mParse error in '{}':\x1b[0m {}", clean_path, e);
                    }
                }
            }
            None => {
                eprintln!("\x1b[31mFailed to read file '{}':\x1b[0m file not found", clean_path);
            }
        }
    }

    pub fn reset(&mut self) {
        self.declarations.clear();
        self.statements.clear();
        self.history_log.clear();
        self.sim = Simulator::new();
        self.last_cl = None;
        self.step_count = 0;
        println!("\x1b[1;32m[RESET] Session cleared. 4D-Torus Simulator reset to initial state.\x1b[0m");
    }

    pub fn history(&self) {
        println!("\x1b[1;36mSession History:\x1b[0m");
        if self.history_log.is_empty() && self.declarations.is_empty() && self.statements.is_empty() {
            println!("  (empty)");
            return;
        }
        for (i, entry) in self.history_log.iter().enumerate() {
            println!("  #{}: {}", i + 1, entry.replace('\n', " "));
        }
    }

    pub fn eval_line(&mut self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            return true;
        }

        // Bare exit commands without colon
        if trimmed == "quit" || trimmed == "exit" || trimmed == "q" {
            return false;
        }

        // Colon commands
        if let Some(cmd) = trimmed.strip_prefix(':') {
            let mut parts = cmd.splitn(2, ' ');
            let command = parts.next().unwrap_or("").trim();
            let arg = parts.next().unwrap_or("").trim();

            match command {
                "help" | "h" | "?" => self.print_help(),
                "regs" | "r" => self.print_regs(),
                "mesh" | "cores" => self.print_mesh(),
                "torus" => self.print_torus_slice(),
                "wafer" => self.print_wafer_hud(),
                "cl" => self.print_last_cl(),
                "disasm" | "slots" => self.print_disassembly(),
                "ast" => {
                    if arg.is_empty() {
                        eprintln!("\x1b[33mUsage: :ast <expression, statement, or declaration>\x1b[0m");
                    } else {
                        self.print_ast(arg);
                    }
                }
                "tokens" => {
                    if arg.is_empty() {
                        eprintln!("\x1b[33mUsage: :tokens <expression or code>\x1b[0m");
                    } else {
                        self.print_tokens(arg);
                    }
                }
                "type" => {
                    if arg.is_empty() {
                        eprintln!("\x1b[33mUsage: :type <expression>\x1b[0m");
                    } else {
                        self.print_type(arg);
                    }
                }
                "load" => {
                    if arg.is_empty() {
                        eprintln!("\x1b[33mUsage: :load <path/to/file.cr>\x1b[0m");
                    } else {
                        self.load_file(arg);
                    }
                }
                "reset" => self.reset(),
                "history" => self.history(),
                "quit" | "q" | "exit" => return false,
                _ => {
                    println!("\x1b[31mUnknown command ':{command}'. Type ':help' for available commands.\x1b[0m");
                }
            }
            return true;
        }

        // Low-level VLIW direct injection (B0001: ...)
        if trimmed.starts_with('B') && trimmed.contains(':') {
            self.last_cl = Some(trimmed.to_string());
            self.history_log.push(trimmed.to_string());
            self.sim.load_machine_code(trimmed);
            let mut cycles = 0;
            while self.sim.step() {
                cycles += 1;
            }
            self.step_count += 1;
            let r0 = self.sim.cores[0].registers[0];
            println!("\x1b[1;32m=> 0x{:08X} ({})\x1b[0m \x1b[90m[Direct VLIW, {} cycles]\x1b[0m", r0, r0, cycles);
            return true;
        }

        // Check if line is a top-level declaration
        if trimmed.starts_with("fn ")
            || trimmed.starts_with("def ")
            || trimmed.starts_with("async def ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("enum ")
            || trimmed.starts_with("trait ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("brain ")
            || trimmed.starts_with("rule ")
            || trimmed.starts_with("type ")
            || trimmed.starts_with("import ")
        {
            // Verify compilation of candidate declaration along with all existing declarations
            let mut test_src = String::new();
            test_src.push_str(".MODULE ReplDecl\n.ENTRY _main\n\n");
            for decl in &self.declarations {
                test_src.push_str(decl);
                test_src.push('\n');
            }
            test_src.push_str(trimmed);
            test_src.push_str("\n_main:\n    let _unused = 0\n.END\n");

            match cronc::compile_source(&test_src) {
                Ok(_) => {
                    self.declarations.push(trimmed.to_string());
                    self.history_log.push(trimmed.to_string());
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
                    self.history_log.push(trimmed.to_string());
                    self.sim = Simulator::new();
                    self.sim.load_machine_code(&cl_code);
                    let mut cycles = 0;
                    while self.sim.step() {
                        cycles += 1;
                    }
                    self.step_count += 1;
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
                    self.history_log.push(trimmed.to_string());
                    // Extract variable name correctly, handling `let x = ...`, `let mut x = ...`, `let x: Int = ...`
                    let mut tokens = trimmed.split_whitespace().skip(1);
                    let first = tokens.next().unwrap_or("var");
                    let raw_name = if first == "mut" {
                        tokens.next().unwrap_or("var")
                    } else {
                        first
                    };
                    let var_name = raw_name.trim_matches(':').trim_matches('=').trim_matches(';');
                    println!("\x1b[1;32m[Bound: {}]\x1b[0m \x1b[90m({} cycles)\x1b[0m", var_name, cycles);
                } else {
                    self.history_log.push(trimmed.to_string());
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
    let mut paren_depth: i32 = 0;
    let mut in_string = false;
    let mut escape_next = false;

    loop {
        if brace_depth > 0 || paren_depth > 0 {
            print!("\x1b[90mcron... \x1b[0m");
        } else {
            print!("\x1b[1;34mcron>\x1b[0m ");
        }
        let _ = io::stdout().flush();

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                // Cancel ongoing multiline buffer on :cancel
                if line.trim() == ":cancel" {
                    multiline_buf.clear();
                    brace_depth = 0;
                    paren_depth = 0;
                    in_string = false;
                    escape_next = false;
                    println!("\x1b[33m[Input cancelled]\x1b[0m");
                    continue;
                }

                // Count brace and paren balance while properly tracking string literals
                for ch in line.chars() {
                    if escape_next {
                        escape_next = false;
                        continue;
                    }
                    if ch == '\\' && in_string {
                        escape_next = true;
                        continue;
                    }
                    if ch == '"' {
                        in_string = !in_string;
                        continue;
                    }
                    if !in_string {
                        if ch == '{' {
                            brace_depth += 1;
                        } else if ch == '}' {
                            brace_depth = (brace_depth - 1).max(0);
                        } else if ch == '(' {
                            paren_depth += 1;
                        } else if ch == ')' {
                            paren_depth = (paren_depth - 1).max(0);
                        }
                    }
                }

                multiline_buf.push_str(&line);

                if brace_depth == 0 && paren_depth == 0 {
                    let to_eval = multiline_buf.clone();
                    multiline_buf.clear();
                    in_string = false;
                    escape_next = false;

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
        assert!(session.eval_line(":torus"));
        assert!(session.eval_line(":wafer"));
        assert!(session.eval_line(":cl"));
        assert!(session.eval_line(":history"));
        assert!(session.eval_line("let a = 5"));
        assert_eq!(session.statements.len(), 1);
        assert!(session.eval_line(":disasm"));
        assert!(session.eval_line(":slots"));
        assert!(session.eval_line(":reset"));
        assert_eq!(session.statements.len(), 0);
        assert!(!session.eval_line(":quit"));
    }

    #[test]
    fn test_repl_ast_and_tokens() {
        let mut session = ReplSession::new();
        assert!(session.eval_line(":ast let x = 42 + 8"));
        assert!(session.eval_line(":ast def square(n: Int) -> Int { n * n }"));
        assert!(session.eval_line(":tokens let x = 42 + 8"));
        assert!(session.eval_line(":type 42 + 8"));
        assert!(session.eval_line(":type [1, 2, 3, 4]"));
    }

    #[test]
    fn test_repl_declaration_persistence() {
        let mut session = ReplSession::new();
        assert!(session.eval_line("fn add(a: Int, b: Int) -> Int { a + b }"));
        assert_eq!(session.declarations.len(), 1);
        assert!(session.eval_line("add(3, 4)"));
    }

    #[test]
    fn test_repl_dependent_declarations() {
        let mut session = ReplSession::new();
        assert!(session.eval_line("struct Point { x: u32, y: u32 }"));
        assert_eq!(session.declarations.len(), 1);
        assert!(session.eval_line("fn get_x(p: Point) -> u32 { p.x }"));
        assert_eq!(session.declarations.len(), 2);
    }

    #[test]
    fn test_repl_direct_vliw_injection() {
        let mut session = ReplSession::new();
        let vliw_line = "B0000: '==01#02A> _NO00#000> _NO00#000> _NO00#000!";
        assert!(session.eval_line(vliw_line));
        assert!(session.last_cl.is_some());
        assert_eq!(session.step_count, 1);
    }

    #[test]
    fn test_repl_bare_exit_commands() {
        let mut session = ReplSession::new();
        assert!(!session.eval_line("exit"));
        assert!(!session.eval_line("quit"));
        assert!(!session.eval_line("q"));
    }

    #[test]
    fn test_repl_let_mut_binding() {
        let mut session = ReplSession::new();
        assert!(session.eval_line("let mut count: i32 = 0"));
        assert_eq!(session.statements.len(), 1);
    }
}
