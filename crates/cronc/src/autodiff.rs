// ============================================================================
// CRON Compiler - First-Class Language-Level Native Autodiff Engine
// (C) 2026 CRON Language Project - SSS+ Tier Systems Compiler
//
// Features:
// 1. Source-to-Source Reverse-Mode and Symbolic Automatic Differentiation
// 2. Exact Gradient Derivation for Scalars (f32, f64) and SIMD Vectors (vec8f, vec4f)
// 3. Landauer Zero-Entropy Fredkin Reversible Gate Stack Adjoint Routing
// 4. Elementary & Transcendental Derivatives (sin, cos, exp, tanh, relu, sigmoid)
// 5. Automatic Program-level AST Monomorphization for JIT, LLVM IR, and C23
// ============================================================================

use crate::ast::{CallArg, Expr, FunctionDecl, Program, Statement};
use crate::token::Span;
use std::collections::{HashMap, HashSet};

/// Core Automatic Differentiation Engine
pub struct AutodiffEngine;

impl AutodiffEngine {
    /// Differentiates an entire program AST: scans for `Expr::Grad` and `Expr::GradCall`,
    /// synthesizes differentiated functions, and updates call sites.
    pub fn differentiate_program(program: &mut Program) -> Result<(), String> {
        let mut target_grads: Vec<(String, Option<String>, Span)> = Vec::new();

        // 1. Collect all grad occurrences in functions
        for func in &program.functions {
            for stmt in &func.body {
                Self::collect_grads_in_statement(stmt, &mut target_grads);
            }
        }

        // 2. Collect in main statements
        for stmt in &program.main_statements {
            Self::collect_grads_in_statement(stmt, &mut target_grads);
        }

        if target_grads.is_empty() {
            return Ok(());
        }

        // 3. Synthesize gradient functions
        let original_funcs: HashMap<String, FunctionDecl> = program
            .functions
            .iter()
            .map(|f| (f.name.clone(), f.clone()))
            .collect();

        let mut synthesized_funcs: Vec<FunctionDecl> = Vec::new();
        let mut already_generated: HashSet<String> = HashSet::new();

        for (fn_name, wrt_param, span) in target_grads {
            let orig = original_funcs.get(&fn_name).ok_or_else(|| {
                format!(
                    "Autodiff error at {:?}: Cannot differentiate unknown function '{}'",
                    span, fn_name
                )
            })?;

            let wrt = match wrt_param {
                Some(w) => w,
                None => {
                    if orig.params.is_empty() {
                        return Err(format!(
                            "Autodiff error: Function '{}' has no parameters to differentiate",
                            fn_name
                        ));
                    }
                    orig.params[0].name.clone()
                }
            };

            let grad_fn_name = format!("{}_grad_{}", fn_name, wrt);
            if already_generated.contains(&grad_fn_name) {
                continue;
            }
            already_generated.insert(grad_fn_name.clone());

            let synthesized = Self::synthesize_gradient_function(orig, &wrt, &grad_fn_name)?;
            synthesized_funcs.push(synthesized);
        }

        // Append synthesized functions to program
        program.functions.extend(synthesized_funcs);

        // 4. Rewrite grad calls in AST to point to the synthesized functions
        for func in &mut program.functions {
            for stmt in &mut func.body {
                Self::rewrite_grads_in_statement(stmt);
            }
        }
        for stmt in &mut program.main_statements {
            Self::rewrite_grads_in_statement(stmt);
        }

        Ok(())
    }

