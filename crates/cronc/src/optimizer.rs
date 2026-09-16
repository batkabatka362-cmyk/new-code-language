// ============================================================================
// CRON Compiler Optimizer — Multi-Pass AST Optimization Pipeline
// Implements industrial-grade optimization passes between parsing and codegen:
//   Pass 1: Constant Folding — evaluate compile-time constant expressions
//   Pass 2: Dead Code Elimination — remove unreachable & unused code
//   Pass 3: Strength Reduction — replace expensive ops with cheaper equivalents
//   Pass 4: Common Subexpression Elimination — reuse computed values
//   Pass 5: Loop Invariant Code Motion — hoist invariant computations
// Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
// ============================================================================

use crate::ast::*;
use std::collections::{HashMap, HashSet};

/// Optimization statistics for reporting
#[derive(Debug, Clone, Default)]
pub struct OptimizationReport {
    pub constants_folded: usize,
    pub dead_stmts_eliminated: usize,
    pub strength_reductions: usize,
    pub subexpressions_eliminated: usize,
    pub loop_invariants_hoisted: usize,
}

impl OptimizationReport {
    pub fn summary(&self) -> String {
        format!(
            "[OPT] Constants folded: {} | Dead code eliminated: {} | Strength reduced: {} | CSE: {} | LICM: {}",
            self.constants_folded,
            self.dead_stmts_eliminated,
            self.strength_reductions,
            self.subexpressions_eliminated,
            self.loop_invariants_hoisted,
        )
    }

    pub fn total_transforms(&self) -> usize {
        self.constants_folded
            + self.dead_stmts_eliminated
            + self.strength_reductions
            + self.subexpressions_eliminated
            + self.loop_invariants_hoisted
    }
}

/// Main optimizer entry point
pub struct Optimizer {
    pub report: OptimizationReport,
    /// Tracks which variables are read during expression analysis
    referenced_vars: HashSet<String>,
    /// CSE cache: expression fingerprint -> variable name holding the result
    cse_cache: HashMap<String, String>,
    /// Counter for generating unique CSE temp variable names
    _cse_counter: usize,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Optimizer {
    pub fn new() -> Self {
        Self {
            report: OptimizationReport::default(),
            referenced_vars: HashSet::new(),
            cse_cache: HashMap::new(),
            _cse_counter: 0,
        }
    }

    /// Run all optimization passes on the program AST (in-place mutation)
    pub fn optimize_program(&mut self, program: &mut Program) {
        // Pass 1: Constant Folding (safe, preserves semantics)
        self.fold_constants_program(program);

        // Pass 2: Dead Code Elimination
        self.eliminate_dead_code_program(program);

        // Pass 3: Strength Reduction
        self.strength_reduce_program(program);

        // Pass 4: Common Subexpression Elimination
        self.cse_program(program);

        // Pass 5: Loop Invariant Code Motion
        self.licm_program(program);
    }

    // ========================================================================
    // Pass 1: Constant Folding
    // ========================================================================

    fn fold_constants_program(&mut self, program: &mut Program) {
        for func in &mut program.functions {
            self.fold_constants_stmts(&mut func.body);
        }
        for brain in &mut program.brains {
            self.fold_constants_stmts(&mut brain.body);
        }
        for imp in &mut program.impls {
            for method in &mut imp.methods {
                self.fold_constants_stmts(&mut method.body);
            }
        }
        self.fold_constants_stmts(&mut program.main_statements);
    }

    fn fold_constants_stmts(&mut self, stmts: &mut Vec<Statement>) {
        for stmt in stmts.iter_mut() {
            self.fold_constants_stmt(stmt);
        }
    }

    fn fold_constants_stmt(&mut self, stmt: &mut Statement) {
        match stmt {
            Statement::Let { value, .. } => {
                *value = self.fold_expr(value.clone());
            }
            Statement::Assign { value, .. } => {
                *value = self.fold_expr(value.clone());
            }
            Statement::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                *condition = self.fold_expr(condition.clone());
                self.fold_constants_stmts(then_body);
                if let Some(eb) = else_body {
                    self.fold_constants_stmts(eb);
                }
            }
            Statement::While {
                condition, body, ..
            } => {
                *condition = self.fold_expr(condition.clone());
                self.fold_constants_stmts(body);
            }
            Statement::For { iterable, body, .. } => {
                *iterable = self.fold_expr(iterable.clone());
                self.fold_constants_stmts(body);
            }
            Statement::Region { body, .. } => {
                self.fold_constants_stmts(body);
            }
            Statement::Resilient {
                body, fallback, ..
            } => {
                self.fold_constants_stmts(body);
                if let Some(fb) = fallback {
                    self.fold_constants_stmts(fb);
                }
            }
            Statement::Brain { body, .. } => {
                self.fold_constants_stmts(body);
            }
            Statement::Expr(e) => {
                *e = self.fold_expr(e.clone());
            }
            Statement::Return(Some(e)) => {
                *e = self.fold_expr(e.clone());
            }
            Statement::Match { expr, arms, .. } => {
                *expr = self.fold_expr(expr.clone());
                for arm in arms {
                    self.fold_constants_stmts(&mut arm.body);
                }
            }
            Statement::Superposition { branches, .. } => {
                for branch in branches {
                    self.fold_constants_stmts(&mut branch.body);
                }
            }
            _ => {}
        }
    }

