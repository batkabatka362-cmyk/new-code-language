// ============================================================================
// CRON Lexer — Tokenizer for the CRON .cr High-Level Language
// Supports full keyword set, operators, comments, literals, and axis values.
// Exact line, column, and byte span tracking on all emitted tokens.
// ============================================================================

use crate::token::{Span, Spanned, Token};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    byte_offset: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            byte_offset: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos])
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.chars.len() {
            Some(self.chars[self.pos + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.pos += 1;
            self.byte_offset += c.len_utf8();
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else if c != '\r' {
                self.col += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn make_span(&self, start_line: usize, start_col: usize, start_byte: usize) -> Span {
        Span::new(
            start_line,
            start_col,
            start_byte,
            self.byte_offset.saturating_sub(start_byte).max(1),
        )
    }

    pub fn tokenize(&mut self) -> Result<Vec<Spanned<Token>>, String> {
        let mut tokens = Vec::new();
        let mut line_has_tokens = false;

        while let Some(c) = self.peek() {
            let start_line = self.line;
            let start_col = self.col;
            let start_byte = self.byte_offset;

            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    line_has_tokens = false;
                }
                '/' if self.peek_next() == Some('/') => {
                    // Line comment //
                    while let Some(ch) = self.peek() {
                        self.advance();
                        if ch == '\n' {
                            line_has_tokens = false;
                            break;
                        }
                    }
                }
                '#' => {
                    // Line comment #
                    while let Some(ch) = self.peek() {
                        self.advance();
                        if ch == '\n' {
                            line_has_tokens = false;
                            break;
                        }
                    }
                }
                '/' if self.peek_next() == Some('*') => {
                    // Block comment /* ... */
                    self.advance(); // /
                    self.advance(); // *
                    let mut depth = 1;
                    while depth > 0 {
                        match self.advance() {
                            Some('/') if self.peek() == Some('*') => {
                                self.advance();
                                depth += 1;
                            }
                            Some('*') if self.peek() == Some('/') => {
                                self.advance();
                                depth -= 1;
                            }
                            None => return Err("Unterminated block comment".to_string()),
                            _ => {}
                        }
                    }
                }
                ';' if !line_has_tokens => {
                    // Start-of-line comment ;
                    while let Some(ch) = self.peek() {
                        self.advance();
                        if ch == '\n' {
                            line_has_tokens = false;
                            break;
                        }
                    }
                }
                ';' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Semicolon, span));
                    line_has_tokens = true;
                }
                '.' => {
                    line_has_tokens = true;
                    self.advance();
                    if self.peek() == Some('.') {
                        self.advance();
                        let span = self.make_span(start_line, start_col, start_byte);
                        tokens.push(Spanned::new(Token::DotDot, span));
                        continue;
                    }
                    // Could be directive or dot
                    let mut word = String::new();
                    while let Some(ch) = self.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            word.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if word == "MODULE" {
                        let span = self.make_span(start_line, start_col, start_byte);
                        tokens.push(Spanned::new(Token::ModuleDir, span));
                    } else if word == "ENTRY" {
                        let span = self.make_span(start_line, start_col, start_byte);
                        tokens.push(Spanned::new(Token::EntryDir, span));
                    } else if word == "END" {
                        let span = self.make_span(start_line, start_col, start_byte);
                        tokens.push(Spanned::new(Token::EndDir, span));
                    } else if word.is_empty() {
                        let span = self.make_span(start_line, start_col, start_byte);
                        tokens.push(Spanned::new(Token::Dot, span));
                    } else {
                        let dot_span = Span::new(start_line, start_col, start_byte, 1);
                        tokens.push(Spanned::new(Token::Dot, dot_span));
                        let ident_span = Span::new(
                            start_line,
                            start_col + 1,
                            start_byte + 1,
                            word.len().max(1),
                        );
                        tokens.push(Spanned::new(Token::Ident(word), ident_span));
                    }
                }
                '(' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::OpenParen, span));
                    line_has_tokens = true;
                }
                ')' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::CloseParen, span));
                    line_has_tokens = true;
                }
                '{' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::OpenBrace, span));
                    line_has_tokens = true;
                }
                '}' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::CloseBrace, span));
                    line_has_tokens = true;
                }
                '[' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::OpenBracket, span));
                    line_has_tokens = true;
                }
                ']' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::CloseBracket, span));
                    line_has_tokens = true;
                }
                ',' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Comma, span));
                    line_has_tokens = true;
                }
                ':' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Colon, span));
                    line_has_tokens = true;
                }
                '-' if self.peek_next() == Some('>') => {
                    self.advance();
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Arrow, span));
                    line_has_tokens = true;
                }
                '-' if self.peek_next() == Some('=') => {
                    self.advance();
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::MinusAssign, span));
                    line_has_tokens = true;
                }
                '+' if self.peek_next() == Some('=') => {
                    self.advance();
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::PlusAssign, span));
                    line_has_tokens = true;
                }
                '*' if self.peek_next() == Some('=') => {
                    self.advance();
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::StarAssign, span));
                    line_has_tokens = true;
                }
                '/' if self.peek_next() == Some('=') => {
                    self.advance();
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::SlashAssign, span));
                    line_has_tokens = true;
                }
                '=' => {
                    self.advance();
                    let tok = if self.peek() == Some('=') {
                        self.advance();
                        Token::EqualEqual
                    } else if self.peek() == Some('>') {
                        self.advance();
                        Token::FatArrow
                    } else {
                        Token::Assign
                    };
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(tok, span));
                    line_has_tokens = true;
                }
                '!' => {
                    self.advance();
                    let tok = if self.peek() == Some('=') {
                        self.advance();
                        Token::NotEqual
                    } else {
                        Token::Bang
                    };
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(tok, span));
                    line_has_tokens = true;
                }
                '<' => {
                    self.advance();
                    let tok = if self.peek() == Some('=') {
                        self.advance();
                        Token::LessEqual
                    } else if self.peek() == Some('<') {
                        self.advance();
                        Token::Shl
                    } else {
                        Token::Less
                    };
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(tok, span));
                    line_has_tokens = true;
                }
                '>' => {
                    self.advance();
                    let tok = if self.peek() == Some('=') {
                        self.advance();
                        Token::GreaterEqual
                    } else if self.peek() == Some('>') {
                        self.advance();
                        Token::Shr
                    } else {
                        Token::Greater
                    };
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(tok, span));
                    line_has_tokens = true;
                }
                '&' => {
                    self.advance();
                    let tok = if self.peek() == Some('&') {
                        self.advance();
                        Token::AmpAmp
                    } else {
                        Token::Amp
                    };
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(tok, span));
                    line_has_tokens = true;
                }
                '|' => {
                    self.advance();
                    let tok = if self.peek() == Some('|') {
                        self.advance();
                        Token::PipePipe
                    } else {
                        Token::Pipe
                    };
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(tok, span));
                    line_has_tokens = true;
                }
                '^' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Caret, span));
                    line_has_tokens = true;
                }
                '+' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Plus, span));
                    line_has_tokens = true;
                }
                '-' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Minus, span));
                    line_has_tokens = true;
                }
                '*' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Star, span));
                    line_has_tokens = true;
                }
                '/' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Slash, span));
                    line_has_tokens = true;
                }
                '%' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Percent, span));
                    line_has_tokens = true;
                }
                '"' => {
                    self.advance();
                    let mut s = String::new();
                    while let Some(ch) = self.peek() {
                        if ch == '"' {
                            self.advance();
                            break;
                        } else if ch == '\\' {
                            self.advance();
                            match self.advance() {
                                Some('n') => s.push('\n'),
                                Some('t') => s.push('\t'),
                                Some('r') => s.push('\r'),
                                Some('\\') => s.push('\\'),
                                Some('"') => s.push('"'),
                                Some('0') => s.push('\0'),
                                Some(esc) => s.push(esc),
                                None => return Err("Unterminated string escape".to_string()),
                            }
                        } else {
                            s.push(ch);
                            self.advance();
                        }
                    }
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::StringLit(s), span));
                    line_has_tokens = true;
                }
                '$' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::Dollar, span));
                    line_has_tokens = true;
                }
                '@' => {
                    self.advance();
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::At, span));
                    line_has_tokens = true;
                }
                '0' if self.peek_next() == Some('x') || self.peek_next() == Some('X') => {
                    // Hex literal
                    self.advance(); // '0'
                    self.advance(); // 'x'
                    let mut hex_str = String::new();
                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_hexdigit() {
                            hex_str.push(ch);
                            self.advance();
                        } else if ch == '_' {
                            self.advance(); // skip underscore in 0x000A_0000
                        } else {
                            break;
                        }
                    }
                    if hex_str.is_empty() {
                        return Err("Expected hex digits after '0x'".to_string());
                    }
                    let val = u64::from_str_radix(&hex_str, 16)
                        .map_err(|e| format!("Invalid hex literal: 0x{}: {}", hex_str, e))?;
                    let span = self.make_span(start_line, start_col, start_byte);
                    tokens.push(Spanned::new(Token::HexLit(val), span));
                    line_has_tokens = true;
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    let mut is_float = false;
                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            if ch != '_' {
                                num_str.push(ch);
                            }
                            self.advance();
                        } else if ch == '.' && self.peek_next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                            is_float = true;
                            num_str.push(ch);
                            self.advance();
                        } else if ch == 'e' || ch == 'E' {
                            // Scientific notation: 1e5, 1.0e-5, etc.
                            is_float = true;
                            num_str.push(ch);
                            self.advance();
                            if let Some(sign) = self.peek() {
                                if sign == '+' || sign == '-' {
                                    num_str.push(sign);
                                    self.advance();
                                }
                            }
                            while let Some(digit) = self.peek() {
                                if digit.is_ascii_digit() || digit == '_' {
                                    if digit != '_' {
                                        num_str.push(digit);
                                    }
                                    self.advance();
                                } else {
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    let span = self.make_span(start_line, start_col, start_byte);
                    if is_float {
                        let val: f64 = num_str.parse().map_err(|e| format!("Invalid float {}: {}", num_str, e))?;
                        tokens.push(Spanned::new(Token::FloatLit(val), span));
                    } else {
                        let val: i64 = num_str.parse().map_err(|e| format!("Invalid integer {}: {}", num_str, e))?;
                        tokens.push(Spanned::new(Token::IntLit(val), span));
                    }
                    line_has_tokens = true;
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut word = String::new();
                    while let Some(ch) = self.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            word.push(ch);
                            self.advance();
                        } else if (ch == '+' || ch == '-') && (word == "X" || word == "Y" || word == "Z" || word == "W") {
                            // Axis literal like X+, X-, Y+, Y-
                            word.push(ch);
                            self.advance();
                            break;
                        } else {
                            break;
                        }
                    }

                    let span = self.make_span(start_line, start_col, start_byte);
                    if word == "X+" || word == "X-" || word == "Y+" || word == "Y-" ||
                       word == "Z+" || word == "Z-" || word == "W+" || word == "W-" {
                        tokens.push(Spanned::new(Token::Axis(word), span));
                    } else {
                        let tok = match word.as_str() {
                            "async" => Token::Async,
                            "def" | "fn" => Token::Def,
                            "let" => Token::Let,
                            "lin" => Token::Lin,
                            "grad" => Token::Grad,
                            "region" => Token::Region,
                            "resilient_compute" | "resilient" => Token::ResilientCompute,
                            "fallback" => Token::Fallback,
                            "return" => Token::Return,
                            "export" => Token::Export,
                            "as" => Token::As,
                            "await" => Token::Await,
                            "spawn" => Token::Spawn,
                            "consume" => Token::Consume,
                            "import" => Token::Import,
                            "from" => Token::From,
                            "proof_contract" => Token::ProofContract,
                            "invariant" => Token::Invariant,
                            "ensures" => Token::Ensures,
                            "if" => Token::If,
                            "else" => Token::Else,
                            "while" => Token::While,
                            "for" => Token::For,
                            "in" => Token::In,
                            "true" => Token::True,
                            "false" => Token::False,
                            "and" => Token::And,
                            "or" => Token::Or,
                            "not" => Token::Not,
                            "struct" => Token::Struct,
                            "enum" => Token::Enum,
                            "match" => Token::Match,
                            "_" => Token::Underscore,
                            "type" => Token::Type,
                            "mut" => Token::Mut,
                            "inline" => Token::Inline,
                            "module" => Token::Module,
                            "trait" => Token::Trait,
                            "impl" => Token::Impl,
                            "schedule" => Token::Schedule,
                            "brain" => Token::Brain,
                            "fork" => Token::Fork,
                            "simulate" => Token::Simulate,
                            "abort" => Token::Abort,
                            "then" => Token::Then,
                            "with" => Token::With,
                            _ => Token::Ident(word),
                        };
                        tokens.push(Spanned::new(tok, span));
                    }
                    line_has_tokens = true;
                }
                _ => {
                    return Err(format!("Unexpected character '{}' at line {}, col {}", c, self.line, self.col));
                }
            }
        }

        tokens.push(Spanned::new(Token::Eof, Span::point(self.line, self.col, self.byte_offset)));
        Ok(tokens)
    }

    pub fn tokenize_raw(&mut self) -> Result<Vec<Token>, String> {
        let spanned = self.tokenize()?;
        Ok(spanned.into_iter().map(|s| s.value).collect())
    }
}
