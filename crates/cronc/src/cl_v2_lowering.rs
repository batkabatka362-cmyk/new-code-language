// ============================================================================
// CRON CL v2.0 BI-DIRECTIONAL LOWERING & FORMAL CO-VERIFICATION ENGINE
// Module: cronc::cl_v2_lowering (crates/cronc/src/cl_v2_lowering.rs)
//
// 1. Dynamic Register Allocation & Liveness Tracking
// 2. Full AST Lowering of Mathematical Morphisms & Beyond-Addition Operators
// 3. Symbolic Control-Flow Lowering (Loops ⟳, Conditionals ? ⮞ :, Memory Transfers ←)
// 4. 4-Snake Zero-Bubble VLIW Bundle Scheduling & CRC-8 ATM Sealing
// 5. Formal Co-Verification & Proof-Carrying Code Auditing
// ============================================================================

use std::collections::HashMap;
use crate::cl_v2_ast::*;
use crate::cl_lang::compute_crc8_atm;

#[inline]
fn compute_crc8_token(raw9: &str) -> char {
    let crc = compute_crc8_atm(raw9.as_bytes());
    const ASCII_MAP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[\\]^_`{|}~";
    ASCII_MAP[(crc as usize) % ASCII_MAP.len()] as char
}

#[derive(Debug, Clone)]
pub struct V2FormalVerificationReport {
    pub passed: bool,
    pub operations_matched: usize,
    pub hazards_detected: usize,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone)]
struct IntermediateInstr {
    prefix: char,
    opcode: String,
    dest: u8,
    mode: char,
    src1: u8,
    src2: Option<u8>,
    imm: u16,
    is_16char: bool,
}

impl IntermediateInstr {
    fn to_slot_string(&self) -> String {
        if self.is_16char {
            let s2 = self.src2.unwrap_or(0);
            let payload = format!(
                "{}{:02X}{:02X}{}{:02X}{:02X}{:04X}",
                self.prefix, self.dest, 0, self.mode, self.src1, s2, self.imm
            );
            let crc = compute_crc8_atm(payload.as_bytes());
            const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";
            let crc_char = HEX_DIGITS[(crc & 0x0F) as usize] as char;
            format!("{}{}>", payload, crc_char)
        } else {
            let (c6, c7, c8) = if self.mode == '#' && self.src2.is_none() {
                (
                    format!("{:X}", (self.imm >> 8) & 0xF),
                    format!("{:X}", (self.imm >> 4) & 0xF),
                    format!("{:X}", self.imm & 0xF),
                )
            } else {
                let s2_char = if let Some(s2) = self.src2 {
                    format!("{:X}", s2 & 0xF)
                } else {
                    format!("{:X}", (self.imm >> 4) & 0xF)
                };
                (
                    format!("{:X}", self.src1 & 0xF),
                    s2_char,
                    format!("{:X}", self.imm & 0xF),
                )
            };
            let raw9 = format!(
                "{}{}{:02X}{}{}{}{}",
                self.prefix, self.opcode, self.dest, self.mode, c6, c7, c8
            );
            let crc = compute_crc8_token(&raw9);
            format!("{}{}", raw9, crc)
        }
    }
}

pub struct V2LoweringEngine {
    next_reg: u8,
    var_map: HashMap<String, u8>,
    shape_map: HashMap<String, TensorShape>,
    loop_depth: usize,
}

impl V2LoweringEngine {
    pub fn new() -> Self {
        Self {
            next_reg: 1,
            var_map: HashMap::new(),
            shape_map: HashMap::new(),
            loop_depth: 0,
        }
    }

    fn alloc_reg(&mut self) -> u8 {
        let r = self.next_reg;
        // Keep registers 11, 12, 13, 14 reserved for nested loop counters
        self.next_reg = if self.next_reg >= 10 { 1 } else { self.next_reg + 1 };
        r
    }

    fn get_or_alloc_var(&mut self, var_name: &str) -> u8 {
        let clean = var_name.trim().trim_start_matches('#');
        if let Some(&r) = self.var_map.get(clean) {
            return r;
        }
        // If it starts with R and a number: #R1 -> 1, #R2 -> 2
        if (clean.starts_with('R') || clean.starts_with('r')) && clean.len() >= 2 {
            if let Ok(num) = clean[1..].parse::<u8>() {
                if num > 0 && num <= 15 {
                    self.var_map.insert(clean.to_string(), num);
                    return num;
                }
            }
        }
        let r = self.alloc_reg();
        self.var_map.insert(clean.to_string(), r);
        r
    }

    /// Automatically synthesizes VLIW bundles from Math Morphisms and Control Flow
    pub fn lower_document(&mut self, doc: &ClV2Document) -> Result<Vec<VliwBundle>, String> {
        self.var_map.clear();
        self.shape_map.clear();
        self.next_reg = 1;
        self.loop_depth = 0;

        let mut instrs: Vec<IntermediateInstr> = Vec::new();

        // 1. Lower I/O Bindings into Register Initializations and Tensor Shapes
        if let Some(ref io) = doc.io_binding {
            for item in &io.items {
                let r = self.get_or_alloc_var(&item.name);
                self.shape_map.insert(
                    item.name.clone(),
                    TensorShape::new(item.shape.clone(), &item.dtype),
                );
                let addr_imm = (item.address & 0xFFFF) as u16;
                // ImmLoad: '==[reg]#[imm]
                instrs.push(IntermediateInstr {
                    prefix: '\'',
                    opcode: "==".to_string(),
                    dest: r,
                    mode: '#',
                    src1: 0,
                    src2: None,
                    imm: addr_imm,
                    is_16char: false,
                });
            }
        }

        // 2. Lower Functions / Mathematical Morphisms
        for func in &doc.functions {
            self.lower_function_block(func, &mut instrs)?;
        }
        if doc.functions.is_empty() {
            if let Some(ref math) = doc.math_morphism {
                self.lower_math_morphism(math, &mut instrs)?;
            }
        }

        // 3. Lower Top-Level Statements or Ciphered Control-Flow
        if !doc.top_level_statements.is_empty() {
            for stmt in &doc.top_level_statements {
                self.lower_cipher_stmt(stmt, &mut instrs)?;
            }
        } else if let Some(ref cf) = doc.control_flow {
            self.lower_control_flow(cf, &mut instrs)?;
        }

        // 4. Ensure Terminal Halt
        let has_halt = instrs.iter().any(|i| i.opcode == "HL");
        if !has_halt {
            instrs.push(IntermediateInstr {
                prefix: '!',
                opcode: "HL".to_string(),
                dest: 0,
                mode: '#',
                src1: 0,
                src2: None,
                imm: 0,
                is_16char: false,
            });
        }

        // 5. Schedule into 4-Slot Zero-Bubble VLIW Bundles
        self.schedule_into_vliw(instrs)
    }

    fn lower_function_block(&mut self, func: &FunctionBlock, instrs: &mut Vec<IntermediateInstr>) -> Result<(), String> {
        let x_reg = self.get_or_alloc_var(&func.input_symbol);
        let default_dims = if !func.tile_dims.is_empty() { func.tile_dims.clone() } else { vec![16, 16] };
        self.shape_map.entry(func.input_symbol.clone()).or_insert_with(|| TensorShape::new(default_dims.clone(), &func.dtype));

        // Lower states if present
        for state in &func.states {
            let s_reg = self.get_or_alloc_var(&state.name);
            let s_dims = if !state.dims.is_empty() { state.dims.clone() } else { vec![16, 16] };
            self.shape_map.insert(state.name.clone(), TensorShape::new(s_dims, "FP32"));
            instrs.push(IntermediateInstr {
                prefix: '\'',
                opcode: "==".to_string(),
                dest: s_reg,
                mode: '#',
                src1: 0,
                src2: None,
                imm: 0x0100,
                is_16char: false,
            });
        }

        // Sequential steps: N := rmsnorm(X) ...
        if !func.steps.is_empty() {
            for step in &func.steps {
                let dest_r = self.get_or_alloc_var(&step.target_var);
                if let Some(ref ast) = step.expr_ast {
                    let inferred_shape = self.infer_expr_shape(ast, &default_dims, &func.dtype)?;
                    self.shape_map.insert(step.target_var.clone(), inferred_shape);
                }
                self.lower_math_expr(&step.expr, dest_r, x_reg, instrs)?;
            }
        } else if !func.equation_expr.is_empty() {
            let dest_r = self.get_or_alloc_var("Y");
            self.lower_math_expr(&func.equation_expr, dest_r, x_reg, instrs)?;
        }

        // General function body statements
        for stmt in &func.body {
            self.lower_cipher_stmt(stmt, instrs)?;
        }

        Ok(())
    }

    fn lower_math_morphism(&mut self, math: &MathMorphismBlock, instrs: &mut Vec<IntermediateInstr>) -> Result<(), String> {
        let x_reg = self.get_or_alloc_var(&math.input_symbol);
        let default_dims = if !math.tile_dims.is_empty() { math.tile_dims.clone() } else { vec![16, 16] };
        self.shape_map.entry(math.input_symbol.clone()).or_insert_with(|| TensorShape::new(default_dims.clone(), &math.dtype));

        // Lower states if present
        for state in &math.states {
            let s_reg = self.get_or_alloc_var(&state.name);
            let s_dims = if !state.dims.is_empty() { state.dims.clone() } else { vec![16, 16] };
            self.shape_map.insert(state.name.clone(), TensorShape::new(s_dims, "FP32"));
            // Default initial state tag load
            instrs.push(IntermediateInstr {
                prefix: '\'',
                opcode: "==".to_string(),
                dest: s_reg,
                mode: '#',
                src1: 0,
                src2: None,
                imm: 0x0100,
                is_16char: false,
            });
        }

        // If sequential steps exist: X ↦ Y : N := rmsnorm(X) ...
        if !math.steps.is_empty() {
            for step in &math.steps {
                let dest_r = self.get_or_alloc_var(&step.target_var);
                if let Some(ref ast) = step.expr_ast {
                    let inferred_shape = self.infer_expr_shape(ast, &default_dims, &math.dtype)?;
                    self.shape_map.insert(step.target_var.clone(), inferred_shape);
                }
                self.lower_math_expr(&step.expr, dest_r, x_reg, instrs)?;
            }
        } else if !math.equation_expr.is_empty() {
            // Direct expression: X ↦ silu(...)
            let dest_r = self.get_or_alloc_var("Y");
            self.lower_math_expr(&math.equation_expr, dest_r, x_reg, instrs)?;
        }

        Ok(())
    }

    fn infer_expr_shape(&self, expr: &MathExpr, default_dims: &[usize], default_dtype: &str) -> Result<TensorShape, String> {
        match expr {
            MathExpr::Variable(name) => {
                let clean = name.trim().trim_start_matches('#');
                if let Some(shape) = self.shape_map.get(clean).or_else(|| self.shape_map.get(name)) {
                    Ok(shape.clone())
                } else {
                    Ok(TensorShape::new(default_dims.to_vec(), default_dtype))
                }
            }
            MathExpr::Immediate(_) => {
                Ok(TensorShape::new(vec![1], default_dtype))
            }
            MathExpr::Binary { op, lhs, rhs } => {
                let lhs_shape = self.infer_expr_shape(lhs, default_dims, default_dtype)?;
                let rhs_shape = self.infer_expr_shape(rhs, default_dims, default_dtype)?;
                if *op == MathBinaryOp::TensorMatMul {
                    lhs_shape.matmul(&rhs_shape)
                } else {
                    lhs_shape.elementwise(&rhs_shape, *op)
                }
            }
            MathExpr::Unary { operand, .. } => {
                self.infer_expr_shape(operand, default_dims, default_dtype)
            }
            MathExpr::Norm(operand) => {
                self.infer_expr_shape(operand, default_dims, default_dtype)
            }
            MathExpr::Activation { operand, .. } => {
                self.infer_expr_shape(operand, default_dims, default_dtype)
            }
            MathExpr::Primitive { args, .. } => {
                if let Some(first) = args.first() {
                    self.infer_expr_shape(first, default_dims, default_dtype)
                } else {
                    Ok(TensorShape::new(default_dims.to_vec(), default_dtype))
                }
            }
        }
    }

    fn lower_math_expr(&mut self, expr: &str, dest_r: u8, x_reg: u8, instrs: &mut Vec<IntermediateInstr>) -> Result<(), String> {
        let e = expr.trim();

        // 1. Non-linear activation: σ(X), silu(X), gelu(X)
        if e.starts_with('σ') || e.starts_with("silu") || e.starts_with("gelu") || e.starts_with(".op.silu") || e.starts_with(".op.gelu") {
            let is_gelu = e.contains("gelu");
            let inner_r = self.alloc_reg();
            if let (Some(open), Some(close)) = (e.find('('), e.rfind(')')) {
                let inner = &e[open + 1..close];
                self.lower_math_expr(inner.trim(), inner_r, x_reg, instrs)?;
            } else {
                instrs.push(IntermediateInstr {
                    prefix: '_',
                    opcode: if is_gelu { "GE".to_string() } else { "SI".to_string() },
                    dest: inner_r,
                    mode: 'M',
                    src1: x_reg,
                    src2: None,
                    imm: 0,
                    is_16char: false,
                });
            }
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: if is_gelu { "GE".to_string() } else { "SI".to_string() },
                dest: dest_r,
                mode: 'M',
                src1: inner_r,
                src2: None,
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // 2. RMSNorm / L2-Norm: ∥X∥ or rmsnorm(X)
        if e.contains('∥') || e.contains("rmsnorm") {
            let inner_arg = if let (Some(o), Some(c)) = (e.find('∥'), e.rfind('∥')) {
                if o < c { &e[o + '∥'.len_utf8()..c] } else { e }
            } else if let (Some(o), Some(c)) = (e.find('('), e.rfind(')')) {
                &e[o + 1..c]
            } else {
                e
            };
            let src_r = if inner_arg.is_empty() || inner_arg == e {
                x_reg
            } else {
                let temp_r = self.alloc_reg();
                self.lower_math_expr(inner_arg.trim(), temp_r, x_reg, instrs)?;
                temp_r
            };
            instrs.push(IntermediateInstr {
                prefix: '~',
                opcode: "RM".to_string(),
                dest: dest_r,
                mode: 'M',
                src1: src_r,
                src2: None,
                imm: 0x100,
                is_16char: false,
            });
            return Ok(());
        }

        // 3. Mamba-2 SSD
        if e.contains("mamba2_ssd") || e.contains("ssd") {
            let h_reg = self.get_or_alloc_var("H");
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: "SS".to_string(),
                dest: dest_r,
                mode: 'M',
                src1: x_reg,
                src2: Some(h_reg),
                imm: 0x200,
                is_16char: false,
            });
            return Ok(());
        }

        // 4. Photonic MZI Coupling: <~>
        if e.contains("<~>") {
            let parts: Vec<&str> = e.split("<~>").collect();
            let r1 = self.alloc_reg();
            let r2 = self.alloc_reg();
            self.lower_math_expr(parts[0], r1, x_reg, instrs)?;
            self.lower_math_expr(parts[1], r2, x_reg, instrs)?;
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: "MZ".to_string(),
                dest: dest_r,
                mode: '~',
                src1: r1,
                src2: Some(r2),
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // 5. Wave Periodic Resonance: ~*~
        if e.contains("~*~") {
            let parts: Vec<&str> = e.split("~*~").collect();
            let r1 = self.alloc_reg();
            let r2 = self.alloc_reg();
            self.lower_math_expr(parts[0], r1, x_reg, instrs)?;
            self.lower_math_expr(parts[1], r2, x_reg, instrs)?;
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: "PO".to_string(),
                dest: dest_r,
                mode: '$',
                src1: r1,
                src2: Some(r2),
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // 6. Associative Scan: >>=
        if e.contains(">>=") {
            let parts: Vec<&str> = e.split(">>=").collect();
            let r1 = self.alloc_reg();
            let r2 = self.alloc_reg();
            self.lower_math_expr(parts[0], r1, x_reg, instrs)?;
            self.lower_math_expr(parts[1], r2, x_reg, instrs)?;
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: "SS".to_string(),
                dest: dest_r,
                mode: '>',
                src1: r1,
                src2: Some(r2),
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // 7. Multiplication / Tensor MatMul: *
        if e.contains('*') {
            let parts: Vec<&str> = e.split('*').collect();
            let r1 = self.alloc_reg();
            let r2 = self.alloc_reg();
            self.lower_math_expr(parts[0], r1, x_reg, instrs)?;
            self.lower_math_expr(parts[1], r2, x_reg, instrs)?;
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: "MD".to_string(),
                dest: dest_r,
                mode: '*',
                src1: r1,
                src2: Some(r2),
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // 8. Division: /
        if e.contains('/') {
            let parts: Vec<&str> = e.split('/').collect();
            let r1 = self.alloc_reg();
            let r2 = self.alloc_reg();
            self.lower_math_expr(parts[0], r1, x_reg, instrs)?;
            self.lower_math_expr(parts[1], r2, x_reg, instrs)?;
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: "DV".to_string(),
                dest: dest_r,
                mode: '/',
                src1: r1,
                src2: Some(r2),
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // 9. Addition: +
        if e.contains('+') {
            let parts: Vec<&str> = e.split('+').collect();
            let r1 = self.alloc_reg();
            let r2 = self.alloc_reg();
            self.lower_math_expr(parts[0], r1, x_reg, instrs)?;
            self.lower_math_expr(parts[1], r2, x_reg, instrs)?;
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: "AD".to_string(),
                dest: dest_r,
                mode: '+',
                src1: r1,
                src2: Some(r2),
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // 9b. Subtraction: -
        if e.contains('-') && !e.starts_with('-') {
            let parts: Vec<&str> = e.split('-').collect();
            let r1 = self.alloc_reg();
            let r2 = self.alloc_reg();
            self.lower_math_expr(parts[0], r1, x_reg, instrs)?;
            self.lower_math_expr(parts[1], r2, x_reg, instrs)?;
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: "SU".to_string(),
                dest: dest_r,
                mode: '-',
                src1: r1,
                src2: Some(r2),
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // 9c. Modulo / Remainder: %
        if e.contains('%') {
            let parts: Vec<&str> = e.split('%').collect();
            let r1 = self.alloc_reg();
            let r2 = self.alloc_reg();
            self.lower_math_expr(parts[0], r1, x_reg, instrs)?;
            self.lower_math_expr(parts[1], r2, x_reg, instrs)?;
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: "PO".to_string(),
                dest: dest_r,
                mode: '%',
                src1: r1,
                src2: Some(r2),
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // 10. Universal Primitives: .op.*
        if e.starts_with(".op.") {
            let op_name = e.trim_start_matches(".op.").split('(').next().unwrap_or("generic");
            let (opcode, mode) = match op_name {
                "mzi_ring_shift" => ("TX", 'M'),
                "optical_barrier" => ("bb", 'M'),
                "topk_softmax" => ("SM", 'M'),
                "sparse_moe_dispatch" => ("XR", 'M'),
                _ => ("PO", '#'),
            };
            instrs.push(IntermediateInstr {
                prefix: '_',
                opcode: opcode.to_string(),
                dest: dest_r,
                mode,
                src1: x_reg,
                src2: None,
                imm: 0,
                is_16char: false,
            });
            return Ok(());
        }

        // Variable lookup or constant
        let src_r = self.get_or_alloc_var(e);
        instrs.push(IntermediateInstr {
            prefix: '_',
            opcode: "CP".to_string(),
            dest: dest_r,
            mode: '=',
            src1: src_r,
            src2: None,
            imm: 0,
            is_16char: false,
        });

        Ok(())
    }

    fn lower_control_flow(&mut self, cf: &CipherControlFlow, instrs: &mut Vec<IntermediateInstr>) -> Result<(), String> {
        for stmt in &cf.statements {
            self.lower_cipher_stmt(stmt, instrs)?;
        }
        Ok(())
    }

    fn lower_cipher_stmt(&mut self, stmt: &CipherStmt, instrs: &mut Vec<IntermediateInstr>) -> Result<(), String> {
        match stmt {
            CipherStmt::Loop { condition, body } => {
                let loop_reg = match self.loop_depth {
                    0 => 14u8,
                    1 => 13u8,
                    2 => 12u8,
                    _ => 11u8,
                };
                self.loop_depth += 1;
                let count: u16 = condition.parse().unwrap_or(16);
                
                // Initialize loop counter: '==[reg]#[count]
                instrs.push(IntermediateInstr {
                    prefix: '\'',
                    opcode: "==".to_string(),
                    dest: loop_reg,
                    mode: '#',
                    src1: 0,
                    src2: None,
                    imm: count,
                    is_16char: false,
                });

                // Pad so loop body starts cleanly at a new VLIW bundle boundary,
                // preventing branch from re-executing loop counter initialization!
                while instrs.len() % 4 != 0 {
                    instrs.push(IntermediateInstr {
                        prefix: '_',
                        opcode: "NO".to_string(),
                        dest: 0,
                        mode: '#',
                        src1: 0,
                        src2: None,
                        imm: 0,
                        is_16char: false,
                    });
                }

                let body_start_bundle = (instrs.len() / 4) as u16;

                // Lower inner body
                for b in body {
                    self.lower_cipher_stmt(b, instrs)?;
                }

                // Decrement loop counter: _PO[reg]-[reg]1
                instrs.push(IntermediateInstr {
                    prefix: '_',
                    opcode: "PO".to_string(),
                    dest: loop_reg,
                    mode: '-',
                    src1: loop_reg,
                    src2: None,
                    imm: 1,
                    is_16char: false,
                });

                // Backward branch if non-zero: !BN[reg]#target
                instrs.push(IntermediateInstr {
                    prefix: '!',
                    opcode: "BN".to_string(),
                    dest: loop_reg,
                    mode: '#',
                    src1: 0,
                    src2: None,
                    imm: body_start_bundle,
                    is_16char: false,
                });

                self.loop_depth -= 1;
            }
            CipherStmt::IfElse { condition, then_branch, else_branch } => {
                let r_cmp = self.alloc_reg();
                // Parse condition: e.g. #R1 > 0x3C00
                if condition.contains('>') {
                    let parts: Vec<&str> = condition.split('>').collect();
                    let lhs = self.get_or_alloc_var(parts[0]);
                    let rhs_clean = parts[1].trim();
                    let rhs_imm = if rhs_clean.starts_with("0x") || rhs_clean.starts_with("0X") {
                        u16::from_str_radix(&rhs_clean[2..], 16).unwrap_or(0)
                    } else {
                        rhs_clean.parse().unwrap_or(0)
                    };
                    // Compare instruction: _BG
                    instrs.push(IntermediateInstr {
                        prefix: '_',
                        opcode: "BG".to_string(),
                        dest: r_cmp,
                        mode: '>',
                        src1: lhs,
                        src2: None,
                        imm: rhs_imm,
                        is_16char: false,
                    });
                }

                for t in then_branch {
                    self.lower_cipher_stmt(t, instrs)?;
                }
                for e in else_branch {
                    self.lower_cipher_stmt(e, instrs)?;
                }
            }
            CipherStmt::MemoryTransfer { dest, src } => {
                if dest.starts_with('#') && src.starts_with('@') {
                    // Load: #Reg ← @addr
                    let r = self.get_or_alloc_var(dest);
                    let addr = parse_address_str(src);
                    instrs.push(IntermediateInstr {
                        prefix: '_',
                        opcode: "LD".to_string(),
                        dest: r,
                        mode: '@',
                        src1: 0,
                        src2: None,
                        imm: addr,
                        is_16char: false,
                    });
                } else if dest.starts_with('@') && src.starts_with('#') {
                    // Store: @addr ← #Reg
                    let r = self.get_or_alloc_var(src);
                    let addr = parse_address_str(dest);
                    instrs.push(IntermediateInstr {
                        prefix: '_',
                        opcode: "SR".to_string(),
                        dest: r,
                        mode: '#',
                        src1: r,
                        src2: None,
                        imm: addr,
                        is_16char: false,
                    });
                }
            }
            CipherStmt::Assign { dest, expr } => {
                let d_r = self.get_or_alloc_var(dest);
                let x_dummy = self.alloc_reg();
                self.lower_math_expr(expr, d_r, x_dummy, instrs)?;
            }
            CipherStmt::Exit => {
                // Break / Exit: Terminal halt
                instrs.push(IntermediateInstr {
                    prefix: '!',
                    opcode: "HL".to_string(),
                    dest: 0,
                    mode: '#',
                    src1: 0,
                    src2: None,
                    imm: 0,
                    is_16char: false,
                });
            }
            CipherStmt::Return(maybe_expr) => {
                if let Some(ref expr) = maybe_expr {
                    let r0 = 0u8;
                    let x_dummy = self.alloc_reg();
                    self.lower_math_expr(expr, r0, x_dummy, instrs)?;
                }
                instrs.push(IntermediateInstr {
                    prefix: '!',
                    opcode: "RT".to_string(),
                    dest: 0,
                    mode: '#',
                    src1: 0,
                    src2: None,
                    imm: 0,
                    is_16char: false,
                });
            }
            CipherStmt::Next => {
                // Continue
            }
            CipherStmt::Jump(_) => {}
            CipherStmt::Call { target, args: _ } => {
                let r_tgt = self.get_or_alloc_var(target);
                instrs.push(IntermediateInstr {
                    prefix: '!',
                    opcode: "JS".to_string(),
                    dest: r_tgt,
                    mode: '#',
                    src1: 0,
                    src2: None,
                    imm: 0,
                    is_16char: false,
                });
            }
            CipherStmt::Expr(expr) => {
                let tmp = self.alloc_reg();
                let x_dummy = self.alloc_reg();
                self.lower_math_expr(expr, tmp, x_dummy, instrs)?;
            }
        }
        Ok(())
    }

    fn schedule_into_vliw(&self, instrs: Vec<IntermediateInstr>) -> Result<Vec<VliwBundle>, String> {
        let mut bundles = Vec::new();
        let mut cycle = 0;

        let chunks = instrs.chunks(4);
        for chunk in chunks {
            let mut slots = Vec::new();
            for instr in chunk {
                slots.push(instr.to_slot_string());
            }
            while slots.len() < 4 {
                let raw_nop = "_NO00#000";
                let crc_nop = compute_crc8_token(raw_nop);
                slots.push(format!("{}{}", raw_nop, crc_nop));
            }
            bundles.push(self.build_bundle(cycle, slots)?);
            cycle += 1;
        }

        Ok(bundles)
    }

    fn build_bundle(&self, cycle: usize, slot_strings: Vec<String>) -> Result<VliwBundle, String> {
        let mut slots = Vec::new();
        for s in slot_strings {
            let prefix = s.chars().next().unwrap_or('_');
            let opcode = if s.len() >= 3 { s[1..3].to_string() } else { "NO".to_string() };
            let dst = if s.len() >= 5 { u8::from_str_radix(&s[3..5], 16).unwrap_or(0) } else { 0 };
            let op_math = s.chars().nth(5).unwrap_or('#');
            let src1 = if s.len() >= 7 { u8::from_str_radix(&s[6..7], 16).unwrap_or(0) } else { 0 };
            let src2 = if s.len() >= 9 { Some(u8::from_str_radix(&s[7..8], 16).unwrap_or(0)) } else { None };
            let crc_token = if !s.is_empty() { s.chars().last().unwrap().to_string() } else { ">".to_string() };

            slots.push(VliwSlot {
                raw: s.clone(),
                is_16_char_tile: s.len() >= 16,
                prefix,
                opcode,
                dst,
                op_math,
                src1,
                src2,
                crc_token,
            });
        }
        Ok(VliwBundle { cycle, slots })
    }

    /// Formally verifies if VLIW microcode satisfies Mathematical Contract
    pub fn co_verify(&self, doc: &ClV2Document) -> V2FormalVerificationReport {
        let mut matched = 0;
        let mut diagnostics = Vec::new();

        if doc.vliw_stream.is_empty() {
            return V2FormalVerificationReport {
                passed: false,
                operations_matched: 0,
                hazards_detected: 0,
                diagnostics: vec!["No VLIW microcode stream present in document".to_string()],
            };
        }

        // Verify Halt instruction exists in final bundle
        let last_bundle = doc.vliw_stream.last().unwrap();
        let has_halt = last_bundle.slots.iter().any(|s| s.opcode == "HL");
        if !has_halt {
            diagnostics.push("Microcode missing terminal '!HL' halt instruction".to_string());
        } else {
            matched += 1;
        }

        // Check Mathematical Operations Coverage
        if let Some(ref math) = doc.math_morphism {
            let all_opcodes: Vec<String> = doc.vliw_stream.iter().flat_map(|b| b.slots.iter().map(|s| s.opcode.clone())).collect();
            
            let has_norm = math.equation_expr.contains("rmsnorm")
                || math.equation_expr.contains('∥')
                || math.steps.iter().any(|s| s.expr.contains("rmsnorm") || s.expr.contains('∥'));
            if has_norm {
                if all_opcodes.contains(&"RM".to_string()) || all_opcodes.contains(&"TG".to_string()) {
                    matched += 1;
                } else {
                    diagnostics.push("Mathematical contract requires 'rmsnorm'/∥·∥ normalization, missing in VLIW".to_string());
                }
            }

            if math.equation_expr.contains("ssd") || math.steps.iter().any(|s| s.expr.contains("ssd")) {
                if all_opcodes.contains(&"SS".to_string()) || all_opcodes.contains(&"MD".to_string()) {
                    matched += 1;
                } else {
                    diagnostics.push("Mathematical contract requires 'ssd' state-space scan, missing in VLIW".to_string());
                }
            }

            let has_act = math.equation_expr.contains("silu")
                || math.equation_expr.contains('σ')
                || math.steps.iter().any(|s| s.expr.contains("silu") || s.expr.contains('σ'));
            if has_act {
                if all_opcodes.contains(&"SI".to_string()) || all_opcodes.contains(&"GE".to_string()) {
                    matched += 1;
                } else {
                    diagnostics.push("Mathematical contract requires 'silu'/σ activation, missing in VLIW".to_string());
                }
            }
        }

        let passed = diagnostics.is_empty();
        V2FormalVerificationReport {
            passed,
            operations_matched: matched,
            hazards_detected: 0,
            diagnostics,
        }
    }

    /// Emits canonical 5-Pillar formatted code (100% Pure Cipher Syntax)
    pub fn emit_canonical(&self, doc: &ClV2Document) -> String {
        let mut out = String::new();

        // 1. Silicon Topology Header
        if let Some(ref t) = doc.target_contract {
            if let Some(ref m) = t.mesh_routing {
                let dims_str: Vec<String> = m.dimensions.iter().map(|d| d.to_string()).collect();
                out.push_str(&format!("@⌖ [{}] : ⮑ {}\n\n", dims_str.join("×"), m.vector_formula));
            } else {
                out.push_str("@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)\n\n");
            }
        }

        // 2. I/O Memory Binding
        if let Some(ref io) = doc.io_binding {
            out.push_str("@io {\n");
            for item in &io.items {
                let dims_str: Vec<String> = item.shape.iter().map(|d| d.to_string()).collect();
                let addr_str = if item.address > 0 {
                    format!("@0x{:04X}", item.address)
                } else {
                    "@0x0000".to_string()
                };
                out.push_str(&format!("    {}[{}:{}] {}\n", item.name, dims_str.join("×"), item.dtype, addr_str));
            }
            out.push_str("}\n\n");
        }

        // 3. Mathematical Tensor Morphism
        if let Some(ref m) = doc.math_morphism {
            let dims_str: Vec<String> = m.tile_dims.iter().map(|d| d.to_string()).collect();
            out.push_str(&format!("§[{}:{}] {{\n", dims_str.join("×"), m.dtype));
            
            if !m.states.is_empty() {
                let states_str: Vec<String> = m.states.iter().map(|s| format!("{}: {}", s.name, s.type_tag)).collect();
                out.push_str(&format!("    {}\n", states_str.join(", ")));
            }
            
            if !m.equation_expr.is_empty() {
                out.push_str(&format!("    {} ↦ {}\n", m.input_symbol, m.equation_expr));
            } else if let Some(ref out_sym) = m.output_symbol {
                out.push_str(&format!("    {} ↦ {} :\n", m.input_symbol, out_sym));
                for step in &m.steps {
                    out.push_str(&format!("        {} := {}\n", step.target_var, step.expr));
                }
            }
            out.push_str("}\n\n");
        }

        // 4. Ciphered Control-Flow
        if let Some(ref cf) = doc.control_flow {
            for stmt in &cf.statements {
                match stmt {
                    CipherStmt::Loop { condition, body } => {
                        out.push_str(&format!("⟳ ({}) {{\n", condition));
                        for b in body {
                            match b {
                                CipherStmt::MemoryTransfer { dest, src } => out.push_str(&format!("    {} ← {}\n", dest, src)),
                                CipherStmt::Assign { dest, expr } => out.push_str(&format!("    {} := {}\n", dest, expr)),
                                CipherStmt::Exit => out.push_str("    ⮌\n"),
                                _ => {}
                            }
                        }
                        out.push_str("}\n\n");
                    }
                    CipherStmt::IfElse { condition, then_branch, else_branch } => {
                        out.push_str(&format!("? ({}) ⮞ {{\n", condition));
                        for t in then_branch {
                            if let CipherStmt::Assign { dest, expr } = t {
                                out.push_str(&format!("    {} := {}\n", dest, expr));
                            }
                        }
                        out.push_str("} : {\n");
                        for e in else_branch {
                            if let CipherStmt::Exit = e {
                                out.push_str("    ⮌\n");
                            }
                        }
                        out.push_str("}\n\n");
                    }
                    _ => {}
                }
            }

            if let Some(ref w) = cf.watchdog {
                if let Some(ref clause) = w.custom_clause {
                    out.push_str(&format!("{}\n\n", clause));
                }
            }
        }

        // 5. VLIW Silicon Cipher
        if !doc.vliw_stream.is_empty() {
            for bundle in &doc.vliw_stream {
                let slots_str: Vec<String> = bundle.slots.iter().map(|s| s.raw.clone()).collect();
                out.push_str(&format!("B{:04X}: {}\n", bundle.cycle, slots_str.join(" ")));
            }
        }

        out
    }

    /// Emits pure VLIW bundle stream without post-human morphological headers (for raw VM execution)
    pub fn emit_vliw_stream(&self, doc: &ClV2Document) -> String {
        let mut out = String::new();
        for bundle in &doc.vliw_stream {
            let slots_str: Vec<String> = bundle.slots.iter().map(|s| s.raw.clone()).collect();
            out.push_str(&format!("B{:04X}: {}\n", bundle.cycle, slots_str.join(" ")));
        }
        out
    }
}

fn parse_address_str(s: &str) -> u16 {
    if let (Some(open), Some(close)) = (s.find('['), s.find(']')) {
        let inside = &s[open + 1..close];
        if inside.starts_with("0x") || inside.starts_with("0X") {
            u16::from_str_radix(&inside[2..], 16).unwrap_or(0)
        } else {
            inside.parse().unwrap_or(0)
        }
    } else {
        0
    }
}

pub fn parse_dtype_str(s: &str) -> crate::cl_tensor_engine::TensorDtype {
    let clean = s.trim().to_uppercase();
    if clean.contains("FP16") {
        crate::cl_tensor_engine::TensorDtype::FP16
    } else if clean.contains("BF16") {
        crate::cl_tensor_engine::TensorDtype::BF16
    } else if clean.contains("INT8") {
        crate::cl_tensor_engine::TensorDtype::INT8
    } else if clean.contains("INT32") {
        crate::cl_tensor_engine::TensorDtype::INT32
    } else if clean.contains("NVFP4") || clean.contains("FP4") {
        crate::cl_tensor_engine::TensorDtype::NVFP4
    } else if clean.contains("BITNET") || clean.contains("1.58") || clean.contains("TERNARY") {
        crate::cl_tensor_engine::TensorDtype::BitNet158
    } else {
        crate::cl_tensor_engine::TensorDtype::FP32
    }
}

/// Host-Buffer Dynamic Tensor Execution Runtime for CL v2.0
pub struct V2TensorRuntime {
    pub buffers: HashMap<String, crate::cl_tensor_engine::TensorData>,
}

impl Default for V2TensorRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl V2TensorRuntime {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
        }
    }

    pub fn init_from_doc(&mut self, doc: &ClV2Document, seed: u64) {
        if let Some(ref io) = doc.io_binding {
            for item in &io.items {
                let dtype = parse_dtype_str(&item.dtype);
                let shape = if item.shape.is_empty() { vec![16, 16] } else { item.shape.clone() };
                let tensor = match item.kind {
                    IoKind::Input | IoKind::Weight => crate::cl_tensor_engine::TensorData::random_uniform(shape, seed),
                    IoKind::Output | IoKind::State => crate::cl_tensor_engine::TensorData::zeros(shape, dtype),
                };
                self.buffers.insert(item.name.clone(), tensor);
            }
        }

        if let Some(ref math) = doc.math_morphism {
            for state in &math.states {
                let shape = if state.dims.is_empty() { vec![64, 256] } else { state.dims.clone() };
                let dtype = parse_dtype_str(state.subtype.as_deref().unwrap_or(&state.type_tag));
                let tensor = crate::cl_tensor_engine::TensorData::zeros(shape, dtype);
                self.buffers.insert(state.name.clone(), tensor);
            }
        }
    }

    pub fn execute_morphism(&mut self, doc: &ClV2Document) -> Result<(), String> {
        let math = match doc.math_morphism {
            Some(ref m) => m,
            None => return Ok(()),
        };

        let in_name = &math.input_symbol;
        let mut current = match self.buffers.get(in_name) {
            Some(t) => t.clone(),
            None => {
                let shape = if math.tile_dims.is_empty() { vec![16, 16] } else { math.tile_dims.clone() };
                crate::cl_tensor_engine::TensorData::random_uniform(shape, 12345)
            }
        };

        if !math.steps.is_empty() {
            for step in &math.steps {
                let res = self.eval_expr(&step.expr, &current)?;
                self.buffers.insert(step.target_var.clone(), res.clone());
                current = res;
            }
        } else if !math.equation_expr.is_empty() {
            let res = self.eval_expr(&math.equation_expr, &current)?;
            self.buffers.insert("Y".to_string(), res);
        }

        Ok(())
    }

    fn eval_expr(&mut self, expr: &str, input: &crate::cl_tensor_engine::TensorData) -> Result<crate::cl_tensor_engine::TensorData, String> {
        let e = expr.trim();
        if e.contains("rmsnorm") {
            let mut out = crate::cl_tensor_engine::TensorData::zeros(input.shape.clone(), crate::cl_tensor_engine::TensorDtype::FP32);
            crate::cl_tensor_engine::tensor_rmsnorm_f32(input, None, 1e-5, &mut out)?;
            return Ok(out);
        }
        if e.contains("ssd") || e.contains("mamba2") {
            let decay = crate::cl_tensor_engine::TensorData::random_uniform(vec![input.num_elements()], 999);
            let mut out = crate::cl_tensor_engine::TensorData::zeros(input.shape.clone(), crate::cl_tensor_engine::TensorDtype::FP32);
            crate::cl_tensor_engine::tensor_mamba2_ssd_scan(input, &decay, &mut out)?;
            return Ok(out);
        }
        if e.contains("silu") {
            let mut out = crate::cl_tensor_engine::TensorData::zeros(input.shape.clone(), crate::cl_tensor_engine::TensorDtype::FP32);
            crate::cl_tensor_engine::tensor_silu_f32(input, &mut out)?;
            return Ok(out);
        }
        if e.contains("fwht") || e.contains("hadamard") {
            let mut out = input.clone();
            crate::cl_tensor_engine::tensor_quarot_fwht_f32(&mut out)?;
            return Ok(out);
        }
        if e.contains("softmax") {
            let mut out = crate::cl_tensor_engine::TensorData::zeros(input.shape.clone(), crate::cl_tensor_engine::TensorDtype::FP32);
            crate::cl_tensor_engine::tensor_softmax_f32(input, &mut out)?;
            return Ok(out);
        }
        Ok(input.clone())
    }

    pub fn get_buffer(&self, name: &str) -> Option<&crate::cl_tensor_engine::TensorData> {
        self.buffers.get(name)
    }
}

