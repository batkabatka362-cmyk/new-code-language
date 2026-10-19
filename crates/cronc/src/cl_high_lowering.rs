// ============================================================================
// CRON & CL HIGH-LEVEL TO 128-BIT VLIW LOWERING ENGINE
// Module: cronc::cl_high_lowering (crates/cronc/src/cl_high_lowering.rs)
//
// Lowers High-Level Declarative AI Models & Multi-Physics Ast Nodes into
// Canonical ClProgramIR and 128-Bit 4-Slot VLIW Machine Code with
// 4-Snake (X, Y, Z, W) Zero-Bubble Scheduling and Register Allocation.
// ============================================================================

use std::collections::HashMap;
use crate::cl_high_ast::*;
use crate::cl_high_parser::parse_high_cl;
use crate::cl_ir::{AluOp, BranchCond, ClBundleIR, ClProgramIR, MemOp, MicroOp, NeuroOp};

/// Configuration options for the High-Level Lowering Engine.
#[derive(Debug, Clone)]
pub struct LoweringConfig {
    pub max_registers: u8,
    pub enable_4snake_schedule: bool,
    pub emit_return_instruction: bool,
    pub default_mode: char,
}

impl Default for LoweringConfig {
    fn default() -> Self {
        Self {
            max_registers: 16,
            enable_4snake_schedule: true,
            emit_return_instruction: true,
            default_mode: '$',
        }
    }
}

/// Register Allocator for High-Level Variables and Temporaries.
#[derive(Debug, Clone)]
pub struct RegisterAllocator {
    var_map: HashMap<String, u8>,
    next_reg: u8,
    max_reg: u8,
}

impl RegisterAllocator {
    pub fn new(max_reg: u8) -> Self {
        Self {
            var_map: HashMap::new(),
            next_reg: 0,
            max_reg,
        }
    }

    pub fn allocate_var(&mut self, name: &str) -> u8 {
        if let Some(&reg) = self.var_map.get(name) {
            return reg;
        }
        let reg = self.next_reg % self.max_reg;
        self.next_reg = (self.next_reg + 1) % self.max_reg;
        self.var_map.insert(name.to_string(), reg);
        reg
    }

    pub fn allocate_temp(&mut self) -> u8 {
        let reg = self.next_reg % self.max_reg;
        self.next_reg = (self.next_reg + 1) % self.max_reg;
        reg
    }

    pub fn get_reg(&self, name: &str) -> Option<u8> {
        self.var_map.get(name).copied()
    }
}

/// The High-Level to VLIW Lowering Engine.
pub struct HighLoweringEngine {
    config: LoweringConfig,
}

impl HighLoweringEngine {
    pub fn new(config: LoweringConfig) -> Self {
        Self { config }
    }

    /// Lower a high-level source string directly to textual CRON Machine Code (.cl).
    pub fn compile_high_source_to_cl(&self, source: &str) -> Result<String, String> {
        let module = parse_high_cl(source)?;
        self.lower_module_to_cl(&module)
    }

    /// Lower a HighModule AST into a complete textual .cl program.
    pub fn lower_module_to_cl(&self, module: &HighModule) -> Result<String, String> {
        let program_ir = self.lower_module_to_ir(module)?;
        let mut out = String::new();

        out.push_str(&format!("; ============================================================================\n"));
        out.push_str(&format!("; CRON & CL SOVEREIGN MACHINE CODE (Lowered from High-Level DSL)\n"));
        out.push_str(&format!("; Module: {}\n", module.name));
        out.push_str(&format!("; 128-Bit VLIW 4-Slot Architecture (4-Snake Zero-Bubble Scheduling)\n"));
        out.push_str(&format!("; ============================================================================\n\n"));

        // Emit stages as directives
        for stage in &module.stages {
            let mut param_strs = Vec::new();
            for (k, v) in &stage.params {
                param_strs.push(format!("{} = \"{}\"", k, v));
            }
            if param_strs.is_empty() {
                out.push_str(&format!(".stage \"{}\"\n", stage.name));
            } else {
                out.push_str(&format!(".stage \"{}\", {}\n", stage.name, param_strs.join(", ")));
            }
        }
        if !module.stages.is_empty() {
            out.push('\n');
        }

        // Emit bundles
        for bundle in &program_ir.bundles {
            out.push_str(&bundle.to_cl_line());
            out.push('\n');
        }

        Ok(out)
    }

