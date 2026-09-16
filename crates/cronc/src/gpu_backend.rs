// ============================================================================
// CRON Heterogeneous GPU Accelerator Backend
// Translates high-level CRON (.cr) programs into:
// 1. NVIDIA CUDA PTX v7.5+ ISA (.ptx) targeting sm_80..sm_90 architectures.
// 2. Apple Metal Shading Language v3.1 (.metal) for Apple Silicon GPU/Neural engines.
// ============================================================================

use crate::ast::*;
use std::collections::HashMap;

pub struct GpuBackend {
    buffer: String,
    reg_i32_count: usize,
    reg_i64_count: usize,
    reg_f32_count: usize,
    reg_pred_count: usize,
    label_count: usize,
    var_map: HashMap<String, (String, GpuType)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuType {
    I32,
    I64,
    F32,
    F64,
    Vec4F,
    Ptr,
    Bool,
}

impl GpuType {
    pub fn ptx_type(&self) -> &'static str {
        match self {
            GpuType::I32 => ".s32",
            GpuType::I64 => ".s64",
            GpuType::F32 => ".f32",
            GpuType::F64 => ".f64",
            GpuType::Vec4F => ".v4.f32",
            GpuType::Ptr => ".u64",
            GpuType::Bool => ".pred",
        }
    }

    pub fn metal_type(&self) -> &'static str {
        match self {
            GpuType::I32 => "int",
            GpuType::I64 => "long",
            GpuType::F32 => "float",
            GpuType::F64 => "double",
            GpuType::Vec4F => "float4",
            GpuType::Ptr => "device float*",
            GpuType::Bool => "bool",
        }
    }
}