    /// Recursively fold constant expressions
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Binary { op, left, right } => {
                let l = self.fold_expr(*left);
                let r = self.fold_expr(*right);

                // Integer constant folding
                if let (Expr::LiteralInt(lv), Expr::LiteralInt(rv)) = (&l, &r) {
                    let result = match op.as_str() {
                        "+" => Some(lv.wrapping_add(*rv)),
                        "-" => Some(lv.wrapping_sub(*rv)),
                        "*" => Some(lv.wrapping_mul(*rv)),
                        "/" if *rv != 0 => Some(lv / rv),
                        "%" if *rv != 0 => Some(lv % rv),
                        "&" => Some(lv & rv),
                        "|" => Some(lv | rv),
                        "^" => Some(lv ^ rv),
                        "<<" => Some(lv << (rv & 63)),
                        ">>" => Some(lv >> (rv & 63)),
                        _ => None,
                    };
                    if let Some(val) = result {
                        self.report.constants_folded += 1;
                        return Expr::LiteralInt(val);
                    }

                    // Boolean-result integer comparisons
                    let bool_result = match op.as_str() {
                        "==" => Some(*lv == *rv),
                        "!=" => Some(*lv != *rv),
                        "<" => Some(*lv < *rv),
                        "<=" => Some(*lv <= *rv),
                        ">" => Some(*lv > *rv),
                        ">=" => Some(*lv >= *rv),
                        _ => None,
                    };
                    if let Some(val) = bool_result {
                        self.report.constants_folded += 1;
                        return Expr::LiteralBool(val);
                    }
                }

                // Float constant folding
                if let (Expr::LiteralFloat(lv), Expr::LiteralFloat(rv)) = (&l, &r) {
                    let result = match op.as_str() {
                        "+" => Some(lv + rv),
                        "-" => Some(lv - rv),
                        "*" => Some(lv * rv),
                        "/" if *rv != 0.0 => Some(lv / rv),
                        _ => None,
                    };
                    if let Some(val) = result {
                        self.report.constants_folded += 1;
                        return Expr::LiteralFloat(val);
                    }
                }

                // Boolean constant folding
                if let (Expr::LiteralBool(lv), Expr::LiteralBool(rv)) = (&l, &r) {
                    let result = match op.as_str() {
                        "and" | "&&" => Some(*lv && *rv),
                        "or" | "||" => Some(*lv || *rv),
                        "==" => Some(*lv == *rv),
                        "!=" => Some(*lv != *rv),
                        _ => None,
                    };
                    if let Some(val) = result {
                        self.report.constants_folded += 1;
                        return Expr::LiteralBool(val);
                    }
                }

                // Identity folding: x + 0 → x, x * 1 → x, x * 0 → 0
                if let Expr::LiteralInt(0) = &r {
                    if op == "+" || op == "-" || op == "|" || op == "^" {
                        self.report.constants_folded += 1;
                        return l;
                    }
                    if op == "*" || op == "&" {
                        self.report.constants_folded += 1;
                        return Expr::LiteralInt(0);
                    }
                }
                if let Expr::LiteralInt(0) = &l {
                    if op == "+" || op == "|" {
                        self.report.constants_folded += 1;
                        return r;
                    }
                    if op == "*" || op == "&" {
                        self.report.constants_folded += 1;
                        return Expr::LiteralInt(0);
                    }
                }
                if let Expr::LiteralInt(1) = &r {
                    if op == "*" || op == "/" {
                        self.report.constants_folded += 1;
                        return l;
                    }
                }
                if let Expr::LiteralInt(1) = &l {
                    if op == "*" {
                        self.report.constants_folded += 1;
                        return r;
                    }
                }

                Expr::Binary {
                    op,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
            Expr::Unary { op, operand } => {
                let inner = self.fold_expr(*operand);

                // Fold unary negation on literals
                if let Expr::LiteralInt(v) = &inner {
                    if op == "-" {
                        self.report.constants_folded += 1;
                        return Expr::LiteralInt(-v);
                    }
                }
                if let Expr::LiteralFloat(v) = &inner {
                    if op == "-" {
                        self.report.constants_folded += 1;
                        return Expr::LiteralFloat(-v);
                    }
                }
                if let Expr::LiteralBool(v) = &inner {
                    if op == "not" || op == "!" {
                        self.report.constants_folded += 1;
                        return Expr::LiteralBool(!v);
                    }
                }

                // Double negation elimination: --x → x
                if op == "-" {
                    if let Expr::Unary {
                        op: ref inner_op,
                        operand: ref inner_operand,
                    } = inner
                    {
                        if inner_op == "-" {
                            self.report.constants_folded += 1;
                            return *inner_operand.clone();
                        }
                    }
                }

                Expr::Unary {
                    op,
                    operand: Box::new(inner),
                }
            }
            Expr::Call { callee, args } => {
                let folded_args: Vec<CallArg> = args
                    .into_iter()
                    .map(|a| CallArg {
                        name: a.name,
                        value: self.fold_expr(a.value),
                    })
                    .collect();
                Expr::Call {
                    callee,
                    args: folded_args,
                }
            }
            Expr::MethodCall { object, method, args } => {
                let folded_obj = self.fold_expr(*object);
                let folded_args: Vec<CallArg> = args
                    .into_iter()
                    .map(|a| CallArg {
                        name: a.name,
                        value: self.fold_expr(a.value),
                    })
                    .collect();
                Expr::MethodCall {
                    object: Box::new(folded_obj),
                    method,
                    args: folded_args,
                }
            }
            Expr::Index { object, index } => Expr::Index {
                object: Box::new(self.fold_expr(*object)),
                index: Box::new(self.fold_expr(*index)),
            },
            Expr::FieldAccess { object, field } => Expr::FieldAccess {
                object: Box::new(self.fold_expr(*object)),
                field,
            },
            Expr::IfExpr {
                condition,
                then_branch,
                else_branch,
            } => {
                let folded_cond = self.fold_expr(*condition);
                // If condition is a known boolean, eliminate the branch
                if let Expr::LiteralBool(val) = &folded_cond {
                    self.report.constants_folded += 1;
                    if *val {
                        return self.fold_expr(*then_branch);
                    } else {
                        return self.fold_expr(*else_branch);
                    }
                }
                Expr::IfExpr {
                    condition: Box::new(folded_cond),
                    then_branch: Box::new(self.fold_expr(*then_branch)),
                    else_branch: Box::new(self.fold_expr(*else_branch)),
                }
            }
            Expr::Cast { expr, target_type } => Expr::Cast {
                expr: Box::new(self.fold_expr(*expr)),
                target_type,
            },
            Expr::Array(items) => {
                Expr::Array(items.into_iter().map(|e| self.fold_expr(e)).collect())
            }
            Expr::Tuple(items) => {
                Expr::Tuple(items.into_iter().map(|e| self.fold_expr(e)).collect())
            }
            Expr::StructInit {
                struct_name,
                fields,
            } => Expr::StructInit {
                struct_name,
                fields: fields
                    .into_iter()
                    .map(|(n, e)| (n, self.fold_expr(e)))
                    .collect(),
            },
            other => other,
        }
    }

