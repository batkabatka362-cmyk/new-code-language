// ============================================================================
// CRON SOVEREIGN MATHEMATICAL LANGUAGE (cl_math.rs)
// Standard Conventional Algebra & Zero-Word AI Compositional Calculus
//
// Key Principles:
// 1. Zero Verbal Words: No 'fn', 'def', 'let', 'return', 'var', 'import', 'if'.
// 2. Standard Mathematical Algebra: f(x) = W @ x + b, sigmoid(z) = 1 / (1 + exp(-z)).
// 3. Pipeline Function Composition: x -> norm -> linear(W, b) -> activate.
// 4. First-Class Vectors, Matrices & Conditional Calculus: [1, 2, 3], W @ x, c ? a : b.
// 5. Native Hardware Dtypes: f16 (GPU half), f32, f64, bf16, i32, t2 (ternary 1.58b).
// 6. Multi-Target Lowering:
//    - Real-Time Scalar/Vector/Matrix Evaluator
//    - ISO C23 Native Source (with strength reduction & dtype inference)
//    - NVIDIA CUDA PTX v7.5+ SSA Compiler
//    - 4-Slot 128-Bit VLIW Silicon Microcode (CRC-8 ATM Verified)
// ============================================================================

use std::collections::HashMap;
use std::fmt;

use crate::cl_v2_ast::{
    ClV2Document, FunctionBlock, MathMorphismBlock, MathStep, ProgramItem, SiliconTargetContract,
    VliwBundle, VliwSlot,
};

/// Supported numeric precisions and hardware storage representations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathDtype {
    F16,
    F32,
    F64,
    Bf16,
    I32,
    I64,
    T2, // Ternary {-1, 0, +1} (BitNet 1.58-bit)
}

impl MathDtype {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "f16" | "half" | "fp16" | "~16" => Some(MathDtype::F16),
            "f32" | "float" | "fp32" | "~32" => Some(MathDtype::F32),
            "f64" | "double" | "fp64" | "~64" => Some(MathDtype::F64),
            "bf16" | "bfloat16" => Some(MathDtype::Bf16),
            "i32" | "int" | "int32" | "+32" => Some(MathDtype::I32),
            "i64" | "long" | "int64" | "+64" => Some(MathDtype::I64),
            "t2" | "ternary" | "b1.58" | "t" | "%2" => Some(MathDtype::T2),
            _ => None,
        }
    }

    pub fn to_c23_type(&self) -> &'static str {
        match self {
            MathDtype::F16 => "_Float16",
            MathDtype::F32 => "float",
            MathDtype::F64 => "double",
            MathDtype::Bf16 => "__bf16",
            MathDtype::I32 => "int32_t",
            MathDtype::I64 => "int64_t",
            MathDtype::T2 => "int8_t",
        }
    }

    pub fn to_ptx_type(&self) -> &'static str {
        match self {
            MathDtype::F16 => ".f16",
            MathDtype::F32 => ".f32",
            MathDtype::F64 => ".f64",
            MathDtype::Bf16 => ".b16",
            MathDtype::I32 => ".s32",
            MathDtype::I64 => ".s64",
            MathDtype::T2 => ".s8",
        }
    }

    pub fn to_v2_dtype_str(&self) -> &'static str {
        match self {
            MathDtype::F16 => "FP16",
            MathDtype::F32 => "FP32",
            MathDtype::F64 => "FP64",
            MathDtype::Bf16 => "BF16",
            MathDtype::I32 => "INT32",
            MathDtype::I64 => "INT64",
            MathDtype::T2 => "T",
        }
    }
}

/// First-class runtime value supporting Scalars, 1D Vectors, and 2D Matrices
#[derive(Debug, Clone, PartialEq)]
pub enum MathValue {
    Scalar(f64),
    Vector(Vec<f64>),
    Matrix(Vec<Vec<f64>>),
}

impl MathValue {
    pub fn as_scalar(&self) -> Option<f64> {
        match self {
            MathValue::Scalar(v) => Some(*v),
            MathValue::Vector(vec) if vec.len() == 1 => Some(vec[0]),
            _ => None,
        }
    }

    pub fn map_unary<F: Fn(f64) -> f64>(&self, f: F) -> MathValue {
        match self {
            MathValue::Scalar(v) => MathValue::Scalar(f(*v)),
            MathValue::Vector(vec) => MathValue::Vector(vec.iter().map(|&x| f(x)).collect()),
            MathValue::Matrix(mat) => MathValue::Matrix(
                mat.iter()
                    .map(|row| row.iter().map(|&x| f(x)).collect())
                    .collect(),
            ),
        }
    }

    pub fn flatten_elements(&self) -> Vec<f64> {
        match self {
            MathValue::Scalar(v) => vec![*v],
            MathValue::Vector(vec) => vec.clone(),
            MathValue::Matrix(mat) => mat.iter().flatten().copied().collect(),
        }
    }
}

impl fmt::Display for MathValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathValue::Scalar(v) => {
                if (v - v.round()).abs() < 1e-12 {
                    write!(f, "{}", *v as i64)
                } else {
                    write!(f, "{:.6}", v)
                }
            }
            MathValue::Vector(vec) => {
                let parts: Vec<String> = vec
                    .iter()
                    .map(|v| {
                        if (v - v.round()).abs() < 1e-12 {
                            format!("{}", *v as i64)
                        } else {
                            format!("{:.4}", v)
                        }
                    })
                    .collect();
                write!(f, "[{}]", parts.join(", "))
            }
            MathValue::Matrix(mat) => {
                let rows: Vec<String> = mat
                    .iter()
                    .map(|r| {
                        let elems: Vec<String> = r.iter().map(|v| format!("{:.4}", v)).collect();
                        format!("[{}]", elems.join(", "))
                    })
                    .collect();
                write!(f, "[{}]", rows.join(", "))
            }
        }
    }
}

/// Function parameter with optional mathematical type and tensor dimensions
#[derive(Debug, Clone, PartialEq)]
pub struct MathParam {
    pub name: String,
    pub dtype: MathDtype,
    pub dims: Vec<usize>,
}

/// Binary mathematical operators with standard precedence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathBinOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    MatMul, // @
    Pow,    // ^
    Eq,     // ==
    Neq,    // !=
    Lt,     // <
    Lte,    // <=
    Gt,     // >
    Gte,    // >=
}

impl fmt::Display for MathBinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathBinOp::Add => write!(f, "+"),
            MathBinOp::Sub => write!(f, "-"),
            MathBinOp::Mul => write!(f, "*"),
            MathBinOp::Div => write!(f, "/"),
            MathBinOp::MatMul => write!(f, "@"),
            MathBinOp::Pow => write!(f, "^"),
            MathBinOp::Eq => write!(f, "=="),
            MathBinOp::Neq => write!(f, "!="),
            MathBinOp::Lt => write!(f, "<"),
            MathBinOp::Lte => write!(f, "<="),
            MathBinOp::Gt => write!(f, ">"),
            MathBinOp::Gte => write!(f, ">="),
        }
    }
}

/// Unary mathematical operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathUnaryOp {
    Neg, // -x
    Not, // !x
}

/// Mathematical Expression AST
#[derive(Debug, Clone, PartialEq)]
pub enum MathExpr {
    Num(f64),
    Var(String),
    Array(Vec<MathExpr>), // Vector or Matrix literal: [1, 2, 3] or [[1, 2], [3, 4]]
    Index(Box<MathExpr>, Box<MathExpr>), // Tensor / Vector indexing: x[i], W[i][j]
    Placeholder,          // '_' in pipeline chaining
    Unary(MathUnaryOp, Box<MathExpr>),
    Binary(Box<MathExpr>, MathBinOp, Box<MathExpr>),
    Ternary {
        cond: Box<MathExpr>,
        then_expr: Box<MathExpr>,
        else_expr: Box<MathExpr>,
    },
    Call(String, Vec<MathExpr>),
    Pipeline(Vec<MathExpr>), // e.g. x -> f -> g
}

impl fmt::Display for MathExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathExpr::Num(n) => write!(f, "{}", n),
            MathExpr::Var(v) => write!(f, "{}", v),
            MathExpr::Array(elems) => {
                let s: Vec<String> = elems.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", s.join(", "))
            }
            MathExpr::Index(base, idx) => write!(f, "{}[{}]", base, idx),
            MathExpr::Placeholder => write!(f, "_"),
            MathExpr::Unary(MathUnaryOp::Neg, e) => write!(f, "(-{})", e),
            MathExpr::Unary(MathUnaryOp::Not, e) => write!(f, "(!{})", e),
            MathExpr::Binary(l, op, r) => write!(f, "({} {} {})", l, op, r),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => write!(f, "({} ? {} : {})", cond, then_expr, else_expr),
            MathExpr::Call(name, args) => {
                let s: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", name, s.join(", "))
            }
            MathExpr::Pipeline(stages) => {
                let s: Vec<String> = stages.iter().map(|st| st.to_string()).collect();
                write!(f, "{}", s.join(" -> "))
            }
        }
    }
}

impl MathExpr {
    /// Beta-reduction variable substitution
    pub fn substitute(&self, subst: &HashMap<String, MathExpr>) -> MathExpr {
        match self {
            MathExpr::Num(n) => MathExpr::Num(*n),
            MathExpr::Var(v) => subst.get(v).cloned().unwrap_or_else(|| MathExpr::Var(v.clone())),
            MathExpr::Array(elems) => {
                MathExpr::Array(elems.iter().map(|e| e.substitute(subst)).collect())
            }
            MathExpr::Index(b, i) => MathExpr::Index(
                Box::new(b.substitute(subst)),
                Box::new(i.substitute(subst)),
            ),
            MathExpr::Placeholder => MathExpr::Placeholder,
            MathExpr::Unary(op, inner) => {
                MathExpr::Unary(*op, Box::new(inner.substitute(subst)))
            }
            MathExpr::Binary(l, op, r) => MathExpr::Binary(
                Box::new(l.substitute(subst)),
                *op,
                Box::new(r.substitute(subst)),
            ),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => MathExpr::Ternary {
                cond: Box::new(cond.substitute(subst)),
                then_expr: Box::new(then_expr.substitute(subst)),
                else_expr: Box::new(else_expr.substitute(subst)),
            },
            MathExpr::Call(name, args) => MathExpr::Call(
                name.clone(),
                args.iter().map(|a| a.substitute(subst)).collect(),
            ),
            MathExpr::Pipeline(stages) => {
                MathExpr::Pipeline(stages.iter().map(|s| s.substitute(subst)).collect())
            }
        }
    }