impl Default for GpuBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuBackend {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(8192),
            reg_i32_count: 1,
            reg_i64_count: 1,
            reg_f32_count: 1,
            reg_pred_count: 1,
            label_count: 1,
            var_map: HashMap::new(),
        }
    }

    fn new_i32_reg(&mut self) -> String {
        let r = format!("%r{}", self.reg_i32_count);
        self.reg_i32_count += 1;
        r
    }

    fn new_i64_reg(&mut self) -> String {
        let r = format!("%rd{}", self.reg_i64_count);
        self.reg_i64_count += 1;
        r
    }

    fn new_f32_reg(&mut self) -> String {
        let r = format!("%f{}", self.reg_f32_count);
        self.reg_f32_count += 1;
        r
    }

    fn new_pred_reg(&mut self) -> String {
        let r = format!("%p{}", self.reg_pred_count);
        self.reg_pred_count += 1;
        r
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let l = format!("L_{}_{}", prefix, self.label_count);
        self.label_count += 1;
        l
    }

    fn emit_line(&mut self, s: &str) {
        if !s.starts_with('.') && !s.ends_with(':') && !s.starts_with("//") && !s.is_empty() {
            self.buffer.push_str("    ");
        }
        self.buffer.push_str(s);
        self.buffer.push('\n');
    }

    // ========================================================================
    // NVIDIA CUDA PTX Assembly Generation
    // ========================================================================

    pub fn generate_ptx(&mut self, program: &Program) -> String {
        self.buffer.clear();
        self.emit_line("// ============================================================================");
        self.emit_line("// CRON NVIDIA CUDA PTX Accelerator Backend v1.0");
        self.emit_line("// Target Architecture: NVIDIA sm_80 / sm_90 (Ampere / Hopper / Blackwell)");
        self.emit_line("// Address Size: 64-bit Unified Virtual Memory (UVM)");
        self.emit_line("// ============================================================================\n");

        self.emit_line(".version 7.5");
        self.emit_line(".target sm_80");
        self.emit_line(".address_size 64\n");

        // Generate PTX kernels for each function
        for func in &program.functions {
            self.generate_ptx_function(func);
        }

        // If program has main_statements, generate a default main compute kernel
        if !program.main_statements.is_empty() {
            self.generate_ptx_main_kernel(&program.main_statements);
        }

        self.buffer.clone()
    }

    fn generate_ptx_function(&mut self, func: &FunctionDecl) {
        self.var_map.clear();
        self.reg_i32_count = 1;
        self.reg_i64_count = 1;
        self.reg_f32_count = 1;
        self.reg_pred_count = 1;

        let is_kernel = func.name.ends_with("_kernel") || func.is_export || func.name == "main";
        let entry_prefix = if is_kernel { ".visible .entry" } else { ".visible .func" };

        self.emit_line(&format!("// Kernel: {}", func.name));
        self.emit_line(&format!("{} {} (", entry_prefix, func.name));

        for (i, param) in func.params.iter().enumerate() {
            let is_last = i == func.params.len() - 1;
            let p_type = if param.param_type.starts_with('[') || param.param_type.contains('*') || param.param_type.starts_with("vec") {
                ".param .u64"
            } else if param.param_type == "f32" || param.param_type == "float" {
                ".param .f32"
            } else {
                ".param .u32"
            };
            let comma = if is_last { "" } else { "," };
            self.emit_line(&format!("    {} param_{}{}", p_type, param.name, comma));
        }
        self.emit_line(")\n{");

        // Reserve register banks
        self.emit_line(".reg .pred %p<16>;");
        self.emit_line(".reg .b32  %r<64>;");
        self.emit_line(".reg .b64  %rd<64>;");
        self.emit_line(".reg .f32  %f<64>;\n");

        // Load parameters
        for param in &func.params {
            if param.param_type.starts_with('[') || param.param_type.contains('*') || param.param_type.starts_with("vec") {
                let rd = self.new_i64_reg();
                self.emit_line(&format!("ld.param.u64 {}, [param_{}];", rd, param.name));
                self.var_map.insert(param.name.clone(), (rd, GpuType::Ptr));
            } else if param.param_type == "f32" || param.param_type == "float" {
                let f = self.new_f32_reg();
                self.emit_line(&format!("ld.param.f32 {}, [param_{}];", f, param.name));
                self.var_map.insert(param.name.clone(), (f, GpuType::F32));
            } else {
                let r = self.new_i32_reg();
                self.emit_line(&format!("ld.param.u32 {}, [param_{}];", r, param.name));
                self.var_map.insert(param.name.clone(), (r, GpuType::I32));
            }
        }

        // Global Thread ID Calculation
        let tid_x = self.new_i32_reg();
        let ctaid_x = self.new_i32_reg();
        let ntid_x = self.new_i32_reg();
        let gid = self.new_i32_reg();

        self.emit_line("\n    // Compute global thread ID: gid = ctaid.x * ntid.x + tid.x");
        self.emit_line(&format!("mov.u32 {}, %tid.x;", tid_x));
        self.emit_line(&format!("mov.u32 {}, %ctaid.x;", ctaid_x));
        self.emit_line(&format!("mov.u32 {}, %ntid.x;", ntid_x));
        self.emit_line(&format!("mad.lo.u32 {}, {}, {}, {};", gid, ctaid_x, ntid_x, tid_x));
        self.var_map.insert("global_tid".to_string(), (gid, GpuType::I32));

        // Generate body statements
        for stmt in &func.body {
            self.generate_ptx_statement(stmt);
        }

        self.emit_line("    ret;");
        self.emit_line("}\n");
    }

    fn generate_ptx_main_kernel(&mut self, stmts: &[Statement]) {
        self.emit_line("// Default Standalone Main Compute Kernel");
        self.emit_line(".visible .entry cron_main_kernel (");
        self.emit_line("    .param .u64 param_d_out,");
        self.emit_line("    .param .u32 param_n");
        self.emit_line(")\n{");
        self.emit_line(".reg .pred %p<16>;");
        self.emit_line(".reg .b32  %r<64>;");
        self.emit_line(".reg .b64  %rd<64>;");
        self.emit_line(".reg .f32  %f<64>;\n");

        for stmt in stmts {
            self.generate_ptx_statement(stmt);
        }

        self.emit_line("    ret;");
        self.emit_line("}\n");
    }

    fn generate_ptx_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let { name, value, .. } => {
                let (reg, val_type) = self.generate_ptx_expr(value);
                self.var_map.insert(name.clone(), (reg, val_type));
            }
            Statement::Assign { target, value, .. } => {
                let (src_reg, val_type) = self.generate_ptx_expr(value);
                if let Some((dst_reg, dst_type)) = self.var_map.get(target).cloned() {
                    match dst_type {
                        GpuType::F32 => self.emit_line(&format!("mov.f32 {}, {};", dst_reg, src_reg)),
                        GpuType::I32 => self.emit_line(&format!("mov.b32 {}, {};", dst_reg, src_reg)),
                        GpuType::Ptr | GpuType::I64 => self.emit_line(&format!("mov.b64 {}, {};", dst_reg, src_reg)),
                        _ => self.emit_line(&format!("mov.b32 {}, {};", dst_reg, src_reg)),
                    }
                } else {
                    self.var_map.insert(target.clone(), (src_reg, val_type));
                }
            }
            Statement::While { condition, body, .. } => {
                let start_lbl = self.new_label("while_start");
                let end_lbl = self.new_label("while_end");

                self.emit_line(&format!("{}:", start_lbl));
                let (cond_reg, _) = self.generate_ptx_expr(condition);
                let pred = self.new_pred_reg();
                self.emit_line(&format!("setp.eq.s32 {}, {}, 0;", pred, cond_reg));
                self.emit_line(&format!("@{} bra {};", pred, end_lbl));

                for s in body {
                    self.generate_ptx_statement(s);
                }

                self.emit_line(&format!("bra {};", start_lbl));
                self.emit_line(&format!("{}:", end_lbl));
            }
            Statement::If { condition, then_body, else_body, .. } => {
                let else_lbl = self.new_label("if_else");
                let end_lbl = self.new_label("if_end");

                let (cond_reg, _) = self.generate_ptx_expr(condition);
                let pred = self.new_pred_reg();
                self.emit_line(&format!("setp.eq.s32 {}, {}, 0;", pred, cond_reg));
                self.emit_line(&format!("@{} bra {};", pred, else_lbl));

                for s in then_body {
                    self.generate_ptx_statement(s);
                }

                if else_body.is_some() {
                    self.emit_line(&format!("bra {};", end_lbl));
                }
                self.emit_line(&format!("{}:", else_lbl));

                if let Some(eb) = else_body {
                    for s in eb {
                        self.generate_ptx_statement(s);
                    }
                    self.emit_line(&format!("{}:", end_lbl));
                }
            }
            Statement::Return(Some(expr)) => {
                let (reg, r_type) = self.generate_ptx_expr(expr);
                match r_type {
                    GpuType::F32 => self.emit_line(&format!("// return %f: {}", reg)),
                    _ => self.emit_line(&format!("// return %r: {}", reg)),
                }
                self.emit_line("    ret;");
            }
            Statement::Return(None) => {
                self.emit_line("    ret;");
            }
            Statement::Expr(e) => {
                let _ = self.generate_ptx_expr(e);
            }
            _ => {}
        }
    }

    fn generate_ptx_expr(&mut self, expr: &Expr) -> (String, GpuType) {
        match expr {
            Expr::LiteralInt(val) => {
                let r = self.new_i32_reg();
                self.emit_line(&format!("mov.u32 {}, {};", r, val));
                (r, GpuType::I32)
            }
            Expr::LiteralHex(val) => {
                let r = self.new_i64_reg();
                self.emit_line(&format!("mov.u64 {}, 0x{:X};", r, val));
                (r, GpuType::I64)
            }
            Expr::LiteralFloat(val) => {
                let f = self.new_f32_reg();
                self.emit_line(&format!("mov.f32 {}, 0f{:08X};", f, (*val as f32).to_bits()));
                (f, GpuType::F32)
            }
            Expr::LiteralBool(val) => {
                let r = self.new_i32_reg();
                self.emit_line(&format!("mov.u32 {}, {};", r, if *val { 1 } else { 0 }));
                (r, GpuType::I32)
            }
            Expr::Ident(name, _) => {
                if let Some((reg, gtype)) = self.var_map.get(name).cloned() {
                    (reg, gtype)
                } else {
                    let r = self.new_i32_reg();
                    (r, GpuType::I32)
                }
            }
            Expr::Binary { op, left, right } => {
                let (lr, lt) = self.generate_ptx_expr(left);
                let (rr, rt) = self.generate_ptx_expr(right);

                let is_float = lt == GpuType::F32 || rt == GpuType::F32;
                if is_float {
                    let dst = self.new_f32_reg();
                    match op.as_str() {
                        "+" => self.emit_line(&format!("add.f32 {}, {}, {};", dst, lr, rr)),
                        "-" => self.emit_line(&format!("sub.f32 {}, {}, {};", dst, lr, rr)),
                        "*" => self.emit_line(&format!("mul.f32 {}, {}, {};", dst, lr, rr)),
                        "/" => self.emit_line(&format!("div.rn.f32 {}, {}, {};", dst, lr, rr)),
                        _ => self.emit_line(&format!("add.f32 {}, {}, {};", dst, lr, rr)),
                    }
                    (dst, GpuType::F32)
                } else {
                    let dst = self.new_i32_reg();
                    match op.as_str() {
                        "+" => self.emit_line(&format!("add.s32 {}, {}, {};", dst, lr, rr)),
                        "-" => self.emit_line(&format!("sub.s32 {}, {}, {};", dst, lr, rr)),
                        "*" => self.emit_line(&format!("mul.lo.s32 {}, {}, {};", dst, lr, rr)),
                        "/" => self.emit_line(&format!("div.s32 {}, {}, {};", dst, lr, rr)),
                        "<" => {
                            let pred = self.new_pred_reg();
                            self.emit_line(&format!("setp.lt.s32 {}, {}, {};", pred, lr, rr));
                            self.emit_line(&format!("selp.u32 {}, 1, 0, {};", dst, pred));
                        }
                        ">" => {
                            let pred = self.new_pred_reg();
                            self.emit_line(&format!("setp.gt.s32 {}, {}, {};", pred, lr, rr));
                            self.emit_line(&format!("selp.u32 {}, 1, 0, {};", dst, pred));
                        }
                        "==" => {
                            let pred = self.new_pred_reg();
                            self.emit_line(&format!("setp.eq.s32 {}, {}, {};", pred, lr, rr));
                            self.emit_line(&format!("selp.u32 {}, 1, 0, {};", dst, pred));
                        }
                        _ => self.emit_line(&format!("add.s32 {}, {}, {};", dst, lr, rr)),
                    }
                    (dst, GpuType::I32)
                }
            }
            Expr::Call { callee, args }
                // Check if calling built-in FMA intrinsic: fma(a, b, c) -> fma.rn.f32
                if callee == "fma" && args.len() == 3 => {
                    let (ra, _) = self.generate_ptx_expr(&args[0].value);
                    let (rb, _) = self.generate_ptx_expr(&args[1].value);
                    let (rc, _) = self.generate_ptx_expr(&args[2].value);
                    let dst = self.new_f32_reg();
                    self.emit_line(&format!("fma.rn.f32 {}, {}, {}, {};", dst, ra, rb, rc));
                    (dst, GpuType::F32)
                }
            _ => {
                let r = self.new_i32_reg();
                (r, GpuType::I32)
            }
        }
    }

    // ========================================================================
    // Apple Metal Shading Language (MSL) Generation
    // ========================================================================

    pub fn generate_metal(&mut self, program: &Program) -> String {
        self.buffer.clear();
        self.emit_line("// ============================================================================");
        self.emit_line("// CRON Apple Metal Shading Language (MSL) Accelerator Backend v1.0");
        self.emit_line("// Target: Apple Silicon M-Series Unified Memory GPU Fabric");
        self.emit_line("// ============================================================================\n");

        self.emit_line("#include <metal_stdlib>");
        self.emit_line("#include <simd/simd.h>");
        self.emit_line("using namespace metal;\n");

        for func in &program.functions {
            self.generate_metal_function(func);
        }

        if !program.main_statements.is_empty() {
            self.generate_metal_main_kernel(&program.main_statements);
        }

        self.buffer.clone()
    }

    fn generate_metal_function(&mut self, func: &FunctionDecl) {
        let is_kernel = func.name.ends_with("_kernel") || func.is_export || func.name == "main";
        if is_kernel {
            self.emit_line(&format!("kernel void {}(", func.name));
            for (i, param) in func.params.iter().enumerate() {
                let m_type = if param.param_type.starts_with('[') || param.param_type.contains('*') || param.param_type.starts_with("vec") {
                    "device float*"
                } else if param.param_type == "f32" || param.param_type == "float" {
                    "constant float&"
                } else {
                    "constant int&"
                };
                self.emit_line(&format!("    {} {} [[buffer({})]],", m_type, param.name, i));
            }
            self.emit_line("    uint gid [[thread_position_in_grid]]");
            self.emit_line(") {");

            for stmt in &func.body {
                self.generate_metal_statement(stmt);
            }

            self.emit_line("}\n");
        } else {
            let ret_type = func.return_type.as_deref().unwrap_or("void");
            let m_ret = match ret_type {
                "f32" | "float" => "float",
                "i32" | "int" => "int",
                "bool" => "bool",
                _ => "void",
            };
            self.emit_line(&format!("{} {}(", m_ret, func.name));
            for (i, param) in func.params.iter().enumerate() {
                let is_last = i == func.params.len() - 1;
                let comma = if is_last { "" } else { ", " };
                let m_type = if param.param_type == "f32" { "float" } else { "int" };
                self.emit_line(&format!("    {} {}{}", m_type, param.name, comma));
            }
            self.emit_line(") {");

            for stmt in &func.body {
                self.generate_metal_statement(stmt);
            }

            self.emit_line("}\n");
        }
    }

    fn generate_metal_main_kernel(&mut self, stmts: &[Statement]) {
        self.emit_line("kernel void cron_main_kernel(");
        self.emit_line("    device float* d_out [[buffer(0)]],");
        self.emit_line("    constant uint& n [[buffer(1)]],");
        self.emit_line("    uint gid [[thread_position_in_grid]]");
        self.emit_line(") {");
        self.emit_line("    if (gid >= n) return;\n");

        for stmt in stmts {
            self.generate_metal_statement(stmt);
        }

        self.emit_line("}\n");
    }

    fn generate_metal_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let { name, value, .. } => {
                let val_str = self.generate_metal_expr(value);
                self.emit_line(&format!("auto {} = {};", name, val_str));
            }
            Statement::Assign { target, value, .. } => {
                let val_str = self.generate_metal_expr(value);
                self.emit_line(&format!("{} = {};", target, val_str));
            }
            Statement::While { condition, body, .. } => {
                let cond_str = self.generate_metal_expr(condition);
                self.emit_line(&format!("while ({}) {{", cond_str));
                for s in body {
                    self.generate_metal_statement(s);
                }
                self.emit_line("}");
            }
            Statement::If { condition, then_body, else_body, .. } => {
                let cond_str = self.generate_metal_expr(condition);
                self.emit_line(&format!("if ({}) {{", cond_str));
                for s in then_body {
                    self.generate_metal_statement(s);
                }
                if let Some(eb) = else_body {
                    self.emit_line("} else {");
                    for s in eb {
                        self.generate_metal_statement(s);
                    }
                }
                self.emit_line("}");
            }
            Statement::Return(Some(expr)) => {
                let expr_str = self.generate_metal_expr(expr);
                self.emit_line(&format!("return {};", expr_str));
            }
            Statement::Return(None) => {
                self.emit_line("return;");
            }
            Statement::Expr(e) => {
                let expr_str = self.generate_metal_expr(e);
                self.emit_line(&format!("{};", expr_str));
            }
            _ => {}
        }
    }

    fn generate_metal_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::LiteralInt(val) => val.to_string(),
            Expr::LiteralHex(val) => format!("0x{:X}", val),
            Expr::LiteralFloat(val) => format!("{}f", val),
            Expr::LiteralBool(val) => val.to_string(),
            Expr::Ident(name, _) => name.clone(),
            Expr::Binary { op, left, right } => {
                let l = self.generate_metal_expr(left);
                let r = self.generate_metal_expr(right);
                format!("({} {} {})", l, op, r)
            }
            Expr::Call { callee, args } => {
                let arg_strs: Vec<String> = args.iter().map(|a| self.generate_metal_expr(&a.value)).collect();
                if callee == "fma" && arg_strs.len() == 3 {
                    format!("fma({}, {}, {})", arg_strs[0], arg_strs[1], arg_strs[2])
                } else {
                    format!("{}({})", callee, arg_strs.join(", "))
                }
            }
            _ => "0".to_string(),
        }
    }
}