    /// Lower a HighModule AST into canonical ClProgramIR.
    pub fn lower_module_to_ir(&self, module: &HighModule) -> Result<ClProgramIR, String> {
        let mut linear_ops = Vec::new();
        let mut reg_alloc = RegisterAllocator::new(self.config.max_registers);

        // Lower architectures and their methods
        for arch in &module.architectures {
            // Allocate states in registers
            for state in &arch.states {
                reg_alloc.allocate_var(&state.name);
            }
            for method in &arch.methods {
                self.lower_kernel(method, &mut reg_alloc, &mut linear_ops)?;
            }
        }

        // Lower standalone kernels
        for kernel in &module.kernels {
            self.lower_kernel(kernel, &mut reg_alloc, &mut linear_ops)?;
        }

        // Add return/halt if configured
        if self.config.emit_return_instruction && !linear_ops.is_empty() {
            linear_ops.push(MicroOp::Branch {
                cond: BranchCond::Return,
                reg: 0,
                target_bundle: 0,
            });
        }

        // Pack linear micro-ops into 128-bit 4-slot VLIW bundles using 4-Snake wave scheduling
        let bundles = self.pack_into_4snake_bundles(linear_ops);

        Ok(ClProgramIR {
            bundles,
            weights: Vec::new(),
            directives: Vec::new(),
        })
    }

    fn lower_kernel(
        &self,
        kernel: &KernelDecl,
        reg_alloc: &mut RegisterAllocator,
        ops: &mut Vec<MicroOp>,
    ) -> Result<(), String> {
        // Allocate parameters
        for (i, param) in kernel.params.iter().enumerate() {
            let reg = (i as u8) % self.config.max_registers;
            reg_alloc.var_map.insert(param.name.clone(), reg);
        }

        // Lower statements
        for stmt in &kernel.body {
            self.lower_stmt(stmt, reg_alloc, ops)?;
        }

        Ok(())
    }