    /// Recursively inlines user-defined functions (`inline_user_fns`) and expands symbolic `grad(f, x)`
    pub fn inline_and_expand(
        &self,
        fns: &HashMap<String, (Vec<String>, MathExpr)>,
        inline_user_fns: bool,
        depth: usize,
    ) -> MathExpr {
        if depth > 16 {
            return self.clone();
        }
        match self {
            MathExpr::Num(_) | MathExpr::Var(_) | MathExpr::Placeholder => self.clone(),
            MathExpr::Array(elems) => MathExpr::Array(
                elems
                    .iter()
                    .map(|e| e.inline_and_expand(fns, inline_user_fns, depth + 1))
                    .collect(),
            ),
            MathExpr::Index(b, i) => MathExpr::Index(
                Box::new(b.inline_and_expand(fns, inline_user_fns, depth + 1)),
                Box::new(i.inline_and_expand(fns, inline_user_fns, depth + 1)),
            ),
            MathExpr::Unary(op, inner) => MathExpr::Unary(
                *op,
                Box::new(inner.inline_and_expand(fns, inline_user_fns, depth + 1)),
            ),
            MathExpr::Binary(l, op, r) => MathExpr::Binary(
                Box::new(l.inline_and_expand(fns, inline_user_fns, depth + 1)),
                *op,
                Box::new(r.inline_and_expand(fns, inline_user_fns, depth + 1)),
            ),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => MathExpr::Ternary {
                cond: Box::new(cond.inline_and_expand(fns, inline_user_fns, depth + 1)),
                then_expr: Box::new(then_expr.inline_and_expand(fns, inline_user_fns, depth + 1)),
                else_expr: Box::new(else_expr.inline_and_expand(fns, inline_user_fns, depth + 1)),
            },
            MathExpr::Pipeline(stages) => {
                let mut curr = stages[0].clone();
                for st in &stages[1..] {
                    curr = MathParser::desugar_pipe_step(curr, st.clone());
                }
                curr.inline_and_expand(fns, inline_user_fns, depth + 1)
            }
            MathExpr::Call(name, args) => {
                if (name == "grad" || name == "diff") && args.len() == 2 {
                    if let MathExpr::Var(wrt) = &args[1] {
                        let expanded_target = args[0].inline_and_expand(fns, true, depth + 1);
                        return expanded_target.differentiate(wrt).simplify();
                    }
                }
                let expanded_args: Vec<MathExpr> = args
                    .iter()
                    .map(|a| a.inline_and_expand(fns, inline_user_fns, depth + 1))
                    .collect();
                if inline_user_fns {
                    if let Some((param_names, body)) = fns.get(name) {
                        if param_names.len() == expanded_args.len() {
                            let mut subst = HashMap::new();
                            for (p, a) in param_names.iter().zip(expanded_args.into_iter()) {
                                subst.insert(p.clone(), a);
                            }
                            let inlined = body.substitute(&subst);
                            return inlined.inline_and_expand(fns, inline_user_fns, depth + 1);
                        }
                    }
                }
                MathExpr::Call(name.clone(), expanded_args)
            }
        }
    }

    /// Compile-Time Symbolic Automatic Differentiation: d(self) / d(var)
    pub fn differentiate(&self, var: &str) -> MathExpr {
        match self {
            MathExpr::Num(_) | MathExpr::Placeholder | MathExpr::Index(_, _) => MathExpr::Num(0.0),
            MathExpr::Var(v) => {
                if v == var {
                    MathExpr::Num(1.0)
                } else {
                    MathExpr::Num(0.0)
                }
            }
            MathExpr::Array(elems) => {
                MathExpr::Array(elems.iter().map(|e| e.differentiate(var)).collect())
            }
            MathExpr::Unary(MathUnaryOp::Neg, inner) => {
                MathExpr::Unary(MathUnaryOp::Neg, Box::new(inner.differentiate(var)))
            }
            MathExpr::Unary(MathUnaryOp::Not, _) => MathExpr::Num(0.0),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => MathExpr::Ternary {
                cond: cond.clone(),
                then_expr: Box::new(then_expr.differentiate(var)),
                else_expr: Box::new(else_expr.differentiate(var)),
            },
            MathExpr::Binary(u, op, v) => {
                let du = u.differentiate(var);
                let dv = v.differentiate(var);
                match op {
                    MathBinOp::Add => MathExpr::Binary(Box::new(du), MathBinOp::Add, Box::new(dv)),
                    MathBinOp::Sub => MathExpr::Binary(Box::new(du), MathBinOp::Sub, Box::new(dv)),
                    MathBinOp::Mul | MathBinOp::MatMul => {
                        // Product rule: du * v + u * dv
                        let left = MathExpr::Binary(Box::new(du), *op, v.clone());
                        let right = MathExpr::Binary(u.clone(), *op, Box::new(dv));
                        MathExpr::Binary(Box::new(left), MathBinOp::Add, Box::new(right))
                    }
                    MathBinOp::Div => {
                        // Quotient rule: (du * v - u * dv) / (v ^ 2)
                        let left = MathExpr::Binary(Box::new(du), MathBinOp::Mul, v.clone());
                        let right = MathExpr::Binary(u.clone(), MathBinOp::Mul, Box::new(dv));
                        let num = MathExpr::Binary(Box::new(left), MathBinOp::Sub, Box::new(right));
                        let den = MathExpr::Binary(
                            v.clone(),
                            MathBinOp::Pow,
                            Box::new(MathExpr::Num(2.0)),
                        );
                        MathExpr::Binary(Box::new(num), MathBinOp::Div, Box::new(den))
                    }
                    MathBinOp::Pow => {
                        if let MathExpr::Num(c) = **v {
                            // Power rule: c * u^(c-1) * du
                            let coeff = MathExpr::Num(c);
                            let pow_minus_1 = MathExpr::Binary(
                                u.clone(),
                                MathBinOp::Pow,
                                Box::new(MathExpr::Num(c - 1.0)),
                            );
                            let term = MathExpr::Binary(
                                Box::new(coeff),
                                MathBinOp::Mul,
                                Box::new(pow_minus_1),
                            );
                            MathExpr::Binary(Box::new(term), MathBinOp::Mul, Box::new(du))
                        } else {
                            MathExpr::Num(0.0)
                        }
                    }
                    _ => MathExpr::Num(0.0),
                }
            }
            MathExpr::Call(func, args) => {
                if args.is_empty() {
                    return MathExpr::Num(0.0);
                }
                let u = &args[0];
                let du = u.differentiate(var);
                let outer_deriv = match func.as_str() {
                    "exp" => MathExpr::Call("exp".to_string(), vec![u.clone()]),
                    "ln" | "log" => MathExpr::Binary(
                        Box::new(MathExpr::Num(1.0)),
                        MathBinOp::Div,
                        Box::new(u.clone()),
                    ),
                    "sqrt" => MathExpr::Binary(
                        Box::new(MathExpr::Num(1.0)),
                        MathBinOp::Div,
                        Box::new(MathExpr::Binary(
                            Box::new(MathExpr::Num(2.0)),
                            MathBinOp::Mul,
                            Box::new(MathExpr::Call("sqrt".to_string(), vec![u.clone()])),
                        )),
                    ),
                    "sin" => MathExpr::Call("cos".to_string(), vec![u.clone()]),
                    "cos" => MathExpr::Unary(
                        MathUnaryOp::Neg,
                        Box::new(MathExpr::Call("sin".to_string(), vec![u.clone()])),
                    ),
                    "tanh" => {
                        let t = MathExpr::Call("tanh".to_string(), vec![u.clone()]);
                        let t2 =
                            MathExpr::Binary(Box::new(t), MathBinOp::Pow, Box::new(MathExpr::Num(2.0)));
                        MathExpr::Binary(
                            Box::new(MathExpr::Num(1.0)),
                            MathBinOp::Sub,
                            Box::new(t2),
                        )
                    }
                    "sigmoid" => {
                        let s = MathExpr::Call("sigmoid".to_string(), vec![u.clone()]);
                        let one_minus_s = MathExpr::Binary(
                            Box::new(MathExpr::Num(1.0)),
                            MathBinOp::Sub,
                            Box::new(s.clone()),
                        );
                        MathExpr::Binary(Box::new(s), MathBinOp::Mul, Box::new(one_minus_s))
                    }
                    "relu" => MathExpr::Ternary {
                        cond: Box::new(MathExpr::Binary(
                            Box::new(u.clone()),
                            MathBinOp::Gt,
                            Box::new(MathExpr::Num(0.0)),
                        )),
                        then_expr: Box::new(MathExpr::Num(1.0)),
                        else_expr: Box::new(MathExpr::Num(0.0)),
                    },
                    _ => MathExpr::Num(1.0),
                };
                MathExpr::Binary(Box::new(outer_deriv), MathBinOp::Mul, Box::new(du))
            }
            MathExpr::Pipeline(stages) => {
                let mut curr = stages[0].clone();
                for st in &stages[1..] {
                    curr = MathParser::desugar_pipe_step(curr, st.clone());
                }
                curr.differentiate(var)
            }
        }
    }

    /// Algebraic simplification and constant folding pass
    pub fn simplify(&self) -> MathExpr {
        match self {
            MathExpr::Num(_) | MathExpr::Var(_) | MathExpr::Placeholder => self.clone(),
            MathExpr::Array(elems) => {
                MathExpr::Array(elems.iter().map(|e| e.simplify()).collect())
            }
            MathExpr::Index(b, i) => {
                MathExpr::Index(Box::new(b.simplify()), Box::new(i.simplify()))
            }
            MathExpr::Unary(op, inner) => {
                let s = inner.simplify();
                if *op == MathUnaryOp::Neg {
                    if let MathExpr::Num(n) = s {
                        return MathExpr::Num(-n);
                    }
                }
                MathExpr::Unary(*op, Box::new(s))
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => MathExpr::Ternary {
                cond: Box::new(cond.simplify()),
                then_expr: Box::new(then_expr.simplify()),
                else_expr: Box::new(else_expr.simplify()),
            },
            MathExpr::Call(name, args) => {
                MathExpr::Call(name.clone(), args.iter().map(|a| a.simplify()).collect())
            }
            MathExpr::Pipeline(stages) => {
                MathExpr::Pipeline(stages.iter().map(|s| s.simplify()).collect())
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let l = lhs.simplify();
                let r = rhs.simplify();
                match (&l, op, &r) {
                    (MathExpr::Num(a), MathBinOp::Add, MathExpr::Num(b)) => MathExpr::Num(a + b),
                    (MathExpr::Num(a), MathBinOp::Sub, MathExpr::Num(b)) => MathExpr::Num(a - b),
                    (MathExpr::Num(a), MathBinOp::Mul, MathExpr::Num(b)) => MathExpr::Num(a * b),
                    (MathExpr::Num(a), MathBinOp::Pow, MathExpr::Num(b)) => MathExpr::Num(a.powf(*b)),
                    (MathExpr::Num(a), MathBinOp::Add, _) if a.abs() < 1e-12 => r,
                    (_, MathBinOp::Add, MathExpr::Num(b)) if b.abs() < 1e-12 => l,
                    (_, MathBinOp::Sub, MathExpr::Num(b)) if b.abs() < 1e-12 => l,
                    (MathExpr::Num(a), MathBinOp::Sub, _) if a.abs() < 1e-12 => {
                        MathExpr::Unary(MathUnaryOp::Neg, Box::new(r))
                    }
                    (MathExpr::Num(a), MathBinOp::Mul, _) if a.abs() < 1e-12 => MathExpr::Num(0.0),
                    (_, MathBinOp::Mul, MathExpr::Num(b)) if b.abs() < 1e-12 => MathExpr::Num(0.0),
                    (MathExpr::Num(a), MathBinOp::Mul, _) if (a - 1.0).abs() < 1e-12 => r,
                    (_, MathBinOp::Mul, MathExpr::Num(b)) if (b - 1.0).abs() < 1e-12 => l,
                    (_, MathBinOp::Div, MathExpr::Num(b)) if (b - 1.0).abs() < 1e-12 => l,
                    (MathExpr::Num(a), MathBinOp::Div, _) if a.abs() < 1e-12 => MathExpr::Num(0.0),
                    (_, MathBinOp::Pow, MathExpr::Num(b)) if (b - 1.0).abs() < 1e-12 => l,
                    (_, MathBinOp::Pow, MathExpr::Num(b)) if b.abs() < 1e-12 => MathExpr::Num(1.0),
                    _ => MathExpr::Binary(Box::new(l), *op, Box::new(r)),
                }
            }
        }
    }
}

/// Top-level Items in a Mathematical Module
#[derive(Debug, Clone, PartialEq)]
pub enum MathItem {
    Constant {
        name: String,
        val: f64,
    },
    Binding {
        name: String,
        expr: MathExpr,
    },
    Function {
        name: String,
        params: Vec<MathParam>,
        ret_type: Option<MathDtype>,
        body: MathExpr,
    },
    Expression(MathExpr),
}

/// Complete parsed mathematical module
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MathProgram {
    pub items: Vec<MathItem>,
}

// ============================================================================
// TOKENIZER / LEXER
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Num(f64),
    Equal,      // =
    Arrow,      // ->
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    At,         // @ (MatMul)
    Caret,      // ^ (Power)
    Question,   // ? (Conditional ternary)
    EqEq,       // ==
    NotEq,      // !=
    Lt,         // <
    Lte,        // <=
    Gt,         // >
    Gte,        // >=
    LParen,     // (
    RParen,     // )
    LBracket,   // [
    RBracket,   // ]
    Comma,      // ,
    Colon,      // :
    Semicolon,  // ;
    Underscore, // _
}

