// ============================================================================
// CRON Codegen — Generates Machine-Native .cl VLIW Assembly
// Converts verified .cr AST into 128-bit VLIW Bundles (4 slots per cycle).
// Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
// ============================================================================

use crate::ast::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct VliwSlot {
    pub raw: String,
}

impl VliwSlot {
    pub fn new(raw: &str) -> Self {
        let mut s = raw.to_string();
        if s.len() < 10 {
            let term = s.pop().unwrap_or('>');
            while s.len() < 9 {
                s.push('0');
            }
            s.push(term);
        } else if s.len() > 10 {
            let term = s.chars().last().unwrap_or('>');
            s.truncate(9);
            s.push(term);
        }
        Self { raw: s }
    }

    pub fn nop() -> Self {
        Self::new("_NO00#000>")
    }
}

#[derive(Clone)]
pub struct VliwBundle {
    pub cycle: usize,
    pub slots: [VliwSlot; 4],
}

impl VliwBundle {
    pub fn format(&self) -> String {
        format!(
            "B{:04}: {} {} {} {}",
            self.cycle, self.slots[0].raw, self.slots[1].raw, self.slots[2].raw, self.slots[3].raw
        )
    }
}

pub fn compute_parity(
    prefix: char,
    op: &str,
    dest_hex: &str,
    mode: char,
    src: char,
    imm: char,
) -> char {
    let mut sum: u32 = prefix as u32 + mode as u32 + src as u32 + imm as u32;
    for b in op.bytes() {
        sum ^= b as u32;
    }
    for b in dest_hex.bytes() {
        sum = sum.wrapping_add(b as u32);
    }
    let hex_chars = b"0123456789ABCDEF";
    hex_chars[(sum as usize) % 16] as char
}

pub fn make_slot(
    prefix: char,
    op: &str,
    dest_reg: usize,
    mode: char,
    src_reg: usize,
    imm: usize,
    terminator: char,
) -> VliwSlot {
    let dest_hex = format!("{:02X}", dest_reg.min(15));
    let src_char = format!("{:1X}", src_reg.min(15)).chars().next().unwrap();
    let imm_char = format!("{:1X}", imm.min(15)).chars().next().unwrap();
    let parity = compute_parity(prefix, op, &dest_hex, mode, src_char, imm_char);
    let raw = format!(
        "{}{}{}{}{}{}{}{}",
        prefix, op, dest_hex, mode, src_char, parity, imm_char, terminator
    );
    VliwSlot::new(&raw)
}