    fn lower_stmt(
        &self,
        stmt: &Stmt,
        reg_alloc: &mut RegisterAllocator,
        ops: &mut Vec<MicroOp>,
    ) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, expr, .. } => {
                let dest_reg = reg_alloc.allocate_var(name);
                self.lower_expr(expr, dest_reg, reg_alloc, ops)?;
            }
            Stmt::Assign { target, expr } => {
                let dest_reg = match target {
                    Expr::Ident(id) => reg_alloc.allocate_var(id),
                    _ => reg_alloc.allocate_temp(),
                };
                self.lower_expr(expr, dest_reg, reg_alloc, ops)?;
            }
            Stmt::Return(maybe_expr) => {
                if let Some(expr) = maybe_expr {
                    // r0 is the standard return register
                    self.lower_expr(expr, 0, reg_alloc, ops)?;
                }
                ops.push(MicroOp::Branch {
                    cond: BranchCond::Return,
                    reg: 0,
                    target_bundle: 0,
                });
            }
            Stmt::ExprStmt(expr) => {
                let temp = reg_alloc.allocate_temp();
                self.lower_expr(expr, temp, reg_alloc, ops)?;
            }
            Stmt::RealtimeBlock { body, .. } => {
                // Realtime barrier before critical section
                ops.push(MicroOp::Sync {
                    is_barrier: true,
                    is_halt: false,
                    is_yield: false,
                });
                for s in body {
                    self.lower_stmt(s, reg_alloc, ops)?;
                }
                ops.push(MicroOp::Sync {
                    is_barrier: true,
                    is_halt: false,
                    is_yield: false,
                });
            }
            Stmt::AsyncTask { body, .. } => {
                ops.push(MicroOp::Sync {
                    is_barrier: false,
                    is_halt: false,
                    is_yield: true,
                });
                for s in body {
                    self.lower_stmt(s, reg_alloc, ops)?;
                }
            }
        }
        Ok(())
    }

    fn lower_expr(
        &self,
        expr: &Expr,
        dest_reg: u8,
        reg_alloc: &mut RegisterAllocator,
        ops: &mut Vec<MicroOp>,
    ) -> Result<u8, String> {
        match expr {
            Expr::Literal(lit) => {
                let val = match lit {
                    Literal::Int(i) => *i as u32,
                    Literal::Float(f) => (*f as f32).to_bits(),
                    Literal::Bool(b) => if *b { 1 } else { 0 },
                    Literal::String(_) => 0,
                };
                ops.push(MicroOp::ImmediateLoad { dest: dest_reg, val });
                Ok(dest_reg)
            }
            Expr::Ident(id) => {
                let src_reg = reg_alloc.allocate_var(id);
                if src_reg != dest_reg {
                    ops.push(MicroOp::Alu {
                        op: AluOp::Copy,
                        dest: dest_reg,
                        src: src_reg,
                        mode: self.config.default_mode,
                        imm: 0,
                    });
                }
                Ok(dest_reg)
            }
            Expr::UnaryOp { op, operand } => {
                let op_reg = self.lower_expr(operand, dest_reg, reg_alloc, ops)?;
                match op {
                    HighUnaryOp::Neg => {
                        ops.push(MicroOp::Alu {
                            op: AluOp::Sub,
                            dest: dest_reg,
                            src: op_reg,
                            mode: '-',
                            imm: 0,
                        });
                    }
                    HighUnaryOp::Not => {
                        ops.push(MicroOp::Alu {
                            op: AluOp::Xor,
                            dest: dest_reg,
                            src: op_reg,
                            mode: '^',
                            imm: 0xFFFF,
                        });
                    }
                    HighUnaryOp::RotorReverse => {
                        // Clifford GA Rotor Reversion ~R
                        ops.push(MicroOp::Neuromorphic {
                            op: NeuroOp::ReversibleInvert,
                            dest: dest_reg,
                            src: op_reg,
                        });
                    }
                }
                Ok(dest_reg)
            }
            Expr::BinaryOp { op, left, right } => {
                let left_reg = reg_alloc.allocate_temp();
                self.lower_expr(left, left_reg, reg_alloc, ops)?;

                let right_reg = reg_alloc.allocate_temp();
                self.lower_expr(right, right_reg, reg_alloc, ops)?;

                match op {
                    // --- The 7 Beyond-Addition Operators ---
                    HighBinaryOp::InterferenceCoupling => {
                        // <~> Photonic Phase Coupling (MZI Phase Shift & Unitary Multiply)
                        ops.push(MicroOp::Neuromorphic {
                            op: NeuroOp::OpticalGemm,
                            dest: dest_reg,
                            src: right_reg,
                        });
                    }
                    HighBinaryOp::AssociativeScan => {
                        // >>= Continuous State-Space Associative Scan (Mamba-2 SSD)
                        ops.push(MicroOp::Neuromorphic {
                            op: NeuroOp::SsmStep,
                            dest: dest_reg,
                            src: right_reg,
                        });
                    }
                    HighBinaryOp::LogSpaceGating => {
                        // <|> Log-Space Entropy Reduction (Softmax, MoE Router)
                        ops.push(MicroOp::Neuromorphic {
                            op: NeuroOp::Softmax,
                            dest: dest_reg,
                            src: right_reg,
                        });
                    }
                    HighBinaryOp::RingReduction => {
                        // <%> Polynomial Crypto-Ring Reduction (FHE Modulo, NTT)
                        ops.push(MicroOp::Alu {
                            op: AluOp::Mod,
                            dest: dest_reg,
                            src: right_reg,
                            mode: '%',
                            imm: 0,
                        });
                    }
                    HighBinaryOp::RadixGraft => {
                        // /@ Radix Tree Subtree Graft (Zero-Copy KV Cache Branching)
                        ops.push(MicroOp::Memory {
                            op: MemOp::BankStore,
                            dest: dest_reg,
                            src: right_reg,
                            offset: 0,
                            mode: '@',
                        });
                    }
                    HighBinaryOp::PeriodicResonance => {
                        // ~*~ Periodic Harmonic Resonance (Snake Activation, Waveform Physics)
                        ops.push(MicroOp::Neuromorphic {
                            op: NeuroOp::WaveResonance,
                            dest: dest_reg,
                            src: right_reg,
                        });
                    }
                    HighBinaryOp::ExteriorWedge => {
                        // ^ Clifford GA Exterior Wedge Product
                        ops.push(MicroOp::Alu {
                            op: AluOp::Xor,
                            dest: dest_reg,
                            src: right_reg,
                            mode: '^',
                            imm: 0,
                        });
                    }
                    HighBinaryOp::InnerScalar => {
                        // . Clifford Inner Scalar Product
                        ops.push(MicroOp::Alu {
                            op: AluOp::SubbyteTernaryDot,
                            dest: dest_reg,
                            src: right_reg,
                            mode: '.',
                            imm: 0,
                        });
                    }

                    // --- Classical Arithmetic Operators ---
                    HighBinaryOp::Add => {
                        ops.push(MicroOp::Alu {
                            op: AluOp::Add,
                            dest: dest_reg,
                            src: right_reg,
                            mode: '+',
                            imm: 0,
                        });
                    }
                    HighBinaryOp::Sub => {
                        ops.push(MicroOp::Alu {
                            op: AluOp::Sub,
                            dest: dest_reg,
                            src: right_reg,
                            mode: '-',
                            imm: 0,
                        });
                    }
                    HighBinaryOp::Mul => {
                        ops.push(MicroOp::Alu {
                            op: AluOp::Mul,
                            dest: dest_reg,
                            src: right_reg,
                            mode: '*',
                            imm: 0,
                        });
                    }
                    HighBinaryOp::Div => {
                        ops.push(MicroOp::Alu {
                            op: AluOp::Div,
                            dest: dest_reg,
                            src: right_reg,
                            mode: '/',
                            imm: 0,
                        });
                    }
                    HighBinaryOp::Mod => {
                        ops.push(MicroOp::Alu {
                            op: AluOp::Mod,
                            dest: dest_reg,
                            src: right_reg,
                            mode: '%',
                            imm: 0,
                        });
                    }
                    HighBinaryOp::Eq | HighBinaryOp::Neq | HighBinaryOp::Lt | HighBinaryOp::Gt | HighBinaryOp::Lte | HighBinaryOp::Gte => {
                        ops.push(MicroOp::Alu {
                            op: AluOp::Compare,
                            dest: dest_reg,
                            src: right_reg,
                            mode: '?',
                            imm: 0,
                        });
                    }
                }
                Ok(dest_reg)
            }
            Expr::PrimitiveCall { op_name, positional_args, named_args } => {
                self.lower_primitive_call(op_name, positional_args, named_args, dest_reg, reg_alloc, ops)
            }
            Expr::TensorAccess { target, indices } => {
                let base_reg = self.lower_expr(target, dest_reg, reg_alloc, ops)?;
                let mut offset = 0u32;
                if let Some(first_idx) = indices.first() {
                    if let Expr::Literal(Literal::Int(idx)) = first_idx {
                        offset = *idx as u32;
                    }
                }
                ops.push(MicroOp::Memory {
                    op: MemOp::LoadWord,
                    dest: dest_reg,
                    src: base_reg,
                    offset,
                    mode: '$',
                });
                Ok(dest_reg)
            }
            Expr::Tuple(elements) => {
                if let Some(first) = elements.first() {
                    self.lower_expr(first, dest_reg, reg_alloc, ops)?;
                }
                Ok(dest_reg)
            }
            Expr::MethodCall { target, method: _, args } => {
                let target_reg = self.lower_expr(target, dest_reg, reg_alloc, ops)?;
                for arg in args {
                    let arg_reg = reg_alloc.allocate_temp();
                    self.lower_expr(arg, arg_reg, reg_alloc, ops)?;
                }
                ops.push(MicroOp::Branch {
                    cond: BranchCond::Call,
                    reg: target_reg,
                    target_bundle: 0,
                });
                Ok(dest_reg)
            }
        }
    }

    fn lower_primitive_call(
        &self,
        op_name: &str,
        positional_args: &[Expr],
        named_args: &[(String, Expr)],
        dest_reg: u8,
        reg_alloc: &mut RegisterAllocator,
        ops: &mut Vec<MicroOp>,
    ) -> Result<u8, String> {
        let mut arg_regs = Vec::new();
        for arg in positional_args {
            let r = reg_alloc.allocate_temp();
            self.lower_expr(arg, r, reg_alloc, ops)?;
            arg_regs.push(r);
        }
        for (_name, arg) in named_args {
            let r = reg_alloc.allocate_temp();
            self.lower_expr(arg, r, reg_alloc, ops)?;
            arg_regs.push(r);
        }

        let primary_src = arg_regs.first().copied().unwrap_or(0);
        let secondary_src = arg_regs.get(1).copied().unwrap_or(0);

        match op_name {
            // High-Performance Silicon GEMM Primitives
            "nvfp4_gemm" | "wgmma_async_16x16" | "bitnet_gemm" => {
                ops.push(MicroOp::Alu {
                    op: AluOp::TileTranspose,
                    dest: dest_reg,
                    src: primary_src,
                    mode: 'T',
                    imm: 0,
                });
                ops.push(MicroOp::Alu {
                    op: AluOp::SubbyteTernaryDot,
                    dest: dest_reg,
                    src: secondary_src,
                    mode: 'D',
                    imm: 0,
                });
            }
            // State-Space & Recurrent Evolution
            "mamba2_ssd" | "selective_scan" => {
                ops.push(MicroOp::Neuromorphic {
                    op: NeuroOp::SsmStep,
                    dest: dest_reg,
                    src: primary_src,
                });
            }
            // Attention & Softmax
            "flash_attention" | "flash_attention_3" => {
                ops.push(MicroOp::Neuromorphic {
                    op: NeuroOp::Softmax,
                    dest: dest_reg,
                    src: primary_src,
                });
                ops.push(MicroOp::Neuromorphic {
                    op: NeuroOp::OpticalGemm,
                    dest: dest_reg,
                    src: secondary_src,
                });
            }
            // Normalization & Gating
            "rmsnorm" => {
                ops.push(MicroOp::Neuromorphic {
                    op: NeuroOp::RmsNorm,
                    dest: dest_reg,
                    src: primary_src,
                });
            }
            "silu" | "swiglu" => {
                ops.push(MicroOp::Neuromorphic {
                    op: NeuroOp::Silu,
                    dest: dest_reg,
                    src: primary_src,
                });
            }
            "gelu" => {
                ops.push(MicroOp::Neuromorphic {
                    op: NeuroOp::Gelu,
                    dest: dest_reg,
                    src: primary_src,
                });
            }
            // Rotary & Clifford Trigonometrics
            "rope" | "cordic_rotate" => {
                ops.push(MicroOp::Alu {
                    op: AluOp::CordicRotate,
                    dest: dest_reg,
                    src: primary_src,
                    mode: 'C',
                    imm: 0,
                });
            }
            "nequip_clebsch_gordan" => {
                ops.push(MicroOp::Neuromorphic {
                    op: NeuroOp::CurvedDeflect,
                    dest: dest_reg,
                    src: primary_src,
                });
                ops.push(MicroOp::Neuromorphic {
                    op: NeuroOp::WaveResonance,
                    dest: dest_reg,
                    src: secondary_src,
                });
            }
            // Numerical Integration / ODE
            "ode_dormand_prince" | "ode_step" => {
                ops.push(MicroOp::Alu {
                    op: AluOp::FixedPoint,
                    dest: dest_reg,
                    src: primary_src,
                    mode: 'F',
                    imm: 0,
                });
                ops.push(MicroOp::Alu {
                    op: AluOp::Add,
                    dest: dest_reg,
                    src: secondary_src,
                    mode: '+',
                    imm: 0,
                });
            }
            // Universal fallback primitive call
            _ => {
                ops.push(MicroOp::Neuromorphic {
                    op: NeuroOp::LayerTransition,
                    dest: dest_reg,
                    src: primary_src,
                });
            }
        }

        Ok(dest_reg)
    }

    /// Pack a sequence of micro-ops into 128-bit 4-slot VLIW bundles (4-Snake X, Y, Z, W).
    fn pack_into_4snake_bundles(&self, ops: Vec<MicroOp>) -> Vec<ClBundleIR> {
        let mut bundles = Vec::new();
        let mut current_slot = 0;
        let mut current_bundle_ops = [
            MicroOp::Nop,
            MicroOp::Nop,
            MicroOp::Nop,
            MicroOp::Nop,
        ];
        let mut cycle = 0;

        for op in ops {
            current_bundle_ops[current_slot] = op;
            current_slot += 1;

            if current_slot == 4 {
                bundles.push(ClBundleIR::new(cycle, current_bundle_ops));
                current_bundle_ops = [
                    MicroOp::Nop,
                    MicroOp::Nop,
                    MicroOp::Nop,
                    MicroOp::Nop,
                ];
                current_slot = 0;
                cycle += 1;
            }
        }

        // Flush partially-filled bundle with NOPs
        if current_slot > 0 {
            bundles.push(ClBundleIR::new(cycle, current_bundle_ops));
        }

        bundles
    }
}

/// Convenience entry point to compile high-level CL code directly to textual .cl.
pub fn compile_high_cl_to_cl(source: &str) -> Result<String, String> {
    let engine = HighLoweringEngine::new(LoweringConfig::default());
    engine.compile_high_source_to_cl(source)
}
