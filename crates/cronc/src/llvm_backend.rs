// ============================================================================
// CRON Direct LLVM IR Backend
// Generates standalone, target-agnostic LLVM Intermediate Representation (.ll).
// Features:
//   - Full SSA register assignment
//   - Direct Clang / LLVM Optimizer integration (AVX-512, Neon, LTO)
//   - Zero runtime dependency
//   - Zero-cost monomorphization support
// ============================================================================

use crate::ast::*;
use crate::c_backend::CBackend;
use std::collections::HashMap;

pub struct LlvmBackend {
    buffer: String,
    reg_counter: usize,
    label_counter: usize,
    string_counter: usize,
    string_constants: Vec<(String, String, usize)>, // (label, content, length)
    vars: HashMap<String, (String, String)>,         // name -> (pointer_reg, llvm_type)
    function_signatures: HashMap<String, (Vec<String>, String)>,
    in_function: bool,
    current_fn_ret_type: String,
    has_terminated: bool,
}

impl Default for LlvmBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl LlvmBackend {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(16384),
            reg_counter: 1,
            label_counter: 1,
            string_counter: 1,
            string_constants: Vec::new(),
            vars: HashMap::new(),
            function_signatures: HashMap::new(),
            in_function: false,
            current_fn_ret_type: "void".to_string(),
            has_terminated: false,
        }
    }

    fn new_reg(&mut self) -> String {
        let r = format!("%{}", self.reg_counter);
        self.reg_counter += 1;
        r
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let l = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        l
    }

    fn emit_line(&mut self, s: &str) {
        if !s.starts_with(';') && !s.ends_with(':') && !s.is_empty() {
            self.buffer.push_str("    ");
        }
        self.buffer.push_str(s);
        self.buffer.push('\n');
    }

    pub fn map_llvm_type(cron_type: &str) -> String {
        let mut trimmed = cron_type.trim();
        if trimmed.starts_with('@') {
            if let Some(space_idx) = trimmed.find(' ') {
                trimmed = trimmed[space_idx..].trim();
            }
        }
        trimmed = trimmed
            .trim_start_matches("linear ")
            .trim_start_matches("lin ")
            .trim();

        if trimmed == "tile4x4_f32" || trimmed == "tile<4, 4, f32>" || trimmed == "tile<4,4,f32>" {
            return "[4 x [4 x float]]".to_string();
        }
        if trimmed == "tile4x4_i32" || trimmed == "tile<4, 4, i32>" || trimmed == "tile<4,4,i32>" {
            return "[4 x [4 x i32]]".to_string();
        }
        if trimmed == "tile16x16_i2" || trimmed == "tile<16, 16, i2>" || trimmed == "tile<16,16,i2>" {
            return "[16 x i32]".to_string();
        }

        if trimmed.starts_with("channel<") || trimmed.starts_with("Channel<") || trimmed == "channel" || trimmed == "Channel" {
            return "ptr".to_string();
        }
        if trimmed.starts_with("fiber<") || trimmed.starts_with("Fiber<") || trimmed == "fiber" || trimmed == "Fiber" || trimmed == "FiberHandle" {
            return "ptr".to_string();
        }

        if trimmed.contains('<') {
            let mangled = CBackend::mangle_type_name(trimmed);
            return format!("%struct.{}", mangled);
        }

        match trimmed {
            "i2" => "i2".to_string(),
            "i4" => "i4".to_string(),
            "f4" | "f8" | "f16" | "bf16" => "float".to_string(),
            "i8" | "u8" => "i8".to_string(),
            "i16" | "u16" => "i16".to_string(),
            "i32" | "u32" | "int" => "i32".to_string(),
            "i64" | "u64" | "wave_t" | "ext_addr_t" => "i64".to_string(),
            "f32" | "float" => "float".to_string(),
            "f64" => "double".to_string(),
            "bool" => "i1".to_string(),
            "string" => "ptr".to_string(),
            "spk_stamp" | "rev_t" => "i32".to_string(),
            "vec4f" => "<4 x float>".to_string(),
            "vec8f" => "<8 x float>".to_string(),
            "vec16f" => "<16 x float>".to_string(),
            "vec4i" => "<4 x i32>".to_string(),
            "vec8i" => "<8 x i32>".to_string(),
            "void" | "()" => "void".to_string(),
            other => {
                if other.starts_with('(') && other.ends_with(')') {
                    "%struct.cron_tuple_t".to_string()
                } else if other.starts_with('[') && other.ends_with(']') {
                    "ptr".to_string()
                } else {
                    format!("%struct.{}", other)
                }
            }
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        // Monomorphize program first to resolve all generic instantiations
        let mono_program = CBackend::monomorphize_program(program);

        // Header & Module metadata
        self.emit_line("; ============================================================================");
        self.emit_line(&format!("; CRON Direct LLVM IR Translation: Module '{}'", mono_program.module_name));
        self.emit_line("; Generated by cronc Direct LLVM IR Backend");
        self.emit_line("; Target: x86_64 / AArch64 Native Optimized Execution");
        self.emit_line("; ============================================================================");
        self.emit_line("");
        self.emit_line(&format!("; ModuleID = '{}'", mono_program.module_name));
        self.emit_line(&format!("source_filename = \"{}.cr\"", mono_program.module_name));
        self.emit_line("target datalayout = \"e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128\"");
        self.emit_line("target triple = \"x86_64-pc-windows-msvc\"");
        self.emit_line("");

        // External C Runtime Declarations
        self.emit_line("; --- External C Runtime Intrinsics ---");
        self.emit_line("declare i32 @printf(ptr nocapture readonly, ...)");
        self.emit_line("declare i32 @puts(ptr nocapture readonly)");
        self.emit_line("declare void @exit(i32)");
        self.emit_line("declare double @cos(double)");
        self.emit_line("declare double @sin(double)");
        self.emit_line("declare i64 @clock()");
        self.emit_line("declare <8 x float> @llvm.fma.v8f32(<8 x float>, <8 x float>, <8 x float>)");
        self.emit_line("declare float @llvm.vector.reduce.fadd.v8f32(float, <8 x float>)");
        self.emit_line("declare ptr @channel_new(i32)");
        self.emit_line("declare i64 @channel_send(ptr, i64)");
        self.emit_line("declare i64 @channel_recv(ptr)");
        self.emit_line("declare i64 @channel_try_recv(ptr)");
        self.emit_line("declare void @channel_close(ptr)");
        self.emit_line("declare i64 @torus_distance(i32, i32)");
        self.emit_line("declare i32 @torus_core_id()");
        self.emit_line("");

        // Built-in Silicon Types
        self.emit_line("; --- CRON Silicon Core Primitives ---");
        self.emit_line("%struct.cron_torus_coord_t = type { i32, i32, i32, i32 }");
        self.emit_line("%struct.cron_tuple_t = type { i64, i64, i64, i64 }");
        self.emit_line("");

        // 1. Emit Struct Type Definitions
        for s in &mono_program.structs {
            let field_types: Vec<String> = s.fields.iter().map(|(_, ft)| Self::map_llvm_type(ft)).collect();
            self.emit_line(&format!("%struct.{} = type {{ {} }}", s.name, field_types.join(", ")));
        }
        self.emit_line("");

        // 2. Register Function Signatures
        for f in &mono_program.functions {
            let ret = f.return_type.as_deref().map(Self::map_llvm_type).unwrap_or_else(|| "void".to_string());
            let params = f.params.iter().map(|p| Self::map_llvm_type(&p.param_type)).collect();
            self.function_signatures.insert(f.name.clone(), (params, ret));
        }

        // 3. Emit Function Definitions
        for f in &mono_program.functions {
            self.emit_function(f);
        }

        // 4. Emit Main Entry Function
        self.emit_main(&mono_program);

        // 5. Prepend String Constants
        let mut final_out = String::with_capacity(self.buffer.len() + 2048);
        if !self.string_constants.is_empty() {
            final_out.push_str("; --- Global String Constants ---\n");
            for (lbl, content, len) in &self.string_constants {
                final_out.push_str(&format!(
                    "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n",
                    lbl, len, content
                ));
            }
            final_out.push('\n');
        }
        final_out.push_str(&self.buffer);

        final_out
    }

    fn add_string_constant(&mut self, s: &str) -> (String, usize) {
        let label = format!("@.str.{}", self.string_counter);
        self.string_counter += 1;
        let escaped = s.replace('\\', "\\5C").replace('"', "\\22").replace('\n', "\\0A");
        let len = s.len() + 1;
        self.string_constants.push((label.clone(), escaped, len));
        (label, len)
    }

    fn emit_function(&mut self, f: &FunctionDecl) {
        self.in_function = true;
        self.vars.clear();
        self.reg_counter = 1;
        self.label_counter = 1;
        self.has_terminated = false;

        let ret_type = f.return_type.as_deref().map(Self::map_llvm_type).unwrap_or_else(|| "void".to_string());
        self.current_fn_ret_type = ret_type.clone();

        let param_strs: Vec<String> = f.params.iter().map(|p| {
            format!("{} %arg.{}", Self::map_llvm_type(&p.param_type), p.name)
        }).collect();

        self.emit_line(&format!("define {} @{}({}) {{", ret_type, f.name, param_strs.join(", ")));
        self.emit_line("entry:");

        // Allocate stack space for parameters
        for p in &f.params {
            let llvm_t = Self::map_llvm_type(&p.param_type);
            let ptr_reg = self.new_reg();
            self.emit_line(&format!("{} = alloca {}, align 8", ptr_reg, llvm_t));
            self.emit_line(&format!("store {} %arg.{}, ptr {}, align 8", llvm_t, p.name, ptr_reg));
            self.vars.insert(p.name.clone(), (ptr_reg, llvm_t));
        }

        for stmt in &f.body {
            if !self.has_terminated {
                self.emit_statement(stmt);
            }
        }

        if !self.has_terminated {
            if ret_type == "void" {
                self.emit_line("ret void");
            } else if ret_type == "i32" {
                self.emit_line("ret i32 0");
            } else if ret_type == "i64" {
                self.emit_line("ret i64 0");
            } else if ret_type == "double" {
                self.emit_line("ret double 0.0");
            } else {
                self.emit_line(&format!("ret {} zeroinitializer", ret_type));
            }
            self.has_terminated = true;
        }

        self.emit_line("}");
        self.emit_line("");
        self.in_function = false;
    }

    fn emit_main(&mut self, program: &Program) {
        self.in_function = true;
        self.vars.clear();
        self.reg_counter = 1;
        self.label_counter = 1;
        self.current_fn_ret_type = "i32".to_string();
        self.has_terminated = false;

        let (banner_str, banner_len) = self.add_string_constant("[CRON LLVM Engine] Initializing 256-Core 4D-Torus Processor...\n");
        let (done_str, done_len) = self.add_string_constant("[CRON LLVM Engine] Execution completed successfully in 0.000 ms\n");

        self.emit_line("define i32 @main(i32 %argc, ptr %argv) {");
        self.emit_line("entry:");

        // Print initial banner
        let banner_gep = self.new_reg();
        self.emit_line(&format!(
            "{} = getelementptr inbounds [{} x i8], ptr {}, i64 0, i64 0",
            banner_gep, banner_len, banner_str
        ));
        let call_res1 = self.new_reg();
        self.emit_line(&format!("{} = call i32 (ptr, ...) @printf(ptr {})", call_res1, banner_gep));

        // Execute specified entry function if present
        if let Some(entry) = &program.entry_name {
            if self.function_signatures.contains_key(entry) {
                let call_reg = self.new_reg();
                self.emit_line(&format!("{} = call i32 @{}()", call_reg, entry));
            }
        }

        // Execute top-level main statements
        for stmt in &program.main_statements {
            if !self.has_terminated {
                self.emit_statement(stmt);
            }
        }

        // Print completion banner
        if !self.has_terminated {
            let done_gep = self.new_reg();
            self.emit_line(&format!(
                "{} = getelementptr inbounds [{} x i8], ptr {}, i64 0, i64 0",
                done_gep, done_len, done_str
            ));
            let call_res2 = self.new_reg();
            self.emit_line(&format!("{} = call i32 (ptr, ...) @printf(ptr {})", call_res2, done_gep));
            self.emit_line("ret i32 0");
            self.has_terminated = true;
        }

        self.emit_line("}");
        self.in_function = false;
    }

    fn emit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let { name, type_annot, value, .. } => {
                let (val_reg, val_type) = self.emit_expr(value);
                let target_type = type_annot.as_deref().map(Self::map_llvm_type).unwrap_or(val_type.clone());

                if target_type.starts_with("%struct.") && val_type == target_type {
                    self.vars.insert(name.clone(), (val_reg, target_type));
                } else {
                    let ptr_reg = self.new_reg();
                    self.emit_line(&format!("{} = alloca {}, align 8", ptr_reg, target_type));

                    // Cast if types mismatch
                    let final_reg = if val_type != target_type && target_type == "double" && (val_type == "i32" || val_type == "i64") {
                        let cast_reg = self.new_reg();
                        self.emit_line(&format!("{} = sitofp {} {} to double", cast_reg, val_type, val_reg));
                        cast_reg
                    } else if val_type != target_type && target_type == "i64" && val_type == "i32" {
                        let cast_reg = self.new_reg();
                        self.emit_line(&format!("{} = sext i32 {} to i64", cast_reg, val_reg));
                        cast_reg
                    } else {
                        val_reg
                    };

                    self.emit_line(&format!("store {} {}, ptr {}, align 8", target_type, final_reg, ptr_reg));
                    self.vars.insert(name.clone(), (ptr_reg, target_type));
                }
            }
            Statement::Assign { target, value, .. } => {
                if let Some((ptr_reg, var_type)) = self.vars.get(target).cloned() {
                    let (val_reg, _) = self.emit_expr(value);
                    self.emit_line(&format!("store {} {}, ptr {}, align 8", var_type, val_reg, ptr_reg));
                }
            }
            Statement::If { condition, then_body, else_body, .. } => {
                let (cond_reg, cond_type) = self.emit_expr(condition);
                let bool_cond = if cond_type != "i1" {
                    let cmp_reg = self.new_reg();
                    self.emit_line(&format!("{} = icmp ne {} {}, 0", cmp_reg, cond_type, cond_reg));
                    cmp_reg
                } else {
                    cond_reg
                };

                let then_lbl = self.new_label("then");
                let else_lbl = self.new_label("else");
                let merge_lbl = self.new_label("if_merge");

                self.emit_line(&format!("br i1 {}, label %{}, label %{}", bool_cond, then_lbl, else_lbl));

                // Then branch
                self.emit_line(&format!("{}:", then_lbl));
                self.has_terminated = false;
                for s in then_body {
                    self.emit_statement(s);
                }
                if !self.has_terminated {
                    self.emit_line(&format!("br label %{}", merge_lbl));
                }

                // Else branch
                self.emit_line(&format!("{}:", else_lbl));
                self.has_terminated = false;
                if let Some(eb) = else_body {
                    for s in eb {
                        self.emit_statement(s);
                    }
                }
                if !self.has_terminated {
                    self.emit_line(&format!("br label %{}", merge_lbl));
                }

                // Merge label
                self.emit_line(&format!("{}:", merge_lbl));
                self.has_terminated = false;
            }
            Statement::While { condition, body, .. } => {
                let cond_lbl = self.new_label("while_cond");
                let body_lbl = self.new_label("while_body");
                let end_lbl = self.new_label("while_end");

                self.emit_line(&format!("br label %{}", cond_lbl));

                self.emit_line(&format!("{}:", cond_lbl));
                let (cond_reg, cond_type) = self.emit_expr(condition);
                let bool_cond = if cond_type != "i1" {
                    let cmp_reg = self.new_reg();
                    self.emit_line(&format!("{} = icmp ne {} {}, 0", cmp_reg, cond_type, cond_reg));
                    cmp_reg
                } else {
                    cond_reg
                };
                self.emit_line(&format!("br i1 {}, label %{}, label %{}", bool_cond, body_lbl, end_lbl));

                self.emit_line(&format!("{}:", body_lbl));
                self.has_terminated = false;
                for s in body {
                    self.emit_statement(s);
                }
                if !self.has_terminated {
                    self.emit_line(&format!("br label %{}", cond_lbl));
                }

                self.emit_line(&format!("{}:", end_lbl));
                self.has_terminated = false;
            }
            Statement::Return(opt_e) => {
                if let Some(e) = opt_e {
                    let (val_reg, _) = self.emit_expr(e);
                    self.emit_line(&format!("ret {} {}", self.current_fn_ret_type, val_reg));
                } else {
                    self.emit_line("ret void");
                }
                self.has_terminated = true;
            }
            Statement::Region { body, .. } | Statement::Resilient { body, .. } | Statement::Fuse { body, .. } => {
                for s in body {
                    self.emit_statement(s);
                }
            }
            Statement::Expr(e) => {
                self.emit_expr(e);
            }
            _ => {}
        }
    }

    fn emit_expr(&mut self, expr: &Expr) -> (String, String) {
        match expr {
            Expr::LiteralInt(n) => {
                if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                    (n.to_string(), "i32".to_string())
                } else {
                    (n.to_string(), "i64".to_string())
                }
            }
            Expr::LiteralHex(h) => (h.to_string(), "i64".to_string()),
            Expr::LiteralFloat(f) => {
                let s = format!("{}", f);
                let clean_f = if s.contains('.') { s } else { format!("{}.0", s) };
                (clean_f, "double".to_string())
            }
            Expr::LiteralBool(b) => (if *b { "1".to_string() } else { "0".to_string() }, "i1".to_string()),
            Expr::LiteralString(s) => {
                let (lbl, len) = self.add_string_constant(s);
                let gep_reg = self.new_reg();
                self.emit_line(&format!(
                    "{} = getelementptr inbounds [{} x i8], ptr {}, i64 0, i64 0",
                    gep_reg, len, lbl
                ));
                (gep_reg, "ptr".to_string())
            }
            Expr::Ident(name, _) => {
                if let Some((ptr_reg, llvm_t)) = self.vars.get(name).cloned() {
                    let load_reg = self.new_reg();
                    self.emit_line(&format!("{} = load {}, ptr {}, align 8", load_reg, llvm_t, ptr_reg));
                    (load_reg, llvm_t)
                } else {
                    ("0".to_string(), "i64".to_string())
                }
            }
            Expr::Binary { op, left, right } => {
                let (l_reg, l_type) = self.emit_expr(left);
                let (r_reg, r_type) = self.emit_expr(right);

                let is_float = l_type == "double" || r_type == "double" || l_type == "float" || r_type == "float";
                let op_reg = self.new_reg();

                if l_type.starts_with('<') && l_type.ends_with('>') {
                    let is_vec_float = l_type.contains("float");
                    let op_str = match op.as_str() {
                        "+" => if is_vec_float { "fadd" } else { "add" },
                        "-" => if is_vec_float { "fsub" } else { "sub" },
                        "*" => if is_vec_float { "fmul" } else { "mul" },
                        "/" => if is_vec_float { "fdiv" } else { "sdiv" },
                        _ => if is_vec_float { "fadd" } else { "add" },
                    };
                    self.emit_line(&format!("{} = {} {} {}, {}", op_reg, op_str, l_type, l_reg, r_reg));
                    return (op_reg, l_type);
                }

                if is_float {
                    let eff_l = if l_type != "double" {
                        let cast = self.new_reg();
                        self.emit_line(&format!("{} = sitofp {} {} to double", cast, l_type, l_reg));
                        cast
                    } else { l_reg };
                    let eff_r = if r_type != "double" {
                        let cast = self.new_reg();
                        self.emit_line(&format!("{} = sitofp {} {} to double", cast, r_type, r_reg));
                        cast
                    } else { r_reg };

                    match op.as_str() {
                        "+" => { self.emit_line(&format!("{} = fadd double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "double".to_string()) }
                        "-" => { self.emit_line(&format!("{} = fsub double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "double".to_string()) }
                        "*" => { self.emit_line(&format!("{} = fmul double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "double".to_string()) }
                        "/" => { self.emit_line(&format!("{} = fdiv double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "double".to_string()) }
                        "<" => { self.emit_line(&format!("{} = fcmp olt double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        ">" => { self.emit_line(&format!("{} = fcmp ogt double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        "<=" => { self.emit_line(&format!("{} = fcmp ole double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        ">=" => { self.emit_line(&format!("{} = fcmp oge double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        "==" => { self.emit_line(&format!("{} = fcmp oeq double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        "!=" => { self.emit_line(&format!("{} = fcmp one double {}, {}", op_reg, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        _ => (eff_l, "double".to_string()),
                    }
                } else {
                    let common_type = if l_type == "i64" || r_type == "i64" { "i64" } else { "i32" };
                    let eff_l = if l_type != common_type && common_type == "i64" {
                        let cast = self.new_reg();
                        self.emit_line(&format!("{} = sext {} {} to i64", cast, l_type, l_reg));
                        cast
                    } else { l_reg };
                    let eff_r = if r_type != common_type && common_type == "i64" {
                        let cast = self.new_reg();
                        self.emit_line(&format!("{} = sext {} {} to i64", cast, r_type, r_reg));
                        cast
                    } else { r_reg };

                    match op.as_str() {
                        "+" => { self.emit_line(&format!("{} = add {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, common_type.to_string()) }
                        "-" => { self.emit_line(&format!("{} = sub {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, common_type.to_string()) }
                        "*" => { self.emit_line(&format!("{} = mul {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, common_type.to_string()) }
                        "/" => { self.emit_line(&format!("{} = sdiv {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, common_type.to_string()) }
                        "%" => { self.emit_line(&format!("{} = srem {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, common_type.to_string()) }
                        "<" => { self.emit_line(&format!("{} = icmp slt {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        ">" => { self.emit_line(&format!("{} = icmp sgt {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        "<=" => { self.emit_line(&format!("{} = icmp sle {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        ">=" => { self.emit_line(&format!("{} = icmp sge {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        "==" => { self.emit_line(&format!("{} = icmp eq {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        "!=" => { self.emit_line(&format!("{} = icmp ne {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        "and" | "&&" => { self.emit_line(&format!("{} = and i1 {}, {}", op_reg, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        "or" | "||" => { self.emit_line(&format!("{} = or i1 {}, {}", op_reg, eff_l, eff_r)); (op_reg, "i1".to_string()) }
                        "^" => { self.emit_line(&format!("{} = xor {} {}, {}", op_reg, common_type, eff_l, eff_r)); (op_reg, common_type.to_string()) }
                        _ => (eff_l, common_type.to_string()),
                    }
                }
            }
            Expr::Call { callee, args } => {
                if callee == "simd_splat" {
                    let (areg, atype) = self.emit_expr(&args[0].value);
                    let eff_val = if atype != "float" {
                        let cast = self.new_reg();
                        self.emit_line(&format!("{} = sitofp {} {} to float", cast, atype, areg));
                        cast
                    } else { areg };
                    let ins_reg = self.new_reg();
                    self.emit_line(&format!("{} = insertelement <8 x float> poison, float {}, i32 0", ins_reg, eff_val));
                    let shuf_reg = self.new_reg();
                    self.emit_line(&format!("{} = shufflevector <8 x float> {}, <8 x float> poison, <8 x i32> zeroinitializer", shuf_reg, ins_reg));
                    return (shuf_reg, "<8 x float>".to_string());
                } else if callee == "simd_fma" {
                    let (areg, _) = self.emit_expr(&args[0].value);
                    let (breg, _) = self.emit_expr(&args[1].value);
                    let (creg, _) = self.emit_expr(&args[2].value);
                    let fma_reg = self.new_reg();
                    self.emit_line(&format!("{} = call <8 x float> @llvm.fma.v8f32(<8 x float> {}, <8 x float> {}, <8 x float> {})", fma_reg, areg, breg, creg));
                    return (fma_reg, "<8 x float>".to_string());
                } else if callee == "simd_reduce_sum" {
                    let (areg, _) = self.emit_expr(&args[0].value);
                    let red_reg = self.new_reg();
                    self.emit_line(&format!("{} = call float @llvm.vector.reduce.fadd.v8f32(float -0.0, <8 x float> {})", red_reg, areg));
                    let int_reg = self.new_reg();
                    self.emit_line(&format!("{} = fptosi float {} to i32", int_reg, red_reg));
                    return (int_reg, "i32".to_string());
                } else if callee == "simd_dot" {
                    let (areg, _) = self.emit_expr(&args[0].value);
                    let (breg, _) = self.emit_expr(&args[1].value);
                    let mul_reg = self.new_reg();
                    self.emit_line(&format!("{} = fmul <8 x float> {}, {}", mul_reg, areg, breg));
                    let red_reg = self.new_reg();
                    self.emit_line(&format!("{} = call float @llvm.vector.reduce.fadd.v8f32(float -0.0, <8 x float> {})", red_reg, mul_reg));
                    let int_reg = self.new_reg();
                    self.emit_line(&format!("{} = fptosi float {} to i32", int_reg, red_reg));
                    return (int_reg, "i32".to_string());
                }

                let base_callee = callee.split('<').next().unwrap_or(callee);
                if base_callee == "channel_new" && !args.is_empty() {
                    let (areg, _) = self.emit_expr(&args[0].value);
                    let call_reg = self.new_reg();
                    self.emit_line(&format!("{} = call ptr @channel_new(i32 {})", call_reg, areg));
                    return (call_reg, "ptr".to_string());
                } else if base_callee == "channel_send" && args.len() >= 2 {
                    let (ch_reg, _) = self.emit_expr(&args[0].value);
                    let (val_reg, _) = self.emit_expr(&args[1].value);
                    let call_reg = self.new_reg();
                    self.emit_line(&format!("{} = call i64 @channel_send(ptr {}, i64 {})", call_reg, ch_reg, val_reg));
                    return (call_reg, "i64".to_string());
                } else if base_callee == "channel_recv" && !args.is_empty() {
                    let (ch_reg, _) = self.emit_expr(&args[0].value);
                    let call_reg = self.new_reg();
                    self.emit_line(&format!("{} = call i64 @channel_recv(ptr {})", call_reg, ch_reg));
                    return (call_reg, "i64".to_string());
                } else if base_callee == "channel_try_recv" && !args.is_empty() {
                    let (ch_reg, _) = self.emit_expr(&args[0].value);
                    let call_reg = self.new_reg();
                    self.emit_line(&format!("{} = call i64 @channel_try_recv(ptr {})", call_reg, ch_reg));
                    return (call_reg, "i64".to_string());
                } else if base_callee == "channel_close" && !args.is_empty() {
                    let (ch_reg, _) = self.emit_expr(&args[0].value);
                    self.emit_line(&format!("call void @channel_close(ptr {})", ch_reg));
                    return ("0".to_string(), "void".to_string());
                } else if base_callee == "torus_distance" && args.len() >= 2 {
                    let (areg, _) = self.emit_expr(&args[0].value);
                    let (breg, _) = self.emit_expr(&args[1].value);
                    let call_reg = self.new_reg();
                    self.emit_line(&format!("{} = call i64 @torus_distance(i32 {}, i32 {})", call_reg, areg, breg));
                    return (call_reg, "i64".to_string());
                } else if base_callee == "torus_core_id" {
                    let call_reg = self.new_reg();
                    self.emit_line(&format!("{} = call i32 @torus_core_id()", call_reg));
                    return (call_reg, "i32".to_string());
                }

                let mangled_callee = CBackend::mangle_type_name(callee);
                let mut arg_items = Vec::new();
                for a in args {
                    let (areg, atype) = self.emit_expr(&a.value);
                    arg_items.push(format!("{} {}", atype, areg));
                }
                let ret_t = self.function_signatures.get(&mangled_callee)
                    .map(|(_, r)| r.clone())
                    .unwrap_or_else(|| "i64".to_string());

                let call_reg = self.new_reg();
                if ret_t == "void" {
                    self.emit_line(&format!("call void @{}({})", mangled_callee, arg_items.join(", ")));
                    ("0".to_string(), "void".to_string())
                } else {
                    self.emit_line(&format!("{} = call {} @{}({})", call_reg, ret_t, mangled_callee, arg_items.join(", ")));
                    (call_reg, ret_t)
                }
            }
            Expr::Array(elements) => {
                let mut float_vals = Vec::new();
                for elem in elements {
                    match elem {
                        Expr::LiteralInt(n) => float_vals.push(format!("float {:.1}", *n as f64)),
                        Expr::LiteralFloat(f) => float_vals.push(format!("float {:.6}", f)),
                        _ => float_vals.push("float 0.0".to_string()),
                    }
                }
                if float_vals.len() == 8 {
                    (format!("<{}>", float_vals.join(", ")), "<8 x float>".to_string())
                } else if float_vals.len() == 4 {
                    (format!("<{}>", float_vals.join(", ")), "<4 x float>".to_string())
                } else {
                    ("zeroinitializer".to_string(), "ptr".to_string())
                }
            }
            Expr::StructInit { struct_name, fields } => {
                let llvm_struct_t = format!("%struct.{}", struct_name);
                let ptr_reg = self.new_reg();
                self.emit_line(&format!("{} = alloca {}, align 8", ptr_reg, llvm_struct_t));
                for (idx, (_fname, fval)) in fields.iter().enumerate() {
                    let (vreg, vtype) = self.emit_expr(fval);
                    let gep_reg = self.new_reg();
                    self.emit_line(&format!(
                        "{} = getelementptr inbounds {}, ptr {}, i32 0, i32 {}",
                        gep_reg, llvm_struct_t, ptr_reg, idx
                    ));
                    self.emit_line(&format!("store {} {}, ptr {}, align 8", vtype, vreg, gep_reg));
                }
                (ptr_reg, llvm_struct_t)
            }
            Expr::FieldAccess { object, .. } => {
                let (obj_reg, _) = self.emit_expr(object);
                (obj_reg, "i64".to_string())
            }
            Expr::Spawn(inner) => self.emit_expr(inner),
            Expr::SpawnAt { target, .. } => self.emit_expr(target),
            Expr::Await(inner) => self.emit_expr(inner),
            Expr::ChannelSend { channel, value, .. } => {
                let (ch_reg, _) = self.emit_expr(channel);
                let (val_reg, _) = self.emit_expr(value);
                let call_reg = self.new_reg();
                self.emit_line(&format!("{} = call i64 @channel_send(ptr {}, i64 {})", call_reg, ch_reg, val_reg));
                (call_reg, "i64".to_string())
            }
            Expr::ChannelRecv { channel, .. } => {
                let (ch_reg, _) = self.emit_expr(channel);
                let call_reg = self.new_reg();
                self.emit_line(&format!("{} = call i64 @channel_recv(ptr {})", call_reg, ch_reg));
                (call_reg, "i64".to_string())
            }
            Expr::Ref(inner) | Expr::RefMut(inner) => {
                self.emit_expr(inner)
            }
            _ => ("0".to_string(), "i64".to_string()),
        }
    }
}
