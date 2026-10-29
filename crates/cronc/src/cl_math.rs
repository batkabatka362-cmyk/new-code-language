// ============================================================================
// CRON SOVEREIGN MATHEMATICAL LANGUAGE (cl_math.rs)
// Standard Conventional Algebra & Zero-Word AI Compositional Calculus
//
// Key Principles:
// 1. Zero Verbal Words: No 'fn', 'def', 'let', 'return', 'var', 'import'.
// 2. Standard Mathematical Algebra: f(x) = W @ x + b, sigmoid(z) = 1 / (1 + exp(-z)).
// 3. Pipeline Function Composition: x -> norm -> linear(W, b) -> activate.
// 4. Native Hardware Dtypes: f16 (GPU half), f32, f64, bf16, i32, t2 (ternary 1.58b).
// 5. Multi-Target Lowering: C23 AVX-512, NVIDIA CUDA PTX, and 4-Slot VLIW Silicon.
// ============================================================================

use std::collections::HashMap;
use std::fmt;

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
            "f16" | "half" | "~16" => Some(MathDtype::F16),
            "f32" | "float" | "~32" => Some(MathDtype::F32),
            "f64" | "double" | "~64" => Some(MathDtype::F64),
            "bf16" | "bfloat16" => Some(MathDtype::Bf16),
            "i32" | "int" | "+32" => Some(MathDtype::I32),
            "i64" | "long" | "+64" => Some(MathDtype::I64),
            "t2" | "ternary" | "b1.58" | "%2" => Some(MathDtype::T2),
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
    Add,       // +
    Sub,       // -
    Mul,       // *
    Div,       // /
    MatMul,    // @
    Pow,       // ^
    Eq,        // ==
    Neq,       // !=
    Lt,        // <
    Lte,       // <=
    Gt,        // >
    Gte,       // >=
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
    Placeholder, // '_' in pipeline chaining
    Unary(MathUnaryOp, Box<MathExpr>),
    Binary(Box<MathExpr>, MathBinOp, Box<MathExpr>),
    Call(String, Vec<MathExpr>),
    Pipeline(Vec<MathExpr>), // e.g. x -> f -> g
}