    // ========================================================================
    // Pass 2: Dead Code Elimination
    // ========================================================================

    fn eliminate_dead_code_program(&mut self, program: &mut Program) {
        for func in &mut program.functions {
            self.eliminate_dead_code_stmts(&mut func.body);
        }
        for brain in &mut program.brains {
            self.eliminate_dead_code_stmts(&mut brain.body);
        }
        for imp in &mut program.impls {
            for method in &mut imp.methods {
                self.eliminate_dead_code_stmts(&mut method.body);
            }
        }
        self.eliminate_dead_code_stmts(&mut program.main_statements);
    }

    fn eliminate_dead_code_stmts(&mut self, stmts: &mut Vec<Statement>) {
        // Phase 1: Truncate after return statements (unreachable code elimination)
        let mut return_idx = None;
        for (i, stmt) in stmts.iter().enumerate() {
            if matches!(stmt, Statement::Return(_)) {
                return_idx = Some(i);
                break;
            }
        }
        if let Some(idx) = return_idx {
            let removed = stmts.len() - idx - 1;
            if removed > 0 {
                stmts.truncate(idx + 1);
                self.report.dead_stmts_eliminated += removed;
            }
        }

        // Phase 2: Collect all referenced variable names in remaining statements
        self.referenced_vars.clear();
        for stmt in stmts.iter() {
            self.collect_references_stmt(stmt);
        }
        let referenced = self.referenced_vars.clone();

        // Phase 3: Remove unused non-linear, non-side-effectful let bindings
        let original_len = stmts.len();
        stmts.retain(|stmt| {
            if let Statement::Let {
                name,
                is_lin,
                is_grad,
                value,
                ..
            } = stmt
            {
                // Keep linear/grad variables (they have ownership semantics)
                if *is_lin || *is_grad {
                    return true;
                }
                // Keep if the variable is referenced elsewhere
                if referenced.contains(name) {
                    return true;
                }
                // Keep if the value has side effects
                if self.expr_has_side_effects(value) {
                    return true;
                }
                // Dead code — remove
                false
            } else {
                true
            }
        });
        self.report.dead_stmts_eliminated += original_len - stmts.len();

        // Phase 4: Recurse into nested blocks
        for stmt in stmts.iter_mut() {
            match stmt {
                Statement::If {
                    condition,
                    then_body,
                    else_body,
                    span,
                    ..
                } => {
                    // If condition is constant true/false, eliminate the dead branch
                    if let Expr::LiteralBool(val) = condition {
                        if *val {
                            // Condition is always true — keep then_body, remove else
                            *else_body = None;
                        } else {
                            // Condition is always false — replace with else body or empty
                            let replacement_body = else_body.take().unwrap_or_default();
                            let s = *span;
                            *stmt = Statement::Region {
                                name: "__dce_folded".to_string(),
                                attrs: vec![],
                                body: replacement_body,
                                span: s,
                            };
                            self.report.dead_stmts_eliminated += 1;
                        }
                    } else {
                        self.eliminate_dead_code_stmts(then_body);
                        if let Some(eb) = else_body {
                            self.eliminate_dead_code_stmts(eb);
                        }
                    }
                }
                Statement::While { body, .. } => {
                    self.eliminate_dead_code_stmts(body);
                }
                Statement::For { body, .. } => {
                    self.eliminate_dead_code_stmts(body);
                }
                Statement::Region { body, .. } => {
                    self.eliminate_dead_code_stmts(body);
                }
                Statement::Resilient {
                    body, fallback, ..
                } => {
                    self.eliminate_dead_code_stmts(body);
                    if let Some(fb) = fallback {
                        self.eliminate_dead_code_stmts(fb);
                    }
                }
                Statement::Brain { body, .. } => {
                    self.eliminate_dead_code_stmts(body);
                }
                Statement::Match { arms, .. } => {
                    for arm in arms {
                        self.eliminate_dead_code_stmts(&mut arm.body);
                    }
                }
                _ => {}
            }
        }
    }

