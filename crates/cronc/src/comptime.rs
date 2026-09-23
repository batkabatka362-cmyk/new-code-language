// ============================================================================
// CRON Compile-Time Metaprogramming (comptime) Engine
// Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
// Zero-overhead compile-time execution, loop unrolling, and photonic MZI
// phase LUT precomputation directly into static AST literals.
// ============================================================================

use std::collections::HashMap;
use crate::ast::*;
use crate::token::Span;

/// Values manipulated during compile-time metaprogramming execution.
#[derive(Debug, Clone, PartialEq)]
pub enum ComptimeValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<ComptimeValue>),
    Void,
}

impl ComptimeValue {
    pub fn to_expr(&self) -> Expr {
        match self {
            ComptimeValue::Int(i) => Expr::LiteralInt(*i),
            ComptimeValue::Float(f) => Expr::LiteralFloat(*f),
            ComptimeValue::Bool(b) => Expr::LiteralBool(*b),
            ComptimeValue::String(s) => Expr::LiteralString(s.clone()),
            ComptimeValue::Array(elems) => {
                Expr::Array(elems.iter().map(|e| e.to_expr()).collect())
            }
            ComptimeValue::Void => Expr::LiteralInt(0),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            ComptimeValue::Int(_) => "int",
            ComptimeValue::Float(_) => "float",
            ComptimeValue::Bool(_) => "bool",
            ComptimeValue::String(_) => "string",
            ComptimeValue::Array(_) => "array",
            ComptimeValue::Void => "void",
        }
    }

    pub fn as_f64(&self) -> Result<f64, String> {
        match self {
            ComptimeValue::Float(f) => Ok(*f),
            ComptimeValue::Int(i) => Ok(*i as f64),
            other => Err(format!("Cannot convert {} to float", other.type_name())),
        }
    }

    pub fn as_i64(&self) -> Result<i64, String> {
        match self {
            ComptimeValue::Int(i) => Ok(*i),
            ComptimeValue::Float(f) => Ok(*f as i64),
            ComptimeValue::Bool(b) => Ok(if *b { 1 } else { 0 }),
            other => Err(format!("Cannot convert {} to int", other.type_name())),
        }
    }
}

/// Evaluation environment for compile-time constants and variables.
#[derive(Debug, Clone)]
pub struct ComptimeEnv {
    scopes: Vec<HashMap<String, ComptimeValue>>,
    iteration_count: usize,
    max_iterations: usize,
}

impl Default for ComptimeEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl ComptimeEnv {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            iteration_count: 0,
            max_iterations: 100_000,
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, val: ComptimeValue) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, val);
        }
    }

    pub fn get(&self, name: &str) -> Option<&ComptimeValue> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val);
            }
        }
        None
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut ComptimeValue> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                return scope.get_mut(name);
            }
        }
        None
    }

    pub fn set(&mut self, name: &str, val: ComptimeValue) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), val);
                return true;
            }
        }
        false
    }

    pub fn tick(&mut self, span: Span) -> Result<(), String> {
        self.iteration_count += 1;
        if self.iteration_count > self.max_iterations {
            Err(format!(
                "Line {}:{}: Compile-time loop iteration limit exceeded ({} iterations)",
                span.line, span.col, self.max_iterations
            ))
        } else {
            Ok(())
        }
    }
}

pub struct ComptimeEvaluator<'a> {
    functions: &'a [FunctionDecl],
}

impl<'a> ComptimeEvaluator<'a> {
    pub fn new(functions: &'a [FunctionDecl]) -> Self {
        Self { functions }
    }

    /// Evaluates a comptime block `comptime { body; result }`
    pub fn eval_block(
        &self,
        body: &[Statement],
        result_expr: Option<&Expr>,
        env: &mut ComptimeEnv,
    ) -> Result<ComptimeValue, String> {
        env.push_scope();

        for stmt in body {
            if let Some(ret_val) = self.eval_statement(stmt, env)? {
                env.pop_scope();
                return Ok(ret_val);
            }
        }

        let final_val = if let Some(expr) = result_expr {
            self.eval_expr(expr, env)?
        } else {
            ComptimeValue::Void
        };

        env.pop_scope();
        Ok(final_val)
    }