pub struct Codegen {
    bundles: Vec<VliwBundle>,
    current_slots: Vec<VliwSlot>,
    pub current_cycle: usize,
    reg_map: HashMap<String, usize>,
    next_reg: usize,
    written_regs_in_cycle: HashSet<usize>,
    optical_in_cycle: bool,
    inline_functions: HashMap<String, FunctionDecl>,
    /// Monomorphized impl methods: (struct_name, method_name) -> FunctionDecl
    impl_methods: HashMap<(String, String), FunctionDecl>,
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            bundles: Vec::new(),
            current_slots: Vec::new(),
            current_cycle: 1,
            reg_map: HashMap::new(),
            next_reg: 1, // R1..R14 for general variables (R0 reserved for immediate/sys)
            written_regs_in_cycle: HashSet::new(),
            optical_in_cycle: false,
            inline_functions: HashMap::new(),
            impl_methods: HashMap::new(),
        }
    }

    pub fn alloc_reg(&mut self, name: &str) -> usize {
        let r = (self.next_reg % 14) + 1;
        self.next_reg += 1;
        self.reg_map.insert(name.to_string(), r);
        r
    }

    pub fn get_reg(&self, name: &str) -> usize {
        *self.reg_map.get(name).unwrap_or(&0)
    }

    pub fn push_slot(&mut self, slot: VliwSlot) {
        let is_writer = if slot.raw.len() >= 3 {
            let op = &slot.raw[1..3];
            op != "SB" && op != "SH" && op != "RS" && op != "HL" && op != "DW" && op != "YD" && op != "NO"
        } else {
            false
        };

        let dest = if is_writer && slot.raw.len() >= 5 {
            usize::from_str_radix(&slot.raw[3..5], 16).unwrap_or(0)
        } else {
            0
        };

        let src = if slot.raw.len() >= 7 {
            slot.raw[6..7]
                .chars()
                .next()
                .and_then(|c| c.to_digit(16))
                .unwrap_or(0) as usize
        } else {
            0
        };

        let is_optical = if slot.raw.len() >= 3 {
            let op = &slot.raw[1..3];
            op == "OP" || op == "WD"
        } else {
            false
        };

        // Hazard-Free VLIW Dependency Scheduler
        // 1. RAW hazard: slot reads a register that was written in the current cycle
        let raw_hazard = src > 0 && self.written_regs_in_cycle.contains(&src);

        // 2. WAW hazard: slot writes to a register that was already written in the current cycle
        let waw_hazard = is_writer && dest > 0 && self.written_regs_in_cycle.contains(&dest);

        // 3. Optical structural hazard: only 1 MZI mesh operation per cycle
        let optical_hazard = is_optical && self.optical_in_cycle;

        if raw_hazard || waw_hazard || optical_hazard {
            // Early bundle flush to resolve hazard and advance to next hardware cycle
            self.flush();
        }

        if is_writer && dest > 0 {
            self.written_regs_in_cycle.insert(dest);
        }
        if is_optical {
            self.optical_in_cycle = true;
        }

        self.current_slots.push(slot);
        if self.current_slots.len() == 4 {
            let bundle = VliwBundle {
                cycle: self.current_cycle,
                slots: [
                    self.current_slots.remove(0),
                    self.current_slots.remove(0),
                    self.current_slots.remove(0),
                    self.current_slots.remove(0),
                ],
            };
            self.bundles.push(bundle);
            self.current_cycle += 1;
            self.written_regs_in_cycle.clear();
            self.optical_in_cycle = false;
        }
    }

    pub fn flush(&mut self) {
        if !self.current_slots.is_empty() {
            while self.current_slots.len() < 4 {
                self.current_slots.push(VliwSlot::nop());
            }
            let bundle = VliwBundle {
                cycle: self.current_cycle,
                slots: [
                    self.current_slots.remove(0),
                    self.current_slots.remove(0),
                    self.current_slots.remove(0),
                    self.current_slots.remove(0),
                ],
            };
            self.bundles.push(bundle);
            self.current_cycle += 1;
            self.written_regs_in_cycle.clear();
            self.optical_in_cycle = false;
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        // Cycle 1 Preamble: Core & 4D Torus Hardware Initialization
        self.push_slot(make_slot('_', "TL", 3, '$', 0, 4, '>')); // 4D Coordinate setup
        self.push_slot(make_slot('_', "SH", 0, '$', 1, 4, '>')); // Brain 6 Sentry watchdog
        self.push_slot(make_slot('_', "SP", 0, '#', 0, 8, '>')); // Fiber 1 background worker
        self.push_slot(make_slot('_', "ST", 0, '$', 1, 0, '>')); // Staging buffer prefetch

        // Register monomorphized impl methods (Milestone #005: zero-vtable dispatch)
        for impl_decl in &program.impls {
            for method in &impl_decl.methods {
                let key = (impl_decl.target_struct.clone(), method.name.clone());
                self.impl_methods.insert(key, method.clone());
                // Also register as inline for direct expansion
                self.inline_functions.insert(
                    format!("{}::{}", impl_decl.target_struct, method.name),
                    method.clone(),
                );
            }
        }

        // First compile function declarations if any
        for func in &program.functions {
            if func.is_inline {
                self.inline_functions.insert(func.name.clone(), func.clone());
            }
            for param in &func.params {
                self.alloc_reg(&param.name);
            }
            self.compile_statements(&func.body);
        }

        // Compile cognitive brain declarations if any
        for brain in &program.brains {
            self.push_slot(make_slot('_', "SH", 0, '$', 1, 4, '>'));
            self.compile_statements(&brain.body);
        }

        // Then compile main entry statements
        self.compile_statements(&program.main_statements);

        // Finalize pipeline: Fiber Join, DMA Sync, Arena Reset & Halt
        self.push_slot(make_slot('_', "FJ", 4, '#', 0, 0, '>'));
        self.push_slot(make_slot('_', "DW", 0, '$', 1, 5, '>'));
        self.push_slot(make_slot('_', "RS", 0, '#', 0, 0xB, '>'));
        self.push_slot(make_slot('_', "HL", 0, '#', 0, 0, '!'));
        self.flush();

        let mut output = String::new();
        for bundle in &self.bundles {
            output.push_str(&bundle.format());
            output.push('\n');
        }

        output
    }

    pub fn compile_statements(&mut self, stmts: &[Statement]) {
        for stmt in stmts {
            self.compile_statement(stmt);
        }
    }

    pub fn compile_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let {
                name,
                value,
                extra_vars,
                ..
            } => {
                let dest = self.alloc_reg(name);
                self.compile_expr(value, dest);

                for (_e_lin, _e_grad, e_name, _e_type) in extra_vars {
                    let e_dest = self.alloc_reg(e_name);
                    self.push_slot(make_slot('_', "FA", e_dest, '$', dest, 0, '>'));
                }
            }
            Statement::Assign { target, value, .. } => {
                let dest = if let Some(&r) = self.reg_map.get(target) {
                    r
                } else {
                    self.alloc_reg(target)
                };
                self.compile_expr(value, dest);
            }
            Statement::Region { body, .. } => {
                self.compile_statements(body);
                // 0-cycle Region Arena Reset
                self.push_slot(make_slot('_', "RS", 0, '#', 0, 0xB, '>'));
            }
            Statement::Resilient { body, fallback, .. } => {
                // Brain 6 Sentry setup
                self.push_slot(make_slot('_', "SH", 0, '$', 1, 4, '>'));
                self.compile_statements(body);
                if let Some(fb) = fallback {
                    self.push_slot(make_slot('_', "RC", 7, '$', 2, 5, '>'));
                    self.compile_statements(fb);
                }
            }
            Statement::Brain { body, .. } => {
                // Cognitive Brain block: Sentry watchdog & bounds
                self.push_slot(make_slot('_', "SH", 0, '$', 1, 4, '>'));
                self.compile_statements(body);
            }
            Statement::Fork { target, .. } => {
                let target_reg = 13;
                self.compile_expr(target, target_reg);
                // Emit Superposition branch / fiber spawn slot
                self.push_slot(make_slot('_', "SW", target_reg, '$', target_reg, 5, '>'));
            }
            Statement::Simulate { action, with_arg, .. } => {
                let act_reg = 12;
                self.compile_expr(action, act_reg);
                if let Some(arg) = with_arg {
                    let arg_reg = 11;
                    self.compile_expr(arg, arg_reg);
                    self.push_slot(make_slot('_', "PS", act_reg, '$', arg_reg, 1, '>'));
                } else {
                    self.push_slot(make_slot('_', "PS", act_reg, '$', act_reg, 0, '>'));
                }
            }
            Statement::Abort(_) => {
                // Halt / cognitive trap
                self.push_slot(make_slot('_', "HL", 0, '#', 0, 0, '!'));
            }
            Statement::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let cond_reg = 14;
                self.compile_expr(condition, cond_reg);
                // Predicated branch evaluation: Superposition Wave condition slot
                self.push_slot(make_slot('_', "SW", cond_reg, '$', cond_reg, 5, '>'));
                self.compile_statements(then_body);
                if let Some(fb) = else_body {
                    self.push_slot(make_slot('_', "RC", 7, '$', cond_reg, 5, '>'));
                    self.compile_statements(fb);
                }
            }
            Statement::While { condition, body, .. } => {
                let cond_reg = 14;
                self.compile_expr(condition, cond_reg);
                self.push_slot(make_slot('_', "SW", cond_reg, '$', cond_reg, 5, '>'));
                self.compile_statements(body);
                self.compile_expr(condition, cond_reg);
                self.push_slot(make_slot('_', "YD", 0, '$', cond_reg, 0, '>'));
            }
            Statement::For {
                var_name,
                iterable,
                body,
                ..
            } => {
                let var_reg = self.alloc_reg(var_name);
                self.compile_expr(iterable, var_reg);
                self.push_slot(make_slot('_', "PK", var_reg, '$', var_reg, 2, '>'));
                self.compile_statements(body);
            }
            Statement::ProofContract(_) => {
                // Parity telemetry assertion (zero hardware cycle overhead)
                self.push_slot(make_slot('_', "PT", 0, '$', 0, 1, '>'));
            }
            Statement::Superposition { branches, .. } => {
                self.push_slot(make_slot('_', "SW", 14, '$', 14, 5, '>'));
                for b in branches {
                    self.compile_statements(&b.body);
                }
                self.push_slot(make_slot('_', "PO", 6, '$', 6, 0, '>'));
            }
            Statement::Export { .. } => {}
            Statement::Match { expr, arms, .. } => {
                self.compile_expr(expr, 1);
                for arm in arms {
                    self.compile_statements(&arm.body);
                }
            }
            Statement::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.compile_expr(expr, 0);
                }
            }
            Statement::Expr(expr) => {
                self.compile_expr(expr, 0);
            }
        }
    }

    pub fn compile_expr(&mut self, expr: &Expr, dest: usize) {
        match expr {
            Expr::LiteralHex(_h) => {
                self.push_slot(make_slot('\'', "=0", dest, '#', 0, 4, '>'));
            }
            Expr::LiteralInt(_i) => {
                self.push_slot(make_slot('\'', "=0", dest, '#', 0, 1, '>'));
            }
            Expr::LiteralFloat(_) => {
                self.push_slot(make_slot('\'', "=0", dest, '#', 0, 2, '>'));
            }
            Expr::LiteralString(s) => {
                // Ground string literal to symbol token ID
                let hash = s.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
                let imm = (hash % 16) as usize;
                self.push_slot(make_slot('_', "SY", dest, '#', 0, imm, '>'));
            }
            Expr::LiteralBool(b) => {
                let imm = if *b { 1 } else { 0 };
                self.push_slot(make_slot('\'', "=0", dest, '#', 0, imm, '>'));
            }
            Expr::LiteralAxis(_) => {
                self.push_slot(make_slot('_', "TL", dest, '$', 0, 4, '>'));
            }
            Expr::Ident(name, _) => {
                let src = self.get_reg(name);
                if src != dest {
                    self.push_slot(make_slot('_', "PO", dest, '$', src, 0, '>'));
                }
            }
            Expr::Consume(name, _) => {
                let src = self.get_reg(name);
                if src != dest {
                    self.push_slot(make_slot('_', "PO", dest, '$', src, 0, '>'));
                }
            }
            Expr::Unary { op, operand } => {
                self.compile_expr(operand, dest);
                match op.as_str() {
                    "!" | "not" => {
                        // Invert predicate / bitwise not
                        self.push_slot(make_slot('_', "PO", dest, '$', dest, 0xC, '>'));
                    }
                    "-" => {
                        // 2's complement negation via SIMD ALU
                        self.push_slot(make_slot('_', "PO", dest, '$', dest, 2, '>'));
                    }
                    "~" => {
                        // Bitwise invert
                        self.push_slot(make_slot('_', "PO", dest, '$', dest, 8, '>'));
                    }
                    _ => {
                        self.push_slot(make_slot('_', "PO", dest, '$', dest, 0, '>'));
                    }
                }
            }
            Expr::Binary { op, left, right } => {
                self.compile_expr(left, dest);
                let r_reg = (dest % 14) + 1;
                self.compile_expr(right, r_reg);
                let imm_mode = match op.as_str() {
                    "+" => 1,
                    "-" => 2,
                    "*" => 3,
                    "/" => 4,
                    "%" => 5,
                    "&" | "and" => 6,
                    "|" | "or" => 7,
                    "^" => 8,
                    "<<" => 9,
                    ">>" => 0xA,
                    "==" => 0xB,
                    "!=" => 0xC,
                    "<" => 0xD,
                    "<=" => 0xE,
                    ">" => 0xF,
                    ">=" => 0,
                    ".." => 0,
                    _ => 2,
                };
                if op == "*" || op == "/" {
                    self.push_slot(make_slot('_', "MD", dest, '$', r_reg, imm_mode, '>'));
                } else if op == ">=" {
                    self.push_slot(make_slot('_', "PO", dest, 'G', r_reg, 0, '>'));
                } else {
                    self.push_slot(make_slot('_', "PO", dest, '$', r_reg, imm_mode, '>'));
                }
            }
            Expr::FieldAccess { object, field } => {
                self.compile_expr(object, dest);
                let offset = (field.bytes().fold(0usize, |acc, b| acc.wrapping_add(b as usize)) % 16).max(1);
                self.push_slot(make_slot('_', "TL", dest, '$', dest, offset, '>'));
            }
            Expr::Index { object, index } => {
                self.compile_expr(object, dest);
                let idx_reg = (dest % 14) + 1;
                self.compile_expr(index, idx_reg);
                self.push_slot(make_slot('_', "MD", dest, '$', idx_reg, 0, '>'));
            }
            Expr::Spawn(inner) => {
                self.push_slot(make_slot('_', "SP", 0, '#', 0, 8, '>'));
                self.compile_expr(inner, dest);
            }
            Expr::SpawnAt { core_id, target, .. } => {
                let core_reg = (dest % 14) + 1;
                self.compile_expr(core_id, core_reg);
                self.push_slot(make_slot('_', "SP", core_reg, '$', core_reg, 8, '>'));
                self.compile_expr(target, dest);
            }
            Expr::ChannelSend { channel, value, .. } => {
                self.compile_expr(channel, dest);
                let val_reg = (dest % 14) + 1;
                self.compile_expr(value, val_reg);
                self.push_slot(make_slot('_', "TX", dest, '$', val_reg, 0, '>'));
            }
            Expr::ChannelRecv { channel, .. } => {
                self.compile_expr(channel, dest);
                self.push_slot(make_slot('_', "RX", dest, '$', dest, 0, '>'));
            }
            Expr::Await(inner) => {
                self.compile_expr(inner, dest);
                self.push_slot(make_slot('_', "FJ", dest, '#', 0, 0, '>'));
            }
            Expr::Cast { expr, .. } => {
                self.compile_expr(expr, dest);
            }
            Expr::Array(elements) => {
                for el in elements {
                    self.compile_expr(el, dest);
                    self.push_slot(make_slot('_', "PK", dest, '$', dest, 2, '>'));
                }
            }
            Expr::Tuple(elements) => {
                for el in elements {
                    self.compile_expr(el, dest);
                }
            }
            Expr::StructInit { fields, .. } => {
                for (idx, (_, val)) in fields.iter().enumerate() {
                    let field_reg = (dest % 14) + 1;
                    self.compile_expr(val, field_reg);
                    self.push_slot(make_slot('_', "TL", dest, '$', field_reg, (idx + 1).min(15), '>'));
                }
            }
            Expr::ArrayRepeat { value, .. } => {
                self.compile_expr(value, dest);
            }
            Expr::IfExpr { condition, then_branch, else_branch } => {
                let cond_reg = 14;
                self.compile_expr(condition, cond_reg);
                self.push_slot(make_slot('_', "SW", cond_reg, '$', cond_reg, 5, '>'));
                self.compile_expr(then_branch, dest);
                self.compile_expr(else_branch, dest);
            }
            Expr::Call { callee, args } => {
                if let Some(inline_func) = self.inline_functions.get(callee).cloned() {
                    for (param, arg) in inline_func.params.iter().zip(args.iter()) {
                        let p_reg = self.alloc_reg(&param.name);
                        self.compile_expr(&arg.value, p_reg);
                    }
                    self.compile_statements(&inline_func.body);
                    return;
                }

                let mut first_arg_reg = 0;
                for arg in args {
                    if let Expr::Consume(n, _) = &arg.value {
                        first_arg_reg = self.get_reg(n);
                    } else if let Expr::Ident(n, _) = &arg.value {
                        first_arg_reg = self.get_reg(n);
                    }
                }

                match callee.as_str() {
                    "channel_send" => {
                        let ch_reg = if !args.is_empty() {
                            self.compile_expr(&args[0].value, dest);
                            dest
                        } else { 0 };
                        let val_reg = if args.len() >= 2 {
                            let vr = (dest % 14) + 1;
                            self.compile_expr(&args[1].value, vr);
                            vr
                        } else { 0 };
                        self.push_slot(make_slot('_', "TX", ch_reg, '$', val_reg, 0, '>'));
                    }
                    "channel_recv" => {
                        let ch_reg = if !args.is_empty() {
                            self.compile_expr(&args[0].value, dest);
                            dest
                        } else { 0 };
                        self.push_slot(make_slot('_', "RX", dest, '$', ch_reg, 0, '>'));
                    }
                    "channel_new" => {
                        self.push_slot(make_slot('\'', "=0", dest, '#', 0, 1, '>'));
                    }
                    "torus_distance" | "torus_dist" => {
                        self.push_slot(make_slot('_', "TL", dest, '$', first_arg_reg, 1, '>'));
                    }
                    "pack_wave" => {
                        self.push_slot(make_slot('\'', "=1", dest, '#', 0, 4, '>'));
                    }
                    "pack_subbyte" => {
                        self.push_slot(make_slot('_', "PK", dest, '$', first_arg_reg, 2, '>'));
                    }
                    "bind_local_tile" | "distribute_image_tiles" => {
                        self.push_slot(make_slot('_', "TL", dest, '$', first_arg_reg, 4, '>'));
                    }
                    "stage_prefetch" | "prefetch" => {
                        self.push_slot(make_slot('_', "ST", 0, '$', 1, 0, '>'));
                    }
                    "compute_attention_head" => {
                        // Photonic MZI Attention: GEMM -> Autodiff -> Fredkin -> Toffoli
                        self.push_slot(make_slot('_', "OP", dest, '$', first_arg_reg, 0xE, '>'));
                        self.push_slot(make_slot('_', "FA", dest, '$', first_arg_reg, 0, '>'));
                        self.push_slot(make_slot('_', "RF", (dest % 14) + 1, '$', dest, 4, '>'));
                        self.push_slot(make_slot('_', "TO", (dest % 14) + 2, '$', dest, 4, '>'));
                    }
                    "optical_gemm" | "optical_gemm_forward" | "optical_conv_layer" => {
                        // Brain 2 Photonic GEMM
                        self.push_slot(make_slot('_', "OP", dest, '$', first_arg_reg, 0xE, '>'));
                    }
                    "optical_autodiff_tap" | "tap_forward_gradient" => {
                        self.push_slot(make_slot('_', "FA", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "predicated_op" => {
                        self.push_slot(make_slot('_', "PO", dest, '$', first_arg_reg, 2, '>'));
                    }
                    "subbyte_dot" | "ternary_dot16" | "simd_ternary_dot" | "simd_f4_gemm" => {
                        self.push_slot(make_slot('_', "MD", dest, '$', first_arg_reg, 5, '>'));
                    }
                    "tile_matmul" | "tile_fma" | "tensor_matmul" => {
                        self.push_slot(make_slot('_', "OP", dest, '$', first_arg_reg, 0xE, '>'));
                    }
                    "tile_transpose" | "tile_swizzle" | "tensor_transpose" => {
                        self.push_slot(make_slot('_', "TT", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "tensor_add" => {
                        self.push_slot(make_slot('_', "PO", dest, '$', first_arg_reg, 1, '>'));
                    }
                    "sram_swizzle_index" => {
                        self.push_slot(make_slot('_', "TL", dest, '$', first_arg_reg, 2, '>'));
                    }
                    "pgas_read" => {
                        self.push_slot(make_slot('_', "RC", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "pgas_write" => {
                        self.push_slot(make_slot('_', "YD", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "pgas_barrier" => {
                        self.push_slot(make_slot('_', "bb", 0, '#', 0, 0, '>'));
                    }
                    "backward" | "reversible_backward_step" => {
                        // Brain 3 Reversible Backward Pass
                        self.push_slot(make_slot('_', "BK", dest, '$', first_arg_reg, 1, '>'));
                    }
                    "blend_staged_halo" => {
                        self.push_slot(make_slot('_', "BL", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "reversible_entangle" | "fredkin_gate" => {
                        // Brain 3 Reversible Fredkin gate
                        self.push_slot(make_slot('_', "RF", dest, '$', first_arg_reg, 4, '>'));
                    }
                    "toffoli_gate" => {
                        // Brain 3 Toffoli gate
                        self.push_slot(make_slot('_', "TO", dest, '$', first_arg_reg, 4, '>'));
                    }
                    "apply_gradient_step" => {
                        self.push_slot(make_slot('_', "GU", dest, '#', first_arg_reg, 0, '>'));
                    }
                    "step_synaptic_plasticity" | "stdp_apply_spike" => {
                        // Brain 4 STDP Synapse Update
                        self.push_slot(make_slot('_', "ST", dest, '$', first_arg_reg, 4, '>'));
                    }
                    "ground_to_symbol" | "ground_and_unify" | "unify_terms" | "symbolify" => {
                        // Brain 1 Symbolic grounding
                        self.push_slot(make_slot('_', "SY", dest, '$', first_arg_reg, 8, '>'));
                    }
                    "match_hyper_edge" => {
                        // Brain 1 Hyper-Edge Matcher (Opcode 7'd134)
                        self.push_slot(make_slot('_', "HE", dest, '$', first_arg_reg, 3, '>'));
                    }
                    "lif_step" => {
                        // Brain 4 Neuromorphic LIF Neuron Step (Opcode 6'd75)
                        self.push_slot(make_slot('_', "LI", dest, '$', first_arg_reg, 5, '>'));
                    }
                    "lorenz_step" => {
                        // Brain 5 Chaos Diffusion / Lorenz Attractor (Opcode 7'd128)
                        self.push_slot(make_slot('_', "OD", dest, '$', first_arg_reg, 7, '>'));
                    }
                    "cross_attention_gate" => {
                        // Brain 5 Cross-Attention Gating (Opcode 7'd132)
                        self.push_slot(make_slot('_', "CA", dest, '$', first_arg_reg, 2, '>'));
                    }
                    "arbiter_update" => {
                        // Brain 6 Dynamic Arbiter Weight Update (Opcode 7'd126)
                        self.push_slot(make_slot('_', "AW", dest, '$', first_arg_reg, 1, '>'));
                    }
                    "cordic_sincos" => {
                        // Hardware CORDIC Sin/Cos (Opcode 6'd63)
                        self.push_slot(make_slot('_', "CD", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "atomic_cas" => {
                        // Hardware Atomic Compare-and-Swap (Opcode 7'd105)
                        self.push_slot(make_slot('_', "CS", dest, '$', first_arg_reg, 1, '>'));
                    }
                    "prefix_sum" => {
                        // Parallel Prefix-Sum Kogge-Stone Adder Tree (Opcode 7'd106)
                        self.push_slot(make_slot('_', "PS", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "transpose_4x4" => {
                        // Tensor Tile Strided Swizzle / Transpose (Opcode 7'd115)
                        self.push_slot(make_slot('_', "TT", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "wormhole_route" => {
                        // Deterministic 4D Hyper-Torus Wormhole Tunnel (Opcode 7'd135)
                        self.push_slot(make_slot('_', "WH", dest, '$', first_arg_reg, 3, '>'));
                    }
                    "deflect_packet" => {
                        // Adaptive Deflection Routing (Opcode 6'd76)
                        self.push_slot(make_slot('_', "DF", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "acquire_capability" => {
                        // Hardware Capability Token Acquisition (Opcode 0x14: _AC)
                        self.push_slot(make_slot('_', "AC", dest, '$', 0, 1, '>'));
                    }
                    "sanitize" => {
                        // Hardware Bounds Sanitization (Opcode 0x15: _SN)
                        self.push_slot(make_slot('_', "SN", dest, '$', first_arg_reg, 0, '>'));
                    }
                    "secure_patch_icache" => {
                        // Hardware Secure I-Cache Patch (Opcode 0x16: _SC)
                        self.push_slot(make_slot('_', "SC", dest, '$', first_arg_reg, 2, '>'));
                    }
                    "deduce_causal_chain" | "assert_triple" | "init_kg_partition" => {
                        // Brain 1 Causal Knowledge Graph query
                        self.push_slot(make_slot('_', "KG", dest, '$', first_arg_reg, 3, '>'));
                    }
                    "branch_in_superposition" | "superposition_branch" | "quantum_collapse_eval" => {
                        // Brain 5 Quantum superposition branch
                        self.push_slot(make_slot('_', "SW", dest, '$', first_arg_reg, 5, '>'));
                    }
                    "multi_head_dispatch" | "spatial_broadcast" => {
                        // Broadcast across 4D torus mesh
                        self.push_slot(make_slot('_', "SB", 0, '$', first_arg_reg, 5, '>'));
                        self.push_slot(make_slot('_', "YD", 0, '$', 2, 5, '>'));
                    }
                    "spatial_gather" => {
                        self.push_slot(make_slot('_', "YD", dest, '$', first_arg_reg, 5, '>'));
                    }
                    "recover_from_neighbor" => {
                        self.push_slot(make_slot('_', "RC", dest, '$', first_arg_reg, 5, '>'));
                    }
                    "await_dma_channel" | "dma_sync" => {
                        self.push_slot(make_slot('_', "DW", 0, '$', 1, 5, '>'));
                    }
                    "verify_slot_parity" | "poll_core_telemetry" => {
                        // Brain 6 Parity telemetry
                        self.push_slot(make_slot('_', "PT", 0, '$', 0, 1, '>'));
                    }
                    "lfsr_rand" | "random" => {
                        // 32-bit Galois LFSR Hardware PRNG (Milestone #042)
                        self.push_slot(make_slot('_', "RN", dest, '$', 0, 0, '>'));
                    }
                    "noc_poll" | "poll_noc" => {
                        // Non-blocking NoC FIFO Poll (Milestone #042)
                        self.push_slot(make_slot('_', "PL", dest, '$', 0, 0, '>'));
                    }
                    "read_csr" | "csr_read" => {
                        // Hardware CSR Performance Counter Read (Milestone #180)
                        let csr_id = match args.first().map(|a| &a.value) {
                            Some(Expr::LiteralInt(i)) => (*i as usize).min(3),
                            _ => first_arg_reg.min(3),
                        };
                        self.push_slot(make_slot('_', "RC", dest, 'C', csr_id, 0, '>'));
                    }
                    "checkpoint" | "save_checkpoint" | "shadow_save" => {
                        // 1-Cycle Hardware Shadow Checkpoint Save (Milestone #053)
                        self.push_slot(make_slot('_', "RC", 0, '!', 0, 0, '>'));
                    }
                    "restore_checkpoint" | "shadow_restore" => {
                        // 1-Cycle Hardware Shadow Checkpoint Restore (Milestone #053)
                        self.push_slot(make_slot('_', "RC", 0, '!', 0, 1, '>'));
                    }
                    "mret" | "return_from_trap" => {
                        // Return from Hardware Trap Handler (Milestone #181)
                        self.push_slot(make_slot('_', "RT", 0, '$', 0, 0, '>'));
                    }
                    "$trap" => {
                        self.push_slot(make_slot('_', "PO", dest, '$', 0, 0, '>'));
                    }
                    _ => {
                        self.push_slot(make_slot('_', "PO", dest, '$', first_arg_reg, 0, '>'));
                    }
                }
            }
            Expr::MethodCall { object, method, args } => {
                // Zero-vtable monomorphized dispatch (Milestone #005)
                self.compile_method_call(object, method, args, dest);
            }
            Expr::Grad { .. } | Expr::GradCall { .. } => {
                self.push_slot(make_slot('\'', "=0", dest, '#', 0, 0, '>'));
            }
        }
    }

    fn compile_method_call(&mut self, object: &Expr, method: &str, args: &[CallArg], dest: usize) {
        // Resolve the struct type from the object expression
        let struct_name = match object {
            Expr::Ident(name, _) => {
                self.reg_map.get(name).map(|_| name.clone())
            }
            _ => None,
        };

        // Compile the object into a register (self argument)
        let self_reg = dest;
        self.compile_expr(object, self_reg);

        // Try to find the monomorphized impl method
        let mut found_method = None;
        if let Some(ref sn) = struct_name {
            // Try direct struct::method lookup
            let key = (sn.clone(), method.to_string());
            if let Some(func) = self.impl_methods.get(&key).cloned() {
                found_method = Some(func);
            }
        }

        // Also try searching all impl methods by method name alone
        if found_method.is_none() {
            for ((_, m_name), func) in &self.impl_methods {
                if m_name == method {
                    found_method = Some(func.clone());
                    break;
                }
            }
        }

        if let Some(func) = found_method {
            // Zero-vtable monomorphization: inline the resolved method body directly
            // Skip 'self' param, bind remaining args to registers
            let non_self_params: Vec<_> = func.params.iter()
                .filter(|p| p.name != "self")
                .collect();
            for (param, arg) in non_self_params.iter().zip(args.iter()) {
                let p_reg = self.alloc_reg(&param.name);
                self.compile_expr(&arg.value, p_reg);
            }
            self.compile_statements(&func.body);
        } else {
            // Fallback: emit as generic predicated operation
            for arg in args {
                self.compile_expr(&arg.value, dest);
            }
            self.push_slot(make_slot('_', "PO", dest, '$', self_reg, 0, '>'));
        }
    }
}