    /// Collect all variable names referenced in expressions within a statement
    fn collect_references_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let { value, .. } => self.collect_references_expr(value),
            Statement::Assign { target, value, .. } => {
                self.referenced_vars.insert(target.clone());
                self.collect_references_expr(value);
            }
            Statement::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.collect_references_expr(condition);
                for s in then_body {
                    self.collect_references_stmt(s);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        self.collect_references_stmt(s);
                    }
                }
            }
            Statement::While {
                condition, body, ..
            } => {
                self.collect_references_expr(condition);
                for s in body {
                    self.collect_references_stmt(s);
                }
            }
            Statement::For {
                var_name,
                iterable,
                body,
                ..
            } => {
                self.referenced_vars.insert(var_name.clone());
                self.collect_references_expr(iterable);
                for s in body {
                    self.collect_references_stmt(s);
                }
            }
            Statement::Region { body, .. } | Statement::Brain { body, .. } => {
                for s in body {
                    self.collect_references_stmt(s);
                }
            }
            Statement::Resilient {
                body, fallback, ..
            } => {
                for s in body {
                    self.collect_references_stmt(s);
                }
                if let Some(fb) = fallback {
                    for s in fb {
                        self.collect_references_stmt(s);
                    }
                }
            }
            Statement::Expr(e) => self.collect_references_expr(e),
            Statement::Return(Some(e)) => self.collect_references_expr(e),
            Statement::Export { source_name, .. } => {
                self.referenced_vars.insert(source_name.clone());
            }
            Statement::Match { expr, arms, .. } => {
                self.collect_references_expr(expr);
                for arm in arms {
                    for s in &arm.body {
                        self.collect_references_stmt(s);
                    }
                }
            }
            Statement::Fork { target, .. } => self.collect_references_expr(target),
            Statement::Simulate { action, with_arg, .. } => {
                self.collect_references_expr(action);
                if let Some(wa) = with_arg {
                    self.collect_references_expr(wa);
                }
            }
            _ => {}
        }
    }

    fn collect_references_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(name, _) => {
                self.referenced_vars.insert(name.clone());
            }
            Expr::Consume(name, _) => {
                self.referenced_vars.insert(name.clone());
            }
            Expr::Binary { left, right, .. } => {
                self.collect_references_expr(left);
                self.collect_references_expr(right);
            }
            Expr::Unary { operand, .. } => {
                self.collect_references_expr(operand);
            }
            Expr::Call { args, .. } => {
                for a in args {
                    self.collect_references_expr(&a.value);
                }
            }
            Expr::MethodCall { object, args, .. } => {
                self.collect_references_expr(object);
                for a in args {
                    self.collect_references_expr(&a.value);
                }
            }
            Expr::FieldAccess { object, .. } => {
                self.collect_references_expr(object);
            }
            Expr::Index { object, index } => {
                self.collect_references_expr(object);
                self.collect_references_expr(index);
            }
            Expr::Array(items) | Expr::Tuple(items) => {
                for i in items {
                    self.collect_references_expr(i);
                }
            }
            Expr::StructInit { fields, .. } => {
                for (_, v) in fields {
                    self.collect_references_expr(v);
                }
            }
            Expr::IfExpr {
                condition,
                then_branch,
                else_branch,
            } => {
                self.collect_references_expr(condition);
                self.collect_references_expr(then_branch);
                self.collect_references_expr(else_branch);
            }
            Expr::Cast { expr, .. } => {
                self.collect_references_expr(expr);
            }
            Expr::Await(e) | Expr::Spawn(e) => {
                self.collect_references_expr(e);
            }
            Expr::SpawnAt { core_id, target, .. } => {
                self.collect_references_expr(core_id);
                self.collect_references_expr(target);
            }
            Expr::ChannelSend { channel, value, .. } => {
                self.collect_references_expr(channel);
                self.collect_references_expr(value);
            }
            Expr::ChannelRecv { channel, .. } => {
                self.collect_references_expr(channel);
            }
            Expr::Grad { callee, .. } | Expr::GradCall { callee, .. } => {
                self.collect_references_expr(callee);
            }
            Expr::ArrayRepeat { value, count } => {
                self.collect_references_expr(value);
                self.collect_references_expr(count);
            }
            _ => {}
        }
    }

    /// Check if an expression has observable side effects
    fn expr_has_side_effects(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call { .. } => true,
            Expr::MethodCall { .. } => true,
            Expr::Spawn(_) | Expr::SpawnAt { .. } => true,
            Expr::ChannelSend { .. } | Expr::ChannelRecv { .. } => true,
            Expr::Consume(_, _) => true,
            Expr::Await(_) => true,
            Expr::Grad { .. } | Expr::GradCall { .. } => true,
            _ => false,
        }
    }

    // ========================================================================
    // Pass 3: Strength Reduction
    // ========================================================================

    fn strength_reduce_program(&mut self, program: &mut Program) {
        for func in &mut program.functions {
            self.strength_reduce_stmts(&mut func.body);
        }
        for brain in &mut program.brains {
            self.strength_reduce_stmts(&mut brain.body);
        }
        for imp in &mut program.impls {
            for method in &mut imp.methods {
                self.strength_reduce_stmts(&mut method.body);
            }
        }
        self.strength_reduce_stmts(&mut program.main_statements);
    }

    fn strength_reduce_stmts(&mut self, stmts: &mut Vec<Statement>) {
        for stmt in stmts.iter_mut() {
            match stmt {
                Statement::Let { value, .. } => {
                    *value = self.strength_reduce_expr(value.clone());
                }
                Statement::Assign { value, .. } => {
                    *value = self.strength_reduce_expr(value.clone());
                }
                Statement::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    *condition = self.strength_reduce_expr(condition.clone());
                    self.strength_reduce_stmts(then_body);
                    if let Some(eb) = else_body {
                        self.strength_reduce_stmts(eb);
                    }
                }
                Statement::While {
                    condition, body, ..
                } => {
                    *condition = self.strength_reduce_expr(condition.clone());
                    self.strength_reduce_stmts(body);
                }
                Statement::For { body, .. } => {
                    self.strength_reduce_stmts(body);
                }
                Statement::Region { body, .. }
                | Statement::Brain { body, .. } => {
                    self.strength_reduce_stmts(body);
                }
                Statement::Resilient {
                    body, fallback, ..
                } => {
                    self.strength_reduce_stmts(body);
                    if let Some(fb) = fallback {
                        self.strength_reduce_stmts(fb);
                    }
                }
                Statement::Expr(e) => {
                    *e = self.strength_reduce_expr(e.clone());
                }
                Statement::Return(Some(e)) => {
                    *e = self.strength_reduce_expr(e.clone());
                }
                _ => {}
            }
        }
    }

    fn strength_reduce_expr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Binary { op, left, right } => {
                let l = self.strength_reduce_expr(*left);
                let r = self.strength_reduce_expr(*right);

                // x * 2^n → x << n (for integer power-of-2 constants)
                if op == "*" {
                    if let Expr::LiteralInt(rv) = &r {
                        if *rv > 0 && rv.count_ones() == 1 {
                            let shift = rv.trailing_zeros() as i64;
                            self.report.strength_reductions += 1;
                            return Expr::Binary {
                                op: "<<".to_string(),
                                left: Box::new(l),
                                right: Box::new(Expr::LiteralInt(shift)),
                            };
                        }
                    }
                    if let Expr::LiteralInt(lv) = &l {
                        if *lv > 0 && lv.count_ones() == 1 {
                            let shift = lv.trailing_zeros() as i64;
                            self.report.strength_reductions += 1;
                            return Expr::Binary {
                                op: "<<".to_string(),
                                left: Box::new(r),
                                right: Box::new(Expr::LiteralInt(shift)),
                            };
                        }
                    }
                }

                // x / 2^n → x >> n (for positive power-of-2 divisors)
                if op == "/" {
                    if let Expr::LiteralInt(rv) = &r {
                        if *rv > 0 && rv.count_ones() == 1 {
                            let shift = rv.trailing_zeros() as i64;
                            self.report.strength_reductions += 1;
                            return Expr::Binary {
                                op: ">>".to_string(),
                                left: Box::new(l),
                                right: Box::new(Expr::LiteralInt(shift)),
                            };
                        }
                    }
                }

                // x % 2^n → x & (2^n - 1)
                if op == "%" {
                    if let Expr::LiteralInt(rv) = &r {
                        if *rv > 0 && rv.count_ones() == 1 {
                            let mask = rv - 1;
                            self.report.strength_reductions += 1;
                            return Expr::Binary {
                                op: "&".to_string(),
                                left: Box::new(l),
                                right: Box::new(Expr::LiteralInt(mask)),
                            };
                        }
                    }
                }

                Expr::Binary {
                    op,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
            Expr::Unary { op, operand } => Expr::Unary {
                op,
                operand: Box::new(self.strength_reduce_expr(*operand)),
            },
            Expr::Call { callee, args } => Expr::Call {
                callee,
                args: args
                    .into_iter()
                    .map(|a| CallArg {
                        name: a.name,
                        value: self.strength_reduce_expr(a.value),
                    })
                    .collect(),
            },
            other => other,
        }
    }

    // ========================================================================
    // Pass 4: Common Subexpression Elimination (CSE)
    // ========================================================================

    fn cse_program(&mut self, program: &mut Program) {
        for func in &mut program.functions {
            self.cse_cache.clear();
            self.cse_stmts(&mut func.body);
        }
        for imp in &mut program.impls {
            for method in &mut imp.methods {
                self.cse_cache.clear();
                self.cse_stmts(&mut method.body);
            }
        }
        self.cse_cache.clear();
        self.cse_stmts(&mut program.main_statements);
    }

    fn cse_stmts(&mut self, stmts: &mut Vec<Statement>) {
        for stmt in stmts.iter_mut() {
            if let Statement::Let {
                name, value, is_lin, ..
            } = stmt
            {
                if !*is_lin && !self.expr_has_side_effects(value) {
                    let fingerprint = self.expr_fingerprint(value);
                    if !fingerprint.is_empty() {
                        if let Some(existing_var) = self.cse_cache.get(&fingerprint) {
                            // Replace the expression with a reference to the existing variable
                            let existing_name = existing_var.clone();
                            *value = Expr::Ident(
                                existing_name,
                                crate::token::Span::default(),
                            );
                            self.report.subexpressions_eliminated += 1;
                        } else {
                            // Cache this expression's result variable
                            self.cse_cache
                                .insert(fingerprint, name.clone());
                        }
                    }
                }
            }
        }
    }

    /// Generate a stable string fingerprint for an expression (for CSE matching)
    fn expr_fingerprint(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { op, left, right } => {
                let l = self.expr_fingerprint(left);
                let r = self.expr_fingerprint(right);
                if l.is_empty() || r.is_empty() {
                    return String::new();
                }
                // Commutative ops: normalize order for better matching
                if (op == "+" || op == "*" || op == "&" || op == "|" || op == "^"
                    || op == "==" || op == "!=")
                    && l > r {
                        return format!("({}{}{})", r, op, l);
                    }
                format!("({}{}{})", l, op, r)
            }
            Expr::Unary { op, operand } => {
                let inner = self.expr_fingerprint(operand);
                if inner.is_empty() {
                    return String::new();
                }
                format!("({}{})", op, inner)
            }
            Expr::Ident(name, _) => name.clone(),
            Expr::LiteralInt(v) => format!("i{}", v),
            Expr::LiteralFloat(v) => format!("f{}", v),
            Expr::LiteralBool(v) => format!("b{}", v),
            Expr::LiteralHex(v) => format!("h{}", v),
            Expr::FieldAccess { object, field } => {
                let obj = self.expr_fingerprint(object);
                if obj.is_empty() {
                    return String::new();
                }
                format!("{}.{}", obj, field)
            }
            Expr::Index { object, index } => {
                let obj = self.expr_fingerprint(object);
                let idx = self.expr_fingerprint(index);
                if obj.is_empty() || idx.is_empty() {
                    return String::new();
                }
                format!("{}[{}]", obj, idx)
            }
            Expr::Cast { expr, target_type } => {
                let inner = self.expr_fingerprint(expr);
                if inner.is_empty() {
                    return String::new();
                }
                format!("({}as{})", inner, target_type)
            }
            // Side-effectful expressions cannot be CSE'd — return empty fingerprint
            _ => String::new(),
        }
    }

    // ========================================================================
    // Pass 5: Loop Invariant Code Motion (LICM)
    // ========================================================================

    fn licm_program(&mut self, program: &mut Program) {
        for func in &mut program.functions {
            self.licm_stmts(&mut func.body);
        }
        for brain in &mut program.brains {
            self.licm_stmts(&mut brain.body);
        }
        for imp in &mut program.impls {
            for method in &mut imp.methods {
                self.licm_stmts(&mut method.body);
            }
        }
        self.licm_stmts(&mut program.main_statements);
    }

    fn licm_stmts(&mut self, stmts: &mut Vec<Statement>) {
        let mut i = 0;
        while i < stmts.len() {
            match &stmts[i] {
                Statement::While { body, .. } | Statement::For { body, .. } => {
                    // Collect variables modified inside the loop
                    let modified = self.collect_modified_vars(body);

                    // Find statements inside the loop body that are invariant
                    let loop_body = match &mut stmts[i] {
                        Statement::While { body, .. } => body,
                        Statement::For { body, .. } => body,
                        _ => unreachable!(),
                    };

                    let mut hoisted = Vec::new();
                    let mut j = 0;
                    while j < loop_body.len() {
                        if let Statement::Let {
                            name,
                            value,
                            is_lin,
                            is_grad,
                            ..
                        } = &loop_body[j]
                        {
                            // Only hoist if:
                            // 1. Not linear/grad (ownership semantics)
                            // 2. Value doesn't reference any modified variable
                            // 3. No side effects
                            if !is_lin
                                && !is_grad
                                && !self.expr_has_side_effects(value)
                                && !self.expr_references_any(value, &modified)
                                && !modified.contains(name)
                            {
                                hoisted.push(loop_body.remove(j));
                                self.report.loop_invariants_hoisted += 1;
                                continue;
                            }
                        }
                        j += 1;
                    }

                    // Insert hoisted statements before the loop
                    for (offset, h) in hoisted.into_iter().enumerate() {
                        stmts.insert(i + offset, h);
                        i += 1;
                    }
                }
                _ => {}
            }

            // Recurse into nested blocks
            match &mut stmts[i] {
                Statement::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    self.licm_stmts(then_body);
                    if let Some(eb) = else_body {
                        self.licm_stmts(eb);
                    }
                }
                Statement::Region { body, .. } | Statement::Brain { body, .. } => {
                    self.licm_stmts(body);
                }
                Statement::Resilient {
                    body, fallback, ..
                } => {
                    self.licm_stmts(body);
                    if let Some(fb) = fallback {
                        self.licm_stmts(fb);
                    }
                }
                _ => {}
            }

            i += 1;
        }
    }

    /// Collect all variable names that are assigned/modified within a block
    fn collect_modified_vars(&self, stmts: &[Statement]) -> HashSet<String> {
        let mut modified = HashSet::new();
        for stmt in stmts {
            match stmt {
                Statement::Let { name, .. } => {
                    modified.insert(name.clone());
                }
                Statement::Assign { target, .. } => {
                    modified.insert(target.clone());
                }
                Statement::For { var_name, body, .. } => {
                    modified.insert(var_name.clone());
                    modified.extend(self.collect_modified_vars(body));
                }
                Statement::While { body, .. } => {
                    modified.extend(self.collect_modified_vars(body));
                }
                Statement::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    modified.extend(self.collect_modified_vars(then_body));
                    if let Some(eb) = else_body {
                        modified.extend(self.collect_modified_vars(eb));
                    }
                }
                Statement::Region { body, .. } | Statement::Brain { body, .. } => {
                    modified.extend(self.collect_modified_vars(body));
                }
                _ => {}
            }
        }
        modified
    }

    /// Check if an expression references any variable in the given set
    fn expr_references_any(&self, expr: &Expr, vars: &HashSet<String>) -> bool {
        match expr {
            Expr::Ident(name, _) => vars.contains(name),
            Expr::Consume(name, _) => vars.contains(name),
            Expr::Binary { left, right, .. } => {
                self.expr_references_any(left, vars) || self.expr_references_any(right, vars)
            }
            Expr::Unary { operand, .. } => self.expr_references_any(operand, vars),
            Expr::Call { args, .. } => args.iter().any(|a| self.expr_references_any(&a.value, vars)),
            Expr::MethodCall { object, args, .. } => {
                self.expr_references_any(object, vars)
                    || args.iter().any(|a| self.expr_references_any(&a.value, vars))
            }
            Expr::FieldAccess { object, .. } => self.expr_references_any(object, vars),
            Expr::Index { object, index } => {
                self.expr_references_any(object, vars) || self.expr_references_any(index, vars)
            }
            Expr::Cast { expr, .. } => self.expr_references_any(expr, vars),
            Expr::IfExpr {
                condition,
                then_branch,
                else_branch,
            } => {
                self.expr_references_any(condition, vars)
                    || self.expr_references_any(then_branch, vars)
                    || self.expr_references_any(else_branch, vars)
            }
            Expr::Array(items) | Expr::Tuple(items) => {
                items.iter().any(|i| self.expr_references_any(i, vars))
            }
            Expr::StructInit { fields, .. } => {
                fields.iter().any(|(_, v)| self.expr_references_any(v, vars))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn parse_program(source: &str) -> Program {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Lex failed");
        let mut parser = Parser::new(tokens);
        parser.parse_program().expect("Parse failed")
    }

    #[test]
    fn test_constant_folding_arithmetic() {
        let mut prog = parse_program(
            r#"
            .MODULE TestOpt
            _main:
                let a = 2 + 3 * 4
                let b = 10 - 5 + 1
            .END
            "#,
        );
        let mut opt = Optimizer::new();
        opt.optimize_program(&mut prog);
        assert!(opt.report.constants_folded > 0, "Expected constants to be folded");
    }

    #[test]
    fn test_strength_reduction_mul_pow2() {
        let mut prog = parse_program(
            r#"
            .MODULE TestOpt
            def compute(x: u32) -> u32 {
                let y = x * 8
                let z = x / 4
                return y + z
            }
            _main:
                let r = compute(10)
            .END
            "#,
        );
        let mut opt = Optimizer::new();
        opt.optimize_program(&mut prog);
        assert!(opt.report.strength_reductions > 0, "Expected strength reductions, got: {:?}", opt.report);
    }

    #[test]
    fn test_dead_code_after_return() {
        let mut prog = parse_program(
            r#"
            .MODULE TestOpt
            def foo() -> u32 {
                return 42
                let dead = 100
                let also_dead = 200
            }
            _main:
                let r = foo()
            .END
            "#,
        );
        let mut opt = Optimizer::new();
        opt.optimize_program(&mut prog);
        assert!(opt.report.dead_stmts_eliminated > 0, "Expected dead code elimination");
        // Verify function body only has the return statement
        assert_eq!(prog.functions[0].body.len(), 1);
    }

    #[test]
    fn test_identity_folding() {
        let mut prog = parse_program(
            r#"
            .MODULE TestOpt
            _main:
                let x = 42
                let a = x + 0
                let b = x * 1
                let c = 0 + x
            .END
            "#,
        );
        let mut opt = Optimizer::new();
        opt.optimize_program(&mut prog);
        assert!(opt.report.constants_folded >= 3, "Expected identity folding");
    }
}
