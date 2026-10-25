// ============================================================================
// CRON & CL HIGH-LEVEL AI-NATIVE PARSER (cl_high_parser.rs)
// High-Level Lexer & Parser for Declarative AI Models & Multi-Physics Compute
// ============================================================================

use std::collections::HashMap;
use crate::cl_high_ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum HighToken {
    // Sovereign CL v2.0 5-Pillar Non-Verbal Tokens
    Section,         // § (Pillar 3: Tensor Morphism Block)
    Morphism,        // ↦ (Pillar 3: Morphism Mapping)
    WalrusAssign,    // := (Functional Tensor Assignment)
    Transfer,        // ← (Register Memory Transfer)
    HardwareLoop,    // ⟳ (Pillar 4: Zero-Overhead Hardware Loop)
    YieldReturn,     // ⮌ (Silicon Yield / Return)
    Watchdog,        // !Δ (Pillar 4: Watchdog Invariant)

    // Legacy Deprecated Verbal Keywords (migrated to Sovereign morphisms)
    Module,
    Import,
    Stage,
    Arch,
    State,
    Kernel,
    Fn,
    Let,
    Return,
    As,

    // Identifiers & Literals
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),

    // Symbols & Delimiters
    LBrace,   // {
    RBrace,   // }
    LParen,   // (
    RParen,   // )
    LBracket, // [
    RBracket, // ]
    Colon,    // :
    Semicolon,// ;
    Comma,    // ,
    Dot,      // .
    Arrow,    // ->
    Assign,   // =
    At,       // @

    // Beyond-Addition 7 Operators
    InterferenceCoupling, // <~>
    AssociativeScan,      // >>=
    LogSpaceGating,       // <|>
    RingReduction,        // <%>
    RadixGraft,           // /@
    PeriodicResonance,    // ~*~
    ExteriorWedge,        // ^
    Tilde,                // ~

    // Classical Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    NotEq,
    Lt,
    Gt,
    Lte,
    Gte,

    Eof,
}

pub struct HighLexer<'a> {
    input: &'a str,
    chars: Vec<char>,
    pos: usize,
}

