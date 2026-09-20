// ============================================================================
// CRON Semantic Checker — Linear Type System & Region Safety Verifier
// Enforces:
//   - Affine linear types (consume exactly once)
//   - Mutability rules (let mut vs let) [E0005]
//   - Region arena escape analysis (export required) [E0004]
//   - Recursive expression checking through all AST nodes
//   - Control flow linear tracking (if/else/while/for)
//   - Exact Span-grounded diagnostics
// ============================================================================

use crate::ast::*;
use crate::token::Span;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TypeError {
    pub code: &'static str,
    pub message: String,
    pub span: Option<Span>,
    pub help: Option<String>,
    pub note: Option<String>,
}

impl TypeError {
    pub fn new(code: &'static str, message: impl Into<String>, span: Span) -> Self {
        Self {
            code,
            message: message.into(),
            span: Some(span),
            help: None,
            note: None,
        }
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct VarInfo {
    pub name: String,
    pub is_lin: bool,
    pub is_grad: bool,
    pub is_mut: bool,
    pub is_consumed: bool,
    pub is_tainted: bool,
    pub is_capability: bool,
    pub def_span: Span,
    pub var_type: Option<String>,
    pub in_region: bool,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub vars: HashMap<String, VarInfo>,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}

pub struct SemanticChecker {
    scopes: Vec<Scope>,
    in_region: bool,
    in_fuse: bool,
    exported_vars: HashSet<String>,
    region_allocated_vars: HashMap<String, VarInfo>,
    known_functions: HashSet<String>,
    known_structs: HashMap<String, HashMap<String, String>>,
    known_types: HashMap<String, String>,
    /// Trait name -> method signatures (for contract verification)
    known_traits: HashMap<String, Vec<(String, Vec<String>, Option<String>)>>,
    /// (trait_name, struct_name) -> method names (for monomorphization lookup)
    known_impls: HashMap<(String, String), HashSet<String>>,
    pub fn_param_types: HashMap<String, Vec<(String, String)>>,
    pub fn_return_types: HashMap<String, String>,
}

impl Default for SemanticChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticChecker {
    pub fn new() -> Self {
        let mut known_functions = HashSet::new();
        for f in &[
            "simd_splat", "simd_load", "simd_store", "simd_fma", "simd_dot",
            "simd_reduce_sum", "simd_add", "simd_sub", "simd_mul", "simd_div",
            "vec4f", "vec8f", "vec16f", "vec4i", "vec8i",
            "channel_new", "channel_send", "channel_recv", "channel_try_recv", "channel_close",
            "spawn_at", "torus_core_id", "torus_coord_x", "torus_coord_y", "torus_coord_z", "torus_coord_w",
            "torus_distance", "torus_send", "torus_recv", "torus_broadcast",
            // Sub-byte & BitNet AI intrinsics
            "simd_ternary_dot", "pack_ternary", "unpack_ternary",
            "pack_i4", "unpack_i4", "simd_f4_gemm",
            // Systolic Tile Hardware intrinsics
            "tile4x4_f32", "tile4x4_i32", "tile_matmul", "tile_transpose", "tile_add", "tile_fma",
            "tile_get", "tile_set",
            "dma_sync", "await_dma_channel",
            // Phase 9: Bank-Free Swizzling & 4D-Torus PGAS intrinsics
            "sram_swizzle_index", "tile_swizzle",
            "pgas_read", "pgas_write", "pgas_barrier",
            // Phase 10: Dependent Tensor & Silicon Autotuner builtins
            "tensor_matmul", "tensor_transpose", "tensor_add", "tensor_reshape", "tensor_slice",
            "tensor_init", "tensor_ones", "tensor_zeros",
            // Phase 12: Kernel Fusion & Streaming builtins
            "tensor_scale", "tensor_softmax_maxsub", "tensor_gelu", "tensor_bias",
            // Real-World Training & AGI Telemetry builtins
            "cron_telemetry_init", "cron_telemetry_log", "cron_telemetry_close", "cron_save_agi_state",
        ] {
            known_functions.insert(f.to_string());
        }

        let mut known_types = HashMap::new();
        for t in &[
            "vec4f", "vec8f", "vec16f", "vec4i", "vec8i", "channel", "Channel",
            "i2", "i4", "f4", "f8", "f16", "bf16",
            "tile4x4_f32", "tile4x4_i32", "tile16x16_i2",
            "tensor",
        ] {
            known_types.insert(t.to_string(), "builtin".to_string());
        }

        Self {
            scopes: vec![Scope::new()],
            in_region: false,
            in_fuse: false,
            exported_vars: HashSet::new(),
            region_allocated_vars: HashMap::new(),
            known_functions,
            known_structs: HashMap::new(),
            known_types,
            known_traits: HashMap::new(),
            known_impls: HashMap::new(),
            fn_param_types: HashMap::new(),
            fn_return_types: HashMap::new(),
        }
    }

    pub fn lookup_outer_var(&self, name: &str) -> Option<&VarInfo> {
        if self.scopes.len() <= 1 {
            return None;
        }
        for scope in self.scopes[..self.scopes.len() - 1].iter().rev() {
            if let Some(var) = scope.vars.get(name) {
                return Some(var);
            }
        }
        None
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn insert_var(&mut self, var: VarInfo) {
        if self.in_region {
            self.region_allocated_vars.insert(var.name.clone(), var.clone());
        }
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.vars.insert(var.name.clone(), var);
        }
    }

    pub fn lookup_var(&self, name: &str) -> Option<&VarInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.vars.get(name) {
                return Some(var);
            }
        }
        self.region_allocated_vars.get(name)
    }