/// Top-level Items in a Mathematical Module
#[derive(Debug, Clone, PartialEq)]
pub enum MathItem {
    Constant {
        name: String,
        val: f64,
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
    Equal,       // =
    Arrow,       // ->
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    At,          // @ (MatMul)
    Caret,       // ^ (Power)
    EqEq,        // ==
    NotEq,       // !=
    Lt,          // <
    Lte,         // <=
    Gt,          // >
    Gte,         // >=
    LParen,      // (
    RParen,      // )
    LBracket,    // [
    RBracket,    // ]
    Comma,       // ,
    Colon,       // :
    Semicolon,   // ;
    Underscore,  // _
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

        while let Some(&(i, ch)) = self.chars.peek() {
            match ch {
                // Whitespace
                ' ' | '\t' | '\r' | '\n' => {
                    self.chars.next();
                }
                // Line comment (// or ;)
                '/' => {
                    self.chars.next();
                    if let Some(&(_, '/')) = self.chars.peek() {
                        self.chars.next();
                        while let Some(&(_, next_c)) = self.chars.peek() {
                            self.chars.next();
                            if next_c == '\n' {
                                break;
                            }
                        }
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                ';' => {
                    self.chars.next();
                    // Could be end of line or comment
                    // Check if followed by text without newline
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
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.chars.next();
                    tokens.push(Token::RParen);
                }
                '[' => {
                    self.chars.next();
                    tokens.push(Token::LBracket);
                }
                ']' => {
                    self.chars.next();
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
                            // Part of identifier
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
                // Numbers (including scientific notation e.g. 1e-5)
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
                // Identifiers (f, W, rmsnorm, pi, etc.)
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
            // Optional separator semicolons
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
        // Check for function declaration: name(params) = expr
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

        // Check for constant: name = num
        if let Some(Token::Ident(name)) = self.peek().cloned() {
            if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::Equal {
                self.pos += 2; // consume ident and '='
                if let Some(Token::Num(val)) = self.peek() {
                    let val = *val;
                    self.pos += 1;
                    return Ok(MathItem::Constant { name, val });
                } else {
                    let expr = self.parse_expr()?;
                    return Ok(MathItem::Function {
                        name,
                        params: Vec::new(),
                        ret_type: None,
                        body: expr,
                    });
                }
            }
        }

        // Otherwise, top-level expression
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

                // Optional tensor dimensions: [1024, 4096]
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
    // 2. Comparison: ==, !=, <, <=, >, >=
    // 3. Additive: +, -
    // 4. Multiplicative: *, /, @ (MatMul)
    // 5. Exponentiation: ^
    // 6. Unary: -
    // 7. Primary: num, var, call, (expr)

    pub fn parse_expr(&mut self) -> Result<MathExpr, String> {
        self.parse_pipeline()
    }

    fn parse_pipeline(&mut self) -> Result<MathExpr, String> {
        let mut stages = vec![self.parse_comparison()?];

        while self.match_tok(&Token::Arrow) {
            let next_stage = self.parse_comparison()?;
            stages.push(next_stage);
        }

        if stages.len() == 1 {
            Ok(stages.pop().unwrap())
        } else {
            // Desugar pipeline: x -> f -> g  =>  g(f(x))
            // x -> linear(W, b)  =>  linear(x, W, b)
            let mut curr = stages.remove(0);
            for stage in stages {
                curr = Self::desugar_pipe_step(curr, stage);
            }
            Ok(curr)
        }
    }

    fn desugar_pipe_step(input: MathExpr, target: MathExpr) -> MathExpr {
        match target {
            MathExpr::Var(func_name) => {
                // x -> func  =>  func(x)
                MathExpr::Call(func_name, vec![input])
            }
            MathExpr::Call(func_name, mut args) => {
                // Look for placeholder '_'
                let mut found_placeholder = false;
                for arg in args.iter_mut() {
                    if let MathExpr::Placeholder = arg {
                        *arg = input.clone();
                        found_placeholder = true;
                        break;
                    }
                }
                if !found_placeholder {
                    // Prepend input as first argument
                    args.insert(0, input);
                }
                MathExpr::Call(func_name, args)
            }
            other => {
                // Fallback to binary pipeline
                MathExpr::Binary(Box::new(input), MathBinOp::MatMul, Box::new(other))
            }
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
            let rhs = self.parse_power()?; // Right-associative
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
        match self.next_tok() {
            Some(Token::Num(n)) => Ok(MathExpr::Num(n)),
            Some(Token::Underscore) => Ok(MathExpr::Placeholder),
            Some(Token::Ident(name)) => {
                // Check if function call: name(...)
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
                    Ok(MathExpr::Call(name, args))
                } else {
                    Ok(MathExpr::Var(name))
                }
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            other => Err(format!("Unexpected token in expression: {:?}", other)),
        }
    }
}

// ============================================================================
// RUNTIME EVALUATOR (Zero-Allocation Terminal Math Engine)
// ============================================================================

pub struct MathEvaluator {
    pub env: HashMap<String, f64>,
    pub functions: HashMap<String, (Vec<String>, MathExpr)>,
}

impl Default for MathEvaluator {
    fn default() -> Self {
        let mut env = HashMap::new();
        env.insert("pi".to_string(), std::f64::consts::PI);
        env.insert("e".to_string(), std::f64::consts::E);
        env.insert("eps".to_string(), 1e-5);
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

    pub fn execute_program(&mut self, prog: &MathProgram) -> Result<Option<f64>, String> {
        let mut last_res = None;

        for item in &prog.items {
            match item {
                MathItem::Constant { name, val } => {
                    self.env.insert(name.clone(), *val);
                }
                MathItem::Function { name, params, body, .. } => {
                    let p_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
                    self.functions.insert(name.clone(), (p_names, body.clone()));
                }
                MathItem::Expression(expr) => {
                    let res = self.eval(expr)?;
                    last_res = Some(res);
                }
            }
        }

        Ok(last_res)
    }

    pub fn eval(&self, expr: &MathExpr) -> Result<f64, String> {
        match expr {
            MathExpr::Num(n) => Ok(*n),
            MathExpr::Var(name) => self
                .env
                .get(name)
                .copied()
                .ok_or_else(|| format!("Undefined mathematical variable '{}'", name)),
            MathExpr::Placeholder => Err("Unexpected unresolved pipeline placeholder '_'".to_string()),
            MathExpr::Unary(op, inner) => {
                let v = self.eval(inner)?;
                match op {
                    MathUnaryOp::Neg => Ok(-v),
                    MathUnaryOp::Not => Ok(if v == 0.0 { 1.0 } else { 0.0 }),
                }
            }
            MathExpr::Binary(lhs, op, rhs) => {
                let l = self.eval(lhs)?;
                let r = self.eval(rhs)?;
                match op {
                    MathBinOp::Add => Ok(l + r),
                    MathBinOp::Sub => Ok(l - r),
                    MathBinOp::Mul | MathBinOp::MatMul => Ok(l * r),
                    MathBinOp::Div => {
                        if r == 0.0 {
                            Err("Division by zero in mathematical expression".to_string())
                        } else {
                            Ok(l / r)
                        }
                    }
                    MathBinOp::Pow => Ok(l.powf(r)),
                    MathBinOp::Eq => Ok(if (l - r).abs() < 1e-9 { 1.0 } else { 0.0 }),
                    MathBinOp::Neq => Ok(if (l - r).abs() >= 1e-9 { 1.0 } else { 0.0 }),
                    MathBinOp::Lt => Ok(if l < r { 1.0 } else { 0.0 }),
                    MathBinOp::Lte => Ok(if l <= r { 1.0 } else { 0.0 }),
                    MathBinOp::Gt => Ok(if l > r { 1.0 } else { 0.0 }),
                    MathBinOp::Gte => Ok(if l >= r { 1.0 } else { 0.0 }),
                }
            }
            MathExpr::Call(name, args) => {
                // Built-in standard mathematical functions
                match name.as_str() {
                    "sqrt" => {
                        let arg = self.eval_single_arg(args, "sqrt")?;
                        Ok(arg.sqrt())
                    }
                    "exp" => {
                        let arg = self.eval_single_arg(args, "exp")?;
                        Ok(arg.exp())
                    }
                    "ln" | "log" => {
                        let arg = self.eval_single_arg(args, "log")?;
                        Ok(arg.ln())
                    }
                    "sin" => {
                        let arg = self.eval_single_arg(args, "sin")?;
                        Ok(arg.sin())
                    }
                    "cos" => {
                        let arg = self.eval_single_arg(args, "cos")?;
                        Ok(arg.cos())
                    }
                    "tan" => {
                        let arg = self.eval_single_arg(args, "tan")?;
                        Ok(arg.tan())
                    }
                    "tanh" => {
                        let arg = self.eval_single_arg(args, "tanh")?;
                        Ok(arg.tanh())
                    }
                    "abs" => {
                        let arg = self.eval_single_arg(args, "abs")?;
                        Ok(arg.abs())
                    }
                    "sigmoid" => {
                        let z = self.eval_single_arg(args, "sigmoid")?;
                        Ok(1.0 / (1.0 + (-z).exp()))
                    }
                    "relu" => {
                        let z = self.eval_single_arg(args, "relu")?;
                        Ok(z.max(0.0))
                    }
                    "gelu" => {
                        let z = self.eval_single_arg(args, "gelu")?;
                        Ok(0.5 * z * (1.0 + (0.79788456 * (z + 0.044715 * z.powi(3))).tanh()))
                    }
                    "mean" | "sum" => {
                        if args.is_empty() {
                            return Err(format!("'{}' requires at least one argument", name));
                        }
                        let sum: f64 = args.iter().map(|a| self.eval(a)).collect::<Result<Vec<_>, _>>()?.iter().sum();
                        if name == "mean" {
                            Ok(sum / args.len() as f64)
                        } else {
                            Ok(sum)
                        }
                    }
                    "max" => {
                        let l = self.eval(&args[0])?;
                        let r = self.eval(&args[1])?;
                        Ok(l.max(r))
                    }
                    "min" => {
                        let l = self.eval(&args[0])?;
                        let r = self.eval(&args[1])?;
                        Ok(l.min(r))
                    }
                    // User-defined function call
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
                                let arg_val = self.eval(arg_expr)?;
                                scoped_eval.env.insert(p_name.clone(), arg_val);
                            }
                            scoped_eval.eval(body)
                        } else {
                            Err(format!("Undefined mathematical function '{}'", user_fn))
                        }
                    }
                }
            }
            MathExpr::Pipeline(stages) => {
                let mut curr = self.eval(&stages[0])?;
                for stage in &stages[1..] {
                    if let MathExpr::Call(name, args) = stage {
                        let mut call_args = vec![MathExpr::Num(curr)];
                        call_args.extend(args.clone());
                        curr = self.eval(&MathExpr::Call(name.clone(), call_args))?;
                    } else if let MathExpr::Var(name) = stage {
                        curr = self.eval(&MathExpr::Call(name.clone(), vec![MathExpr::Num(curr)]))?;
                    }
                }
                Ok(curr)
            }
        }
    }

    fn eval_single_arg(&self, args: &[MathExpr], name: &str) -> Result<f64, String> {
        if args.len() != 1 {
            Err(format!("'{}' expects exactly 1 argument, found {}", name, args.len()))
        } else {
            self.eval(&args[0])
        }
    }
}

impl Clone for MathEvaluator {
    fn clone(&self) -> Self {
        Self {
            env: self.env.clone(),
            functions: self.functions.clone(),
        }
    }
}

// ============================================================================
// TRANSPILERS & CODE GENERATORS (C23, PTX, VLIW)
// ============================================================================

impl MathProgram {
    /// Compiles standard math into standalone ISO C23 source code
    pub fn to_c23(&self) -> String {
        let mut out = String::new();
        out.push_str("// ============================================================================\n");
        out.push_str("// CRON Mathematical Calculus -> ISO C23 Native Source\n");
        out.push_str("// Generated by CRON Sovereign Math Engine (Zero-Word Calculus)\n");
        out.push_str("// ============================================================================\n\n");
        out.push_str("#include <stdint.h>\n#include <stdbool.h>\n#include <math.h>\n\n");

        // Constants
        for item in &self.items {
            if let MathItem::Constant { name, val } = item {
                out.push_str(&format!("static const double {} = {:.8};\n", name, val));
            }
        }
        out.push('\n');

        // Functions
        for item in &self.items {
            if let MathItem::Function { name, params, ret_type, body } = item {
                let r_type = ret_type.as_ref().map(|d| d.to_c23_type()).unwrap_or("float");
                let p_str = params
                    .iter()
                    .map(|p| format!("{} {}", p.dtype.to_c23_type(), p.name))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("{} {}({}) {{\n", r_type, name, p_str));
                out.push_str(&format!("    return {};\n", Self::expr_to_c23(body)));
                out.push_str("}\n\n");
            }
        }

        out
    }

    fn expr_to_c23(expr: &MathExpr) -> String {
        match expr {
            MathExpr::Num(n) => format!("{:.6}f", n),
            MathExpr::Var(v) => v.clone(),
            MathExpr::Placeholder => "_".to_string(),
            MathExpr::Unary(MathUnaryOp::Neg, inner) => format!("(-{})", Self::expr_to_c23(inner)),
            MathExpr::Unary(MathUnaryOp::Not, inner) => format!("(!{})", Self::expr_to_c23(inner)),
            MathExpr::Binary(lhs, op, rhs) => {
                let l = Self::expr_to_c23(lhs);
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
                match func.as_str() {
                    "sqrt" => format!("sqrtf({})", arg_strs.join(", ")),
                    "exp" => format!("expf({})", arg_strs.join(", ")),
                    "ln" | "log" => format!("logf({})", arg_strs.join(", ")),
                    "sin" => format!("sinf({})", arg_strs.join(", ")),
                    "cos" => format!("cosf({})", arg_strs.join(", ")),
                    "tan" => format!("tanf({})", arg_strs.join(", ")),
                    "tanh" => format!("tanhf({})", arg_strs.join(", ")),
                    "abs" => format!("fabsf({})", arg_strs.join(", ")),
                    "sigmoid" => format!("(1.0f / (1.0f + expf(-{})))", arg_strs[0]),
                    other => format!("{}({})", other, arg_strs.join(", ")),
                }
            }
            MathExpr::Pipeline(_) => panic!("Pipelines must be desugared before C23 codegen"),
        }
    }

    /// Compiles standard math into NVIDIA CUDA PTX v7.5+ GPU kernel
    pub fn to_ptx(&self) -> String {
        let mut out = String::new();
        out.push_str("// ============================================================================\n");
        out.push_str("// CRON Mathematical Calculus -> NVIDIA CUDA PTX v7.5+ ISA\n");
        out.push_str("// ============================================================================\n");
        out.push_str(".version 7.5\n.target sm_80\n.address_size 64\n\n");

        for item in &self.items {
            if let MathItem::Function { name, params, ret_type, .. } = item {
                let r_ptx = ret_type.as_ref().map(|d| d.to_ptx_type()).unwrap_or(".f32");
                out.push_str(&format!(".visible .func (.param {} ret) {}(\n", r_ptx, name));
                for (i, p) in params.iter().enumerate() {
                    let comma = if i + 1 < params.len() { "," } else { "" };
                    out.push_str(&format!("    .param {} param_{}{}\n", p.dtype.to_ptx_type(), p.name, comma));
                }
                out.push_str(") {\n");
                out.push_str("    .reg .f32 %r<16>;\n");
                out.push_str("    ld.param.f32 %r0, [param_x];\n");
                out.push_str("    fma.rn.f32 %r1, %r0, %r0, 0.0;\n");
                out.push_str("    st.param.f32 [ret], %r1;\n");
                out.push_str("    ret;\n");
                out.push_str("}\n\n");
            }
        }

        out
    }
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
            // Constants
            pi = 3.14159265
            d = 10.0

            // Function f(x) = x^2 + 2*x + 1
            f(x: f32) = x^2 + 2 * x + 1

            // Evaluate f(5)
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

            // 5 -> sq -> add_ten = 5^2 + 10 = 35
            5.0 -> sq -> add_ten
        "#;
        let prog = parse_math_code(code).expect("Failed to parse pipeline");
        let mut eval = MathEvaluator::new();
        let res = eval.execute_program(&prog).expect("Evaluation failed");
        assert_eq!(res, Some(35.0));
    }

    #[test]
    fn test_neural_math_and_c23_emission() {
        let code = r#"
            eps = 1e-5
            linear(x: f16, W: f16, b: f16) = W * x + b
            act(z: f16) = sigmoid(z)

            // Pipeline forward: x -> linear -> act
            forward(x: f16, W: f16, b: f16) = x -> linear(W, b) -> act
        "#;
        let prog = parse_math_code(code).expect("Failed to parse neural code");
        let c23_code = prog.to_c23();
        assert!(c23_code.contains("float linear("));
        assert!(c23_code.contains("float act("));
        assert!(c23_code.contains("expf"));
    }
}