impl<'a> HighLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn source(&self) -> &'a str {
        self.input
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos])
        } else {
            None
        }
    }

    fn peek_ahead(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.chars.len() {
            Some(self.chars[self.pos + offset])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            self.pos += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek_ahead(1) == Some('/') {
                // Line comment
                while let Some(c) = self.peek() {
                    self.advance();
                    if c == '\n' {
                        break;
                    }
                }
            } else if ch == ';' {
                // Semicolon line comment (CL assembly heritage)
                while let Some(c) = self.peek() {
                    self.advance();
                    if c == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<HighToken, String> {
        self.skip_whitespace_and_comments();

        let ch = match self.peek() {
            Some(c) => c,
            None => return Ok(HighToken::Eof),
        };

        // Check multi-character beyond-addition operators
        // <~>
        if ch == '<' && self.peek_ahead(1) == Some('~') && self.peek_ahead(2) == Some('>') {
            self.pos += 3;
            return Ok(HighToken::InterferenceCoupling);
        }
        // >>=
        if ch == '>' && self.peek_ahead(1) == Some('>') && self.peek_ahead(2) == Some('=') {
            self.pos += 3;
            return Ok(HighToken::AssociativeScan);
        }
        // <|>
        if ch == '<' && self.peek_ahead(1) == Some('|') && self.peek_ahead(2) == Some('>') {
            self.pos += 3;
            return Ok(HighToken::LogSpaceGating);
        }
        // <%>
        if ch == '<' && self.peek_ahead(1) == Some('%') && self.peek_ahead(2) == Some('>') {
            self.pos += 3;
            return Ok(HighToken::RingReduction);
        }
        // /@
        if ch == '/' && self.peek_ahead(1) == Some('@') {
            self.pos += 2;
            return Ok(HighToken::RadixGraft);
        }
        // ~*~
        if ch == '~' && self.peek_ahead(1) == Some('*') && self.peek_ahead(2) == Some('~') {
            self.pos += 3;
            return Ok(HighToken::PeriodicResonance);
        }
        // ->
        if ch == '-' && self.peek_ahead(1) == Some('>') {
            self.pos += 2;
            return Ok(HighToken::Arrow);
        }
        // ==
        if ch == '=' && self.peek_ahead(1) == Some('=') {
            self.pos += 2;
            return Ok(HighToken::EqEq);
        }
        // :=
        if ch == ':' && self.peek_ahead(1) == Some('=') {
            self.pos += 2;
            return Ok(HighToken::WalrusAssign);
        }
        // <-
        if ch == '<' && self.peek_ahead(1) == Some('-') {
            self.pos += 2;
            return Ok(HighToken::Transfer);
        }
        // !Δ or !delta
        if ch == '!' && (self.peek_ahead(1) == Some('Δ') || self.peek_ahead(1) == Some('\u{0394}')) {
            self.pos += 2;
            return Ok(HighToken::Watchdog);
        }
        // !=
        if ch == '!' && self.peek_ahead(1) == Some('=') {
            self.pos += 2;
            return Ok(HighToken::NotEq);
        }
        // <=
        if ch == '<' && self.peek_ahead(1) == Some('=') {
            self.pos += 2;
            return Ok(HighToken::Lte);
        }
        // >=
        if ch == '>' && self.peek_ahead(1) == Some('=') {
            self.pos += 2;
            return Ok(HighToken::Gte);
        }

        // Single character symbols
        match ch {
            '§' => { self.advance(); Ok(HighToken::Section) }
            '↦' => { self.advance(); Ok(HighToken::Morphism) }
            '←' => { self.advance(); Ok(HighToken::Transfer) }
            '⟳' => { self.advance(); Ok(HighToken::HardwareLoop) }
            '⮌' => { self.advance(); Ok(HighToken::YieldReturn) }
            '{' => { self.advance(); Ok(HighToken::LBrace) }
            '}' => { self.advance(); Ok(HighToken::RBrace) }
            '(' => { self.advance(); Ok(HighToken::LParen) }
            ')' => { self.advance(); Ok(HighToken::RParen) }
            '[' => { self.advance(); Ok(HighToken::LBracket) }
            ']' => { self.advance(); Ok(HighToken::RBracket) }
            ':' => { self.advance(); Ok(HighToken::Colon) }
            ';' => { self.advance(); Ok(HighToken::Semicolon) }
            ',' => { self.advance(); Ok(HighToken::Comma) }
            '.' => { self.advance(); Ok(HighToken::Dot) }
            '=' => { self.advance(); Ok(HighToken::Assign) }
            '@' => { self.advance(); Ok(HighToken::At) }
            '+' => { self.advance(); Ok(HighToken::Plus) }
            '-' => { self.advance(); Ok(HighToken::Minus) }
            '*' => { self.advance(); Ok(HighToken::Star) }
            '/' => { self.advance(); Ok(HighToken::Slash) }
            '%' => { self.advance(); Ok(HighToken::Percent) }
            '<' => { self.advance(); Ok(HighToken::Lt) }
            '>' => { self.advance(); Ok(HighToken::Gt) }
            '^' => { self.advance(); Ok(HighToken::ExteriorWedge) }
            '~' => { self.advance(); Ok(HighToken::Tilde) }
            '"' => self.lex_string(),
            _ if ch.is_alphabetic() || ch == '_' => self.lex_ident_or_keyword(),
            _ if ch.is_digit(10) => self.lex_number(),
            _ => {
                self.advance();
                Err(format!("Unexpected character in high-level CL source: '{}'", ch))
            }
        }
    }

    fn lex_string(&mut self) -> Result<HighToken, String> {
        self.advance(); // consume opening quote
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // consume closing quote
                return Ok(HighToken::Str(s));
            } else if ch == '\\' {
                self.advance();
                if let Some(esc) = self.advance() {
                    match esc {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        'r' => s.push('\r'),
                        '\\' => s.push('\\'),
                        '"' => s.push('"'),
                        other => s.push(other),
                    }
                }
            } else {
                s.push(ch);
                self.advance();
            }
        }
        Err("Unterminated string literal in high-level CL code".to_string())
    }

    fn lex_ident_or_keyword(&mut self) -> Result<HighToken, String> {
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let tok = match s.as_str() {
            "module" => HighToken::Module,
            "import" => HighToken::Import,
            "stage" => HighToken::Stage,
            "arch" => HighToken::Arch,
            "state" => HighToken::State,
            "kernel" => HighToken::Kernel,
            "fn" => HighToken::Fn,
            "let" => HighToken::Let,
            "return" => HighToken::Return,
            "as" => HighToken::As,
            "true" => HighToken::Bool(true),
            "false" => HighToken::Bool(false),
            _ => HighToken::Ident(s),
        };

        Ok(tok)
    }

    fn lex_number(&mut self) -> Result<HighToken, String> {
        let mut s = String::new();
        let mut is_float = false;

        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                s.push(ch);
                self.advance();
            } else if ch == '.' && self.peek_ahead(1).map(|c| c.is_digit(10)).unwrap_or(false) {
                is_float = true;
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            s.parse::<f64>()
                .map(HighToken::Float)
                .map_err(|e| format!("Invalid float literal '{}': {}", s, e))
        } else {
            s.parse::<i64>()
                .map(HighToken::Int)
                .map_err(|e| format!("Invalid integer literal '{}': {}", s, e))
        }
    }
}