    pub fn lookup_var_mut(&mut self, name: &str) -> Option<&mut VarInfo> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.vars.get_mut(name) {
                return Some(var);
            }
        }
        self.region_allocated_vars.get_mut(name)
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), TypeError> {
        // Register all function names and parameter/return types
        for func in &program.functions {
            self.known_functions.insert(func.name.clone());
            let params: Vec<(String, String)> = func.params.iter().map(|p| (p.name.clone(), p.param_type.clone())).collect();
            self.fn_param_types.insert(func.name.clone(), params);
            if let Some(ret) = &func.return_type {
                self.fn_return_types.insert(func.name.clone(), ret.clone());
            }
        }

        // Register structs
        for s in &program.structs {
            let mut field_map = HashMap::new();
            for (f_name, f_type) in &s.fields {
                field_map.insert(f_name.clone(), f_type.clone());
            }
            self.known_structs.insert(s.name.clone(), field_map);
        }

        // Register type aliases
        for t in &program.type_aliases {
            self.known_types.insert(t.name.clone(), t.target_type.clone());
        }

        // Register algebraic enum types
        for e in &program.enums {
            self.known_types.insert(e.name.clone(), "enum".to_string());
        }

        // Register trait contracts (Milestone #005: zero-vtable static traits)
        for trait_decl in &program.traits {
            let methods: Vec<(String, Vec<String>, Option<String>)> = trait_decl
                .methods
                .iter()
                .map(|m| {
                    let param_types: Vec<String> = m.params.iter()
                        .filter(|p| p.name != "self")
                        .map(|p| p.param_type.clone())
                        .collect();
                    (m.name.clone(), param_types, m.return_type.clone())
                })
                .collect();
            self.known_traits.insert(trait_decl.name.clone(), methods);
        }

        // Verify impl blocks satisfy trait contracts
        for impl_decl in &program.impls {
            if let Some(trait_methods) = self.known_traits.get(&impl_decl.trait_name) {
                let impl_method_names: HashSet<String> = impl_decl.methods.iter().map(|m| m.name.clone()).collect();
                for (trait_method_name, _param_types, _ret_type) in trait_methods {
                    if !impl_method_names.contains(trait_method_name) {
                        let span = if let Some(first_method) = impl_decl.methods.first() {
                            first_method.params.first().map(|p| p.span).unwrap_or(Span::default())
                        } else {
                            Span::default()
                        };
                        return Err(TypeError::new(
                            "E0008",
                            format!(
                                "Trait contract violation: impl {} for {} is missing method '{}'",
                                impl_decl.trait_name, impl_decl.target_struct, trait_method_name
                            ),
                            span,
                        )
                        .with_note("Milestone #005: All trait methods must be monomorphized at compile-time")
                        .with_help(format!("Add `def {}(...)` to the impl block", trait_method_name)));
                    }
                }
                self.known_impls.insert(
                    (impl_decl.trait_name.clone(), impl_decl.target_struct.clone()),
                    impl_method_names,
                );
            }

            // Register impl methods as known functions and check their bodies
            for method in &impl_decl.methods {
                self.known_functions.insert(method.name.clone());
                let mut func_checker = SemanticChecker::new();
                func_checker.known_functions = self.known_functions.clone();
                func_checker.known_structs = self.known_structs.clone();
                func_checker.known_types = self.known_types.clone();
                for gp in &impl_decl.generic_params {
                    func_checker.known_types.insert(gp.clone(), "generic_param".to_string());
                }
                for gp in &method.generic_params {
                    func_checker.known_types.insert(gp.clone(), "generic_param".to_string());
                }
                func_checker.known_traits = self.known_traits.clone();
                func_checker.known_impls = self.known_impls.clone();
                func_checker.fn_param_types = self.fn_param_types.clone();
                func_checker.fn_return_types = self.fn_return_types.clone();
                for param in &method.params {
                    if param.name == "self" { continue; }
                    func_checker.insert_var(VarInfo {
                        name: param.name.clone(),
                        is_lin: param.is_lin,
                        is_grad: param.is_grad,
                        is_mut: false,
                        is_consumed: false,
                        is_tainted: false,
                        is_capability: false,
                        def_span: param.span,
                        var_type: Some(param.param_type.clone()),
                        in_region: false,
                    });
                }
                func_checker.check_statements(&method.body)?;
                func_checker.verify_all_linear_consumed()?;
            }
        }

        // Check functions
        for func in &program.functions {
            let mut func_checker = SemanticChecker::new();
            func_checker.known_functions = self.known_functions.clone();
            func_checker.known_structs = self.known_structs.clone();
            func_checker.known_types = self.known_types.clone();
            for gp in &func.generic_params {
                func_checker.known_types.insert(gp.clone(), "generic_param".to_string());
            }
            func_checker.known_traits = self.known_traits.clone();
            func_checker.known_impls = self.known_impls.clone();
            func_checker.fn_param_types = self.fn_param_types.clone();
            func_checker.fn_return_types = self.fn_return_types.clone();
            for param in &func.params {
                func_checker.insert_var(VarInfo {
                    name: param.name.clone(),
                    is_lin: param.is_lin,
                    is_grad: param.is_grad,
                    is_mut: false,
                    is_consumed: false,
                    is_tainted: false,
                    is_capability: false,
                    def_span: param.span,
                    var_type: Some(param.param_type.clone()),
                    in_region: false,
                });
            }
            func_checker.check_statements(&func.body)?;
            func_checker.verify_all_linear_consumed()?;
        }

        // Check cognitive brain blocks
        for brain in &program.brains {
            let mut brain_checker = SemanticChecker::new();
            brain_checker.known_functions = self.known_functions.clone();
            brain_checker.known_structs = self.known_structs.clone();
            brain_checker.known_types = self.known_types.clone();
            brain_checker.known_traits = self.known_traits.clone();
            brain_checker.known_impls = self.known_impls.clone();
            brain_checker.fn_param_types = self.fn_param_types.clone();
            brain_checker.fn_return_types = self.fn_return_types.clone();
            brain_checker.check_statements(&brain.body)?;
            brain_checker.verify_all_linear_consumed()?;
        }

        // Check decoupled schedule declarations
        for sched in &program.schedules {
            let fn_exists = self.known_functions.contains(&sched.target_fn)
                || program.functions.iter().any(|f| f.name == sched.target_fn)
                || program.impls.iter().any(|imp| imp.methods.iter().any(|m| m.name == sched.target_fn));
            if !fn_exists {
                return Err(TypeError::new(
                    "E0011",
                    format!("Schedule target function '{}' not found in program", sched.target_fn),
                    sched.span,
                )
                .with_note("Decoupled silicon schedules must target an existing function or kernel")
                .with_help(format!("Define `def {}(...)` before or after this schedule", sched.target_fn)));
            }

            for dir in &sched.directives {
                match dir {
                    ScheduleDirective::TileSize(w, h) => {
                        if *w == 0 || *h == 0 {
                            return Err(TypeError::new(
                                "E0012",
                                format!("Invalid tile_size({}, {}): dimensions must be non-zero", w, h),
                                sched.span,
                            ));
                        }
                    }
                    ScheduleDirective::Unroll(factor) => {
                        if *factor == 0 {
                            return Err(TypeError::new(
                                "E0012",
                                format!("Invalid unroll factor {}: must be greater than 0", factor),
                                sched.span,
                            ));
                        }
                    }
                    ScheduleDirective::Distribute4D { axis, cores } => {
                        if *cores == 0 {
                            return Err(TypeError::new(
                                "E0012",
                                format!("Invalid core count {} in distribute_4d: must be greater than 0", cores),
                                sched.span,
                            ));
                        }
                        let clean_axis = axis.trim_matches('"');
                        if !["X", "Y", "Z", "W", "X+", "X-", "Y+", "Y-", "Z+", "Z-", "W+", "W-"].contains(&clean_axis) {
                            return Err(TypeError::new(
                                "E0012",
                                format!("Invalid 4D-Torus axis '{}' in distribute_4d directive", axis),
                                sched.span,
                            )
                            .with_help("Supported axes are: X, Y, Z, W, X+, X-, Y+, Y-, Z+, Z-, W+, W-"));
                        }
                    }
                    ScheduleDirective::Vectorize(width)
                        if *width == 0 || (*width & (*width - 1)) != 0 =>
                    {
                        return Err(TypeError::new(
                            "E0012",
                            format!("Invalid vector width {}: must be a power of two", width),
                            sched.span,
                        ));
                    }
                    ScheduleDirective::Autotune { tile_sizes, unrolls, vectorize_widths, metric } => {
                        if tile_sizes.is_empty() {
                            return Err(TypeError::new(
                                "E0012",
                                "Autotune directive requires at least one candidate tile_size",
                                sched.span,
                            ));
                        }
                        for (w, h) in tile_sizes {
                            if *w == 0 || *h == 0 {
                                return Err(TypeError::new(
                                    "E0012",
                                    format!("Invalid autotune tile_size({}, {}): dimensions must be non-zero", w, h),
                                    sched.span,
                                ));
                            }
                        }
                        for u in unrolls {
                            if *u == 0 {
                                return Err(TypeError::new(
                                    "E0012",
                                    format!("Invalid autotune unroll factor {}: must be greater than 0", u),
                                    sched.span,
                                ));
                            }
                        }
                        for vw in vectorize_widths {
                            if *vw == 0 || (*vw & (*vw - 1)) != 0 {
                                return Err(TypeError::new(
                                    "E0012",
                                    format!("Invalid autotune vectorize width {}: must be a power of two", vw),
                                    sched.span,
                                ));
                            }
                        }
                        let clean_metric = metric.trim_matches('"');
                        if !["min_latency", "max_throughput", "energy_efficient"].contains(&clean_metric) {
                            return Err(TypeError::new(
                                "E0012",
                                format!("Unknown autotune optimization metric '{}'", metric),
                                sched.span,
                            ).with_help("Supported metrics: \"min_latency\", \"max_throughput\", \"energy_efficient\""));
                        }
                    }
                    _ => {}
                }
            }
        }

        // Check main statements
        self.check_statements(&program.main_statements)?;
        self.verify_all_linear_consumed()?;

        Ok(())
    }

    /// Parses a tensor shape string like `tensor<1, 32, 64, f32>` or `tensor<B, M, K, f32>`
    pub fn parse_tensor_shape(type_str: &str) -> Option<(Vec<String>, String)> {
        let trimmed = type_str.trim();
        if !trimmed.starts_with("tensor<") || !trimmed.ends_with('>') {
            return None;
        }
        let inner = trimmed[7..trimmed.len() - 1].trim();
        let parts: Vec<String> = inner.split(',').map(|s| s.trim().to_string()).collect();
        if parts.len() < 2 {
            return None;
        }
        let elem_type = parts.last().unwrap().clone();
        let dims = parts[..parts.len() - 1].to_vec();
        Some((dims, elem_type))
    }

    /// Infers the static type of an expression, including dependent tensor dimensions
    pub fn infer_expr_type(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Ident(name, _) | Expr::Consume(name, _) => {
                self.lookup_var(name).and_then(|v| v.var_type.clone())
            }
            Expr::Call { callee, args } => {
                if callee == "tensor_matmul" && args.len() >= 2 {
                    let type_a = self.infer_expr_type(&args[0].value)?;
                    let type_b = self.infer_expr_type(&args[1].value)?;
                    let (dims_a, elem_a) = Self::parse_tensor_shape(&type_a)?;
                    let (dims_b, _) = Self::parse_tensor_shape(&type_b)?;
                    if dims_a.len() >= 2 && dims_b.len() >= 2 {
                        let m = dims_a[dims_a.len() - 2].clone();
                        let n = dims_b[dims_b.len() - 1].clone();
                        let mut out_dims = Vec::new();
                        if dims_a.len() > 2 {
                            for d in &dims_a[..dims_a.len() - 2] {
                                out_dims.push(d.clone());
                            }
                        } else if dims_b.len() > 2 {
                            for d in &dims_b[..dims_b.len() - 2] {
                                out_dims.push(d.clone());
                            }
                        }
                        out_dims.push(m);
                        out_dims.push(n);
                        return Some(format!("tensor<{}, {}>", out_dims.join(", "), elem_a));
                    }
                } else if callee == "tensor_transpose" && !args.is_empty() {
                    let type_a = self.infer_expr_type(&args[0].value)?;
                    let (mut dims_a, elem_a) = Self::parse_tensor_shape(&type_a)?;
                    if dims_a.len() >= 2 {
                        let len = dims_a.len();
                        dims_a.swap(len - 2, len - 1);
                        return Some(format!("tensor<{}, {}>", dims_a.join(", "), elem_a));
                    }
                } else if (callee == "tensor_add" || callee == "tensor_scale" || callee == "tensor_softmax_maxsub" || callee == "tensor_gelu" || callee == "tensor_bias") && !args.is_empty() {
                    return self.infer_expr_type(&args[0].value);
                } else if let Some(ret) = self.fn_return_types.get(callee) {
                    if let Some(expected_params) = self.fn_param_types.get(callee) {
                        let mut symbol_map = HashMap::new();
                        for (arg, (_pname, ptype)) in args.iter().zip(expected_params.iter()) {
                            if let Some(actual_type) = self.infer_expr_type(&arg.value) {
                                if let (Some((p_dims, _)), Some((a_dims, _))) = (Self::parse_tensor_shape(ptype), Self::parse_tensor_shape(&actual_type)) {
                                    if p_dims.len() == a_dims.len() {
                                        for (p_dim, a_dim) in p_dims.iter().zip(a_dims.iter()) {
                                            if !p_dim.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(true) {
                                                symbol_map.insert(p_dim.clone(), a_dim.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !symbol_map.is_empty() {
                            if let Some((ret_dims, ret_elem)) = Self::parse_tensor_shape(ret) {
                                let substituted_dims: Vec<String> = ret_dims.iter().map(|d| {
                                    symbol_map.get(d).cloned().unwrap_or_else(|| d.clone())
                                }).collect();
                                return Some(format!("tensor<{}, {}>", substituted_dims.join(", "), ret_elem));
                            }
                        }
                    }
                    return Some(ret.clone());
                }
                None
            }
            Expr::Binary { op, left, right } => {
                if op == "@" {
                    let type_a = self.infer_expr_type(left)?;
                    let type_b = self.infer_expr_type(right)?;
                    let (dims_a, elem_a) = Self::parse_tensor_shape(&type_a)?;
                    let (dims_b, _) = Self::parse_tensor_shape(&type_b)?;
                    if dims_a.len() >= 2 && dims_b.len() >= 2 {
                        let m = dims_a[dims_a.len() - 2].clone();
                        let n = dims_b[dims_b.len() - 1].clone();
                        let mut out_dims = Vec::new();
                        if dims_a.len() > 2 {
                            for d in &dims_a[..dims_a.len() - 2] {
                                out_dims.push(d.clone());
                            }
                        } else if dims_b.len() > 2 {
                            for d in &dims_b[..dims_b.len() - 2] {
                                out_dims.push(d.clone());
                            }
                        }
                        out_dims.push(m);
                        out_dims.push(n);
                        return Some(format!("tensor<{}, {}>", out_dims.join(", "), elem_a));
                    }
                } else if matches!(op.as_str(), "+" | "-" | "*" | "/") {
                    if let Some(t_left) = self.infer_expr_type(left) {
                        if t_left.starts_with("tensor<") {
                            return Some(t_left);
                        }
                    }
                    if let Some(t_right) = self.infer_expr_type(right) {
                        if t_right.starts_with("tensor<") {
                            return Some(t_right);
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn check_statements(&mut self, stmts: &[Statement]) -> Result<(), TypeError> {
        for stmt in stmts {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), TypeError> {
        match stmt {
            Statement::Let {
                is_lin,
                is_grad,
                is_mut,
                name,
                type_annot,
                value,
                extra_vars,
                span,
            } => {
                self.check_expr(value)?;
                let is_tainted_by_name_or_type = name.contains("tainted") || type_annot.as_deref().unwrap_or("").contains("tainted");
                let is_sanitized_call = matches!(value, Expr::Call { callee, .. } if callee == "sanitize");
                let is_tainted = is_tainted_by_name_or_type && !is_sanitized_call;
                let is_capability = name.contains("cap")
                    || type_annot.as_deref().unwrap_or("").contains("cap_t")
                    || matches!(value, Expr::Call { callee, .. } if callee == "acquire_capability");

                // Check spatial memory domain consistency on declaration
                if let Some(target_type) = &type_annot {
                    let target_domain = if target_type.starts_with("@sram") {
                        Some("@sram")
                    } else if target_type.starts_with("@hbm") {
                        Some("@hbm")
                    } else if target_type.starts_with("@noc") {
                        Some("@noc")
                    } else {
                        None
                    };

                    if let Some(t_dom) = target_domain {
                        if let Expr::Ident(src_name, _) = value {
                            if let Some(src_var) = self.lookup_var(src_name) {
                                if let Some(src_type) = &src_var.var_type {
                                    let src_domain = if src_type.starts_with("@sram") {
                                        Some("@sram")
                                    } else if src_type.starts_with("@hbm") {
                                        Some("@hbm")
                                    } else if src_type.starts_with("@noc") {
                                        Some("@noc")
                                    } else {
                                        None
                                    };

                                    if let Some(s_dom) = src_domain {
                                        if t_dom != s_dom {
                                            return Err(TypeError::new(
                                                "E0015",
                                                format!(
                                                    "Spatial memory domain crossing violation: Direct assignment from {} to {} is forbidden across hardware domains without explicit DMA transfer (dma_sync)",
                                                    s_dom, t_dom
                                                ),
                                                *span,
                                            )
                                            .with_note("Hardware memory boundaries (@sram, @hbm, @noc) prevent cache stalls and raw pointer aliasing")
                                            .with_help("Use `dma_sync` or staged prefetch buffer before reading from or writing across memory tiers"));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                let inferred_type = if type_annot.is_none() {
                    self.infer_expr_type(value)
                } else {
                    type_annot.clone()
                };

                // Statically verify tensor dimension compatibility if annotated
                if let (Some(expected_str), Some(actual_str)) = (&type_annot, self.infer_expr_type(value)) {
                    if let (Some((exp_dims, exp_elem)), Some((act_dims, act_elem))) = (Self::parse_tensor_shape(expected_str), Self::parse_tensor_shape(&actual_str)) {
                        if exp_dims != act_dims || exp_elem != act_elem {
                            return Err(TypeError::new(
                                "E0016",
                                format!("Tensor dimension mismatch: expected type '{}', but found expression of type '{}'", expected_str, actual_str),
                                *span,
                            ).with_note("Milestone #010: Symbolic Shape Inference enforces strict dependent tensor invariants"));
                        }
                    }
                }

                self.insert_var(VarInfo {
                    name: name.clone(),
                    is_lin: *is_lin,
                    is_grad: *is_grad,
                    is_mut: *is_mut,
                    is_consumed: false,
                    is_tainted,
                    is_capability,
                    def_span: *span,
                    var_type: inferred_type,
                    in_region: self.in_region,
                });
                for (e_lin, e_grad, e_name, e_type) in extra_vars {
                    self.insert_var(VarInfo {
                        name: e_name.clone(),
                        is_lin: *e_lin,
                        is_grad: *e_grad,
                        is_mut: *is_mut,
                        is_consumed: false,
                        is_tainted: false,
                        is_capability: false,
                        def_span: *span,
                        var_type: e_type.clone(),
                        in_region: self.in_region,
                    });
                }
            }
            Statement::Assign { target, value, span } => {
                if let Some(var) = self.lookup_var(target) {
                    if var.is_consumed {
                        return Err(TypeError::new(
                            "E0003",
                            format!(
                                "Linear type violation: Variable '{}' was already consumed and cannot be reassigned",
                                target
                            ),
                            *span,
                        )
                        .with_note("Target Architecture: 256-Core 4D-Torus Photonic Neuromorphic Silicon")
                        .with_help("Linear types enforce affine uniqueness (cannot be duplicated or consumed multiple times)"));
                    }
                    if !var.is_mut {
                        return Err(TypeError::new(
                            "E0005",
                            format!(
                                "Cannot assign twice to immutable variable '{}'",
                                target
                            ),
                            *span,
                        )
                        .with_note("Variables are immutable by default")
                        .with_help(format!("Consider declaring as mutable: `let mut {}`", target)));
                    }

                    if self.in_fuse {
                        if let Some(_outer_var) = self.lookup_outer_var(target) {
                            return Err(TypeError::new(
                                "E0017",
                                format!(
                                    "Fusion scope escape violation: Cannot assign streaming intermediate value to outer variable '{}'",
                                    target
                                ),
                                *span,
                            )
                            .with_note("Milestone #012: Zero-allocation fusion prohibits intermediate streaming buffers from escaping to outer scopes")
                            .with_help("Return the final result directly from the fuse block or use an export directive"));
                        }
                    }

                    // Check spatial memory domain consistency on assignment
                    if let Some(target_type) = &var.var_type {
                        let target_domain = if target_type.starts_with("@sram") {
                            Some("@sram")
                        } else if target_type.starts_with("@hbm") {
                            Some("@hbm")
                        } else if target_type.starts_with("@noc") {
                            Some("@noc")
                        } else {
                            None
                        };

                        if let Some(t_dom) = target_domain {
                            if let Expr::Ident(src_name, _) = value {
                                if let Some(src_var) = self.lookup_var(src_name) {
                                    if let Some(src_type) = &src_var.var_type {
                                        let src_domain = if src_type.starts_with("@sram") {
                                            Some("@sram")
                                        } else if src_type.starts_with("@hbm") {
                                            Some("@hbm")
                                        } else if src_type.starts_with("@noc") {
                                            Some("@noc")
                                        } else {
                                            None
                                        };

                                        if let Some(s_dom) = src_domain {
                                            if t_dom != s_dom {
                                                return Err(TypeError::new(
                                                    "E0015",
                                                    format!(
                                                        "Spatial memory domain crossing violation: Direct assignment from {} to {} is forbidden across hardware domains without explicit DMA transfer (dma_sync)",
                                                        s_dom, t_dom
                                                    ),
                                                    *span,
                                                )
                                                .with_note("Hardware memory boundaries (@sram, @hbm, @noc) prevent cache stalls and raw pointer aliasing")
                                                .with_help("Use `dma_sync` or staged prefetch buffer before reading from or writing across memory tiers"));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self.check_expr(value)?;
            }
            Statement::Region { body, .. } => {
                let prev_in_region = self.in_region;
                self.in_region = true;
                self.push_scope();

                for s in body {
                    self.check_statement(s)?;
                }

                if let Some(scope) = self.scopes.last() {
                    for (var_name, var) in &scope.vars {
                        if var.is_lin && !var.is_consumed && !self.exported_vars.contains(var_name) {
                            return Err(TypeError::new(
                                "E0002",
                                format!(
                                    "Linear type leak: Linear variable '{}' was allocated in region but never consumed or exported",
                                    var_name
                                ),
                                var.def_span,
                            )
                            .with_note("Scoped region memory is reclaimed in 0 hardware cycles upon region exit")
                            .with_help("Consume the linear variable or export it before the region closes"));
                        }
                    }
                }

                self.pop_scope();
                self.in_region = prev_in_region;
            }
            Statement::Resilient { body, fallback, .. } => {
                self.push_scope();
                self.check_statements(body)?;
                self.pop_scope();

                if let Some(fb) = fallback {
                    self.push_scope();
                    self.check_statements(fb)?;
                    self.pop_scope();
                }
            }
            Statement::Fuse { attrs, body, span } => {
                for (k, v) in attrs {
                    if k == "tile" {
                        let clean = v.trim_matches(|c| c == '(' || c == ')' || c == '[' || c == ']');
                        let parts: Vec<&str> = clean.split(',').map(|s| s.trim()).collect();
                        if parts.len() == 2 {
                            if let (Ok(m), Ok(n)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                                if m * n > 256 {
                                    return Err(TypeError::new(
                                        "E0017",
                                        format!("Tile size {}x{} ({} elements) exceeds local SRAM streaming limit of 256 elements", m, n, m * n),
                                        *span,
                                    ).with_note("Milestone #012: Kernel fusion requires tile sizes within on-chip register and SRAM boundaries")
                                     .with_help("Reduce tile size to (16, 16) or smaller"));
                                }
                            }
                        }
                    }
                }

                let prev_in_fuse = self.in_fuse;
                self.in_fuse = true;
                self.push_scope();

                for s in body {
                    self.check_statement(s)?;
                }

                if let Some(scope) = self.scopes.last() {
                    for (var_name, var) in &scope.vars {
                        if var.is_lin && !var.is_consumed && !self.exported_vars.contains(var_name) {
                            return Err(TypeError::new(
                                "E0002",
                                format!(
                                    "Linear type leak: Linear variable '{}' was allocated in fuse block but never consumed or exported",
                                    var_name
                                ),
                                var.def_span,
                            )
                            .with_note("Fused streaming variables must be consumed or exported before the fuse block exits")
                            .with_help("Consume the linear variable or export it before the fuse block closes"));
                        }
                    }
                }

                self.pop_scope();
                self.in_fuse = prev_in_fuse;
            }
            Statement::Brain { body, .. } => {
                self.push_scope();
                self.check_statements(body)?;
                self.pop_scope();
            }
            Statement::Fork { target, .. } => {
                self.check_expr(target)?;
            }
            Statement::Simulate { action, with_arg, .. } => {
                self.check_expr(action)?;
                if let Some(arg) = with_arg {
                    self.check_expr(arg)?;
                }
            }
            Statement::Abort(_) => {}
            Statement::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.check_expr(condition)?;
                let snapshot = self.scopes.clone();
                self.push_scope();
                self.check_statements(then_body)?;
                let then_scopes = self.scopes.clone();

                if let Some(else_stmts) = else_body {
                    self.scopes = snapshot;
                    self.push_scope();
                    self.check_statements(else_stmts)?;
                    let else_scopes = self.scopes.clone();

                    // Merge linear consumption back into parent scopes
                    for (i, scope) in then_scopes.iter().enumerate() {
                        if i < self.scopes.len() {
                            for (name, var) in &scope.vars {
                                if let Some(else_scope) = else_scopes.get(i) {
                                    if let Some(else_var) = else_scope.vars.get(name) {
                                        if let Some(target_var) = self.scopes[i].vars.get_mut(name) {
                                            target_var.is_consumed = var.is_consumed || else_var.is_consumed;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self.scopes = snapshot;
                }
            }
            Statement::While { condition, body, .. } => {
                self.check_expr(condition)?;
                self.push_scope();
                self.check_statements(body)?;
                self.pop_scope();
            }
            Statement::For { var_name, iterable, body, span } => {
                self.check_expr(iterable)?;
                self.push_scope();
                self.insert_var(VarInfo {
                    name: var_name.clone(),
                    is_lin: false,
                    is_grad: false,
                    is_mut: true,
                    is_consumed: false,
                    is_tainted: false,
                    is_capability: false,
                    def_span: *span,
                    var_type: None,
                    in_region: self.in_region,
                });
                self.check_statements(body)?;
                self.pop_scope();
            }
            Statement::ProofContract(clauses) => {
                self.check_statements(clauses)?;
            }
            Statement::Superposition { branches, .. } => {
                for b in branches {
                    self.push_scope();
                    self.check_statements(&b.body)?;
                    self.pop_scope();
                }
            }
            Statement::Match { expr, arms, span: _ } => {
                self.check_expr(expr)?;
                for arm in arms {
                    self.push_scope();
                    if let MatchPattern::Variant { bindings, .. } = &arm.pattern {
                        for b in bindings {
                            if b != "_" {
                                self.insert_var(VarInfo {
                                    name: b.clone(),
                                    is_lin: false,
                                    is_grad: false,
                                    is_mut: false,
                                    is_consumed: false,
                                    is_tainted: false,
                                    is_capability: false,
                                    def_span: arm.span,
                                    var_type: None,
                                    in_region: self.in_region,
                                });
                            }
                        }
                    }
                    self.check_statements(&arm.body)?;
                    self.pop_scope();
                }
            }
            Statement::Export {
                source_name,
                exported_name,
                span,
            } => {
                let found = self.lookup_var(source_name).is_some();
                if !found && !self.exported_vars.contains(source_name) {
                    return Err(TypeError::new(
                        "E0007",
                        format!("Cannot export unknown symbol '{}'", source_name),
                        *span,
                    ));
                }
                self.exported_vars.insert(exported_name.clone());
                if let Some(var) = self.lookup_var_mut(source_name) {
                    var.is_consumed = true;
                }
                let outer_scope_idx = if self.scopes.len() >= 2 { self.scopes.len() - 2 } else { 0 };
                let exported_info = VarInfo {
                    name: exported_name.clone(),
                    is_lin: false,
                    is_grad: false,
                    is_mut: false,
                    is_consumed: false,
                    is_tainted: false,
                    is_capability: false,
                    def_span: *span,
                    var_type: None,
                    in_region: false,
                };
                self.scopes[outer_scope_idx].vars.insert(exported_name.clone(), exported_info);
            }
            Statement::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.mark_linear_consumed_in_expr(expr);
                    self.check_expr(expr)?;
                }
            }
            Statement::Expr(expr) => {
                self.check_expr(expr)?;
            }
            Statement::Comptime { body, .. } => {
                for s in body {
                    self.check_statement(s)?;
                }
            }
            // Milestone #022: Esolang-Inspired Language Checking
            Statement::InlineVliw { raw_bundles, span } => {
                if raw_bundles.is_empty() {
                    return Err(TypeError::new("E0020", "Empty __vliw_asm__ block: At least one VLIW bundle expected".to_string(), *span));
                }
                let program_text = raw_bundles.join("\n");
                if let Err(e) = crate::cl_lang::verify_cl_program(&program_text) {
                    return Err(TypeError::new("E0021", format!("Inline VLIW verification failure: {}", e), *span));
                }
            }
            Statement::TapeDecl { name, capacity, span, .. } => {
                if *capacity == 0 {
                    return Err(TypeError::new("E0022", format!("Invalid tape capacity for '{}': Capacity must be > 0", name), *span));
                }
                self.insert_var(VarInfo {
                    name: name.clone(),
                    is_lin: false,
                    is_grad: false,
                    is_mut: true,
                    is_consumed: false,
                    is_tainted: false,
                    is_capability: false,
                    def_span: *span,
                    var_type: Some("RingTape".to_string()),
                    in_region: false,
                });
            }
            Statement::TapeStream { target_tape, value, span, .. } => {
                self.check_expr(value)?;
                if self.lookup_var(target_tape).is_none() {
                    return Err(TypeError::new("E0023", format!("Undefined tape stream buffer '{}'", target_tape), *span));
                }
            }
            Statement::SystolicBlock { flows, body, .. } => {
                for f in flows {
                    let d = f.direction.to_uppercase();
                    if d != "EAST" && d != "WEST" && d != "NORTH" && d != "SOUTH" && d != "UP" && d != "DOWN" {
                        return Err(TypeError::new("E0024", format!("Invalid systolic flow direction '{}' for tensor '{}'. Expected EAST, WEST, NORTH, SOUTH", f.direction, f.tensor_name), f.span));
                    }
                }
                self.push_scope();
                self.check_statements(body)?;
                self.pop_scope();
            }
            Statement::RuleDecl { body_exprs, .. } => {
                for e in body_exprs {
                    self.check_expr(e)?;
                }
            }
        }
        Ok(())
    }

    fn mark_linear_consumed_in_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(name, _) => {
                if let Some(var) = self.lookup_var_mut(name) {
                    var.is_consumed = true;
                }
            }
            Expr::Tuple(elements) => {
                for el in elements {
                    self.mark_linear_consumed_in_expr(el);
                }
            }
            _ => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<(), TypeError> {
        match expr {
            Expr::Ident(name, span) => {
                if !self.in_region {
                    if let Some(var) = self.lookup_var(name) {
                        if var.in_region && !self.exported_vars.contains(name) {
                            return Err(TypeError::new(
                                "E0004",
                                format!(
                                    "Region violation: '{}' was declared in a region arena and cannot escape without 'export'",
                                    name
                                ),
                                *span,
                            )
                            .with_note("Scoped region memory is reclaimed in 0 hardware cycles upon region exit")
                            .with_help(format!("Use `export {} as exported_{}` inside the region block", name, name)));
                        }
                    }
                }
            }
            Expr::Consume(name, span) => {
                let root_name = name.split('.').next().unwrap_or(name);
                match self.lookup_var_mut(root_name) {
                    Some(var) => {
                        if var.is_consumed {
                            return Err(TypeError::new(
                                "E0003",
                                format!(
                                    "Linear type violation: '{}' was already consumed and cannot be reused",
                                    name
                                ),
                                *span,
                            )
                            .with_note("Target Architecture: 256-Core 4D-Torus Photonic Neuromorphic Silicon")
                            .with_help("Linear types enforce affine uniqueness (cannot be duplicated or consumed multiple times)"));
                        }
                        var.is_consumed = true;
                    }
                    None => {
                        // Consuming a non-linear or outer variable — allowed
                    }
                }
            }
            Expr::Call { callee, args } => {
                if callee == "secure_patch_icache" {
                    // Taint checking (Pages 749-752)
                    for arg in args {
                        let arg_var_name = match &arg.value {
                            Expr::Ident(n, _) => Some(n.as_str()),
                            Expr::Consume(n, _) => Some(n.as_str()),
                            _ => None,
                        };
                        if let Some(vname) = arg_var_name {
                            if let Some(var) = self.lookup_var(vname) {
                                if var.is_tainted {
                                    let span = match &arg.value {
                                        Expr::Ident(_, sp) => *sp,
                                        Expr::Consume(_, sp) => *sp,
                                        _ => Span::default(),
                                    };
                                    return Err(TypeError::new(
                                        "E0009",
                                        format!("Security violation: Tainted variable '{}' cannot be used in secure_patch_icache without sanitize()", vname),
                                        span,
                                    )
                                    .with_note("Pages 749-752: Hardware Capability and Taint Tracking Zero-Trust Verification")
                                    .with_help(format!("Call `let clean_{} = sanitize({}, min, max)` before patching I-Cache.", vname, vname)));
                                }
                            }
                        }
                    }

                    // Capability token checking (Pages 749-752)
                    let has_capability = args.iter().any(|arg| {
                        match &arg.value {
                            Expr::Ident(n, _) | Expr::Consume(n, _) => {
                                self.lookup_var(n).map(|v| v.is_capability).unwrap_or(false)
                            }
                            _ => false,
                        }
                    });
                    if !has_capability {
                        let span = args.first().map(|a| a.value.span()).unwrap_or_default();
                        return Err(TypeError::new(
                            "E0010",
                            "Security violation: secure_patch_icache requires an acquired capability token",
                            span,
                        )
                        .with_note("Pages 749-752: Capability-based access control enforces least privilege")
                        .with_help("Acquire a capability token using `acquire_capability(...)` before patching."));
                    }
                }

                // Static verification of tensor_matmul contraction dimensions (K_a == K_b)
                if callee == "tensor_matmul" {
                    if args.len() < 2 {
                        let span = args.first().map(|a| a.value.span()).unwrap_or_default();
                        return Err(TypeError::new("E0016", "tensor_matmul requires at least 2 arguments", span));
                    }
                    let type_a = self.infer_expr_type(&args[0].value);
                    let type_b = self.infer_expr_type(&args[1].value);
                    if let (Some(t_a), Some(t_b)) = (type_a, type_b) {
                        if let (Some((dims_a, _)), Some((dims_b, _))) = (Self::parse_tensor_shape(&t_a), Self::parse_tensor_shape(&t_b)) {
                            if dims_a.len() < 2 || dims_b.len() < 2 {
                                return Err(TypeError::new(
                                    "E0016",
                                    format!("tensor_matmul requires tensors of rank >= 2, but received rank {} ({}) and rank {} ({})", dims_a.len(), t_a, dims_b.len(), t_b),
                                    args[0].value.span(),
                                ));
                            }
                            let inner_a = &dims_a[dims_a.len() - 1];
                            let outer_b = &dims_b[dims_b.len() - 2];
                            if inner_a != outer_b {
                                return Err(TypeError::new(
                                    "E0016",
                                    format!("Tensor dimension mismatch: cannot multiply tensor with inner dimension '{}' ({}) by tensor with outer dimension '{}' ({})", inner_a, t_a, outer_b, t_b),
                                    args[1].value.span(),
                                ).with_note("Milestone #010: Dependent Tensor Dimensions guarantee inner contraction dimension equality at compile-time")
                                 .with_help(format!("Ensure inner dimension '{}' equals outer dimension '{}'", inner_a, outer_b)));
                            }
                            let batch_rank_a = dims_a.len() - 2;
                            let batch_rank_b = dims_b.len() - 2;
                            let check_batches = batch_rank_a.min(batch_rank_b);
                            for i in 0..check_batches {
                                if dims_a[i] != dims_b[i] {
                                    return Err(TypeError::new(
                                        "E0016",
                                        format!("Tensor batch dimension mismatch at axis {}: '{}' ({}) vs '{}' ({})", i, dims_a[i], t_a, dims_b[i], t_b),
                                        args[1].value.span(),
                                    ));
                                }
                            }
                        }
                    }
                }

                // Check generic function call site dependent tensor shape unification
                if let Some(expected_params) = self.fn_param_types.get(callee).cloned() {
                    let mut symbol_map: HashMap<String, String> = HashMap::new();
                    for (arg, (_param_name, param_type)) in args.iter().zip(expected_params.iter()) {
                        if let Some(actual_type) = self.infer_expr_type(&arg.value) {
                            if let (Some((p_dims, _p_elem)), Some((a_dims, _a_elem))) = (Self::parse_tensor_shape(param_type), Self::parse_tensor_shape(&actual_type)) {
                                if p_dims.len() != a_dims.len() {
                                    return Err(TypeError::new(
                                        "E0016",
                                        format!("Tensor rank mismatch in call to '{}': parameter expects rank {}, but received rank {}", callee, p_dims.len(), a_dims.len()),
                                        arg.value.span(),
                                    ));
                                }
                                for (p_dim, a_dim) in p_dims.iter().zip(a_dims.iter()) {
                                    let is_generic_symbol = !p_dim.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(true);
                                    if is_generic_symbol {
                                        if let Some(bound_dim) = symbol_map.get(p_dim) {
                                            if bound_dim != a_dim {
                                                return Err(TypeError::new(
                                                    "E0016",
                                                    format!("Tensor generic dimension conflict in call to '{}': symbol '{}' was previously bound to '{}', but argument provides '{}'", callee, p_dim, bound_dim, a_dim),
                                                    arg.value.span(),
                                                ).with_note("Milestone #010: Symbolic Shape Inference enforces consistent dependent dimension unification")
                                                 .with_help(format!("Align tensor dimensions so that symbol '{}' has a single consistent dimension", p_dim)));
                                            }
                                        } else {
                                            symbol_map.insert(p_dim.clone(), a_dim.clone());
                                        }
                                    } else if p_dim != a_dim {
                                        return Err(TypeError::new(
                                            "E0016",
                                            format!("Tensor dimension mismatch in call to '{}': expected dimension '{}', but argument has '{}'", callee, p_dim, a_dim),
                                            arg.value.span(),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }

                for arg in args {
                    self.check_expr(&arg.value)?;
                }
            }
            Expr::MethodCall { object, args, .. } => {
                self.check_expr(object)?;
                for arg in args {
                    self.check_expr(&arg.value)?;
                }
            }
            Expr::Await(inner) | Expr::Spawn(inner) => {
                self.check_expr(inner)?;
            }
            Expr::SpawnAt { core_id, target, .. } => {
                self.check_expr(core_id)?;
                self.check_expr(target)?;
            }
            Expr::ChannelSend { channel, value, .. } => {
                self.check_expr(channel)?;
                self.check_expr(value)?;
            }
            Expr::ChannelRecv { channel, .. } => {
                self.check_expr(channel)?;
            }
            Expr::Array(elements) | Expr::Tuple(elements) => {
                for el in elements {
                    self.check_expr(el)?;
                }
            }
            Expr::Binary { op, left, right } => {
                self.check_expr(left)?;
                self.check_expr(right)?;
                if op == "@" {
                    let type_a = self.infer_expr_type(left);
                    let type_b = self.infer_expr_type(right);
                    if let (Some(t_a), Some(t_b)) = (type_a, type_b) {
                        if let (Some((dims_a, _)), Some((dims_b, _))) = (Self::parse_tensor_shape(&t_a), Self::parse_tensor_shape(&t_b)) {
                            if dims_a.len() < 2 || dims_b.len() < 2 {
                                return Err(TypeError::new(
                                    "E0016",
                                    format!("Matrix multiplication '@' requires tensors of rank >= 2, but received rank {} ({}) and rank {} ({})", dims_a.len(), t_a, dims_b.len(), t_b),
                                    left.span(),
                                ));
                            }
                            let inner_a = &dims_a[dims_a.len() - 1];
                            let outer_b = &dims_b[dims_b.len() - 2];
                            if inner_a != outer_b {
                                return Err(TypeError::new(
                                    "E0016",
                                    format!("Tensor dimension mismatch: cannot multiply tensor with inner dimension '{}' ({}) by tensor with outer dimension '{}' ({})", inner_a, t_a, outer_b, t_b),
                                    right.span(),
                                ).with_note("Milestone #030: First-class '@' matrix operator guarantees inner contraction dimension equality at compile-time")
                                 .with_help(format!("Ensure inner dimension '{}' equals outer dimension '{}'", inner_a, outer_b)));
                            }
                            let batch_rank_a = dims_a.len() - 2;
                            let batch_rank_b = dims_b.len() - 2;
                            let check_batches = batch_rank_a.min(batch_rank_b);
                            for i in 0..check_batches {
                                if dims_a[i] != dims_b[i] {
                                    return Err(TypeError::new(
                                        "E0016",
                                        format!("Tensor batch dimension mismatch at axis {}: '{}' ({}) vs '{}' ({})", i, dims_a[i], t_a, dims_b[i], t_b),
                                        right.span(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Expr::Unary { operand, .. } => {
                self.check_expr(operand)?;
            }
            Expr::Cast { expr, .. } => {
                self.check_expr(expr)?;
            }
            Expr::FieldAccess { object, .. } => {
                self.check_expr(object)?;
            }
            Expr::Index { object, index } => {
                self.check_expr(object)?;
                self.check_expr(index)?;
            }
            Expr::StructInit { fields, .. } => {
                for (_, val) in fields {
                    self.check_expr(val)?;
                }
            }
            Expr::ArrayRepeat { value, count } => {
                self.check_expr(value)?;
                self.check_expr(count)?;
            }
            Expr::IfExpr { condition, then_branch, else_branch } => {
                self.check_expr(condition)?;
                self.check_expr(then_branch)?;
                self.check_expr(else_branch)?;
            }
            Expr::Grad { callee, .. } => {
                self.check_expr(callee)?;
            }
            Expr::GradCall { callee, args, .. } => {
                self.check_expr(callee)?;
                for arg in args {
                    self.check_expr(&arg.value)?;
                }
            }
            Expr::LiteralInt(_) | Expr::LiteralHex(_) | Expr::LiteralFloat(_)
            | Expr::LiteralAxis(_) | Expr::LiteralString(_) | Expr::LiteralBool(_) => {}
            Expr::Comptime { body, result, .. } => {
                for s in body {
                    self.check_statement(s)?;
                }
                if let Some(res) = result {
                    self.check_expr(res)?;
                }
            }
        }
        Ok(())
    }

    fn verify_all_linear_consumed(&self) -> Result<(), TypeError> {
        for scope in &self.scopes {
            for (var_name, var) in &scope.vars {
                if var.is_lin && !var.is_consumed && !self.exported_vars.contains(var_name) {
                    return Err(TypeError::new(
                        "E0002",
                        format!(
                            "Linear type leak: Linear variable '{}' was allocated but never consumed or exported",
                            var_name
                        ),
                        var.def_span,
                    )
                    .with_note("Target Architecture: 256-Core 4D-Torus Photonic Neuromorphic Silicon")
                    .with_help("Linear resources (`lin`) must be consumed exactly once using `consume(...)` or exported"));
                }
            }
        }
        Ok(())
    }
}