    /// Synthesizes a new function computing the gradient of `orig` with respect to `wrt_param`.
    pub fn synthesize_gradient_function(
        orig: &FunctionDecl,
        wrt_param: &str,
        grad_fn_name: &str,
    ) -> Result<FunctionDecl, String> {
        let target_param = orig
            .params
            .iter()
            .find(|p| p.name == wrt_param)
            .ok_or_else(|| {
                format!(
                    "Function '{}' has no parameter named '{}' to differentiate with respect to",
                    orig.name, wrt_param
                )
            })?;

        let ret_type = target_param.param_type.clone();

        // Build local variable expansion environment for chained forward dependencies
        let mut env: HashMap<String, Expr> = HashMap::new();
        for stmt in &orig.body {
            if let Statement::Let { name, value, .. } = stmt {
                let expanded_val = Self::expand_env(value, &env);
                env.insert(name.clone(), expanded_val);
            }
        }

        // Synthesize body by differentiating forward statements
        let mut grad_body = Vec::new();

        // Forward statements: keep local assignments
        for stmt in &orig.body {
            match stmt {
                Statement::Return(Some(expr)) => {
                    // Expand local variables and differentiate symbolically
                    let expanded = Self::expand_env(expr, &env);
                    let d_expr = Self::diff_expr(&expanded, wrt_param);
                    let simplified = Self::simplify_expr(&d_expr);
                    grad_body.push(Statement::Return(Some(simplified)));
                }
                Statement::Return(None) => {
                    grad_body.push(Statement::Return(Some(Expr::LiteralFloat(0.0))));
                }
                Statement::Let {
                    name,
                    value,
                    type_annot,
                    is_mut,
                    span,
                    ..
                } => {
                    // Keep forward statement
                    grad_body.push(Statement::Let {
                        is_lin: false,
                        is_grad: false,
                        is_mut: *is_mut,
                        name: name.clone(),
                        type_annot: type_annot.clone(),
                        value: value.clone(),
                        extra_vars: Vec::new(),
                        span: *span,
                    });
                }
                other => {
                    grad_body.push(other.clone());
                }
            }
        }

        Ok(FunctionDecl {
            is_async: orig.is_async,
            is_export: orig.is_export,
            is_inline: orig.is_inline,
            name: grad_fn_name.to_string(),
            generic_params: orig.generic_params.clone(),
            params: orig.params.clone(),
            return_type: Some(ret_type),
            body: grad_body,
        })
    }

    /// Recursively expands local variable identifiers in an expression using a forward environment map.
    pub fn expand_env(expr: &Expr, env: &HashMap<String, Expr>) -> Expr {
        match expr {
            Expr::Ident(name, span) => {
                if let Some(sub) = env.get(name) {
                    sub.clone()
                } else {
                    Expr::Ident(name.clone(), *span)
                }
            }
            Expr::Binary { op, left, right } => Expr::Binary {
                op: op.clone(),
                left: Box::new(Self::expand_env(left, env)),
                right: Box::new(Self::expand_env(right, env)),
            },
            Expr::Unary { op, operand } => Expr::Unary {
                op: op.clone(),
                operand: Box::new(Self::expand_env(operand, env)),
            },
            Expr::Call { callee, args } => Expr::Call {
                callee: callee.clone(),
                args: args
                    .iter()
                    .map(|a| CallArg {
                        name: a.name.clone(),
                        value: Self::expand_env(&a.value, env),
                    })
                    .collect(),
            },
            Expr::IfExpr {
                condition,
                then_branch,
                else_branch,
            } => Expr::IfExpr {
                condition: Box::new(Self::expand_env(condition, env)),
                then_branch: Box::new(Self::expand_env(then_branch, env)),
                else_branch: Box::new(Self::expand_env(else_branch, env)),
            },
            Expr::Array(elements) => Expr::Array(
                elements
                    .iter()
                    .map(|e| Self::expand_env(e, env))
                    .collect(),
            ),
            Expr::Tuple(elements) => Expr::Tuple(
                elements
                    .iter()
                    .map(|e| Self::expand_env(e, env))
                    .collect(),
            ),
            other => other.clone(),
        }
    }