pub struct MathLexer<'a> {
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    src: &'a str,
}

impl<'a> MathLexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.char_indices().peekable(),
            src,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut depth: usize = 0;

        while let Some(&(i, ch)) = self.chars.peek() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.chars.next();
                }
                '\n' => {
                    self.chars.next();
                    if depth == 0
                        && matches!(
                            tokens.last(),
                            Some(
                                Token::Ident(_)
                                    | Token::Num(_)
                                    | Token::RParen
                                    | Token::RBracket
                            )
                        )
                    {
                        tokens.push(Token::Semicolon);
                    }
                }
                '/' => {
                    self.chars.next();
                    if let Some(&(_, '/')) = self.chars.peek() {
                        self.chars.next();
                        while let Some(&(_, next_c)) = self.chars.peek() {
                            if next_c == '\n' {
                                break;
                            }
                            self.chars.next();
                        }
                    } else if let Some(&(_, '*')) = self.chars.peek() {
                        self.chars.next();
                        while let Some((_, next_c)) = self.chars.next() {
                            if next_c == '*' {
                                if let Some(&(_, '/')) = self.chars.peek() {
                                    self.chars.next();
                                    break;
                                }
                            }
                        }
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                ';' => {
                    self.chars.next();
                    tokens.push(Token::Semicolon);
                }
                '=' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::EqEq);
                    } else {
                        tokens.push(Token::Equal);
                    }
                }
                '-' => {
                    self.chars.next();
                    if let Some(&(_, '>')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::Arrow);
                    } else {
                        tokens.push(Token::Minus);
                    }
                }
                '+' => {
                    self.chars.next();
                    tokens.push(Token::Plus);
                }
                '*' => {
                    self.chars.next();
                    tokens.push(Token::Star);
                }
                '@' => {
                    self.chars.next();
                    tokens.push(Token::At);
                }
                '^' => {
                    self.chars.next();
                    tokens.push(Token::Caret);
                }
                '?' => {
                    self.chars.next();
                    tokens.push(Token::Question);
                }
                '!' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::NotEq);
                    } else {
                        return Err("Unexpected '!' without '='".to_string());
                    }
                }
                '<' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::Lte);
                    } else {
                        tokens.push(Token::Lt);
                    }
                }
                '>' => {
                    self.chars.next();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::Gte);
                    } else {
                        tokens.push(Token::Gt);
                    }
                }
                '(' => {
                    self.chars.next();
                    depth += 1;
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.chars.next();
                    depth = depth.saturating_sub(1);
                    tokens.push(Token::RParen);
                }
                '[' => {
                    self.chars.next();
                    depth += 1;
                    tokens.push(Token::LBracket);
                }
                ']' => {
                    self.chars.next();
                    depth = depth.saturating_sub(1);
                    tokens.push(Token::RBracket);
                }
                ',' => {
                    self.chars.next();
                    tokens.push(Token::Comma);
                }
                ':' => {
                    self.chars.next();
                    tokens.push(Token::Colon);
                }
                '_' => {
                    self.chars.next();
                    if let Some(&(_, next_c)) = self.chars.peek() {
                        if next_c.is_alphanumeric() {
                            let mut ident = String::from("_");
                            while let Some(&(_, c)) = self.chars.peek() {
                                if c.is_alphanumeric() || c == '_' {
                                    ident.push(c);
                                    self.chars.next();
                                } else {
                                    break;
                                }
                            }
                            tokens.push(Token::Ident(ident));
                            continue;
                        }
                    }
                    tokens.push(Token::Underscore);
                }
                '0'..='9' => {
                    let start = i;
                    let mut end = i;
                    while let Some(&(curr_i, c)) = self.chars.peek() {
                        if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' {
                            end = curr_i + c.len_utf8();
                            self.chars.next();
                            if c == 'e' || c == 'E' {
                                if let Some(&(_, sign)) = self.chars.peek() {
                                    if sign == '+' || sign == '-' {
                                        end += sign.len_utf8();
                                        self.chars.next();
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    let num_str = &self.src[start..end];
                    let val: f64 = num_str
                        .parse()
                        .map_err(|e| format!("Invalid number '{}': {}", num_str, e))?;
                    tokens.push(Token::Num(val));
                }
                'a'..='z' | 'A'..='Z' => {
                    let start = i;
                    let mut end = i;
                    while let Some(&(curr_i, c)) = self.chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            end = curr_i + c.len_utf8();
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    let ident = &self.src[start..end];
                    tokens.push(Token::Ident(ident.to_string()));
                }
                _ => {
                    return Err(format!("Unexpected character '{}' at byte offset {}", ch, i));
                }
            }
        }

        Ok(tokens)
    }
}

// ============================================================================
// PARSER
// ============================================================================