    pub fn eval_statement(
        &self,
        stmt: &Statement,
        env: &mut ComptimeEnv,
    ) -> Result<Option<ComptimeValue>, String> {
        match stmt {
            Statement::Let { name, value, extra_vars, .. } => {
                let val = self.eval_expr(value, env)?;
                env.define(name.clone(), val.clone());
                for (_, _, ex_name, _) in extra_vars {
                    env.define(ex_name.clone(), val.clone());
                }
                Ok(None)
            }
            Statement::Assign { target, value, span } => {
                let val = self.eval_expr(value, env)?;
                if !env.set(target, val) {
                    return Err(format!(
                        "Line {}:{}: Undefined variable '{}' in compile-time assignment",
                        span.line, span.col, target
                    ));
                }
                Ok(None)
            }
            Statement::Expr(expr) => {
                match expr {
                    Expr::Index { object, index } => {
                        let _ = (object, index);
                    }
                    _ => {
                        self.eval_expr(expr, env)?;
                    }
                }
                Ok(None)
            }
            Statement::Return(opt_expr) => {
                let val = if let Some(e) = opt_expr {
                    self.eval_expr(e, env)?
                } else {
                    ComptimeValue::Void
                };
                Ok(Some(val))
            }
            Statement::If { condition, then_body, else_body, .. } => {
                let cond_val = self.eval_expr(condition, env)?;
                let is_true = match cond_val {
                    ComptimeValue::Bool(b) => b,
                    ComptimeValue::Int(i) => i != 0,
                    _ => return Err("Expected boolean or integer condition in comptime if".to_string()),
                };

                if is_true {
                    for s in then_body {
                        if let Some(ret) = self.eval_statement(s, env)? {
                            return Ok(Some(ret));
                        }
                    }
                } else if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        if let Some(ret) = self.eval_statement(s, env)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }
            Statement::While { condition, body, span } => {
                loop {
                    env.tick(*span)?;
                    let cond_val = self.eval_expr(condition, env)?;
                    let is_true = match cond_val {
                        ComptimeValue::Bool(b) => b,
                        ComptimeValue::Int(i) => i != 0,
                        _ => return Err("Expected boolean or integer condition in comptime while".to_string()),
                    };
                    if !is_true {
                        break;
                    }
                    for s in body {
                        if let Some(ret) = self.eval_statement(s, env)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }
            Statement::For { var_name, iterable, body, span } => {
                let iter_val = self.eval_expr(iterable, env)?;
                let elements = match iter_val {
                    ComptimeValue::Array(items) => items,
                    _ => return Err("Expected array or range in comptime for-in loop".to_string()),
                };

                for item in elements {
                    env.tick(*span)?;
                    env.push_scope();
                    env.define(var_name.clone(), item);
                    for s in body {
                        if let Some(ret) = self.eval_statement(s, env)? {
                            env.pop_scope();
                            return Ok(Some(ret));
                        }
                    }
                    env.pop_scope();
                }
                Ok(None)
            }
            Statement::Comptime { body, .. } => {
                for s in body {
                    if let Some(ret) = self.eval_statement(s, env)? {
                        return Ok(Some(ret));
                    }
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub fn eval_expr(
        &self,
        expr: &Expr,
        env: &mut ComptimeEnv,
    ) -> Result<ComptimeValue, String> {
        match expr {
            Expr::LiteralInt(i) => Ok(ComptimeValue::Int(*i)),
            Expr::LiteralHex(h) => Ok(ComptimeValue::Int(*h as i64)),
            Expr::LiteralFloat(f) => Ok(ComptimeValue::Float(*f)),
            Expr::LiteralBool(b) => Ok(ComptimeValue::Bool(*b)),
            Expr::LiteralString(s) => Ok(ComptimeValue::String(s.clone())),
            Expr::Ident(name, span) => {
                if let Some(val) = env.get(name) {
                    Ok(val.clone())
                } else if name == "PI" || name == "pi" {
                    Ok(ComptimeValue::Float(std::f64::consts::PI))
                } else if name == "E" || name == "e" {
                    Ok(ComptimeValue::Float(std::f64::consts::E))
                } else {
                    Err(format!(
                        "Line {}:{}: Undefined compile-time variable '{}'",
                        span.line, span.col, name
                    ))
                }
            }
            Expr::Array(items) => {
                let mut res = Vec::new();
                for item in items {
                    res.push(self.eval_expr(item, env)?);
                }
                Ok(ComptimeValue::Array(res))
            }
            Expr::ArrayRepeat { value, count } => {
                let val = self.eval_expr(value, env)?;
                let count_val = self.eval_expr(count, env)?.as_i64()?;
                if count_val < 0 {
                    return Err("Negative repeat count in compile-time array repeat".to_string());
                }
                let count_usize = count_val as usize;
                if count_usize > 1_000_000 {
                    return Err(format!("Array repeat count {} exceeds compile-time threshold", count_usize));
                }
                Ok(ComptimeValue::Array(vec![val; count_usize]))
            }
            Expr::Unary { op, operand } => {
                let val = self.eval_expr(operand, env)?;
                match op.as_str() {
                    "-" => match val {
                        ComptimeValue::Int(i) => Ok(ComptimeValue::Int(-i)),
                        ComptimeValue::Float(f) => Ok(ComptimeValue::Float(-f)),
                        other => Err(format!("Unary '-' not supported on {}", other.type_name())),
                    },
                    "!" | "not" => match val {
                        ComptimeValue::Bool(b) => Ok(ComptimeValue::Bool(!b)),
                        ComptimeValue::Int(i) => Ok(ComptimeValue::Bool(i == 0)),
                        other => Err(format!("Unary 'not' not supported on {}", other.type_name())),
                    },
                    _ => Err(format!("Unsupported compile-time unary operator '{}'", op)),
                }
            }
            Expr::Binary { op, left, right } => {
                // Range expression: start..end
                if op == ".." {
                    let start = self.eval_expr(left, env)?.as_i64()?;
                    let end = self.eval_expr(right, env)?.as_i64()?;
                    if end < start {
                        return Ok(ComptimeValue::Array(Vec::new()));
                    }
                    let count = (end - start) as usize;
                    if count > 100_000 {
                        return Err(format!("Range size {} exceeds comptime limit", count));
                    }
                    let items = (start..end).map(ComptimeValue::Int).collect();
                    return Ok(ComptimeValue::Array(items));
                }

                let left_val = self.eval_expr(left, env)?;
                let right_val = self.eval_expr(right, env)?;

                self.eval_binary_op(op, &left_val, &right_val)
            }
            Expr::Cast { expr, target_type } => {
                let val = self.eval_expr(expr, env)?;
                match target_type.as_str() {
                    "f64" | "f32" | "float" => Ok(ComptimeValue::Float(val.as_f64()?)),
                    "i64" | "i32" | "u64" | "u32" | "int" => Ok(ComptimeValue::Int(val.as_i64()?)),
                    "bool" => Ok(ComptimeValue::Bool(val.as_i64()? != 0)),
                    _ => Ok(val),
                }
            }
            Expr::IfExpr { condition, then_branch, else_branch } => {
                let cond_val = self.eval_expr(condition, env)?;
                let is_true = match cond_val {
                    ComptimeValue::Bool(b) => b,
                    ComptimeValue::Int(i) => i != 0,
                    _ => return Err("Expected boolean condition in comptime if expression".to_string()),
                };
                if is_true {
                    self.eval_expr(then_branch, env)
                } else {
                    self.eval_expr(else_branch, env)
                }
            }
            Expr::Index { object, index } => {
                let obj_val = self.eval_expr(object, env)?;
                let idx_val = self.eval_expr(index, env)?.as_i64()?;
                match obj_val {
                    ComptimeValue::Array(items) => {
                        if idx_val < 0 || (idx_val as usize) >= items.len() {
                            return Err(format!(
                                "Index out of bounds in comptime array access: index {} with length {}",
                                idx_val, items.len()
                            ));
                        }
                        Ok(items[idx_val as usize].clone())
                    }
                    ComptimeValue::String(s) => {
                        let chars: Vec<char> = s.chars().collect();
                        if idx_val < 0 || (idx_val as usize) >= chars.len() {
                            return Err(format!("String index out of bounds: {} >= {}", idx_val, chars.len()));
                        }
                        Ok(ComptimeValue::String(chars[idx_val as usize].to_string()))
                    }
                    other => Err(format!("Cannot index into comptime value of type {}", other.type_name())),
                }
            }
            Expr::MethodCall { object, method, args } => {
                self.eval_method_call(object, method, args, env)
            }
            Expr::Call { callee, args } => {
                self.eval_call(callee, args, env)
            }
            Expr::Comptime { body, result, .. } => {
                self.eval_block(body, result.as_deref(), env)
            }
            Expr::Ref(inner) | Expr::RefMut(inner) => {
                self.eval_expr(inner, env)
            }
            _ => Err(format!("Unsupported expression in compile-time evaluation: {:?}", expr)),
        }
    }

    fn eval_binary_op(
        &self,
        op: &str,
        left: &ComptimeValue,
        right: &ComptimeValue,
    ) -> Result<ComptimeValue, String> {
        // String concatenation
        if op == "+" {
            if let (ComptimeValue::String(s1), ComptimeValue::String(s2)) = (left, right) {
                return Ok(ComptimeValue::String(format!("{}{}", s1, s2)));
            }
        }

        // Float arithmetic if either operand is Float
        if matches!(left, ComptimeValue::Float(_)) || matches!(right, ComptimeValue::Float(_)) {
            let l = left.as_f64()?;
            let r = right.as_f64()?;
            return match op {
                "+" => Ok(ComptimeValue::Float(l + r)),
                "-" => Ok(ComptimeValue::Float(l - r)),
                "*" => Ok(ComptimeValue::Float(l * r)),
                "/" => {
                    if r == 0.0 {
                        Err("Divide by zero in comptime float division".to_string())
                    } else {
                        Ok(ComptimeValue::Float(l / r))
                    }
                }
                "==" => Ok(ComptimeValue::Bool((l - r).abs() < 1e-12)),
                "!=" => Ok(ComptimeValue::Bool((l - r).abs() >= 1e-12)),
                "<" => Ok(ComptimeValue::Bool(l < r)),
                "<=" => Ok(ComptimeValue::Bool(l <= r)),
                ">" => Ok(ComptimeValue::Bool(l > r)),
                ">=" => Ok(ComptimeValue::Bool(l >= r)),
                _ => Err(format!("Unsupported operator '{}' for comptime floats", op)),
            };
        }

        // Integer arithmetic
        let l = left.as_i64()?;
        let r = right.as_i64()?;
        match op {
            "+" => Ok(ComptimeValue::Int(l.wrapping_add(r))),
            "-" => Ok(ComptimeValue::Int(l.wrapping_sub(r))),
            "*" => Ok(ComptimeValue::Int(l.wrapping_mul(r))),
            "/" => {
                if r == 0 {
                    Err("Divide by zero in compile-time integer division".to_string())
                } else {
                    Ok(ComptimeValue::Int(l / r))
                }
            }
            "%" => {
                if r == 0 {
                    Err("Modulo by zero in compile-time evaluation".to_string())
                } else {
                    Ok(ComptimeValue::Int(l % r))
                }
            }
            "<<" => Ok(ComptimeValue::Int(l.wrapping_shl(r as u32))),
            ">>" => Ok(ComptimeValue::Int(l.wrapping_shr(r as u32))),
            "&" => Ok(ComptimeValue::Int(l & r)),
            "|" => Ok(ComptimeValue::Int(l | r)),
            "^" => Ok(ComptimeValue::Int(l ^ r)),
            "==" => Ok(ComptimeValue::Bool(l == r)),
            "!=" => Ok(ComptimeValue::Bool(l != r)),
            "<" => Ok(ComptimeValue::Bool(l < r)),
            "<=" => Ok(ComptimeValue::Bool(l <= r)),
            ">" => Ok(ComptimeValue::Bool(l > r)),
            ">=" => Ok(ComptimeValue::Bool(l >= r)),
            "and" | "&&" => Ok(ComptimeValue::Bool(l != 0 && r != 0)),
            "or" | "||" => Ok(ComptimeValue::Bool(l != 0 || r != 0)),
            _ => Err(format!("Unsupported operator '{}' for comptime integers", op)),
        }
    }

    fn eval_method_call(
        &self,
        object: &Expr,
        method: &str,
        args: &[CallArg],
        env: &mut ComptimeEnv,
    ) -> Result<ComptimeValue, String> {
        match method {
            "len" => {
                let obj = self.eval_expr(object, env)?;
                match obj {
                    ComptimeValue::Array(arr) => Ok(ComptimeValue::Int(arr.len() as i64)),
                    ComptimeValue::String(s) => Ok(ComptimeValue::Int(s.len() as i64)),
                    other => Err(format!("Cannot call .len() on {}", other.type_name())),
                }
            }
            "push" => {
                // If object is an Ident, we mutate it in place in the environment
                if let Expr::Ident(var_name, span) = object {
                    if args.is_empty() {
                        return Err(format!("Line {}:{}: .push() expects 1 argument", span.line, span.col));
                    }
                    let item = self.eval_expr(&args[0].value, env)?;
                    if let Some(ComptimeValue::Array(ref mut arr)) = env.get_mut(var_name) {
                        arr.push(item);
                        return Ok(ComptimeValue::Void);
                    } else {
                        return Err(format!("Line {}:{}: Variable '{}' is not an array", span.line, span.col, var_name));
                    }
                }
                Err("Cannot call .push() on non-variable array in comptime".to_string())
            }
            _ => Err(format!("Unsupported comptime method: .{}()", method)),
        }
    }

    fn eval_call(
        &self,
        callee: &str,
        args: &[CallArg],
        env: &mut ComptimeEnv,
    ) -> Result<ComptimeValue, String> {
        let mut evaluated_args = Vec::new();
        for arg in args {
            evaluated_args.push(self.eval_expr(&arg.value, env)?);
        }

        // Built-in pure mathematical functions for Photonic MZI & LUT generation
        match callee {
            "sin" => {
                if evaluated_args.is_empty() {
                    return Err("sin() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                Ok(ComptimeValue::Float(val.sin()))
            }
            "cos" => {
                if evaluated_args.is_empty() {
                    return Err("cos() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                Ok(ComptimeValue::Float(val.cos()))
            }
            "tan" => {
                if evaluated_args.is_empty() {
                    return Err("tan() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                Ok(ComptimeValue::Float(val.tan()))
            }
            "sqrt" => {
                if evaluated_args.is_empty() {
                    return Err("sqrt() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                if val < 0.0 {
                    return Err("sqrt() called on negative number in compile-time evaluation".to_string());
                }
                Ok(ComptimeValue::Float(val.sqrt()))
            }
            "exp" => {
                if evaluated_args.is_empty() {
                    return Err("exp() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                Ok(ComptimeValue::Float(val.exp()))
            }
            "ln" | "log" => {
                if evaluated_args.is_empty() {
                    return Err("ln() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                if val <= 0.0 {
                    return Err("ln() called on non-positive number in comptime".to_string());
                }
                Ok(ComptimeValue::Float(val.ln()))
            }
            "abs" => {
                if evaluated_args.is_empty() {
                    return Err("abs() requires 1 argument".to_string());
                }
                match &evaluated_args[0] {
                    ComptimeValue::Int(i) => Ok(ComptimeValue::Int(i.abs())),
                    ComptimeValue::Float(f) => Ok(ComptimeValue::Float(f.abs())),
                    other => Err(format!("abs() not supported on {}", other.type_name())),
                }
            }
            "pow" => {
                if evaluated_args.len() < 2 {
                    return Err("pow() requires 2 arguments: base, exp".to_string());
                }
                let base = evaluated_args[0].as_f64()?;
                let exp = evaluated_args[1].as_f64()?;
                Ok(ComptimeValue::Float(base.powf(exp)))
            }
            "atan2" => {
                if evaluated_args.len() < 2 {
                    return Err("atan2() requires 2 arguments: y, x".to_string());
                }
                let y = evaluated_args[0].as_f64()?;
                let x = evaluated_args[1].as_f64()?;
                Ok(ComptimeValue::Float(y.atan2(x)))
            }
            "acos" | "arccos" => {
                if evaluated_args.is_empty() {
                    return Err("acos() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                if !(-1.0..=1.0).contains(&val) {
                    return Err(format!("acos() out of domain [-1.0, 1.0]: {}", val));
                }
                Ok(ComptimeValue::Float(val.acos()))
            }
            "asin" | "arcsin" => {
                if evaluated_args.is_empty() {
                    return Err("asin() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                if !(-1.0..=1.0).contains(&val) {
                    return Err(format!("asin() out of domain [-1.0, 1.0]: {}", val));
                }
                Ok(ComptimeValue::Float(val.asin()))
            }
            "round" => {
                if evaluated_args.is_empty() {
                    return Err("round() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                Ok(ComptimeValue::Float(val.round()))
            }
            "floor" => {
                if evaluated_args.is_empty() {
                    return Err("floor() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                Ok(ComptimeValue::Float(val.floor()))
            }
            "ceil" => {
                if evaluated_args.is_empty() {
                    return Err("ceil() requires 1 argument".to_string());
                }
                let val = evaluated_args[0].as_f64()?;
                Ok(ComptimeValue::Float(val.ceil()))
            }
            "pi" => Ok(ComptimeValue::Float(std::f64::consts::PI)),
            "len" => {
                if evaluated_args.is_empty() {
                    return Err("len() requires 1 argument".to_string());
                }
                match &evaluated_args[0] {
                    ComptimeValue::Array(arr) => Ok(ComptimeValue::Int(arr.len() as i64)),
                    ComptimeValue::String(s) => Ok(ComptimeValue::Int(s.len() as i64)),
                    other => Err(format!("len() not supported on {}", other.type_name())),
                }
            }
            _ => {
                // Check if user defined a pure function in the program
                if let Some(fn_decl) = self.functions.iter().find(|f| f.name == callee) {
                    if fn_decl.params.len() != evaluated_args.len() {
                        return Err(format!(
                            "Function '{}' expects {} arguments, got {}",
                            callee, fn_decl.params.len(), evaluated_args.len()
                        ));
                    }
                    let mut fn_env = ComptimeEnv::new();
                    for (param, arg_val) in fn_decl.params.iter().zip(evaluated_args) {
                        fn_env.define(param.name.clone(), arg_val);
                    }
                    for stmt in &fn_decl.body {
                        if let Some(ret) = self.eval_statement(stmt, &mut fn_env)? {
                            return Ok(ret);
                        }
                    }
                    return Ok(ComptimeValue::Void);
                }

                Err(format!("Unknown compile-time function: '{}'", callee))
            }
        }
    }
}

/// Program-wide AST transformation: Evaluates all `Expr::Comptime` and `Statement::Comptime`
/// blocks at compile-time and folds them directly into static literals and array AST nodes.
pub fn evaluate_and_fold_program(program: &mut Program) -> Result<(), String> {
    let functions = program.functions.clone();

    // 1. Fold in functions
    for func in &mut program.functions {
        fold_statements(&mut func.body, &functions)?;
    }

    // 2. Fold in main statements
    fold_statements(&mut program.main_statements, &functions)?;

    // 3. Fold in brains
    for brain in &mut program.brains {
        fold_statements(&mut brain.body, &functions)?;
    }

    // 4. Fold in impl blocks
    for imp in &mut program.impls {
        for func in &mut imp.methods {
            fold_statements(&mut func.body, &functions)?;
        }
    }

    Ok(())
}

fn fold_statements(stmts: &mut Vec<Statement>, functions: &[FunctionDecl]) -> Result<(), String> {
    let mut new_stmts = Vec::with_capacity(stmts.len());

    for mut stmt in stmts.drain(..) {
        match stmt {
            Statement::Comptime { ref body, .. } => {
                let evaluator = ComptimeEvaluator::new(functions);
                let mut env = ComptimeEnv::new();
                for s in body {
                    evaluator.eval_statement(s, &mut env)?;
                }
                // Once executed at comptime, the comptime block statement is folded away.
            }
            Statement::Let { ref mut value, .. } => {
                fold_expr(value, functions)?;
                new_stmts.push(stmt);
            }
            Statement::Assign { ref mut value, .. } => {
                fold_expr(value, functions)?;
                new_stmts.push(stmt);
            }
            Statement::Expr(ref mut expr) => {
                fold_expr(expr, functions)?;
                new_stmts.push(stmt);
            }
            Statement::If { ref mut condition, ref mut then_body, ref mut else_body, .. } => {
                fold_expr(condition, functions)?;
                fold_statements(then_body, functions)?;
                if let Some(else_stmts) = else_body {
                    fold_statements(else_stmts, functions)?;
                }
                new_stmts.push(stmt);
            }
            Statement::While { ref mut condition, ref mut body, .. } => {
                fold_expr(condition, functions)?;
                fold_statements(body, functions)?;
                new_stmts.push(stmt);
            }
            Statement::For { ref mut iterable, ref mut body, .. } => {
                fold_expr(iterable, functions)?;
                fold_statements(body, functions)?;
                new_stmts.push(stmt);
            }
            Statement::Region { ref mut body, .. }
            | Statement::Resilient { ref mut body, .. }
            | Statement::Fuse { ref mut body, .. }
            | Statement::Brain { ref mut body, .. } => {
                fold_statements(body, functions)?;
                new_stmts.push(stmt);
            }
            Statement::Return(Some(ref mut expr)) => {
                fold_expr(expr, functions)?;
                new_stmts.push(stmt);
            }
            _ => new_stmts.push(stmt),
        }
    }

    *stmts = new_stmts;
    Ok(())
}

fn fold_expr(expr: &mut Expr, functions: &[FunctionDecl]) -> Result<(), String> {
    match expr {
        Expr::Comptime { body, result, .. } => {
            let evaluator = ComptimeEvaluator::new(functions);
            let mut env = ComptimeEnv::new();
            let evaluated_val = evaluator.eval_block(body, result.as_deref(), &mut env)?;
            *expr = evaluated_val.to_expr();
        }
        Expr::Binary { left, right, .. } => {
            fold_expr(left, functions)?;
            fold_expr(right, functions)?;
        }
        Expr::Unary { operand, .. } => {
            fold_expr(operand, functions)?;
        }
        Expr::Cast { expr: inner, .. } => {
            fold_expr(inner, functions)?;
        }
        Expr::IfExpr { condition, then_branch, else_branch } => {
            fold_expr(condition, functions)?;
            fold_expr(then_branch, functions)?;
            fold_expr(else_branch, functions)?;
        }
        Expr::Array(elements) => {
            for elem in elements {
                fold_expr(elem, functions)?;
            }
        }
        Expr::ArrayRepeat { value, count } => {
            fold_expr(value, functions)?;
            fold_expr(count, functions)?;
        }
        Expr::Index { object, index } => {
            fold_expr(object, functions)?;
            fold_expr(index, functions)?;
        }
        Expr::Call { args, .. } => {
            for arg in args {
                fold_expr(&mut arg.value, functions)?;
            }
        }
        Expr::MethodCall { object, args, .. } => {
            fold_expr(object, functions)?;
            for arg in args {
                fold_expr(&mut arg.value, functions)?;
            }
        }
        Expr::Ref(inner) | Expr::RefMut(inner) => {
            fold_expr(inner, functions)?;
        }
        _ => {}
    }
    Ok(())
}
