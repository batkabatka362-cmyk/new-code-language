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
    exported_vars: HashSet<String>,
    region_allocated_vars: HashMap<String, VarInfo>,
    known_functions: HashSet<String>,
    known_structs: HashMap<String, HashMap<String, String>>,
    known_types: HashMap<String, String>,
    /// Trait name -> method signatures (for contract verification)
    known_traits: HashMap<String, Vec<(String, Vec<String>, Option<String>)>>,
    /// (trait_name, struct_name) -> method names (for monomorphization lookup)
    known_impls: HashMap<(String, String), HashSet<String>>,
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
        ] {
            known_functions.insert(f.to_string());
        }

        let mut known_types = HashMap::new();
        for t in &["vec4f", "vec8f", "vec16f", "vec4i", "vec8i", "channel", "Channel"] {
            known_types.insert(t.to_string(), "builtin".to_string());
        }

        Self {
            scopes: vec![Scope::new()],
            in_region: false,
            exported_vars: HashSet::new(),
            region_allocated_vars: HashMap::new(),
            known_functions,
            known_structs: HashMap::new(),
            known_types,
            known_traits: HashMap::new(),
            known_impls: HashMap::new(),
        }
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
        // Register all function names
        for func in &program.functions {
            self.known_functions.insert(func.name.clone());
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
            brain_checker.check_statements(&brain.body)?;
            brain_checker.verify_all_linear_consumed()?;
        }

        // Check main statements
        self.check_statements(&program.main_statements)?;
        self.verify_all_linear_consumed()?;

        Ok(())
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

                self.insert_var(VarInfo {
                    name: name.clone(),
                    is_lin: *is_lin,
                    is_grad: *is_grad,
                    is_mut: *is_mut,
                    is_consumed: false,
                    is_tainted,
                    is_capability,
                    def_span: *span,
                    var_type: type_annot.clone(),
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
            Expr::Binary { left, right, .. } => {
                self.check_expr(left)?;
                self.check_expr(right)?;
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