pub struct HighParser<'a> {
    lexer: HighLexer<'a>,
    current: HighToken,
    peek: HighToken,
}

impl<'a> HighParser<'a> {
    pub fn new(mut lexer: HighLexer<'a>) -> Result<Self, String> {
        let current = lexer.next_token()?;
        let peek = lexer.next_token()?;
        Ok(Self { lexer, current, peek })
    }

    fn advance(&mut self) -> Result<HighToken, String> {
        let next = self.lexer.next_token()?;
        let prev = std::mem::replace(&mut self.current, std::mem::replace(&mut self.peek, next));
        Ok(prev)
    }

    fn expect(&mut self, expected: HighToken) -> Result<(), String> {
        if self.current == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(format!("Expected token {:?}, found {:?}", expected, self.current))
        }
    }

    pub fn parse_module(&mut self) -> Result<HighModule, String> {
        let mut name = "main".to_string();
        let mut imports = Vec::new();
        let mut stages = Vec::new();
        let mut architectures = Vec::new();
        let mut kernels = Vec::new();

        while self.current != HighToken::Eof {
            match self.current {
                HighToken::Module => {
                    self.advance()?;
                    if let HighToken::Ident(mod_name) = self.advance()? {
                        name = mod_name;
                        if self.current == HighToken::Semicolon {
                            self.advance()?;
                        }
                    } else {
                        return Err("Expected module name after 'module'".to_string());
                    }
                }
                HighToken::Import => {
                    imports.push(self.parse_import()?);
                }
                HighToken::Stage => {
                    self.advance()?;
                    stages.push(self.parse_stage()?);
                }
                HighToken::Dot => {
                    // Check for directives like .stage or .kernel
                    self.advance()?;
                    match &self.current {
                        HighToken::Stage => {
                            self.advance()?;
                            stages.push(self.parse_stage()?);
                        }
                        HighToken::Ident(s) if s == "stage" => {
                            self.advance()?;
                            stages.push(self.parse_stage()?);
                        }
                        HighToken::Kernel => {
                            self.advance()?;
                            kernels.push(self.parse_kernel()?);
                        }
                        HighToken::Ident(s) if s == "kernel" => {
                            self.advance()?;
                            if let HighToken::Str(kname) = self.advance()? {
                                kernels.push(KernelDecl {
                                    name: kname,
                                    params: Vec::new(),
                                    return_type: None,
                                    attributes: Vec::new(),
                                    body: Vec::new(),
                                });
                            }
                        }
                        other => {
                            return Err(format!("Unknown top-level directive '.{:?}'", other));
                        }
                    }
                }
                HighToken::Arch => {
                    architectures.push(self.parse_arch()?);
                }
                HighToken::Kernel | HighToken::Fn => {
                    kernels.push(self.parse_kernel()?);
                }
                HighToken::At => {
                    // Attribute applied to kernel or arch
                    let attr = self.parse_attribute()?;
                    if self.current == HighToken::Kernel || self.current == HighToken::Fn {
                        let mut k = self.parse_kernel()?;
                        k.attributes.push(attr);
                        kernels.push(k);
                    }
                }
                HighToken::Semicolon => {
                    self.advance()?;
                }
                _ => {
                    return Err(format!("Unexpected token at module top-level: {:?}", self.current));
                }
            }
        }

        Ok(HighModule {
            name,
            imports,
            stages,
            architectures,
            kernels,
        })
    }

    fn parse_import(&mut self) -> Result<ImportDecl, String> {
        self.expect(HighToken::Import)?;
        let mut path_parts = Vec::new();

        while let HighToken::Ident(part) = &self.current {
            path_parts.push(part.clone());
            self.advance()?;
            if self.current == HighToken::Dot {
                self.advance()?;
            } else {
                break;
            }
        }

        let path = path_parts.join(".");
        let mut alias = None;

        if self.current == HighToken::As {
            self.advance()?;
            if let HighToken::Ident(a) = self.advance()? {
                alias = Some(a);
            }
        }

        if self.current == HighToken::Semicolon {
            self.advance()?;
        }

        Ok(ImportDecl { path, alias })
    }

    fn parse_stage(&mut self) -> Result<StageDecl, String> {
        let name = if let HighToken::Str(s) = self.advance()? {
            s
        } else {
            return Err("Expected stage name string".to_string());
        };

        let mut params = HashMap::new();
        while self.current == HighToken::Comma {
            self.advance()?;
            if let HighToken::Ident(key) = self.advance()? {
                self.expect(HighToken::Assign)?;
                let val = match self.advance()? {
                    HighToken::Str(s) => s,
                    HighToken::Ident(s) => s,
                    HighToken::Int(i) => i.to_string(),
                    HighToken::Float(f) => f.to_string(),
                    HighToken::Bool(b) => b.to_string(),
                    other => return Err(format!("Invalid stage parameter value: {:?}", other)),
                };
                params.insert(key, val);
            }
        }

        if self.current == HighToken::Semicolon {
            self.advance()?;
        }

        Ok(StageDecl { name, params })
    }

    fn parse_arch(&mut self) -> Result<ArchDecl, String> {
        self.expect(HighToken::Arch)?;
        let name = if let HighToken::Ident(n) = self.advance()? {
            n
        } else {
            return Err("Expected architecture name".to_string());
        };

        self.expect(HighToken::LBrace)?;
        let mut states = Vec::new();
        let mut methods = Vec::new();

        while self.current != HighToken::RBrace && self.current != HighToken::Eof {
            match self.current {
                HighToken::State => {
                    self.advance()?;
                    let sname = if let HighToken::Ident(n) = self.advance()? { n } else { return Err("Expected state name".to_string()); };
                    self.expect(HighToken::Colon)?;
                    let ty = self.parse_type()?;
                    if self.current == HighToken::Semicolon { self.advance()?; }
                    states.push(StateField { name: sname, ty });
                }
                HighToken::Kernel | HighToken::Fn => {
                    methods.push(self.parse_kernel()?);
                }
                HighToken::At => {
                    let attr = self.parse_attribute()?;
                    let mut k = self.parse_kernel()?;
                    k.attributes.push(attr);
                    methods.push(k);
                }
                _ => return Err(format!("Unexpected token inside arch: {:?}", self.current)),
            }
        }

        self.expect(HighToken::RBrace)?;
        Ok(ArchDecl { name, states, methods })
    }

    fn parse_kernel(&mut self) -> Result<KernelDecl, String> {
        if self.current == HighToken::Kernel || self.current == HighToken::Fn {
            self.advance()?;
        }

        let name = if let HighToken::Ident(n) = self.advance()? {
            n
        } else {
            return Err("Expected kernel name identifier".to_string());
        };

        self.expect(HighToken::LParen)?;
        let mut params = Vec::new();

        while self.current != HighToken::RParen && self.current != HighToken::Eof {
            let pname = if let HighToken::Ident(n) = self.advance()? {
                n
            } else {
                return Err("Expected parameter name".to_string());
            };

            self.expect(HighToken::Colon)?;
            let ty = self.parse_type()?;
            params.push(KernelParam { name: pname, ty });

            if self.current == HighToken::Comma {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect(HighToken::RParen)?;

        let mut return_type = None;
        if self.current == HighToken::Arrow {
            self.advance()?;
            return_type = Some(self.parse_type()?);
        }

        self.expect(HighToken::LBrace)?;
        let mut body = Vec::new();

        while self.current != HighToken::RBrace && self.current != HighToken::Eof {
            body.push(self.parse_stmt()?);
        }

        self.expect(HighToken::RBrace)?;

        Ok(KernelDecl {
            name,
            params,
            return_type,
            attributes: Vec::new(),
            body,
        })
    }

    fn parse_attribute(&mut self) -> Result<Attribute, String> {
        self.expect(HighToken::At)?;
        let name = if let HighToken::Ident(n) = self.advance()? {
            n
        } else {
            return Err("Expected attribute name".to_string());
        };

        let mut args = HashMap::new();
        if self.current == HighToken::LParen {
            self.advance()?;
            while self.current != HighToken::RParen && self.current != HighToken::Eof {
                if let HighToken::Ident(key) = self.advance()? {
                    self.expect(HighToken::Assign)?;
                    let val = match self.advance()? {
                        HighToken::Ident(s) => s,
                        HighToken::Str(s) => s,
                        HighToken::Int(i) => i.to_string(),
                        HighToken::Float(f) => f.to_string(),
                        HighToken::Bool(b) => b.to_string(),
                        other => return Err(format!("Invalid attribute value: {:?}", other)),
                    };
                    args.insert(key, val);
                }
                if self.current == HighToken::Comma {
                    self.advance()?;
                } else {
                    break;
                }
            }
            self.expect(HighToken::RParen)?;
        }

        Ok(Attribute { name, args })
    }

    fn parse_type(&mut self) -> Result<HighType, String> {
        let type_name = if let HighToken::Ident(n) = self.advance()? {
            n
        } else {
            return Err("Expected type identifier".to_string());
        };

        if self.current == HighToken::LBracket {
            self.advance()?;
            match type_name.as_str() {
                "Tensor" => {
                    let dtype = self.parse_dtype()?;
                    let mut dims = Vec::new();
                    while self.current == HighToken::Comma {
                        self.advance()?;
                        dims.push(self.parse_dim_expr()?);
                    }
                    self.expect(HighToken::RBracket)?;
                    Ok(HighType::Tensor { dtype, dims })
                }
                "RegTile" => {
                    let rows = self.parse_usize()?;
                    self.expect(HighToken::Comma)?;
                    let cols = self.parse_usize()?;
                    self.expect(HighToken::Comma)?;
                    let dtype = self.parse_dtype()?;
                    self.expect(HighToken::RBracket)?;
                    Ok(HighType::RegTile { rows, cols, dtype })
                }
                "Multivector" => {
                    let algebra = if let HighToken::Ident(a) = self.advance()? { a } else { "Cl3".to_string() };
                    self.expect(HighToken::Comma)?;
                    let dim = self.parse_usize()?;
                    self.expect(HighToken::RBracket)?;
                    Ok(HighType::Multivector { algebra, dim })
                }
                "Spikes" => {
                    let width = self.parse_usize()?;
                    self.expect(HighToken::Comma)?;
                    let height = self.parse_usize()?;
                    self.expect(HighToken::Comma)?;
                    let time_steps = self.parse_usize()?;
                    self.expect(HighToken::RBracket)?;
                    Ok(HighType::Spikes { width, height, time_steps })
                }
                "StateSpace" => {
                    let d_state = self.parse_usize()?;
                    self.expect(HighToken::Comma)?;
                    let d_inner = self.parse_usize()?;
                    self.expect(HighToken::RBracket)?;
                    Ok(HighType::StateSpace { d_state, d_inner })
                }
                _ => {
                    // Generic custom bracketed type
                    self.skip_until_rbracket();
                    Ok(HighType::Custom(type_name))
                }
            }
        } else {
            Ok(HighType::Custom(type_name))
        }
    }

    fn parse_dtype(&mut self) -> Result<DType, String> {
        if let HighToken::Ident(s) = self.advance()? {
            let dt = match s.to_lowercase().as_str() {
                "nvfp4" => DType::NVFP4,
                "mxfp4" => DType::MXFP4,
                "mxfp6" => DType::MXFP6,
                "fp8_e4m3" => DType::FP8_E4M3,
                "fp8_e5m2" => DType::FP8_E5M2,
                "bf16" => DType::BF16,
                "fp16" => DType::FP16,
                "fp32" | "float32" | "f32" => DType::FP32,
                "fp64" | "float64" | "f64" => DType::FP64,
                "trit16" | "ternary" => DType::Trit16,
                "int4" => DType::Int4,
                "int8" | "i8" => DType::Int8,
                "int16" | "i16" => DType::Int16,
                "int32" | "i32" => DType::Int32,
                "int64" | "i64" => DType::Int64,
                "bool" => DType::Bool,
                _ => DType::Custom(s),
            };
            Ok(dt)
        } else {
            Err("Expected data type identifier".to_string())
        }
    }

    fn parse_dim_expr(&mut self) -> Result<DimExpr, String> {
        match self.advance()? {
            HighToken::Int(i) => Ok(DimExpr::Static(i as usize)),
            HighToken::Ident(s) => Ok(DimExpr::Symbolic(s)),
            other => Err(format!("Expected dimension literal or symbolic identifier, found {:?}", other)),
        }
    }

    fn parse_usize(&mut self) -> Result<usize, String> {
        match self.advance()? {
            HighToken::Int(i) => Ok(i as usize),
            other => Err(format!("Expected integer, found {:?}", other)),
        }
    }

    fn skip_until_rbracket(&mut self) {
        while self.current != HighToken::RBracket && self.current != HighToken::Eof {
            let _ = self.advance();
        }
        let _ = self.advance(); // consume RBracket
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.current {
            HighToken::YieldReturn | HighToken::Return => {
                self.advance()?;
                let expr = if self.current != HighToken::Semicolon && self.current != HighToken::RBrace {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                if self.current == HighToken::Semicolon {
                    self.advance()?;
                }
                Ok(Stmt::Return(expr))
            }
            HighToken::HardwareLoop => {
                self.advance()?;
                self.expect(HighToken::LParen)?;
                let iters = self.parse_usize()?;
                self.expect(HighToken::RParen)?;
                self.expect(HighToken::LBrace)?;
                let mut loop_body = Vec::new();
                while self.current != HighToken::RBrace && self.current != HighToken::Eof {
                    loop_body.push(self.parse_stmt()?);
                }
                self.expect(HighToken::RBrace)?;
                Ok(Stmt::RealtimeBlock { clock_hz: iters as u64, max_jitter_ns: 0, body: loop_body })
            }
            HighToken::Watchdog => {
                self.advance()?;
                if self.current == HighToken::LBrace {
                    self.advance()?;
                    while self.current != HighToken::RBrace && self.current != HighToken::Eof {
                        self.advance()?;
                    }
                    if self.current == HighToken::RBrace {
                        self.advance()?;
                    }
                }
                Ok(Stmt::RealtimeBlock { clock_hz: 1, max_jitter_ns: 0, body: Vec::new() })
            }
            HighToken::Let => {
                eprintln!("[DEPRECATED] 'let' keyword is deprecated in High-Level CL. Use ':=' for functional tensor morphisms.");
                self.advance()?;
                let name = if let HighToken::Ident(n) = self.advance()? {
                    n
                } else {
                    return Err("Expected variable name after 'let'".to_string());
                };

                let mut ty = None;
                if self.current == HighToken::Colon {
                    self.advance()?;
                    ty = Some(self.parse_type()?);
                }

                self.expect(HighToken::Assign)?;
                let expr = self.parse_expr()?;

                if self.current == HighToken::Semicolon {
                    self.advance()?;
                }

                Ok(Stmt::Let { name, ty, expr })
            }
            _ => {
                let expr = self.parse_expr()?;
                if self.current == HighToken::WalrusAssign || self.current == HighToken::Assign || self.current == HighToken::Transfer {
                    self.advance()?;
                    let val = self.parse_expr()?;
                    if self.current == HighToken::Semicolon {
                        self.advance()?;
                    }
                    Ok(Stmt::Assign { target: expr, expr: val })
                } else {
                    if self.current == HighToken::Semicolon {
                        self.advance()?;
                    }
                    Ok(Stmt::ExprStmt(expr))
                }
            }
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_binary_expr(0)
    }

    fn get_op_precedence(&self, op: &HighBinaryOp) -> u8 {
        match op {
            HighBinaryOp::AssociativeScan => 1,      // >>= lowest precedence fold
            HighBinaryOp::InterferenceCoupling => 2, // <~>
            HighBinaryOp::LogSpaceGating => 3,       // <|>
            HighBinaryOp::RingReduction => 4,        // <%>
            HighBinaryOp::RadixGraft => 5,           // /@
            HighBinaryOp::PeriodicResonance => 6,    // ~*~
            HighBinaryOp::Eq | HighBinaryOp::Neq | HighBinaryOp::Lt | HighBinaryOp::Gt | HighBinaryOp::Lte | HighBinaryOp::Gte => 7,
            HighBinaryOp::Add | HighBinaryOp::Sub => 8,
            HighBinaryOp::Mul | HighBinaryOp::Div | HighBinaryOp::Mod => 9,
            HighBinaryOp::ExteriorWedge | HighBinaryOp::InnerScalar => 10,
        }
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
        let mut left = self.parse_unary_or_primary()?;

        while let Some(op) = self.current_binary_op() {
            let prec = self.get_op_precedence(&op);
            if prec < min_prec {
                break;
            }
            self.advance()?; // consume operator
            let right = self.parse_binary_expr(prec + 1)?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn current_binary_op(&self) -> Option<HighBinaryOp> {
        match self.current {
            HighToken::InterferenceCoupling => Some(HighBinaryOp::InterferenceCoupling),
            HighToken::AssociativeScan => Some(HighBinaryOp::AssociativeScan),
            HighToken::LogSpaceGating => Some(HighBinaryOp::LogSpaceGating),
            HighToken::RingReduction => Some(HighBinaryOp::RingReduction),
            HighToken::RadixGraft => Some(HighBinaryOp::RadixGraft),
            HighToken::PeriodicResonance => Some(HighBinaryOp::PeriodicResonance),
            HighToken::ExteriorWedge => Some(HighBinaryOp::ExteriorWedge),
            HighToken::Plus => Some(HighBinaryOp::Add),
            HighToken::Minus => Some(HighBinaryOp::Sub),
            HighToken::Star => Some(HighBinaryOp::Mul),
            HighToken::Slash => Some(HighBinaryOp::Div),
            HighToken::Percent => Some(HighBinaryOp::Mod),
            HighToken::EqEq => Some(HighBinaryOp::Eq),
            HighToken::NotEq => Some(HighBinaryOp::Neq),
            HighToken::Lt => Some(HighBinaryOp::Lt),
            HighToken::Gt => Some(HighBinaryOp::Gt),
            HighToken::Lte => Some(HighBinaryOp::Lte),
            HighToken::Gte => Some(HighBinaryOp::Gte),
            _ => None,
        }
    }

    fn parse_unary_or_primary(&mut self) -> Result<Expr, String> {
        match self.current {
            HighToken::Minus => {
                self.advance()?;
                let operand = self.parse_unary_or_primary()?;
                Ok(Expr::UnaryOp { op: HighUnaryOp::Neg, operand: Box::new(operand) })
            }
            HighToken::Tilde => {
                // ~ Clifford rotor reversion
                self.advance()?;
                let operand = self.parse_unary_or_primary()?;
                Ok(Expr::UnaryOp { op: HighUnaryOp::RotorReverse, operand: Box::new(operand) })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current.clone() {
            HighToken::Int(i) => {
                self.advance()?;
                Ok(Expr::Literal(Literal::Int(i)))
            }
            HighToken::Float(f) => {
                self.advance()?;
                Ok(Expr::Literal(Literal::Float(f)))
            }
            HighToken::Str(s) => {
                self.advance()?;
                Ok(Expr::Literal(Literal::String(s)))
            }
            HighToken::Bool(b) => {
                self.advance()?;
                Ok(Expr::Literal(Literal::Bool(b)))
            }
            HighToken::LParen => {
                self.advance()?;
                let expr = self.parse_expr()?;
                if self.current == HighToken::Comma {
                    let mut tuple = vec![expr];
                    while self.current == HighToken::Comma {
                        self.advance()?;
                        if self.current == HighToken::RParen { break; }
                        tuple.push(self.parse_expr()?);
                    }
                    self.expect(HighToken::RParen)?;
                    Ok(Expr::Tuple(tuple))
                } else {
                    self.expect(HighToken::RParen)?;
                    Ok(expr)
                }
            }
            HighToken::Dot => {
                // Universal Machine Primitive: `.op.<name>(...)`
                self.advance()?;
                if let HighToken::Ident(op_prefix) = self.advance()? {
                    if op_prefix == "op" {
                        self.expect(HighToken::Dot)?;
                        if let HighToken::Ident(op_name) = self.advance()? {
                            return self.parse_primitive_call(op_name);
                        }
                    }
                }
                Err("Invalid primitive call syntax. Expected '.op.<name>(...)'".to_string())
            }
            HighToken::Ident(id) => {
                self.advance()?;
                let mut expr = Expr::Ident(id);

                // Handle post-ident dot, index, or method call
                while self.current == HighToken::Dot || self.current == HighToken::LBracket {
                    if self.current == HighToken::Dot {
                        self.advance()?;
                        if let HighToken::Ident(method) = self.advance()? {
                            if self.current == HighToken::LParen {
                                self.advance()?;
                                let mut args = Vec::new();
                                while self.current != HighToken::RParen && self.current != HighToken::Eof {
                                    args.push(self.parse_expr()?);
                                    if self.current == HighToken::Comma { self.advance()?; } else { break; }
                                }
                                self.expect(HighToken::RParen)?;
                                expr = Expr::MethodCall { target: Box::new(expr), method, args };
                            } else {
                                expr = Expr::MethodCall { target: Box::new(expr), method, args: Vec::new() };
                            }
                        }
                    } else if self.current == HighToken::LBracket {
                        self.advance()?;
                        let mut indices = Vec::new();
                        while self.current != HighToken::RBracket && self.current != HighToken::Eof {
                            indices.push(self.parse_expr()?);
                            if self.current == HighToken::Comma { self.advance()?; } else { break; }
                        }
                        self.expect(HighToken::RBracket)?;
                        expr = Expr::TensorAccess { target: Box::new(expr), indices };
                    }
                }

                Ok(expr)
            }
            _ => Err(format!("Unexpected expression token: {:?}", self.current)),
        }
    }

    fn parse_primitive_call(&mut self, op_name: String) -> Result<Expr, String> {
        self.expect(HighToken::LParen)?;
        let mut positional_args = Vec::new();
        let mut named_args = Vec::new();

        while self.current != HighToken::RParen && self.current != HighToken::Eof {
            // Check if this is a named argument `key = val` using 2-token lookahead
            if let HighToken::Ident(k) = &self.current {
                if self.peek == HighToken::Assign {
                    let ident = k.clone();
                    self.advance()?; // consume ident
                    self.expect(HighToken::Assign)?; // consume '='
                    let val = self.parse_expr()?;
                    named_args.push((ident, val));
                    if self.current == HighToken::Comma {
                        self.advance()?;
                    } else {
                        break;
                    }
                    continue;
                }
            }

            let expr = self.parse_expr()?;
            positional_args.push(expr);

            if self.current == HighToken::Comma {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect(HighToken::RParen)?;
        Ok(Expr::PrimitiveCall { op_name, positional_args, named_args })
    }
}

/// Convenience entry point to parse a High-Level CL source into a HighModule AST.
pub fn parse_high_cl(source: &str) -> Result<HighModule, String> {
    // If source is Sovereign CL v2.0 (Post-Human Standard: @kernel, @⌖, §Block, ↦):
    if source.contains('§') || source.contains("@kernel") || source.contains("@⌖") || source.contains('↦') {
        let doc = crate::cl_v2_parser::parse_cl_v2(source)?;
        return Ok(cl_v2_to_high_module(&doc));
    }

    if source.contains("module ") || source.contains("import ") || source.contains("let ") {
        eprintln!("[CRON NOTICE] Legacy verbal keywords ('module', 'import', 'let') are deprecated in High-Level CL.");
        eprintln!("              Prefer Sovereign CL v2.0 5-Pillar Standard (@kernel, @mesh, @io, §Block, :=).");
    }

    let lexer = HighLexer::new(source);
    let mut parser = HighParser::new(lexer)?;
    parser.parse_module()
}

/// Convert Sovereign CL v2.0 Document into HighModule representation.
pub fn cl_v2_to_high_module(doc: &crate::cl_v2_ast::ClV2Document) -> HighModule {
    let name = doc
        .target_contract
        .as_ref()
        .map(|t| t.kernel_name.clone())
        .unwrap_or_else(|| "sovereign_kernel".to_string());

    let mut stages = Vec::new();
    if let Some(target) = &doc.target_contract {
        let mut params = std::collections::HashMap::new();
        params.insert("target".to_string(), target.target.clone());
        params.insert("cores".to_string(), target.cores.to_string());
        params.insert("ipc".to_string(), target.ipc_target.to_string());
        if let Some(mesh) = &target.mesh_routing {
            params.insert("mesh".to_string(), mesh.vector_formula.clone());
        }
        stages.push(StageDecl {
            name: "SiliconTarget".to_string(),
            params,
        });
    }

    let mut kernels = Vec::new();
    if let Some(morphism) = &doc.math_morphism {
        let mut body = Vec::new();
        for step in &morphism.steps {
            body.push(Stmt::Assign {
                target: Expr::Ident(step.target_var.clone()),
                expr: Expr::Ident(step.expr.clone()),
            });
        }
        kernels.push(KernelDecl {
            name: morphism.name.clone(),
            params: vec![KernelParam {
                name: morphism.input_symbol.clone(),
                ty: HighType::Tensor {
                    dtype: DType::FP32,
                    dims: morphism
                        .tile_dims
                        .iter()
                        .map(|&d| DimExpr::Static(d))
                        .collect(),
                },
            }],
            return_type: Some(HighType::Tensor {
                dtype: DType::FP32,
                dims: morphism
                    .tile_dims
                    .iter()
                    .map(|&d| DimExpr::Static(d))
                    .collect(),
            }),
            attributes: Vec::new(),
            body,
        });
    }

    HighModule {
        name,
        imports: Vec::new(),
        stages,
        architectures: Vec::new(),
        kernels,
    }
}