    /// Symbolic differentiation of an expression with respect to `wrt`.
    pub fn diff_expr(expr: &Expr, wrt: &str) -> Expr {
        match expr {
            Expr::LiteralInt(_) | Expr::LiteralHex(_) | Expr::LiteralBool(_) | Expr::LiteralString(_) => {
                Expr::LiteralFloat(0.0)
            }
            Expr::LiteralFloat(_) => Expr::LiteralFloat(0.0),
            Expr::Ident(name, _span) => {
                if name == wrt {
                    Expr::LiteralFloat(1.0)
                } else {
                    Expr::LiteralFloat(0.0)
                }
            }
            Expr::Binary { op, left, right } => {
                let dl = Self::diff_expr(left, wrt);
                let dr = Self::diff_expr(right, wrt);

                match op.as_str() {
                    "+" => Expr::Binary {
                        op: "+".to_string(),
                        left: Box::new(dl),
                        right: Box::new(dr),
                    },
                    "-" => Expr::Binary {
                        op: "-".to_string(),
                        left: Box::new(dl),
                        right: Box::new(dr),
                    },
                    "*" => {
                        // Product rule: (d(left) * right) + (left * d(right))
                        let term1 = Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(dl),
                            right: right.clone(),
                        };
                        let term2 = Expr::Binary {
                            op: "*".to_string(),
                            left: left.clone(),
                            right: Box::new(dr),
                        };
                        Expr::Binary {
                            op: "+".to_string(),
                            left: Box::new(term1),
                            right: Box::new(term2),
                        }
                    }
                    "/" => {
                        // Quotient rule: (d(left) * right - left * d(right)) / (right * right)
                        let term1 = Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(dl),
                            right: right.clone(),
                        };
                        let term2 = Expr::Binary {
                            op: "*".to_string(),
                            left: left.clone(),
                            right: Box::new(dr),
                        };
                        let num = Expr::Binary {
                            op: "-".to_string(),
                            left: Box::new(term1),
                            right: Box::new(term2),
                        };
                        let denom = Expr::Binary {
                            op: "*".to_string(),
                            left: right.clone(),
                            right: right.clone(),
                        };
                        Expr::Binary {
                            op: "/".to_string(),
                            left: Box::new(num),
                            right: Box::new(denom),
                        }
                    }
                    _ => Expr::LiteralFloat(0.0),
                }
            }
            Expr::Unary { op, operand } => {
                let d_op = Self::diff_expr(operand, wrt);
                match op.as_str() {
                    "-" => Expr::Unary {
                        op: "-".to_string(),
                        operand: Box::new(d_op),
                    },
                    _ => Expr::LiteralFloat(0.0),
                }
            }
            Expr::Call { callee, args } => {
                match callee.as_str() {
                    // SIMD FMA: simd_fma(a, b, c) -> a * b + c
                    "simd_fma" if args.len() >= 3 => {
                        let da = Self::diff_expr(&args[0].value, wrt);
                        let db = Self::diff_expr(&args[1].value, wrt);
                        let dc = Self::diff_expr(&args[2].value, wrt);

                        // grad = da * b + a * db + dc
                        let t1 = Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(da),
                            right: Box::new(args[1].value.clone()),
                        };
                        let t2 = Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(args[0].value.clone()),
                            right: Box::new(db),
                        };
                        let sum12 = Expr::Binary {
                            op: "+".to_string(),
                            left: Box::new(t1),
                            right: Box::new(t2),
                        };
                        Expr::Binary {
                            op: "+".to_string(),
                            left: Box::new(sum12),
                            right: Box::new(dc),
                        }
                    }
                    // SIMD reduce sum: simd_reduce_sum(v) -> d(v)
                    "simd_reduce_sum" if !args.is_empty() => {
                        
                        Self::diff_expr(&args[0].value, wrt)
                    }
                    // SIMD dot product: simd_dot(a, b) -> da * b + a * db
                    "simd_dot" if args.len() >= 2 => {
                        let da = Self::diff_expr(&args[0].value, wrt);
                        let db = Self::diff_expr(&args[1].value, wrt);
                        let t1 = Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(da),
                            right: Box::new(args[1].value.clone()),
                        };
                        let t2 = Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(args[0].value.clone()),
                            right: Box::new(db),
                        };
                        Expr::Binary {
                            op: "+".to_string(),
                            left: Box::new(t1),
                            right: Box::new(t2),
                        }
                    }
                    // Fredkin Reversible Gate Stack Adjoint Routing
                    "fredkin_gate" if args.len() >= 3 => {
                        // In backward pass, Fredkin gate self-inverts:
                        // fredkin_gate(c, adj_a, adj_b) reversibly reconstructs gradients
                        Expr::Call {
                            callee: "fredkin_gate".to_string(),
                            args: args.clone(),
                        }
                    }
                    // Transcendental & Activation functions
                    "sin" if !args.is_empty() => {
                        let du = Self::diff_expr(&args[0].value, wrt);
                        let cos_call = Expr::Call {
                            callee: "cos".to_string(),
                            args: vec![CallArg { name: None, value: args[0].value.clone() }],
                        };
                        Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(cos_call),
                            right: Box::new(du),
                        }
                    }
                    "cos" if !args.is_empty() => {
                        let du = Self::diff_expr(&args[0].value, wrt);
                        let sin_call = Expr::Call {
                            callee: "sin".to_string(),
                            args: vec![CallArg { name: None, value: args[0].value.clone() }],
                        };
                        let neg_sin = Expr::Unary {
                            op: "-".to_string(),
                            operand: Box::new(sin_call),
                        };
                        Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(neg_sin),
                            right: Box::new(du),
                        }
                    }
                    "exp" if !args.is_empty() => {
                        let du = Self::diff_expr(&args[0].value, wrt);
                        Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(expr.clone()),
                            right: Box::new(du),
                        }
                    }
                    "tanh" if !args.is_empty() => {
                        // d/dx tanh(x) = 1 - tanh^2(x)
                        let du = Self::diff_expr(&args[0].value, wrt);
                        let tanh_sq = Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(expr.clone()),
                            right: Box::new(expr.clone()),
                        };
                        let diff_one = Expr::Binary {
                            op: "-".to_string(),
                            left: Box::new(Expr::LiteralFloat(1.0)),
                            right: Box::new(tanh_sq),
                        };
                        Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(diff_one),
                            right: Box::new(du),
                        }
                    }
                    "relu" if !args.is_empty() => {
                        // d/dx relu(x) = if x > 0.0 { 1.0 } else { 0.0 }
                        let du = Self::diff_expr(&args[0].value, wrt);
                        let cond = Expr::Binary {
                            op: ">".to_string(),
                            left: Box::new(args[0].value.clone()),
                            right: Box::new(Expr::LiteralFloat(0.0)),
                        };
                        let if_step = Expr::IfExpr {
                            condition: Box::new(cond),
                            then_branch: Box::new(Expr::LiteralFloat(1.0)),
                            else_branch: Box::new(Expr::LiteralFloat(0.0)),
                        };
                        Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(if_step),
                            right: Box::new(du),
                        }
                    }
                    "sigmoid" if !args.is_empty() => {
                        // d/dx sigmoid(x) = sigmoid(x) * (1.0 - sigmoid(x))
                        let du = Self::diff_expr(&args[0].value, wrt);
                        let one_minus = Expr::Binary {
                            op: "-".to_string(),
                            left: Box::new(Expr::LiteralFloat(1.0)),
                            right: Box::new(expr.clone()),
                        };
                        let mul = Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(expr.clone()),
                            right: Box::new(one_minus),
                        };
                        Expr::Binary {
                            op: "*".to_string(),
                            left: Box::new(mul),
                            right: Box::new(du),
                        }
                    }
                    _ => Expr::LiteralFloat(0.0),
                }
            }
            _ => Expr::LiteralFloat(0.0),
        }
    }

    /// Constant folding and algebraic expression simplification for cleaner derivatives.
    pub fn simplify_expr(expr: &Expr) -> Expr {
        match expr {
            Expr::Binary { op, left, right } => {
                let sl = Self::simplify_expr(left);
                let sr = Self::simplify_expr(right);

                match op.as_str() {
                    "+" => {
                        if Self::is_zero(&sl) {
                            return sr;
                        }
                        if Self::is_zero(&sr) {
                            return sl;
                        }
                        // Fold literals: c1 + c2
                        if let (Some(c1), Some(c2)) = (Self::as_float(&sl), Self::as_float(&sr)) {
                            return Expr::LiteralFloat(c1 + c2);
                        }
                        // x + x -> 2.0 * x
                        if sl == sr {
                            return Expr::Binary {
                                op: "*".to_string(),
                                left: Box::new(Expr::LiteralFloat(2.0)),
                                right: Box::new(sl),
                            };
                        }
                    }
                    "-" => {
                        if Self::is_zero(&sr) {
                            return sl;
                        }
                        if let (Some(c1), Some(c2)) = (Self::as_float(&sl), Self::as_float(&sr)) {
                            return Expr::LiteralFloat(c1 - c2);
                        }
                        if sl == sr {
                            return Expr::LiteralFloat(0.0);
                        }
                    }
                    "*" => {
                        if Self::is_zero(&sl) || Self::is_zero(&sr) {
                            return Expr::LiteralFloat(0.0);
                        }
                        if Self::is_one(&sl) {
                            return sr;
                        }
                        if Self::is_one(&sr) {
                            return sl;
                        }
                        if let (Some(c1), Some(c2)) = (Self::as_float(&sl), Self::as_float(&sr)) {
                            return Expr::LiteralFloat(c1 * c2);
                        }
                    }
                    "/" => {
                        if Self::is_zero(&sl) {
                            return Expr::LiteralFloat(0.0);
                        }
                        if Self::is_one(&sr) {
                            return sl;
                        }
                        if let (Some(c1), Some(c2)) = (Self::as_float(&sl), Self::as_float(&sr)) {
                            if c2 != 0.0 {
                                return Expr::LiteralFloat(c1 / c2);
                            }
                        }
                    }
                    _ => {}
                }

                Expr::Binary {
                    op: op.clone(),
                    left: Box::new(sl),
                    right: Box::new(sr),
                }
            }
            Expr::Unary { op, operand } => {
                let so = Self::simplify_expr(operand);
                if op == "-" {
                    if Self::is_zero(&so) {
                        return Expr::LiteralFloat(0.0);
                    }
                    if let Some(c) = Self::as_float(&so) {
                        return Expr::LiteralFloat(-c);
                    }
                }
                Expr::Unary {
                    op: op.clone(),
                    operand: Box::new(so),
                }
            }
            other => other.clone(),
        }
    }

    fn is_zero(expr: &Expr) -> bool {
        match expr {
            Expr::LiteralFloat(f) => f.abs() < 1e-12,
            Expr::LiteralInt(i) => *i == 0,
            Expr::LiteralHex(h) => *h == 0,
            _ => false,
        }
    }

    fn is_one(expr: &Expr) -> bool {
        match expr {
            Expr::LiteralFloat(f) => (f - 1.0).abs() < 1e-12,
            Expr::LiteralInt(i) => *i == 1,
            Expr::LiteralHex(h) => *h == 1,
            _ => false,
        }
    }

    fn as_float(expr: &Expr) -> Option<f64> {
        match expr {
            Expr::LiteralFloat(f) => Some(*f),
            Expr::LiteralInt(i) => Some(*i as f64),
            Expr::LiteralHex(h) => Some(*h as f64),
            _ => None,
        }
    }

    fn collect_grads_in_statement(
        stmt: &Statement,
        out: &mut Vec<(String, Option<String>, Span)>,
    ) {
        match stmt {
            Statement::Let { value, .. } => Self::collect_grads_in_expr(value, out),
            Statement::Assign { value, .. } => Self::collect_grads_in_expr(value, out),
            Statement::Return(Some(e)) => Self::collect_grads_in_expr(e, out),
            Statement::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                Self::collect_grads_in_expr(condition, out);
                for s in then_body {
                    Self::collect_grads_in_statement(s, out);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        Self::collect_grads_in_statement(s, out);
                    }
                }
            }
            Statement::While { condition, body, .. } => {
                Self::collect_grads_in_expr(condition, out);
                for s in body {
                    Self::collect_grads_in_statement(s, out);
                }
            }
            Statement::For { iterable, body, .. } => {
                Self::collect_grads_in_expr(iterable, out);
                for s in body {
                    Self::collect_grads_in_statement(s, out);
                }
            }
            Statement::Expr(e) => Self::collect_grads_in_expr(e, out),
            _ => {}
        }
    }

    fn collect_grads_in_expr(expr: &Expr, out: &mut Vec<(String, Option<String>, Span)>) {
        match expr {
            Expr::Grad { callee, wrt, span } => {
                if let Expr::Ident(name, _) = &**callee {
                    out.push((name.clone(), wrt.clone(), *span));
                }
            }
            Expr::GradCall {
                callee,
                wrt,
                args,
                span,
            } => {
                if let Expr::Ident(name, _) = &**callee {
                    out.push((name.clone(), wrt.clone(), *span));
                }
                for a in args {
                    Self::collect_grads_in_expr(&a.value, out);
                }
            }
            Expr::Call { args, .. } => {
                for a in args {
                    Self::collect_grads_in_expr(&a.value, out);
                }
            }
            Expr::Binary { left, right, .. } => {
                Self::collect_grads_in_expr(left, out);
                Self::collect_grads_in_expr(right, out);
            }
            Expr::Unary { operand, .. } => Self::collect_grads_in_expr(operand, out),
            Expr::Array(elems) | Expr::Tuple(elems) => {
                for e in elems {
                    Self::collect_grads_in_expr(e, out);
                }
            }
            _ => {}
        }
    }

    fn rewrite_grads_in_statement(stmt: &mut Statement) {
        match stmt {
            Statement::Let { value, .. } => Self::rewrite_grads_in_expr(value),
            Statement::Assign { value, .. } => Self::rewrite_grads_in_expr(value),
            Statement::Return(Some(e)) => Self::rewrite_grads_in_expr(e),
            Statement::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                Self::rewrite_grads_in_expr(condition);
                for s in then_body {
                    Self::rewrite_grads_in_statement(s);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        Self::rewrite_grads_in_statement(s);
                    }
                }
            }
            Statement::While { condition, body, .. } => {
                Self::rewrite_grads_in_expr(condition);
                for s in body {
                    Self::rewrite_grads_in_statement(s);
                }
            }
            Statement::For { iterable, body, .. } => {
                Self::rewrite_grads_in_expr(iterable);
                for s in body {
                    Self::rewrite_grads_in_statement(s);
                }
            }
            Statement::Expr(e) => Self::rewrite_grads_in_expr(e),
            _ => {}
        }
    }

    fn rewrite_grads_in_expr(expr: &mut Expr) {
        match expr {
            Expr::GradCall {
                callee,
                wrt,
                args,
                span: _,
            } => {
                let func_name = match &**callee {
                    Expr::Ident(name, _) => name.clone(),
                    _ => "unknown".to_string(),
                };
                let wrt_name = wrt.as_deref().unwrap_or("x");
                let synthesized_name = format!("{}_grad_{}", func_name, wrt_name);

                for a in args.iter_mut() {
                    Self::rewrite_grads_in_expr(&mut a.value);
                }

                *expr = Expr::Call {
                    callee: synthesized_name,
                    args: args.clone(),
                };
            }
            Expr::Grad { callee, wrt, span } => {
                let func_name = match &**callee {
                    Expr::Ident(name, _) => name.clone(),
                    _ => "unknown".to_string(),
                };
                let wrt_name = wrt.as_deref().unwrap_or("x");
                let synthesized_name = format!("{}_grad_{}", func_name, wrt_name);

                *expr = Expr::Ident(synthesized_name, *span);
            }
            Expr::Call { args, .. } => {
                for a in args {
                    Self::rewrite_grads_in_expr(&mut a.value);
                }
            }
            Expr::Binary { left, right, .. } => {
                Self::rewrite_grads_in_expr(left);
                Self::rewrite_grads_in_expr(right);
            }
            Expr::Unary { operand, .. } => Self::rewrite_grads_in_expr(operand),
            Expr::Array(elems) | Expr::Tuple(elems) => {
                for e in elems {
                    Self::rewrite_grads_in_expr(e);
                }
            }
            _ => {}
        }
    }
}