pub struct MathParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl MathParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next_tok(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn match_tok(&mut self, expected: &Token) -> bool {
        if let Some(t) = self.peek() {
            if t == expected {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.match_tok(expected) {
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                expected,
                self.peek()
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<MathProgram, String> {
        let mut program = MathProgram::default();

        while self.pos < self.tokens.len() {
            if self.match_tok(&Token::Semicolon) {
                continue;
            }

            let item = self.parse_item()?;
            program.items.push(item);
        }

        Ok(program)
    }

    fn is_func_def(&self) -> bool {
        if let Some(Token::Ident(_)) = self.tokens.get(self.pos) {
            if let Some(Token::LParen) = self.tokens.get(self.pos + 1) {
                let mut depth = 0;
                let mut i = self.pos + 1;
                while i < self.tokens.len() {
                    match &self.tokens[i] {
                        Token::LParen => depth += 1,
                        Token::RParen => {
                            depth -= 1;
                            if depth == 0 {
                                let mut next_idx = i + 1;
                                if let Some(Token::Colon) = self.tokens.get(next_idx) {
                                    next_idx += 2;
                                }
                                return matches!(self.tokens.get(next_idx), Some(Token::Equal));
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
        }
        false
    }

    fn parse_item(&mut self) -> Result<MathItem, String> {
        // 1. Check for function declaration: name(params) = expr
        if self.is_func_def() {
            if let Some(Token::Ident(name)) = self.next_tok() {
                self.expect(&Token::LParen)?;
                let params = self.parse_params()?;
                self.expect(&Token::RParen)?;

                let mut ret_type = None;
                if self.match_tok(&Token::Colon) {
                    ret_type = Some(self.parse_dtype()?);
                }

                self.expect(&Token::Equal)?;
                let body = self.parse_expr()?;
                return Ok(MathItem::Function {
                    name,
                    params,
                    ret_type,
                    body,
                });
            }
        }

        // 2. Check for constant or variable binding: name = expr
        if let Some(Token::Ident(name)) = self.peek().cloned() {
            if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::Equal {
                self.pos += 2; // consume ident and '='
                let expr = self.parse_expr()?;
                if let MathExpr::Num(val) = expr {
                    return Ok(MathItem::Constant { name, val });
                } else {
                    return Ok(MathItem::Binding { name, expr });
                }
            }
        }

        // 3. Top-level expression
        let expr = self.parse_expr()?;
        Ok(MathItem::Expression(expr))
    }

    fn parse_params(&mut self) -> Result<Vec<MathParam>, String> {
        let mut params = Vec::new();

        if let Some(Token::RParen) = self.peek() {
            return Ok(params);
        }

        loop {
            let name = match self.next_tok() {
                Some(Token::Ident(n)) => n,
                other => return Err(format!("Expected parameter name, found {:?}", other)),
            };

            let mut dtype = MathDtype::F32;
            let mut dims = Vec::new();

            if self.match_tok(&Token::Colon) {
                dtype = self.parse_dtype()?;

                if self.match_tok(&Token::LBracket) {
                    loop {
                        if let Some(Token::Num(n)) = self.next_tok() {
                            dims.push(n as usize);
                        } else {
                            return Err("Expected dimension size in tensor brackets".to_string());
                        }

                        if self.match_tok(&Token::Comma) {
                            continue;
                        } else if self.match_tok(&Token::RBracket) {
                            break;
                        } else {
                            return Err("Expected ',' or ']' in tensor dimension".to_string());
                        }
                    }
                }
            }

            params.push(MathParam { name, dtype, dims });

            if self.match_tok(&Token::Comma) {
                continue;
            } else {
                break;
            }
        }

        Ok(params)
    }

    fn parse_dtype(&mut self) -> Result<MathDtype, String> {
        if let Some(Token::Ident(s)) = self.next_tok() {
            MathDtype::from_str(&s).ok_or_else(|| format!("Unknown data type '{}'", s))
        } else {
            Err("Expected data type identifier (e.g. f16, f32, t2)".to_string())
        }
    }

    // Precedence hierarchy:
    // 1. Pipeline: expr -> expr
    // 2. Conditional Ternary: cond ? then : else
    // 3. Comparison: ==, !=, <, <=, >, >=
    // 4. Additive: +, -
    // 5. Multiplicative: *, /, @ (MatMul)
    // 6. Exponentiation: ^
    // 7. Unary: -
    // 8. Primary: num, [array], var, call, (expr)

    pub fn parse_expr(&mut self) -> Result<MathExpr, String> {
        self.parse_pipeline()
    }

    fn parse_pipeline(&mut self) -> Result<MathExpr, String> {
        let mut stages = vec![self.parse_ternary()?];

        while self.match_tok(&Token::Arrow) {
            let next_stage = self.parse_ternary()?;
            stages.push(next_stage);
        }

        if stages.len() == 1 {
            Ok(stages.pop().unwrap())
        } else {
            let mut curr = stages.remove(0);
            for stage in stages {
                curr = Self::desugar_pipe_step(curr, stage);
            }
            Ok(curr)
        }
    }

    fn desugar_pipe_step(input: MathExpr, target: MathExpr) -> MathExpr {
        match target {
            MathExpr::Var(func_name) => MathExpr::Call(func_name, vec![input]),
            MathExpr::Call(func_name, mut args) => {
                let mut found_placeholder = false;
                for arg in args.iter_mut() {
                    if let MathExpr::Placeholder = arg {
                        *arg = input.clone();
                        found_placeholder = true;
                        break;
                    }
                }
                if !found_placeholder {
                    args.insert(0, input);
                }
                MathExpr::Call(func_name, args)
            }
            other => MathExpr::Binary(Box::new(input), MathBinOp::MatMul, Box::new(other)),
        }
    }

    fn parse_ternary(&mut self) -> Result<MathExpr, String> {
        let cond = self.parse_comparison()?;

        if self.match_tok(&Token::Question) {
            let then_expr = self.parse_expr()?;
            self.expect(&Token::Colon)?;
            let else_expr = self.parse_ternary()?;
            Ok(MathExpr::Ternary {
                cond: Box::new(cond),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            })
        } else {
            Ok(cond)
        }
    }

    fn parse_comparison(&mut self) -> Result<MathExpr, String> {
        let mut lhs = self.parse_additive()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::EqEq => MathBinOp::Eq,
                Token::NotEq => MathBinOp::Neq,
                Token::Lt => MathBinOp::Lt,
                Token::Lte => MathBinOp::Lte,
                Token::Gt => MathBinOp::Gt,
                Token::Gte => MathBinOp::Gte,
                _ => break,
            };
            self.pos += 1;
            let rhs = self.parse_additive()?;
            lhs = MathExpr::Binary(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_additive(&mut self) -> Result<MathExpr, String> {
        let mut lhs = self.parse_multiplicative()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Plus => MathBinOp::Add,
                Token::Minus => MathBinOp::Sub,
                _ => break,
            };
            self.pos += 1;
            let rhs = self.parse_multiplicative()?;
            lhs = MathExpr::Binary(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_multiplicative(&mut self) -> Result<MathExpr, String> {
        let mut lhs = self.parse_power()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Star => MathBinOp::Mul,
                Token::Slash => MathBinOp::Div,
                Token::At => MathBinOp::MatMul,
                _ => break,
            };
            self.pos += 1;
            let rhs = self.parse_power()?;
            lhs = MathExpr::Binary(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_power(&mut self) -> Result<MathExpr, String> {
        let lhs = self.parse_unary()?;

        if self.match_tok(&Token::Caret) {
            let rhs = self.parse_power()?;
            Ok(MathExpr::Binary(Box::new(lhs), MathBinOp::Pow, Box::new(rhs)))
        } else {
            Ok(lhs)
        }
    }

    fn parse_unary(&mut self) -> Result<MathExpr, String> {
        if self.match_tok(&Token::Minus) {
            let expr = self.parse_unary()?;
            Ok(MathExpr::Unary(MathUnaryOp::Neg, Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<MathExpr, String> {
        let mut base = match self.next_tok() {
            Some(Token::Num(n)) => MathExpr::Num(n),
            Some(Token::Underscore) => MathExpr::Placeholder,
            Some(Token::LBracket) => {
                // Vector or Matrix literal: [1, 2, 3] or [[1, 2], [3, 4]]
                let mut elems = Vec::new();
                if !self.match_tok(&Token::RBracket) {
                    loop {
                        elems.push(self.parse_expr()?);
                        if self.match_tok(&Token::Comma) {
                            continue;
                        } else if self.match_tok(&Token::RBracket) {
                            break;
                        } else {
                            return Err("Expected ',' or ']' in array/tensor literal".to_string());
                        }
                    }
                }
                MathExpr::Array(elems)
            }
            Some(Token::Ident(name)) => {
                if self.match_tok(&Token::LParen) {
                    let mut args = Vec::new();
                    if !self.match_tok(&Token::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.match_tok(&Token::Comma) {
                                continue;
                            } else if self.match_tok(&Token::RParen) {
                                break;
                            } else {
                                return Err("Expected ',' or ')' in argument list".to_string());
                            }
                        }
                    }
                    MathExpr::Call(name, args)
                } else {
                    MathExpr::Var(name)
                }
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                expr
            }
            other => return Err(format!("Unexpected token in expression: {:?}", other)),
        };

        // Support postfix indexing: x[0], W[1][0]
        while self.match_tok(&Token::LBracket) {
            let idx = self.parse_expr()?;
            self.expect(&Token::RBracket)?;
            base = MathExpr::Index(Box::new(base), Box::new(idx));
        }

        Ok(base)
    }
}

// ============================================================================
// RUNTIME EVALUATOR (Scalars, Vectors, Matrices & Tensor Reductions)
// ============================================================================

#[derive(Clone)]
pub struct MathEvaluator {
    pub env: HashMap<String, MathValue>,
    pub functions: HashMap<String, (Vec<String>, MathExpr)>,
}

impl Default for MathEvaluator {
    fn default() -> Self {
        let mut env = HashMap::new();
        env.insert("pi".to_string(), MathValue::Scalar(std::f64::consts::PI));
        env.insert("e".to_string(), MathValue::Scalar(std::f64::consts::E));
        env.insert("eps".to_string(), MathValue::Scalar(1e-5));
        Self {
            env,
            functions: HashMap::new(),
        }
    }
}

impl MathEvaluator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes program and returns last evaluated scalar (if scalar) for backward compatibility
    pub fn execute_program(&mut self, prog: &MathProgram) -> Result<Option<f64>, String> {
        let val = self.execute_program_value(prog)?;
        Ok(val.and_then(|v| v.as_scalar()))
    }

    /// Executes program and returns full MathValue (Scalar, Vector, or Matrix)
    pub fn execute_program_value(&mut self, prog: &MathProgram) -> Result<Option<MathValue>, String> {
        let mut last_res = None;

        for item in &prog.items {
            match item {
                MathItem::Constant { name, val } => {
                    self.env.insert(name.clone(), MathValue::Scalar(*val));
                }
                MathItem::Binding { name, expr } => {
                    let val = self.eval_value(expr)?;
                    self.env.insert(name.clone(), val);
                }
                MathItem::Function {
                    name, params, body, ..
                } => {
                    let p_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
                    self.functions.insert(name.clone(), (p_names, body.clone()));
                }
                MathItem::Expression(expr) => {
                    let res = self.eval_value(expr)?;
                    last_res = Some(res);
                }
            }
        }

        Ok(last_res)
    }

    pub fn eval(&self, expr: &MathExpr) -> Result<f64, String> {
        let val = self.eval_value(expr)?;
        val.as_scalar()
            .ok_or_else(|| format!("Expected scalar result, got tensor {}", val))
    }

    pub fn eval_value(&self, expr: &MathExpr) -> Result<MathValue, String> {
        match expr {
            MathExpr::Num(n) => Ok(MathValue::Scalar(*n)),
            MathExpr::Var(name) => self
                .env
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined mathematical variable '{}'", name)),
            MathExpr::Array(elems) => {
                let mut evaluated = Vec::with_capacity(elems.len());
                for e in elems {
                    evaluated.push(self.eval_value(e)?);
                }
                if evaluated.is_empty() {
                    return Ok(MathValue::Vector(Vec::new()));
                }
                if matches!(evaluated[0], MathValue::Vector(_)) {
                    let mut rows = Vec::with_capacity(evaluated.len());
                    for item in evaluated {
                        if let MathValue::Vector(r) = item {
                            rows.push(r);
                        } else {
                            return Err("Inconsistent matrix row types in array literal".to_string());
                        }
                    }
                    Ok(MathValue::Matrix(rows))
                } else {
                    let mut vec = Vec::with_capacity(evaluated.len());
                    for item in evaluated {
                        if let Some(s) = item.as_scalar() {
                            vec.push(s);
                        } else {
                            return Err("Expected scalar elements inside 1D vector".to_string());
                        }
                    }
                    Ok(MathValue::Vector(vec))
                }
            }
            MathExpr::Index(base, idx) => {
                let b_val = self.eval_value(base)?;
                let i_val = self.eval_value(idx)?;
                let i_num = i_val
                    .as_scalar()
                    .ok_or_else(|| "Index must evaluate to a scalar integer".to_string())?
                    as usize;
                match b_val {
                    MathValue::Vector(vec) => vec
                        .get(i_num)
                        .copied()
                        .map(MathValue::Scalar)
                        .ok_or_else(|| format!("Vector index {} out of bounds (len {})", i_num, vec.len())),
                    MathValue::Matrix(mat) => mat
                        .get(i_num)
                        .cloned()
                        .map(MathValue::Vector)
                        .ok_or_else(|| format!("Matrix row index {} out of bounds (rows {})", i_num, mat.len())),
                    MathValue::Scalar(s) => {
                        if i_num == 0 {
                            Ok(MathValue::Scalar(s))
                        } else {
                            Err("Cannot index into a scalar with index > 0".to_string())
                        }
                    }
                }
            }
            MathExpr::Placeholder => {
                Err("Unexpected unresolved pipeline placeholder '_'".to_string())
            }
            MathExpr::Unary(op, inner) => {
                let v = self.eval_value(inner)?;
                match op {
                    MathUnaryOp::Neg => Ok(v.map_unary(|x| -x)),
                    MathUnaryOp::Not => Ok(v.map_unary(|x| if x == 0.0 { 1.0 } else { 0.0 })),
                }
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                let c = self.eval_value(cond)?;
                if let Some(s) = c.as_scalar() {
                    if s != 0.0 {
                        self.eval_value(then_expr)
                    } else {
                        self.eval_value(else_expr)
                    }
                } else {
                    Err("Ternary condition must evaluate to a scalar".to_string())
                }
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let l = self.eval_value(lhs)?;
                let r = self.eval_value(rhs)?;
                self.eval_binary_op(&l, *op, &r)
            }
            MathExpr::Call(name, args) => {
                match name.as_str() {
                    "sqrt" => Ok(self.eval_single_val(args, "sqrt")?.map_unary(|x| x.sqrt())),
                    "exp" => Ok(self.eval_single_val(args, "exp")?.map_unary(|x| x.exp())),
                    "ln" | "log" => Ok(self.eval_single_val(args, "log")?.map_unary(|x| x.ln())),
                    "sin" => Ok(self.eval_single_val(args, "sin")?.map_unary(|x| x.sin())),
                    "cos" => Ok(self.eval_single_val(args, "cos")?.map_unary(|x| x.cos())),
                    "tan" => Ok(self.eval_single_val(args, "tan")?.map_unary(|x| x.tan())),
                    "tanh" => Ok(self.eval_single_val(args, "tanh")?.map_unary(|x| x.tanh())),
                    "abs" => Ok(self.eval_single_val(args, "abs")?.map_unary(|x| x.abs())),
                    "sigmoid" => Ok(self
                        .eval_single_val(args, "sigmoid")?
                        .map_unary(|z| 1.0 / (1.0 + (-z).exp()))),
                    "relu" => Ok(self
                        .eval_single_val(args, "relu")?
                        .map_unary(|z| z.max(0.0))),
                    "gelu" => Ok(self.eval_single_val(args, "gelu")?.map_unary(|z| {
                        0.5 * z * (1.0 + (0.79788456 * (z + 0.044715 * z.powi(3))).tanh())
                    })),
                    "softmax" => {
                        let val = self.eval_single_val(args, "softmax")?;
                        let elems = val.flatten_elements();
                        let max_v = elems.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let exps: Vec<f64> = elems.iter().map(|&x| (x - max_v).exp()).collect();
                        let sum_exp: f64 = exps.iter().sum();
                        Ok(MathValue::Vector(
                            exps.into_iter().map(|e| e / sum_exp).collect(),
                        ))
                    }
                    "norm" => {
                        let val = self.eval_single_val(args, "norm")?;
                        let elems = val.flatten_elements();
                        let sq_sum: f64 = elems.iter().map(|x| x * x).sum();
                        Ok(MathValue::Scalar(sq_sum.sqrt()))
                    }
                    "mean" | "sum" => {
                        if args.is_empty() {
                            return Err(format!("'{}' requires at least one argument", name));
                        }
                        let mut all_elems = Vec::new();
                        for a in args {
                            all_elems.extend(self.eval_value(a)?.flatten_elements());
                        }
                        let sum: f64 = all_elems.iter().sum();
                        if name == "mean" {
                            Ok(MathValue::Scalar(sum / all_elems.len().max(1) as f64))
                        } else {
                            Ok(MathValue::Scalar(sum))
                        }
                    }
                    "max" | "min" => {
                        if args.is_empty() {
                            return Err(format!("'{}' requires at least one argument", name));
                        }
                        let mut all_elems = Vec::new();
                        for a in args {
                            all_elems.extend(self.eval_value(a)?.flatten_elements());
                        }
                        let res = if name == "max" {
                            all_elems.into_iter().fold(f64::NEG_INFINITY, f64::max)
                        } else {
                            all_elems.into_iter().fold(f64::INFINITY, f64::min)
                        };
                        Ok(MathValue::Scalar(res))
                    }
                    "prod" => {
                        if args.is_empty() {
                            return Err("'prod' requires at least one argument".to_string());
                        }
                        let mut all_elems = Vec::new();
                        for a in args {
                            all_elems.extend(self.eval_value(a)?.flatten_elements());
                        }
                        let prod: f64 = all_elems.iter().product();
                        Ok(MathValue::Scalar(prod))
                    }
                    "scan" | "cumsum" => {
                        let val = self.eval_single_val(args, name)?;
                        let elems = val.flatten_elements();
                        let mut running = 0.0;
                        let mut out = Vec::with_capacity(elems.len());
                        for x in elems {
                            running += x;
                            out.push(running);
                        }
                        Ok(MathValue::Vector(out))
                    }
                    "dot" => {
                        if args.len() != 2 {
                            return Err("'dot' expects 2 arguments".to_string());
                        }
                        let a = self.eval_value(&args[0])?;
                        let b = self.eval_value(&args[1])?;
                        self.eval_binary_op(&a, MathBinOp::MatMul, &b)
                    }
                    "range" => {
                        let (start, end) = if args.len() == 1 {
                            let n = self
                                .eval_value(&args[0])?
                                .as_scalar()
                                .ok_or_else(|| "'range' expects scalar bound".to_string())?
                                as i64;
                            (0, n)
                        } else if args.len() == 2 {
                            let s = self
                                .eval_value(&args[0])?
                                .as_scalar()
                                .ok_or_else(|| "'range' expects scalar start".to_string())?
                                as i64;
                            let e = self
                                .eval_value(&args[1])?
                                .as_scalar()
                                .ok_or_else(|| "'range' expects scalar end".to_string())?
                                as i64;
                            (s, e)
                        } else {
                            return Err("'range' expects 1 or 2 arguments".to_string());
                        };
                        let vec: Vec<f64> = (start..end).map(|i| i as f64).collect();
                        Ok(MathValue::Vector(vec))
                    }
                    "grad" | "diff" => {
                        if args.len() != 2 {
                            return Err(format!("'{}' expects (expr, var)", name));
                        }
                        let wrt = match &args[1] {
                            MathExpr::Var(v) => v.clone(),
                            _ => return Err("Second argument to 'grad' must be a variable name".to_string()),
                        };
                        let inlined = args[0].inline_and_expand(&self.functions, true, 0);
                        let deriv_expr = inlined.differentiate(&wrt).simplify();
                        self.eval_value(&deriv_expr)
                    }
                    user_fn => {
                        if let Some((param_names, body)) = self.functions.get(user_fn) {
                            if param_names.len() != args.len() {
                                return Err(format!(
                                    "Function '{}' expects {} arguments, got {}",
                                    user_fn,
                                    param_names.len(),
                                    args.len()
                                ));
                            }
                            let mut scoped_eval = self.clone();
                            for (p_name, arg_expr) in param_names.iter().zip(args.iter()) {
                                let arg_val = self.eval_value(arg_expr)?;
                                scoped_eval.env.insert(p_name.clone(), arg_val);
                            }
                            scoped_eval.eval_value(body)
                        } else {
                            Err(format!("Undefined mathematical function '{}'", user_fn))
                        }
                    }
                }
            }
            MathExpr::Pipeline(stages) => {
                let mut curr_expr = stages[0].clone();
                for stage in &stages[1..] {
                    curr_expr = MathParser::desugar_pipe_step(curr_expr, stage.clone());
                }
                self.eval_value(&curr_expr)
            }
        }
    }

    fn eval_single_val(&self, args: &[MathExpr], name: &str) -> Result<MathValue, String> {
        if args.len() != 1 {
            Err(format!(
                "'{}' expects exactly 1 argument, found {}",
                name,
                args.len()
            ))
        } else {
            self.eval_value(&args[0])
        }
    }

    fn eval_binary_op(
        &self,
        lhs: &MathValue,
        op: MathBinOp,
        rhs: &MathValue,
    ) -> Result<MathValue, String> {
        // Matrix Multiplication (@)
        if op == MathBinOp::MatMul {
            return match (lhs, rhs) {
                (MathValue::Scalar(a), MathValue::Scalar(b)) => Ok(MathValue::Scalar(a * b)),
                (MathValue::Vector(u), MathValue::Vector(v)) => {
                    if u.len() != v.len() {
                        return Err(format!(
                            "Dot product dimension mismatch: {} vs {}",
                            u.len(),
                            v.len()
                        ));
                    }
                    let dot: f64 = u.iter().zip(v.iter()).map(|(a, b)| a * b).sum();
                    Ok(MathValue::Scalar(dot))
                }
                (MathValue::Matrix(mat), MathValue::Vector(vec)) => {
                    let mut out = Vec::with_capacity(mat.len());
                    for row in mat {
                        if row.len() != vec.len() {
                            return Err(format!(
                                "Matrix-Vector dimension mismatch: row len {} vs vec len {}",
                                row.len(),
                                vec.len()
                            ));
                        }
                        let dot: f64 = row.iter().zip(vec.iter()).map(|(a, b)| a * b).sum();
                        out.push(dot);
                    }
                    Ok(MathValue::Vector(out))
                }
                (MathValue::Matrix(a), MathValue::Matrix(b)) => {
                    if a.is_empty() || b.is_empty() || a[0].len() != b.len() {
                        return Err("Matrix-Matrix dimension mismatch".to_string());
                    }
                    let m = a.len();
                    let k = b.len();
                    let n = b[0].len();
                    let mut out = vec![vec![0.0; n]; m];
                    for i in 0..m {
                        for p in 0..k {
                            for j in 0..n {
                                out[i][j] += a[i][p] * b[p][j];
                            }
                        }
                    }
                    Ok(MathValue::Matrix(out))
                }
                _ => Err("Invalid operand shapes for '@' matrix multiplication".to_string()),
            };
        }

        // Element-wise scalar operation helper
        let scalar_op = |a: f64, b: f64| -> Result<f64, String> {
            match op {
                MathBinOp::Add => Ok(a + b),
                MathBinOp::Sub => Ok(a - b),
                MathBinOp::Mul | MathBinOp::MatMul => Ok(a * b),
                MathBinOp::Div => {
                    if b == 0.0 {
                        Err("Division by zero in mathematical expression".to_string())
                    } else {
                        Ok(a / b)
                    }
                }
                MathBinOp::Pow => Ok(a.powf(b)),
                MathBinOp::Eq => Ok(if (a - b).abs() < 1e-9 { 1.0 } else { 0.0 }),
                MathBinOp::Neq => Ok(if (a - b).abs() >= 1e-9 { 1.0 } else { 0.0 }),
                MathBinOp::Lt => Ok(if a < b { 1.0 } else { 0.0 }),
                MathBinOp::Lte => Ok(if a <= b { 1.0 } else { 0.0 }),
                MathBinOp::Gt => Ok(if a > b { 1.0 } else { 0.0 }),
                MathBinOp::Gte => Ok(if a >= b { 1.0 } else { 0.0 }),
            }
        };

        match (lhs, rhs) {
            (MathValue::Scalar(a), MathValue::Scalar(b)) => Ok(MathValue::Scalar(scalar_op(*a, *b)?)),
            (MathValue::Vector(vec), MathValue::Scalar(s)) => {
                let mut out = Vec::with_capacity(vec.len());
                for &x in vec {
                    out.push(scalar_op(x, *s)?);
                }
                Ok(MathValue::Vector(out))
            }
            (MathValue::Scalar(s), MathValue::Vector(vec)) => {
                let mut out = Vec::with_capacity(vec.len());
                for &x in vec {
                    out.push(scalar_op(*s, x)?);
                }
                Ok(MathValue::Vector(out))
            }
            (MathValue::Vector(u), MathValue::Vector(v)) => {
                if u.len() != v.len() {
                    return Err(format!(
                        "Vector length mismatch in '{}' operation: {} vs {}",
                        op,
                        u.len(),
                        v.len()
                    ));
                }
                let mut out = Vec::with_capacity(u.len());
                for (&a, &b) in u.iter().zip(v.iter()) {
                    out.push(scalar_op(a, b)?);
                }
                Ok(MathValue::Vector(out))
            }
            (MathValue::Matrix(mat), MathValue::Scalar(s)) => {
                let mut out = Vec::with_capacity(mat.len());
                for row in mat {
                    let mut r = Vec::with_capacity(row.len());
                    for &x in row {
                        r.push(scalar_op(x, *s)?);
                    }
                    out.push(r);
                }
                Ok(MathValue::Matrix(out))
            }
            (MathValue::Scalar(s), MathValue::Matrix(mat)) => {
                let mut out = Vec::with_capacity(mat.len());
                for row in mat {
                    let mut r = Vec::with_capacity(row.len());
                    for &x in row {
                        r.push(scalar_op(*s, x)?);
                    }
                    out.push(r);
                }
                Ok(MathValue::Matrix(out))
            }
            (MathValue::Matrix(a), MathValue::Matrix(b)) => {
                if a.len() != b.len() {
                    return Err("Matrix row count mismatch".to_string());
                }
                let mut out = Vec::with_capacity(a.len());
                for (ra, rb) in a.iter().zip(b.iter()) {
                    if ra.len() != rb.len() {
                        return Err("Matrix column count mismatch".to_string());
                    }
                    let mut row = Vec::with_capacity(ra.len());
                    for (&x, &y) in ra.iter().zip(rb.iter()) {
                        row.push(scalar_op(x, y)?);
                    }
                    out.push(row);
                }
                Ok(MathValue::Matrix(out))
            }
            _ => Err("Incompatible tensor shapes for binary operation".to_string()),
        }
    }
}

// ============================================================================
// TRANSPILERS & CODE GENERATORS (ISO C23, NVIDIA CUDA PTX, 4-SLOT VLIW)
// ============================================================================

impl MathProgram {
    /// Collects all defined mathematical functions for beta-reduction inlining and symbolic autodiff
    pub fn collect_functions(&self) -> HashMap<String, (Vec<String>, MathExpr)> {
        let mut fns = HashMap::new();
        for item in &self.items {
            if let MathItem::Function {
                name, params, body, ..
            } = item
            {
                let p_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
                fns.insert(name.clone(), (p_names, body.clone()));
            }
        }
        fns
    }

    /// Infers function return dtype from explicit annotation or parameter precisions
    fn infer_return_dtype(params: &[MathParam], explicit: &Option<MathDtype>) -> MathDtype {
        if let Some(dt) = explicit {
            return dt.clone();
        }
        if !params.is_empty() && params.iter().all(|p| p.dtype == MathDtype::F16) {
            return MathDtype::F16;
        }
        if params.iter().any(|p| p.dtype == MathDtype::F64) {
            return MathDtype::F64;
        }
        MathDtype::F32
    }

    /// Compiles standard math into standalone ISO C23 source code with strength reduction,
    /// symbolic autodiff (`grad`), and shaped tensor pointer loops (`f16[M, N]`)
    pub fn to_c23(&self) -> String {
        let mut out = String::new();
        out.push_str("// ============================================================================\n");
        out.push_str("// CRON Mathematical Calculus -> ISO C23 Native Source\n");
        out.push_str("// Generated by CRON Sovereign Math Engine (Zero-Word Calculus)\n");
        out.push_str("// ============================================================================\n\n");
        out.push_str("#include <stdint.h>\n#include <stdbool.h>\n#include <math.h>\n\n");

        let fns = self.collect_functions();

        for item in &self.items {
            if let MathItem::Constant { name, val } = item {
                out.push_str(&format!("static const float {} = {:.8}f;\n", name, val));
            }
        }
        out.push('\n');

        for item in &self.items {
            if let MathItem::Function {
                name,
                params,
                ret_type,
                body,
            } = item
            {
                let inferred_ret = Self::infer_return_dtype(params, ret_type);
                let r_type = inferred_ret.to_c23_type();
                let expanded_body = body.inline_and_expand(&fns, false, 0).simplify();

                // Check if function operates on shaped tensor tiles (e.g., W: f16[16, 16], x: f16[16])
                let has_shaped_params = params.iter().any(|p| !p.dims.is_empty());
                if has_shaped_params {
                    let mut p_decls: Vec<String> = params
                        .iter()
                        .map(|p| {
                            if p.dims.is_empty() {
                                format!("{} {}", p.dtype.to_c23_type(), p.name)
                            } else {
                                format!("const {}* restrict {}", p.dtype.to_c23_type(), p.name)
                            }
                        })
                        .collect();
                    p_decls.push(format!("{}* restrict out", r_type));
                    out.push_str(&format!("void {}({}) {{\n", name, p_decls.join(", ")));

                    // Detect 2D Matrix-Vector Linear Layer pattern: W[M, N] @ x[N] (+ b[M])
                    let mat_param = params.iter().find(|p| p.dims.len() == 2);
                    let vec_param = params.iter().find(|p| p.dims.len() == 1);
                    if let (Some(w_p), Some(x_p)) = (mat_param, vec_param) {
                        let m = w_p.dims[0];
                        let n = w_p.dims[1];
                        let bias_param = params
                            .iter()
                            .find(|p| p.dims.len() == 1 && p.name != x_p.name);
                        out.push_str(&format!("    for (int i = 0; i < {}; ++i) {{\n", m));
                        if let Some(b_p) = bias_param {
                            out.push_str(&format!(
                                "        float acc = (float){}[i];\n",
                                b_p.name
                            ));
                        } else {
                            out.push_str("        float acc = 0.0f;\n");
                        }
                        out.push_str(&format!("        for (int j = 0; j < {}; ++j) {{\n", n));
                        out.push_str(&format!(
                            "            acc += (float){}[i * {} + j] * (float){}[j];\n",
                            w_p.name, n, x_p.name
                        ));
                        out.push_str("        }\n");
                        out.push_str(&format!("        out[i] = ({})acc;\n", r_type));
                        out.push_str("    }\n}\n\n");
                        continue;
                    } else if let Some(v_p) = vec_param {
                        let n = v_p.dims[0];
                        out.push_str(&format!("    for (int i = 0; i < {}; ++i) {{\n", n));
                        out.push_str(&format!(
                            "        out[i] = ({} )({});\n",
                            r_type,
                            Self::expr_to_c23(&expanded_body).replace(&v_p.name, &format!("{}[i]", v_p.name))
                        ));
                        out.push_str("    }\n}\n\n");
                        continue;
                    }
                }

                let p_str = params
                    .iter()
                    .map(|p| format!("{} {}", p.dtype.to_c23_type(), p.name))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("{} {}({}) {{\n", r_type, name, p_str));
                out.push_str(&format!(
                    "    return ({} )({});\n",
                    r_type,
                    Self::expr_to_c23(&expanded_body)
                ));
                out.push_str("}\n\n");
            }
        }

        out
    }

    fn expr_to_c23(expr: &MathExpr) -> String {
        match expr {
            MathExpr::Num(n) => format!("{:.6}f", n),
            MathExpr::Var(v) => v.clone(),
            MathExpr::Array(elems) => {
                let s: Vec<String> = elems.iter().map(Self::expr_to_c23).collect();
                format!("{{ {} }}", s.join(", "))
            }
            MathExpr::Index(base, idx) => {
                format!("{}[(int)({})]", Self::expr_to_c23(base), Self::expr_to_c23(idx))
            }
            MathExpr::Placeholder => "_".to_string(),
            MathExpr::Unary(MathUnaryOp::Neg, inner) => format!("(-{})", Self::expr_to_c23(inner)),
            MathExpr::Unary(MathUnaryOp::Not, inner) => format!("(!{})", Self::expr_to_c23(inner)),
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => format!(
                "({} ? {} : {})",
                Self::expr_to_c23(cond),
                Self::expr_to_c23(then_expr),
                Self::expr_to_c23(else_expr)
            ),
            MathExpr::Binary(lhs, op, rhs) => {
                let l = Self::expr_to_c23(lhs);
                // Algebraic strength reduction for integer & half powers
                if *op == MathBinOp::Pow {
                    if let MathExpr::Num(exp) = **rhs {
                        if (exp - 2.0).abs() < 1e-9 {
                            return format!("({} * {})", l, l);
                        } else if (exp - 3.0).abs() < 1e-9 {
                            return format!("(({} * {}) * {})", l, l, l);
                        } else if (exp - 0.5).abs() < 1e-9 {
                            return format!("sqrtf({})", l);
                        } else if (exp + 0.5).abs() < 1e-9 {
                            return format!("(1.0f / sqrtf({}))", l);
                        }
                    }
                }
                let r = Self::expr_to_c23(rhs);
                match op {
                    MathBinOp::Add => format!("({} + {})", l, r),
                    MathBinOp::Sub => format!("({} - {})", l, r),
                    MathBinOp::Mul | MathBinOp::MatMul => format!("({} * {})", l, r),
                    MathBinOp::Div => format!("({} / {})", l, r),
                    MathBinOp::Pow => format!("powf({}, {})", l, r),
                    MathBinOp::Eq => format!("({} == {})", l, r),
                    MathBinOp::Neq => format!("({} != {})", l, r),
                    MathBinOp::Lt => format!("({} < {})", l, r),
                    MathBinOp::Lte => format!("({} <= {})", l, r),
                    MathBinOp::Gt => format!("({} > {})", l, r),
                    MathBinOp::Gte => format!("({} >= {})", l, r),
                }
            }
            MathExpr::Call(func, args) => {
                let arg_strs: Vec<String> = args.iter().map(Self::expr_to_c23).collect();
                let a0 = arg_strs.first().cloned().unwrap_or_else(|| "0.0f".to_string());
                match func.as_str() {
                    "sqrt" => format!("sqrtf({})", a0),
                    "exp" => format!("expf({})", a0),
                    "ln" | "log" => format!("logf({})", a0),
                    "sin" => format!("sinf({})", a0),
                    "cos" => format!("cosf({})", a0),
                    "tan" => format!("tanf({})", a0),
                    "tanh" => format!("tanhf({})", a0),
                    "abs" => format!("fabsf({})", a0),
                    "sigmoid" => format!("(1.0f / (1.0f + expf(-{})))", a0),
                    "relu" => format!("(({}) > 0.0f ? ({}) : 0.0f)", a0, a0),
                    "gelu" => format!(
                        "(0.5f * ({0}) * (1.0f + tanhf(0.79788456f * (({0}) + 0.044715f * (({0}) * ({0}) * ({0}))))))",
                        a0
                    ),
                    "mean" | "sum" => {
                        if arg_strs.len() <= 1 {
                            a0
                        } else if func == "mean" {
                            format!("(({}) / {:.1}f)", arg_strs.join(" + "), arg_strs.len() as f32)
                        } else {
                            format!("({})", arg_strs.join(" + "))
                        }
                    }
                    "max" if arg_strs.len() == 2 => format!("fmaxf({}, {})", arg_strs[0], arg_strs[1]),
                    "min" if arg_strs.len() == 2 => format!("fminf({}, {})", arg_strs[0], arg_strs[1]),
                    other => format!("{}({})", other, arg_strs.join(", ")),
                }
            }
            MathExpr::Pipeline(stages) => {
                let mut curr = stages[0].clone();
                for st in &stages[1..] {
                    curr = MathParser::desugar_pipe_step(curr, st.clone());
                }
                Self::expr_to_c23(&curr)
            }
        }
    }

    /// Compiles standard math into real SSA NVIDIA CUDA PTX v7.5+ GPU kernels with full function inlining
    pub fn to_ptx(&self) -> String {
        let mut out = String::new();
        out.push_str("// ============================================================================\n");
        out.push_str("// CRON Mathematical Calculus -> NVIDIA CUDA PTX v7.5+ SSA Kernel\n");
        out.push_str("// ============================================================================\n");
        out.push_str(".version 7.5\n.target sm_80\n.address_size 64\n\n");

        let fns = self.collect_functions();
        let mut constants = HashMap::new();
        for item in &self.items {
            if let MathItem::Constant { name, val } = item {
                constants.insert(name.clone(), *val);
            }
        }

        for item in &self.items {
            if let MathItem::Function {
                name,
                params,
                ret_type,
                body,
            } = item
            {
                let inferred_ret = Self::infer_return_dtype(params, ret_type);
                let r_ptx = inferred_ret.to_ptx_type();
                let fused_body = body.inline_and_expand(&fns, true, 0).simplify();

                out.push_str(&format!(".visible .func (.param {} ret) {}(\n", r_ptx, name));
                for (i, p) in params.iter().enumerate() {
                    let comma = if i + 1 < params.len() { "," } else { "" };
                    out.push_str(&format!(
                        "    .param {} param_{}{}\n",
                        p.dtype.to_ptx_type(),
                        p.name,
                        comma
                    ));
                }
                out.push_str(") {\n");

                let mut builder = PtxBuilder::new(&constants);
                let mut instr_lines = Vec::new();

                // Load parameters into SSA virtual registers
                for p in params {
                    let r = builder.alloc_reg();
                    if p.dtype == MathDtype::F16 {
                        let hr = builder.alloc_half_reg();
                        instr_lines.push(format!("    ld.param.f16 {}, [param_{}];", hr, p.name));
                        instr_lines.push(format!("    cvt.f32.f16 {}, {};", r, hr));
                    } else {
                        instr_lines.push(format!("    ld.param.f32 {}, [param_{}];", r, p.name));
                    }
                    builder.var_regs.insert(p.name.clone(), r);
                }

                let final_reg = builder.emit_expr(&fused_body, &mut instr_lines);

                out.push_str(&format!(
                    "    .reg .f32 %f<{}>;\n",
                    builder.next_reg.max(2)
                ));
                if builder.next_half_reg > 0 {
                    out.push_str(&format!("    .reg .f16 %h<{}>;\n", builder.next_half_reg));
                }
                if builder.next_pred > 0 {
                    out.push_str(&format!("    .reg .pred %p<{}>;\n", builder.next_pred));
                }

                for line in instr_lines {
                    out.push_str(&line);
                    out.push('\n');
                }

                if inferred_ret == MathDtype::F16 {
                    let hr = builder.alloc_half_reg();
                    out.push_str(&format!("    cvt.rn.f16.f32 {}, {};\n", hr, final_reg));
                    out.push_str(&format!("    st.param.f16 [ret], {};\n", hr));
                } else {
                    out.push_str(&format!("    st.param.f32 [ret], {};\n", final_reg));
                }
                out.push_str("    ret;\n}\n\n");
            }
        }

        out
    }

    /// Lowers mathematical functions into 4-slot 128-bit VLIW bundles with CRC-8 ATM sealing and inlining
    pub fn to_vliw_bundles(&self) -> Vec<VliwBundle> {
        let mut raw_slots: Vec<String> = Vec::new();
        let mut reg_map: HashMap<String, u8> = HashMap::new();
        let mut next_reg: u8 = 1;
        let fns = self.collect_functions();

        for item in &self.items {
            match item {
                MathItem::Constant { name, val } => {
                    let r = Self::alloc_vliw_reg(&mut next_reg);
                    reg_map.insert(name.clone(), r);
                    let imm = (*val as u16) & 0xFFF;
                    raw_slots.push(Self::seal_slot(&format!("'=={:02X}#{:03X}", r, imm), '>'));
                }
                MathItem::Function { params, body, .. } => {
                    for p in params {
                        if !reg_map.contains_key(&p.name) {
                            let r = Self::alloc_vliw_reg(&mut next_reg);
                            reg_map.insert(p.name.clone(), r);
                            raw_slots.push(Self::seal_slot(&format!("'=={:02X}#{:03X}", r, r as u16 * 16), '>'));
                        }
                    }
                    let fused_body = body.inline_and_expand(&fns, true, 0).simplify();
                    let res_reg = Self::lower_expr_to_vliw(&fused_body, &mut reg_map, &mut next_reg, &mut raw_slots);
                    raw_slots.push(Self::seal_slot(&format!("_ST{:02X}#040", res_reg), '>'));
                }
                MathItem::Expression(expr) | MathItem::Binding { expr, .. } => {
                    let fused_expr = expr.inline_and_expand(&fns, true, 0).simplify();
                    let res_reg = Self::lower_expr_to_vliw(&fused_expr, &mut reg_map, &mut next_reg, &mut raw_slots);
                    raw_slots.push(Self::seal_slot(&format!("_ST{:02X}#040", res_reg), '>'));
                }
            }
        }

        raw_slots.push(Self::seal_slot("_HL00$000", '!'));

        let nop_slot = Self::seal_slot("_NO00#000", '>');
        while raw_slots.len() % 4 != 0 {
            raw_slots.push(nop_slot.clone());
        }

        let mut draft_lines = String::new();
        for (idx, chunk) in raw_slots.chunks(4).enumerate() {
            draft_lines.push_str(&format!("B{:04X}: {}\n", idx, chunk.join(" ")));
        }

        let healed = crate::cl_heal::heal_cl_program(&draft_lines)
            .map(|r| r.canonical_code)
            .unwrap_or(draft_lines);

        let mut bundles = Vec::new();
        for (idx, line) in healed.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') {
                continue;
            }
            if let Some((_, slots_str)) = trimmed.split_once(':') {
                let mut slots = Vec::new();
                for raw_tok in slots_str.split_whitespace() {
                    if raw_tok.starts_with(';') {
                        break;
                    }
                    slots.push(Self::build_vliw_slot(raw_tok));
                }
                if slots.len() == 4 {
                    bundles.push(VliwBundle {
                        cycle: idx,
                        slots,
                    });
                }
            }
        }

        bundles
    }

    fn build_vliw_slot(raw_slot: &str) -> VliwSlot {
        let is_16_char = raw_slot.len() >= 16;
        let prefix = raw_slot.chars().next().unwrap_or('_');
        let opcode = if raw_slot.len() >= 3 {
            raw_slot[1..3].to_string()
        } else {
            "NO".to_string()
        };
        let dst = if raw_slot.len() >= 5 {
            u8::from_str_radix(&raw_slot[3..5], 16).unwrap_or(0)
        } else {
            0
        };
        let op_math = raw_slot.chars().nth(5).unwrap_or('#');
        let src1 = if raw_slot.len() >= 7 {
            u8::from_str_radix(&raw_slot[6..7], 16).unwrap_or(0)
        } else {
            0
        };
        let src2 = if raw_slot.len() >= 9 {
            Some(u8::from_str_radix(&raw_slot[7..8], 16).unwrap_or(0))
        } else {
            None
        };
        let crc_token = raw_slot
            .chars()
            .last()
            .map(|c| c.to_string())
            .unwrap_or_else(|| ">".to_string());

        VliwSlot {
            raw: raw_slot.to_string(),
            is_16_char_tile: is_16_char,
            prefix,
            opcode,
            dst,
            op_math,
            src1,
            src2,
            crc_token,
        }
    }

    /// Emits canonical 4-slot 128-bit VLIW microcode text
    pub fn to_vliw(&self) -> String {
        let bundles = self.to_vliw_bundles();
        let mut out = String::new();
        out.push_str("; ============================================================================\n");
        out.push_str("; CRON Mathematical Calculus -> 4-Slot 128-Bit VLIW Silicon Microcode\n");
        out.push_str("; Target: 256-Core 4D-Torus Silicon | 100% CRC-8 ATM Sealed\n");
        out.push_str("; ============================================================================\n\n");
        for (idx, b) in bundles.iter().enumerate() {
            out.push_str(&format!(
                "B{:04X}: {} {} {} {}\n",
                idx, b.slots[0].raw, b.slots[1].raw, b.slots[2].raw, b.slots[3].raw
            ));
        }
        out
    }

    /// Converts Mathematical Program into unified Sovereign ClV2Document
    pub fn to_cl_v2_document(&self) -> ClV2Document {
        let mut doc = ClV2Document::new();
        let tc = SiliconTargetContract {
            kernel_name: "math_calculus_kernel".to_string(),
            target: "silicon.torus4d".to_string(),
            ipc_target: 4.0,
            cores: 256,
            mesh_routing: None,
        };
        doc.target_contract = Some(tc.clone());
        doc.items.push(ProgramItem::TargetContract(tc));

        for item in &self.items {
            if let MathItem::Function {
                name,
                params,
                ret_type,
                body,
            } = item
            {
                let dt = Self::infer_return_dtype(params, ret_type);
                let in_sym = params
                    .first()
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| "X".to_string());
                let eq_str = body.to_string();
                let step = MathStep {
                    target_var: "Y".to_string(),
                    expr: eq_str.clone(),
                    expr_ast: None,
                };
                let fb = FunctionBlock {
                    name: name.clone(),
                    tile_dims: vec![16, 16],
                    dtype: dt.to_v2_dtype_str().to_string(),
                    params: params.iter().map(|p| p.name.clone()).collect(),
                    states: Vec::new(),
                    input_symbol: in_sym.clone(),
                    output_symbol: Some("Y".to_string()),
                    equation_expr: eq_str.clone(),
                    steps: vec![step.clone()],
                    body: Vec::new(),
                };
                if doc.math_morphism.is_none() {
                    doc.math_morphism = Some(MathMorphismBlock {
                        name: name.clone(),
                        tile_dims: vec![16, 16],
                        dtype: dt.to_v2_dtype_str().to_string(),
                        states: Vec::new(),
                        input_symbol: in_sym,
                        output_symbol: Some("Y".to_string()),
                        equation_expr: eq_str,
                        steps: vec![step],
                    });
                }
                doc.functions.push(fb.clone());
                doc.items.push(ProgramItem::Function(fb));
            }
        }

        let bundles = self.to_vliw_bundles();
        for b in &bundles {
            doc.items.push(ProgramItem::VliwBundle(b.clone()));
        }
        doc.vliw_stream = bundles;
        doc
    }

    fn seal_slot(raw9: &str, term: char) -> String {
        let draft = format!("{}{}", raw9, term);
        crate::cl_heal::heal_slot_crc(&draft).0
    }

    fn alloc_vliw_reg(next_reg: &mut u8) -> u8 {
        let r = 1 + ((*next_reg).saturating_sub(1) % 14);
        *next_reg = next_reg.wrapping_add(1);
        r
    }

    fn lower_expr_to_vliw(
        expr: &MathExpr,
        reg_map: &mut HashMap<String, u8>,
        next_reg: &mut u8,
        slots: &mut Vec<String>,
    ) -> u8 {
        match expr {
            MathExpr::Num(n) => {
                let r = Self::alloc_vliw_reg(next_reg);
                let imm = (*n as u16) & 0xFFF;
                slots.push(Self::seal_slot(&format!("'=={:02X}#{:03X}", r, imm), '>'));
                r
            }
            MathExpr::Var(name) => {
                if let Some(&r) = reg_map.get(name) {
                    r
                } else {
                    let r = Self::alloc_vliw_reg(next_reg);
                    reg_map.insert(name.clone(), r);
                    slots.push(Self::seal_slot(&format!("'=={:02X}#010", r), '>'));
                    r
                }
            }
            MathExpr::Unary(MathUnaryOp::Neg, inner) => {
                let src = Self::lower_expr_to_vliw(inner, reg_map, next_reg, slots);
                let dst = Self::alloc_vliw_reg(next_reg);
                slots.push(Self::seal_slot(&format!("_SU{:02X}-0{:X}0", dst, src & 0xF), '>'));
                dst
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let l_reg = Self::lower_expr_to_vliw(lhs, reg_map, next_reg, slots);
                let r_reg = Self::lower_expr_to_vliw(rhs, reg_map, next_reg, slots);
                let dst = Self::alloc_vliw_reg(next_reg);
                let raw8 = match op {
                    MathBinOp::Add => format!("_PO{:02X}+{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF),
                    MathBinOp::Sub => format!("_SU{:02X}-{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF),
                    MathBinOp::MatMul => {
                        format!("_MD{:02X}*{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF)
                    }
                    MathBinOp::Mul | MathBinOp::Pow => {
                        format!("_ML{:02X}*{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF)
                    }
                    MathBinOp::Div => format!("_DV{:02X}/{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF),
                    _ => format!("_CO{:02X}?{:X}{:X}0", dst, l_reg & 0xF, r_reg & 0xF),
                };
                slots.push(Self::seal_slot(&raw8, '>'));
                dst
            }
            MathExpr::Call(func, args) => {
                let arg_reg = if let Some(first) = args.first() {
                    Self::lower_expr_to_vliw(first, reg_map, next_reg, slots)
                } else {
                    1
                };
                let dst = Self::alloc_vliw_reg(next_reg);
                let raw8 = match func.as_str() {
                    "sigmoid" | "silu" | "tanh" => format!("_SI{:02X}${:02X}0", dst, arg_reg),
                    "gelu" => format!("_GE{:02X}${:02X}0", dst, arg_reg),
                    "relu" | "max" | "min" => format!("_MX{:02X}${:02X}0", dst, arg_reg),
                    "sqrt" => format!("_SQ{:02X}${:02X}0", dst, arg_reg),
                    "exp" | "ln" | "log" => format!("_EX{:02X}${:02X}0", dst, arg_reg),
                    "rmsnorm" | "norm" | "mean" => format!("_RM{:02X}${:02X}0", dst, arg_reg),
                    "softmax" => format!("_SM{:02X}${:02X}0", dst, arg_reg),
                    "sin" | "cos" | "tan" => format!("_CD{:02X}${:02X}0", dst, arg_reg),
                    _ => format!("_TT{:02X}${:02X}0", dst, arg_reg),
                };
                slots.push(Self::seal_slot(&raw8, '>'));
                dst
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                let _c = Self::lower_expr_to_vliw(cond, reg_map, next_reg, slots);
                let t = Self::lower_expr_to_vliw(then_expr, reg_map, next_reg, slots);
                let e = Self::lower_expr_to_vliw(else_expr, reg_map, next_reg, slots);
                let dst = Self::alloc_vliw_reg(next_reg);
                slots.push(Self::seal_slot(
                    &format!("_PO{:02X}+{:X}{:X}0", dst, t & 0xF, e & 0xF),
                    '>',
                ));
                dst
            }
            _ => 1,
        }
    }
}

struct PtxBuilder<'a> {
    next_reg: usize,
    next_half_reg: usize,
    next_pred: usize,
    var_regs: HashMap<String, String>,
    constants: &'a HashMap<String, f64>,
}

impl<'a> PtxBuilder<'a> {
    fn new(constants: &'a HashMap<String, f64>) -> Self {
        Self {
            next_reg: 0,
            next_half_reg: 0,
            next_pred: 0,
            var_regs: HashMap::new(),
            constants,
        }
    }

    fn alloc_reg(&mut self) -> String {
        let r = format!("%f{}", self.next_reg);
        self.next_reg += 1;
        r
    }

    fn alloc_half_reg(&mut self) -> String {
        let r = format!("%h{}", self.next_half_reg);
        self.next_half_reg += 1;
        r
    }

    fn alloc_pred(&mut self) -> String {
        let p = format!("%p{}", self.next_pred);
        self.next_pred += 1;
        p
    }

    fn emit_expr(&mut self, expr: &MathExpr, lines: &mut Vec<String>) -> String {
        match expr {
            MathExpr::Num(n) => {
                let r = self.alloc_reg();
                lines.push(format!("    mov.f32 {}, {:.6};", r, n));
                r
            }
            MathExpr::Var(name) => {
                if let Some(r) = self.var_regs.get(name) {
                    r.clone()
                } else if let Some(&val) = self.constants.get(name) {
                    let r = self.alloc_reg();
                    lines.push(format!("    mov.f32 {}, {:.6};", r, val));
                    r
                } else {
                    let r = self.alloc_reg();
                    lines.push(format!("    mov.f32 {}, 0.0;", r));
                    r
                }
            }
            MathExpr::Unary(MathUnaryOp::Neg, inner) => {
                let src = self.emit_expr(inner, lines);
                let dst = self.alloc_reg();
                lines.push(format!("    neg.f32 {}, {};", dst, src));
                dst
            }
            MathExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                let c_reg = self.emit_expr(cond, lines);
                let t_reg = self.emit_expr(then_expr, lines);
                let e_reg = self.emit_expr(else_expr, lines);
                let pred = self.alloc_pred();
                let dst = self.alloc_reg();
                lines.push(format!("    setp.ne.f32 {}, {}, 0.0;", pred, c_reg));
                lines.push(format!("    selp.f32 {}, {}, {}, {};", dst, t_reg, e_reg, pred));
                dst
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let l = self.emit_expr(lhs, lines);
                if *op == MathBinOp::Pow {
                    if let MathExpr::Num(exp) = **rhs {
                        if (exp - 2.0).abs() < 1e-9 {
                            let dst = self.alloc_reg();
                            lines.push(format!("    mul.f32 {}, {}, {};", dst, l, l));
                            return dst;
                        }
                    }
                }
                let r = self.emit_expr(rhs, lines);
                let dst = self.alloc_reg();
                match op {
                    MathBinOp::Add => lines.push(format!("    add.f32 {}, {}, {};", dst, l, r)),
                    MathBinOp::Sub => lines.push(format!("    sub.f32 {}, {}, {};", dst, l, r)),
                    MathBinOp::Mul | MathBinOp::MatMul => {
                        lines.push(format!("    mul.f32 {}, {}, {};", dst, l, r))
                    }
                    MathBinOp::Div => lines.push(format!("    div.rn.f32 {}, {}, {};", dst, l, r)),
                    MathBinOp::Pow => {
                        let tmp = self.alloc_reg();
                        lines.push(format!("    lg2.approx.f32 {}, {};", tmp, l));
                        lines.push(format!("    mul.f32 {}, {}, {};", tmp, tmp, r));
                        lines.push(format!("    ex2.approx.f32 {}, {};", dst, tmp));
                    }
                    _ => {
                        let pred = self.alloc_pred();
                        lines.push(format!("    setp.gt.f32 {}, {}, {};", pred, l, r));
                        lines.push(format!("    selp.f32 {}, 1.0, 0.0, {};", dst, pred));
                    }
                }
                dst
            }
            MathExpr::Call(func, args) => {
                let arg0 = args
                    .first()
                    .map(|a| self.emit_expr(a, lines))
                    .unwrap_or_else(|| {
                        let r = self.alloc_reg();
                        lines.push(format!("    mov.f32 {}, 0.0;", r));
                        r
                    });
                let dst = self.alloc_reg();
                match func.as_str() {
                    "sqrt" => lines.push(format!("    sqrt.rn.f32 {}, {};", dst, arg0)),
                    "sin" => lines.push(format!("    sin.approx.f32 {}, {};", dst, arg0)),
                    "cos" => lines.push(format!("    cos.approx.f32 {}, {};", dst, arg0)),
                    "abs" => lines.push(format!("    abs.f32 {}, {};", dst, arg0)),
                    "relu" => lines.push(format!("    max.f32 {}, {}, 0.0;", dst, arg0)),
                    "exp" => {
                        let tmp = self.alloc_reg();
                        lines.push(format!("    mul.f32 {}, {}, 1.442695;", tmp, arg0));
                        lines.push(format!("    ex2.approx.f32 {}, {};", dst, tmp));
                    }
                    "sigmoid" => {
                        let neg = self.alloc_reg();
                        let scaled = self.alloc_reg();
                        let ex = self.alloc_reg();
                        let denom = self.alloc_reg();
                        lines.push(format!("    neg.f32 {}, {};", neg, arg0));
                        lines.push(format!("    mul.f32 {}, {}, 1.442695;", scaled, neg));
                        lines.push(format!("    ex2.approx.f32 {}, {};", ex, scaled));
                        lines.push(format!("    add.f32 {}, {}, 1.0;", denom, ex));
                        lines.push(format!("    rcp.rn.f32 {}, {};", dst, denom));
                    }
                    _ => lines.push(format!("    mov.f32 {}, {};", dst, arg0)),
                }
                dst
            }
            _ => {
                let r = self.alloc_reg();
                lines.push(format!("    mov.f32 {}, 0.0;", r));
                r
            }
        }
    }
}

/// Checks whether a source string represents a Standard Mathematical Calculus program
pub fn is_math_calculus_source(src: &str) -> bool {
    let mut has_math_def_or_pipe = false;
    for line in src.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("//") || t.starts_with(';') {
            continue;
        }
        // Exclude low-level VLIW or 5-pillar Unicode headers
        if t.starts_with("@⌖")
            || t.starts_with("@kernel")
            || t.starts_with("@io")
            || t.starts_with('§')
            || t.starts_with('⟳')
            || t.starts_with("!Δ")
            || t.starts_with(".stage")
            || (t.len() >= 6
                && t.starts_with('B')
                && t.as_bytes()[5] == b':'
                && t[1..5].chars().all(|c| c.is_ascii_hexdigit()))
        {
            return false;
        }
        if t.contains("->") || (t.contains('=') && !t.contains(":=")) {
            has_math_def_or_pipe = true;
        }
    }
    has_math_def_or_pipe && parse_math_code(src).is_ok()
}

/// Convenience parser function
pub fn parse_math_code(src: &str) -> Result<MathProgram, String> {
    let mut lexer = MathLexer::new(src);
    let tokens = lexer.tokenize()?;
    let mut parser = MathParser::new(tokens);
    parser.parse_program()
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_math_evaluation() {
        let code = r#"
            pi = 3.14159265
            d = 10.0
            f(x: f32) = x^2 + 2 * x + 1
            f(5)
        "#;
        let prog = parse_math_code(code).expect("Failed to parse math code");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Evaluation failed");
        assert_eq!(res, Some(36.0));
    }

    #[test]
    fn test_pipeline_chaining() {
        let code = r#"
            sq(x: f32) = x * x
            add_ten(x: f32) = x + 10.0
            5.0 -> sq -> add_ten
        "#;
        let prog = parse_math_code(code).expect("Failed to parse pipeline");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Evaluation failed");
        assert_eq!(res, Some(35.0));
    }

    #[test]
    fn test_vector_matrix_and_rmsnorm_evaluation() {
        let code = r#"
            eps = 0.0
            rmsnorm(x: f32) = x / sqrt(mean(x^2) + eps)
            linear(x: f16, W: f16, b: f16) = W @ x + b

            x = [3.0, 4.0]
            W = [[1.0, 2.0], [3.0, 4.0]]
            b = [1.0, -1.0]

            // W @ x + b = [1*3 + 2*4 + 1, 3*3 + 4*4 - 1] = [12.0, 24.0]
            y = x -> linear(W, b)
            sum(y)
        "#;
        let prog = parse_math_code(code).expect("Failed to parse tensor math");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Tensor evaluation failed");
        assert_eq!(res, Some(36.0));
    }

    #[test]
    fn test_ternary_conditional_operator() {
        let code = r#"
            leaky(x: f32) = x > 0 ? x : 0.1 * x
            leaky(-20)
        "#;
        let prog = parse_math_code(code).expect("Failed to parse ternary");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Ternary eval failed");
        assert!((res.unwrap() - (-2.0)).abs() < 1e-6);
    }

    #[test]
    fn test_neural_math_c23_ptx_and_vliw_emission() {
        let code = r#"
            eps = 1e-5
            norm(x: f32) = x / sqrt(x^2 + eps)
            linear(x: f16, W: f16, b: f16) = W * x + b
            act(z: f16) = sigmoid(z)
            forward(x: f16, W: f16, b: f16) = x -> norm -> linear(W, b) -> act
        "#;
        let prog = parse_math_code(code).expect("Failed to parse neural code");

        // 1. Verify C23 Type Inference & Strength Reduction (x * x instead of powf)
        let c23_code = prog.to_c23();
        assert!(c23_code.contains("_Float16 linear(_Float16 x, _Float16 W, _Float16 b)"));
        assert!(c23_code.contains("(x * x)"));
        assert!(!c23_code.contains("powf"));

        // 2. Verify Real AST-Driven PTX Generation
        let ptx_code = prog.to_ptx();
        assert!(ptx_code.contains("sqrt.rn.f32"));
        assert!(ptx_code.contains("div.rn.f32"));
        assert!(ptx_code.contains("ex2.approx.f32"));

        // 3. Verify 4-Slot VLIW Microcode Lowering with CRC-8 Verification
        let vliw_code = prog.to_vliw();
        assert!(vliw_code.contains("B0000:"));
        let verify_rep = crate::cl_lang::verify_cl_program(&vliw_code)
            .expect("Generated VLIW failed hardware CRC-8/hazard verification");
        assert!(verify_rep.total_bundles > 0);
        assert!(verify_rep.crc_verified > 0);
    }

    #[test]
    fn test_unified_cl_single_source_of_truth_pipeline() {
        let code = r#"
            eps = 1e-5
            norm(x: f32) = x / sqrt(mean(x^2) + eps)
            linear(x: f16, W: f16, b: f16) = W @ x + b
            forward(x: f16, W: f16, b: f16) = x -> norm -> linear(W, b)
        "#;
        let doc = crate::parse_unified_cl(code).expect("Unified parser failed on standard math");
        assert!(!doc.functions.is_empty());
        assert!(!doc.vliw_stream.is_empty());
        let diags = crate::check_source_diagnostics(code);
        assert!(diags.is_empty(), "Expected 0 diagnostics, got: {:?}", diags);
    }

    #[test]
    fn test_symbolic_autodiff_grad_and_c23_lowering() {
        let code = r#"
            loss(w: f32, x: f32, y: f32) = (w * x - y)^2
            d_loss(w: f32, x: f32, y: f32) = grad(loss(w, x, y), w)
            // At w=3, x=2, y=1: loss = (6-1)^2 = 25, d_loss/dw = 2*(w*x - y)*x = 2*5*2 = 20
            d_loss(3, 2, 1)
        "#;
        let prog = parse_math_code(code).expect("Failed to parse autodiff code");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Autodiff eval failed");
        assert_eq!(res, Some(20.0));

        // Verify C23 emits simplified analytical derivative without 'grad(' call
        let c23 = prog.to_c23();
        assert!(!c23.contains("grad("));
        assert!(c23.contains("((2.000000f * ((w * x) - y)) * x)"));
    }

    #[test]
    fn test_shaped_tensor_c23_gemv_and_indexing() {
        let code = r#"
            linear_tile(x: f16[16], W: f16[16, 16], b: f16[16]) = W @ x + b
            seq = range(1, 5) -> scan
            seq[3]
        "#;
        let prog = parse_math_code(code).expect("Failed to parse shaped tensor & scan");
        let mut eval = MathEvaluator::new();
        // range(1, 5) = [1, 2, 3, 4], scan = [1, 3, 6, 10], seq[3] = 10
        let res = eval.execute_program(&prog).expect("Scan/index eval failed");
        assert_eq!(res, Some(10.0));

        let c23 = prog.to_c23();
        assert!(c23.contains("const _Float16* restrict W"));
        assert!(c23.contains("for (int i = 0; i < 16; ++i)"));
        assert!(c23.contains("W[i * 16 + j]"));
    }
}
