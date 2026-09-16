// ============================================================================
// CRON Parser — Full-Featured Recursive Descent Parser
// Parses .cr high-level source code into a strongly-typed AST.
// Supports: operator precedence, if/else, while/for, field access, indexing
// ============================================================================

use crate::ast::*;
use crate::token::{Span, Spanned, Token};

pub struct Parser {
    tokens: Vec<Spanned<Token>>,
    pos: usize,
    prev_span: Span,
}

impl Parser {
    pub fn new(tokens: Vec<Spanned<Token>>) -> Self {
        let first_span = tokens.first().map(|t| t.span).unwrap_or_default();
        Self {
            tokens,
            pos: 0,
            prev_span: first_span,
        }
    }

    pub fn from_raw_tokens(tokens: Vec<Token>) -> Self {
        let spanned = tokens
            .into_iter()
            .map(|t| Spanned::new(t, Span::default()))
            .collect();
        Self::new(spanned)
    }

    fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos].value
        } else {
            &Token::Eof
        }
    }

    fn peek_offset(&self, offset: usize) -> &Token {
        if self.pos + offset < self.tokens.len() {
            &self.tokens[self.pos + offset].value
        } else {
            &Token::Eof
        }
    }

    fn current_span(&self) -> Span {
        if self.pos < self.tokens.len() {
            self.tokens[self.pos].span
        } else {
            self.prev_span
        }
    }

    #[allow(dead_code)]
    fn prev_span(&self) -> Span {
        self.prev_span
    }

    fn advance(&mut self) -> Token {
        if self.pos < self.tokens.len() {
            let sp = &self.tokens[self.pos];
            self.prev_span = sp.span;
            let t = sp.value.clone();
            self.pos += 1;
            t
        } else {
            Token::Eof
        }
    }

    fn check(&self, expected: &Token) -> bool {
        self.peek() == expected
    }

    fn match_token(&mut self, expected: &Token) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            let sp = self.current_span();
            Err(format!(
                "Line {}:{}: Expected token {:?}, found {:?}",
                sp.line,
                sp.col,
                expected,
                self.peek()
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut module_name = String::from("Main");
        let mut entry_name = None;
        let mut imports = Vec::new();
        let mut type_aliases = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut traits = Vec::new();
        let mut impls = Vec::new();
        let mut functions = Vec::new();
        let mut schedules = Vec::new();
        let mut brains = Vec::new();
        let mut main_statements = Vec::new();

        while !self.check(&Token::Eof) && !self.check(&Token::EndDir) {
            if self.match_token(&Token::ModuleDir) || self.match_token(&Token::Module) {
                let mut name = match self.advance() {
                    Token::Ident(n) => n,
                    other => return Err(format!("Expected module name, got {:?}", other)),
                };
                while self.match_token(&Token::Dot) {
                    if let Token::Ident(sub) = self.advance() {
                        name.push('.');
                        name.push_str(&sub);
                    }
                }
                module_name = name;
            } else if self.match_token(&Token::EntryDir) {
                match self.advance() {
                    Token::Ident(name) => entry_name = Some(name),
                    other => return Err(format!("Expected entry name after .ENTRY, got {:?}", other)),
                }
            } else if self.match_token(&Token::Import) {
                let mut items = Vec::new();
                if self.match_token(&Token::OpenBrace) {
                    while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
                        if let Token::Ident(item) = self.advance() {
                            items.push(item);
                        }
                        self.match_token(&Token::Comma);
                    }
                    self.expect(&Token::CloseBrace)?;
                } else if let Token::Ident(single) = self.advance() {
                    let mut full_name = single;
                    while self.match_token(&Token::Dot) {
                        if let Token::Ident(sub) = self.advance() {
                            full_name.push('.');
                            full_name.push_str(&sub);
                        }
                    }
                    items.push(full_name);
                }
                let mut path = String::new();
                if self.match_token(&Token::From) {
                    match self.advance() {
                        Token::StringLit(p) => path = p,
                        Token::Ident(p) => path = p,
                        other => return Err(format!("Expected import path after 'from', got {:?}", other)),
                    }
                }
                if self.match_token(&Token::As) {
                    let _ = self.advance();
                }
                self.match_token(&Token::Semicolon);
                imports.push(ImportDecl { items, path });
            } else if self.match_token(&Token::Type) {
                let name = match self.advance() {
                    Token::Ident(n) => n,
                    other => return Err(format!("Expected type alias name, got {:?}", other)),
                };
                self.expect(&Token::Assign)?;
                let target_type = self.parse_type_str()?;
                self.match_token(&Token::Semicolon);
                type_aliases.push(TypeAliasDecl { name, target_type });
            } else if self.match_token(&Token::Struct) {
                let name = match self.advance() {
                    Token::Ident(n) => n,
                    other => return Err(format!("Expected struct name, got {:?}", other)),
                };
                let mut generic_params = Vec::new();
                if self.match_token(&Token::Less) {
                    while !self.check(&Token::Greater) && !self.check(&Token::Eof) {
                        match self.advance() {
                            Token::Ident(p) => generic_params.push(p),
                            other => return Err(format!("Expected generic parameter in struct {}, got {:?}", name, other)),
                        }
                        self.match_token(&Token::Comma);
                    }
                    self.expect(&Token::Greater)?;
                }
                self.expect(&Token::OpenBrace)?;
                let mut fields = Vec::new();
                while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
                    let field_name = match self.advance() {
                        Token::Ident(n) => n,
                        Token::Fallback => "fallback".to_string(),
                        Token::Region => "region".to_string(),
                        Token::Type => "type".to_string(),
                        Token::Export => "export".to_string(),
                        Token::From => "from".to_string(),
                        Token::As => "as".to_string(),
                        Token::Lin => "lin".to_string(),
                        other => return Err(format!("Expected field name in struct {}, got {:?}", name, other)),
                    };
                    self.expect(&Token::Colon)?;
                    let field_type = self.parse_type_str()?;
                    fields.push((field_name, field_type));
                    self.match_token(&Token::Comma);
                    self.match_token(&Token::Semicolon);
                }
                self.expect(&Token::CloseBrace)?;
                structs.push(StructDecl { name, generic_params, fields });
            } else if self.match_token(&Token::Enum) {
                let name = match self.advance() {
                    Token::Ident(n) => n,
                    other => return Err(format!("Expected enum name, got {:?}", other)),
                };
                let mut generic_params = Vec::new();
                if self.match_token(&Token::Less) {
                    while !self.check(&Token::Greater) && !self.check(&Token::Eof) {
                        match self.advance() {
                            Token::Ident(p) => generic_params.push(p),
                            other => return Err(format!("Expected generic parameter in enum {}, got {:?}", name, other)),
                        }
                        self.match_token(&Token::Comma);
                    }
                    self.expect(&Token::Greater)?;
                }
                self.expect(&Token::OpenBrace)?;
                let mut variants = Vec::new();
                while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
                    let variant_name = match self.advance() {
                        Token::Ident(n) => n,
                        other => return Err(format!("Expected enum variant name in enum {}, got {:?}", name, other)),
                    };
                    let payload = if self.match_token(&Token::OpenParen) {
                        let mut types = Vec::new();
                        while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                            types.push(self.parse_type_str()?);
                            self.match_token(&Token::Comma);
                        }
                        self.expect(&Token::CloseParen)?;
                        Some(types)
                    } else {
                        None
                    };
                    variants.push(EnumVariant { name: variant_name, payload });
                    self.match_token(&Token::Comma);
                    self.match_token(&Token::Semicolon);
                }
                self.expect(&Token::CloseBrace)?;
                enums.push(EnumDecl { name, generic_params, variants });
            } else if self.check(&Token::Trait) {
                let trait_decl = self.parse_trait_decl()?;
                traits.push(trait_decl);
            } else if self.check(&Token::Impl) {
                let impl_decl = self.parse_impl_decl()?;
                impls.push(impl_decl);
            } else if (self.check(&Token::Export) && matches!(self.peek_offset(1), Token::Def | Token::Async | Token::Inline))
                || self.check(&Token::Async) || self.check(&Token::Def) || self.check(&Token::Inline) {
                let func = self.parse_function_decl()?;
                functions.push(func);
            } else if self.check(&Token::Schedule) {
                let schedule_decl = self.parse_schedule_decl()?;
                schedules.push(schedule_decl);
            } else if self.check(&Token::Brain) {
                let brain_decl = self.parse_brain_decl()?;
                brains.push(brain_decl);
            } else if let Token::Ident(_name) = self.peek() {
                if (self.peek_offset(1) == &Token::Colon
                    && self.peek_offset(2) != &Token::Colon
                    && !matches!(self.peek_offset(2), Token::Ident(_) | Token::OpenParen))
                    || (self.peek_offset(1) == &Token::Colon
                        && self.peek_offset(2) != &Token::Colon
                        && matches!(self.peek_offset(2), Token::Ident(_))
                        && self.peek_offset(3) != &Token::Assign
                        && self.peek_offset(3) != &Token::Comma)
                {
                    // Label like _main:
                    let is_label = if self.peek_offset(1) == &Token::Colon && self.peek_offset(2) != &Token::Colon {
                        match self.peek_offset(2) {
                            Token::Ident(_) => {
                                !matches!(self.peek_offset(3), Token::Assign | Token::Comma)
                                    && !matches!(self.peek_offset(2), Token::Ident(ref t) if
                                        t.ends_with("_t") || t == "u32" || t == "u64" || t == "i32"
                                        || t == "i64" || t == "f32" || t == "f64" || t == "bool"
                                        || t == "wave_t" || t == "ext_addr_t" || t == "spk_stamp"
                                        || t == "rev_t" || t == "linear")
                            }
                            _ => true,
                        }
                    } else {
                        false
                    };

                    if is_label {
                        self.advance(); // name
                        self.advance(); // :
                        continue;
                    }
                }
                let stmt = self.parse_statement()?;
                main_statements.push(stmt);
            } else {
                let stmt = self.parse_statement()?;
                main_statements.push(stmt);
            }
        }

        // Match optional .END
        self.match_token(&Token::EndDir);

        Ok(Program {
            module_name,
            entry_name,
            imports,
            type_aliases,
            structs,
            enums,
            traits,
            impls,
            functions,
            schedules,
            brains,
            main_statements,
        })
    }

    /// Parse `trait <Name> { def <method>(<params>) -> <type>; ... }`
    fn parse_trait_decl(&mut self) -> Result<TraitDecl, String> {
        self.expect(&Token::Trait)?;
        let name = match self.advance() {
            Token::Ident(n) => n,
            other => return Err(format!("Expected trait name, got {:?}", other)),
        };
        let mut generic_params = Vec::new();
        if self.match_token(&Token::Less) {
            while !self.check(&Token::Greater) && !self.check(&Token::Eof) {
                match self.advance() {
                    Token::Ident(p) => generic_params.push(p),
                    other => return Err(format!("Expected generic parameter in trait {}, got {:?}", name, other)),
                }
                self.match_token(&Token::Comma);
            }
            self.expect(&Token::Greater)?;
        }
        self.expect(&Token::OpenBrace)?;
        let mut methods = Vec::new();
        while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
            self.expect(&Token::Def)?;
            let method_name = match self.advance() {
                Token::Ident(n) => n,
                other => return Err(format!("Expected method name in trait {}, got {:?}", name, other)),
            };
            self.expect(&Token::OpenParen)?;
            let mut params = Vec::new();
            if !self.check(&Token::CloseParen) {
                loop {
                    // 'self' param for trait methods
                    if matches!(self.peek(), Token::Ident(ref n) if n == "self") {
                        let param_span = self.current_span();
                        self.advance();
                        params.push(Param {
                            is_lin: false,
                            is_grad: false,
                            name: "self".to_string(),
                            param_type: "Self".to_string(),
                            span: param_span,
                        });
                    } else {
                        let is_lin = self.match_token(&Token::Lin);
                        let is_grad = self.match_token(&Token::Grad);
                        let param_span = self.current_span();
                        let param_name = match self.advance() {
                            Token::Ident(n) => n,
                            other => return Err(format!("Expected parameter name, found {:?}", other)),
                        };
                        self.expect(&Token::Colon)?;
                        let param_type = self.parse_type_str()?;
                        params.push(Param {
                            is_lin,
                            is_grad,
                            name: param_name,
                            param_type,
                            span: param_span,
                        });
                    }
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.expect(&Token::CloseParen)?;
            let return_type = if self.match_token(&Token::Arrow) {
                Some(self.parse_type_str()?)
            } else {
                None
            };
            self.match_token(&Token::Semicolon);
            methods.push(TraitMethodSig {
                name: method_name,
                params,
                return_type,
            });
        }
        self.expect(&Token::CloseBrace)?;
        Ok(TraitDecl { name, generic_params, methods })
    }

    /// Parse `impl <Trait> for <Struct> { def <method>(<params>) { body } ... }`
    fn parse_impl_decl(&mut self) -> Result<ImplDecl, String> {
        self.expect(&Token::Impl)?;
        let mut generic_params = Vec::new();
        if self.match_token(&Token::Less) {
            while !self.check(&Token::Greater) && !self.check(&Token::Eof) {
                match self.advance() {
                    Token::Ident(p) => generic_params.push(p),
                    other => return Err(format!("Expected generic parameter in impl, got {:?}", other)),
                }
                self.match_token(&Token::Comma);
            }
            self.expect(&Token::Greater)?;
        }
        let first_name = match self.advance() {
            Token::Ident(n) => n,
            other => return Err(format!("Expected trait or struct name after 'impl', got {:?}", other)),
        };
        // Optional type arguments after first identifier, e.g. `impl<T> Channel<T>`
        if self.match_token(&Token::Less) {
            while !self.check(&Token::Greater) && !self.check(&Token::Eof) {
                self.advance();
                self.match_token(&Token::Comma);
            }
            self.expect(&Token::Greater)?;
        }

        // Check if 'for' keyword follows (indicating trait implementation)
        let (trait_name, target_struct) = if self.match_token(&Token::For) {
            let target_struct = match self.advance() {
                Token::Ident(n) => n,
                other => return Err(format!("Expected struct name after 'for', got {:?}", other)),
            };
            if self.match_token(&Token::Less) {
                while !self.check(&Token::Greater) && !self.check(&Token::Eof) {
                    self.advance();
                    self.match_token(&Token::Comma);
                }
                self.expect(&Token::Greater)?;
            }
            (first_name, target_struct)
        } else {
            // Inherent impl: impl Struct { ... }
            (String::new(), first_name)
        };
        self.expect(&Token::OpenBrace)?;
        let mut methods = Vec::new();
        while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
            let func = self.parse_function_decl()?;
            methods.push(func);
        }
        self.expect(&Token::CloseBrace)?;
        Ok(ImplDecl {
            trait_name,
            target_struct,
            generic_params,
            methods,
        })
    }

    fn parse_function_decl(&mut self) -> Result<FunctionDecl, String> {
        let mut is_async = false;
        let mut is_export = false;
        let mut is_inline = false;

        loop {
            if self.match_token(&Token::Export) {
                is_export = true;
            } else if self.match_token(&Token::Async) {
                is_async = true;
            } else if self.match_token(&Token::Inline) {
                is_inline = true;
            } else {
                break;
            }
        }

        self.expect(&Token::Def)?;
        if self.match_token(&Token::Inline) {
            is_inline = true;
        }

        let name = match self.advance() {
            Token::Ident(n) => n,
            other => return Err(format!("Expected function name, found {:?}", other)),
        };

        let mut generic_params = Vec::new();
        if self.match_token(&Token::Less) {
            while !self.check(&Token::Greater) && !self.check(&Token::Eof) {
                match self.advance() {
                    Token::Ident(p) => generic_params.push(p),
                    other => return Err(format!("Expected generic parameter in function {}, got {:?}", name, other)),
                }
                self.match_token(&Token::Comma);
            }
            self.expect(&Token::Greater)?;
        }

        self.expect(&Token::OpenParen)?;
        let mut params = Vec::new();
        if !self.check(&Token::CloseParen) {
            loop {
                // Handle 'self' parameter for impl methods (no type annotation needed)
                if matches!(self.peek(), Token::Ident(ref n) if n == "self") {
                    let param_span = self.current_span();
                    self.advance();
                    params.push(Param {
                        is_lin: false,
                        is_grad: false,
                        name: "self".to_string(),
                        param_type: "Self".to_string(),
                        span: param_span,
                    });
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                    continue;
                }

                let is_lin = self.match_token(&Token::Lin);
                let is_grad = self.match_token(&Token::Grad);
                let param_span = self.current_span();
                let param_name = match self.advance() {
                    Token::Ident(n) => n,
                    Token::Fallback => "fallback".to_string(),
                    Token::Region => "region".to_string(),
                    Token::Type => "type".to_string(),
                    Token::Export => "export".to_string(),
                    Token::From => "from".to_string(),
                    Token::As => "as".to_string(),
                    Token::Lin => "lin".to_string(),
                    other => return Err(format!("Expected parameter name, found {:?}", other)),
                };
                self.expect(&Token::Colon)?;
                let param_type = self.parse_type_str()?;
                let is_lin = is_lin || param_type.starts_with("linear") || param_type.starts_with("lin ");
                params.push(Param {
                    is_lin,
                    is_grad,
                    name: param_name,
                    param_type,
                    span: param_span,
                });

                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        self.expect(&Token::CloseParen)?;

        let return_type = if self.match_token(&Token::Arrow) {
            Some(self.parse_type_str()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(FunctionDecl {
            is_async,
            is_export,
            is_inline,
            name,
            generic_params,
            params,
            return_type,
            body,
        })
    }

    /// Parse `brain <Name> [attrs] { ... }`
    fn parse_brain_decl(&mut self) -> Result<BrainDecl, String> {
        let span = self.current_span();
        self.expect(&Token::Brain)?;
        let name = match self.advance() {
            Token::Ident(n) => n,
            other => return Err(format!("Expected brain name, got {:?}", other)),
        };
        let attrs = self.parse_bracket_attributes()?;
        let body = self.parse_block()?;
        Ok(BrainDecl { name, attrs, body, span })
    }

    /// Parse `schedule <target_fn> [for <target_arch>] { <directives>* }`
    fn parse_schedule_decl(&mut self) -> Result<ScheduleDecl, String> {
        let span = self.current_span();
        self.expect(&Token::Schedule)?;
        let target_fn = match self.advance() {
            Token::Ident(n) => n,
            other => return Err(format!("Expected target function name after schedule, got {:?}", other)),
        };

        let target_arch = if self.match_token(&Token::For) {
            match self.advance() {
                Token::Ident(a) => a,
                Token::StringLit(s) => s,
                other => return Err(format!("Expected target architecture after 'for', got {:?}", other)),
            }
        } else {
            "generic".to_string()
        };

        self.expect(&Token::OpenBrace)?;
        let mut directives = Vec::new();

        while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
            if self.match_token(&Token::Semicolon) {
                continue;
            }

            let dir_name = match self.advance() {
                Token::Ident(n) => n,
                other => return Err(format!("Expected schedule directive name, got {:?}", other)),
            };

            if dir_name == "autotune" {
                let has_brace = self.match_token(&Token::OpenBrace);
                if !has_brace {
                    self.expect(&Token::OpenParen)?;
                }
                let mut tile_sizes = Vec::new();
                let mut unrolls = Vec::new();
                let mut vectorize_widths = Vec::new();
                let mut metric = "min_latency".to_string();

                let close_tok = if has_brace { Token::CloseBrace } else { Token::CloseParen };
                while !self.check(&close_tok) && !self.check(&Token::Eof) {
                    if self.match_token(&Token::Semicolon) || self.match_token(&Token::Comma) {
                        continue;
                    }
                    let key = match self.advance() {
                        Token::Ident(k) => k,
                        other => return Err(format!("Expected autotune parameter key, got {:?}", other)),
                    };
                    self.expect(&Token::Colon)?;
                    match key.as_str() {
                        "tile_size" | "tile_sizes" => {
                            self.expect(&Token::OpenBracket)?;
                            while !self.check(&Token::CloseBracket) && !self.check(&Token::Eof) {
                                self.expect(&Token::OpenParen)?;
                                let w = self.parse_usize_lit("tile width")?;
                                self.expect(&Token::Comma)?;
                                let h = self.parse_usize_lit("tile height")?;
                                self.expect(&Token::CloseParen)?;
                                tile_sizes.push((w, h));
                                self.match_token(&Token::Comma);
                            }
                            self.expect(&Token::CloseBracket)?;
                        }
                        "unroll" | "unrolls" => {
                            self.expect(&Token::OpenBracket)?;
                            while !self.check(&Token::CloseBracket) && !self.check(&Token::Eof) {
                                unrolls.push(self.parse_usize_lit("unroll candidate")?);
                                self.match_token(&Token::Comma);
                            }
                            self.expect(&Token::CloseBracket)?;
                        }
                        "vectorize" | "vectorize_widths" => {
                            self.expect(&Token::OpenBracket)?;
                            while !self.check(&Token::CloseBracket) && !self.check(&Token::Eof) {
                                vectorize_widths.push(self.parse_usize_lit("vectorize width candidate")?);
                                self.match_token(&Token::Comma);
                            }
                            self.expect(&Token::CloseBracket)?;
                        }
                        "metric" => {
                            metric = self.parse_directive_string_or_raw()?;
                        }
                        _ => {
                            return Err(format!("Unknown autotune parameter key '{}'", key));
                        }
                    }
                    self.match_token(&Token::Comma);
                    self.match_token(&Token::Semicolon);
                }
                self.expect(&close_tok)?;
                self.match_token(&Token::Semicolon);
                directives.push(ScheduleDirective::Autotune {
                    tile_sizes,
                    unrolls,
                    vectorize_widths,
                    metric,
                });
                continue;
            }

            self.expect(&Token::OpenParen)?;

            let directive = match dir_name.as_str() {
                "tile_size" => {
                    let m = self.parse_usize_lit("tile_size width")?;
                    self.expect(&Token::Comma)?;
                    let n = self.parse_usize_lit("tile_size height")?;
                    ScheduleDirective::TileSize(m, n)
                }
                "prefetch_to" => {
                    let target = self.parse_directive_string_or_raw()?;
                    ScheduleDirective::PrefetchTo(target)
                }
                "unroll" => {
                    let factor = self.parse_usize_lit("unroll factor")?;
                    ScheduleDirective::Unroll(factor)
                }
                "distribute_4d" => {
                    let axis = if matches!(self.peek(), Token::Ident(ref id) if id == "axis")
                        && self.peek_offset(1) == &Token::Colon
                    {
                        self.advance(); // axis
                        self.advance(); // :
                        self.parse_directive_string_or_raw()?
                    } else {
                        self.parse_directive_string_or_raw()?
                    };
                    self.expect(&Token::Comma)?;
                    let cores = if matches!(self.peek(), Token::Ident(ref id) if id == "cores")
                        && self.peek_offset(1) == &Token::Colon
                    {
                        self.advance(); // cores
                        self.advance(); // :
                        self.parse_usize_lit("cores count")?
                    } else {
                        self.parse_usize_lit("cores count")?
                    };
                    ScheduleDirective::Distribute4D { axis, cores }
                }
                "vectorize" => {
                    let width = self.parse_usize_lit("vectorize width")?;
                    ScheduleDirective::Vectorize(width)
                }
                _ => {
                    let mut args = Vec::new();
                    while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                        args.push(self.parse_directive_string_or_raw()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    ScheduleDirective::Custom {
                        name: dir_name,
                        args,
                    }
                }
            };

            self.expect(&Token::CloseParen)?;
            self.match_token(&Token::Semicolon);
            directives.push(directive);
        }

        self.expect(&Token::CloseBrace)?;

        Ok(ScheduleDecl {
            target_fn,
            target_arch,
            directives,
            span,
        })
    }

    fn parse_directive_string_or_raw(&mut self) -> Result<String, String> {
        match self.peek() {
            Token::StringLit(_) => {
                if let Token::StringLit(s) = self.advance() {
                    Ok(s)
                } else {
                    unreachable!()
                }
            }
            Token::Ident(_) => {
                if let Token::Ident(s) = self.advance() {
                    Ok(s)
                } else {
                    unreachable!()
                }
            }
            Token::Axis(ax) => {
                let s = ax.clone();
                self.advance();
                Ok(s)
            }
            other => Err(format!(
                "Expected string or identifier in schedule directive, got {:?}",
                other
            )),
        }
    }

    fn parse_usize_lit(&mut self, ctx: &str) -> Result<usize, String> {
        match self.advance() {
            Token::IntLit(val) => {
                if val >= 0 {
                    Ok(val as usize)
                } else {
                    Err(format!("Expected positive integer for {}, got {}", ctx, val))
                }
            }
            Token::HexLit(val) => Ok(val as usize),
            other => Err(format!(
                "Expected integer literal for {}, got {:?}",
                ctx, other
            )),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        self.expect(&Token::OpenBrace)?;
        let mut stmts = Vec::new();
        while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
            if self.match_token(&Token::Semicolon) {
                continue;
            }
            stmts.push(self.parse_statement()?);
            self.match_token(&Token::Semicolon);
        }
        self.expect(&Token::CloseBrace)?;
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Token::Let => {
                let _stmt_span = self.current_span();
                self.advance(); // let
                let is_mut = self.match_token(&Token::Mut);
                let has_paren = self.match_token(&Token::OpenParen);
                let mut vars = Vec::new();
                loop {
                    let is_tainted = if matches!(self.peek(), Token::Ident(ref n) if n == "tainted") && matches!(self.peek_offset(1), Token::Ident(_)) {
                        self.advance();
                        true
                    } else {
                        false
                    };
                    let is_lin = self.match_token(&Token::Lin);
                    let is_grad = self.match_token(&Token::Grad);
                    let _is_lin2 = self.match_token(&Token::Lin);
                    let is_lin = is_lin || _is_lin2;

                    let var_span = self.current_span();
                    let name = match self.advance() {
                        Token::Ident(n) => n,
                        other => return Err(format!("Expected variable name in let, got {:?}", other)),
                    };

                    let type_annot = if self.match_token(&Token::Colon) {
                        let t = self.parse_type_str()?;
                        if is_tainted && !t.contains("tainted") {
                            Some(format!("tainted {}", t))
                        } else {
                            Some(t)
                        }
                    } else if is_tainted {
                        Some("tainted".to_string())
                    } else {
                        None
                    };

                    vars.push((is_lin, is_grad, name, type_annot, var_span));

                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
                if has_paren {
                    self.expect(&Token::CloseParen)?;
                }

                self.expect(&Token::Assign)?;
                let value = self.parse_expr()?;
                let (is_lin, is_grad, name, type_annot, var_span) = vars.remove(0);
                let extra_vars = vars.into_iter().map(|(l, g, n, t, _)| (l, g, n, t)).collect();
                Ok(Statement::Let {
                    is_lin,
                    is_grad,
                    is_mut,
                    name,
                    type_annot,
                    value,
                    extra_vars,
                    span: var_span,
                })
            }
            Token::Region => {
                let span = self.current_span();
                self.advance(); // region
                let name = match self.advance() {
                    Token::Ident(n) | Token::StringLit(n) => n,
                    other => return Err(format!("Expected region name, got {:?}", other)),
                };

                let attrs = self.parse_bracket_attributes()?;
                let body = self.parse_block()?;
                Ok(Statement::Region { name, attrs, body, span })
            }
            Token::ResilientCompute => {
                let span = self.current_span();
                self.advance(); // resilient_compute
                let attrs = self.parse_bracket_attributes()?;
                let body = self.parse_block()?;

                let fallback = if self.match_token(&Token::Fallback) {
                    Some(self.parse_block()?)
                } else {
                    None
                };

                Ok(Statement::Resilient {
                    attrs,
                    body,
                    fallback,
                    span,
                })
            }
            Token::Fuse => {
                let span = self.current_span();
                self.advance(); // fuse
                let attrs = self.parse_bracket_attributes()?;
                let body = self.parse_block()?;
                Ok(Statement::Fuse { attrs, body, span })
            }
            Token::Brain => {
                let span = self.current_span();
                self.advance(); // brain
                let name = match self.advance() {
                    Token::Ident(n) => n,
                    other => return Err(format!("Expected brain name, got {:?}", other)),
                };
                let attrs = self.parse_bracket_attributes()?;
                let body = self.parse_block()?;
                Ok(Statement::Brain { name, attrs, body, span })
            }
            Token::Fork => {
                let span = self.current_span();
                self.advance(); // fork
                let target = self.parse_expr()?;
                self.match_token(&Token::Semicolon);
                Ok(Statement::Fork { target, span })
            }
            Token::Simulate => {
                let span = self.current_span();
                self.advance(); // simulate
                let action = self.parse_expr()?;
                let with_arg = if self.match_token(&Token::With) {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.match_token(&Token::Semicolon);
                Ok(Statement::Simulate { action, with_arg, span })
            }
            Token::Abort => {
                let span = self.current_span();
                self.advance(); // abort
                self.match_token(&Token::Semicolon);
                Ok(Statement::Abort(span))
            }
            Token::If => {
                let span = self.current_span();
                self.advance(); // if
                let condition = self.parse_expr()?;
                let then_body = if self.match_token(&Token::Then) {
                    if self.check(&Token::OpenBrace) {
                        self.parse_block()?
                    } else {
                        vec![self.parse_statement()?]
                    }
                } else {
                    self.parse_block()?
                };

                let else_body = if self.match_token(&Token::Else) {
                    if self.check(&Token::If) {
                        // else if -> nested
                        let nested_if = self.parse_statement()?;
                        Some(vec![nested_if])
                    } else if self.check(&Token::OpenBrace) {
                        Some(self.parse_block()?)
                    } else {
                        Some(vec![self.parse_statement()?])
                    }
                } else {
                    None
                };

                Ok(Statement::If {
                    condition,
                    then_body,
                    else_body,
                    span,
                })
            }
            Token::While => {
                let span = self.current_span();
                self.advance(); // while
                let condition = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Statement::While { condition, body, span })
            }
            Token::Match => {
                let span = self.current_span();
                self.advance(); // match
                let expr = self.parse_expr()?;
                self.expect(&Token::OpenBrace)?;
                let mut arms = Vec::new();
                while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
                    let arm_span = self.current_span();
                    println!("Inside match loop, current token: {:?}", self.peek());
                    let pattern = self.parse_match_pattern()?;
                    self.expect(&Token::FatArrow)?;
                    let body = if self.check(&Token::OpenBrace) {
                        self.parse_block()?
                    } else {
                        vec![self.parse_statement()?]
                    };
                    arms.push(MatchArm { pattern, body, span: arm_span });
                    self.match_token(&Token::Comma);
                }
                self.expect(&Token::CloseBrace)?;
                Ok(Statement::Match { expr, arms, span })
            }
            Token::For => {
                let span = self.current_span();
                self.advance(); // for
                let var_name = match self.advance() {
                    Token::Ident(n) => n,
                    other => return Err(format!("Expected variable name in for, got {:?}", other)),
                };
                self.expect(&Token::In)?;
                let iterable = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Statement::For {
                    var_name,
                    iterable,
                    body,
                    span,
                })
            }
            Token::Return => {
                self.advance(); // return
                if self.check(&Token::CloseBrace) || self.check(&Token::Eof) {
                    Ok(Statement::Return(None))
                } else {
                    let expr = self.parse_expr()?;
                    Ok(Statement::Return(Some(expr)))
                }
            }
            Token::Export => {
                let span = self.current_span();
                self.advance(); // export
                let source_name = match self.advance() {
                    Token::Ident(n) => n,
                    other => return Err(format!("Expected identifier after export, got {:?}", other)),
                };
                self.expect(&Token::As)?;
                let exported_name = match self.advance() {
                    Token::Ident(n) => n,
                    other => return Err(format!("Expected target identifier after 'as', got {:?}", other)),
                };
                Ok(Statement::Export {
                    source_name,
                    exported_name,
                    span,
                })
            }
            Token::ProofContract => {
                self.advance(); // proof_contract
                self.expect(&Token::OpenBrace)?;
                let mut clauses = Vec::new();
                while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
                    clauses.push(self.parse_statement()?);
                }
                self.expect(&Token::CloseBrace)?;
                Ok(Statement::ProofContract(clauses))
            }
            Token::Invariant | Token::Ensures => {
                let kw = match self.advance() {
                    Token::Invariant => "invariant",
                    Token::Ensures => "ensures",
                    _ => unreachable!(),
                }.to_string();
                self.expect(&Token::OpenParen)?;
                let condition = self.parse_expr()?;
                self.expect(&Token::CloseParen)?;
                Ok(Statement::Expr(Expr::Call {
                    callee: kw,
                    args: vec![CallArg { name: None, value: condition }],
                }))
            }
            Token::Ident(ref n) if n == "superposition" => {
                let span = self.current_span();
                self.advance(); // consume "superposition"
                let mut params = Vec::new();
                if self.match_token(&Token::OpenParen) {
                    while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                        let arg = self.parse_call_arg()?;
                        params.push(arg);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    self.expect(&Token::CloseParen)?;
                }

                self.expect(&Token::OpenBrace)?;
                let mut branches = Vec::new();
                while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
                    let branch_name = match self.advance() {
                        Token::Ident(b) => b,
                        other => return Err(format!("Expected superposition branch name, got {:?}", other)),
                    };
                    self.expect(&Token::Colon)?;
                    let body = self.parse_block()?;
                    branches.push(SuperpositionBranch {
                        name: branch_name,
                        body,
                    });
                    self.match_token(&Token::Comma);
                }
                self.expect(&Token::CloseBrace)?;

                let mut collapse_args = None;
                if let Token::Ident(ref c) = self.peek() {
                    if c == "collapse_with" {
                        self.advance(); // collapse_with
                        self.expect(&Token::OpenParen)?;
                        let mut c_args = Vec::new();
                        while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                            let arg = self.parse_call_arg()?;
                            c_args.push(arg);
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                        self.expect(&Token::CloseParen)?;
                        collapse_args = Some(c_args);
                    }
                }

                Ok(Statement::Superposition {
                    params,
                    branches,
                    collapse_args,
                    span,
                })
            }
            _ => {
                let expr = self.parse_expr()?;

                // Check for assignment: ident = expr  or  ident += expr
                if let Expr::Ident(ref name, id_span) = expr {
                    let target_name = name.clone();
                    if self.match_token(&Token::Assign) {
                        let value = self.parse_expr()?;
                        return Ok(Statement::Assign {
                            target: target_name,
                            value,
                            span: id_span,
                        });
                    }
                    if self.match_token(&Token::PlusAssign) {
                        let right = self.parse_expr()?;
                        return Ok(Statement::Assign {
                            target: target_name,
                            value: Expr::Binary {
                                op: "+".to_string(),
                                left: Box::new(expr),
                                right: Box::new(right),
                            },
                            span: id_span,
                        });
                    }
                    if self.match_token(&Token::MinusAssign) {
                        let right = self.parse_expr()?;
                        return Ok(Statement::Assign {
                            target: target_name,
                            value: Expr::Binary {
                                op: "-".to_string(),
                                left: Box::new(expr),
                                right: Box::new(right),
                            },
                            span: id_span,
                        });
                    }
                }

                Ok(Statement::Expr(expr))
            }
        }
    }

    fn parse_bracket_attributes(&mut self) -> Result<Vec<(String, String)>, String> {
        let mut attrs = Vec::new();
        if self.match_token(&Token::OpenBracket) {
            while !self.check(&Token::CloseBracket) && !self.check(&Token::Eof) {
                let key = match self.advance() {
                    Token::Ident(k) => k,
                    other => return Err(format!("Expected attribute key, got {:?}", other)),
                };
                self.expect(&Token::Assign)?;
                let val = match self.advance() {
                    Token::Ident(v) => v,
                    Token::Axis(v) => v,
                    Token::IntLit(v) => v.to_string(),
                    Token::HexLit(v) => format!("0x{:X}", v),
                    Token::StringLit(v) => v,
                    Token::OpenParen => {
                        let mut tuple_parts = Vec::new();
                        while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                            match self.advance() {
                                Token::IntLit(n) => tuple_parts.push(n.to_string()),
                                Token::Ident(n) => tuple_parts.push(n),
                                Token::Comma => {},
                                other => return Err(format!("Unexpected token in attribute tuple: {:?}", other)),
                            }
                        }
                        self.expect(&Token::CloseParen)?;
                        format!("({})", tuple_parts.join(", "))
                    }
                    other => return Err(format!("Expected attribute value, got {:?}", other)),
                };
                attrs.push((key, val));
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
            self.expect(&Token::CloseBracket)?;
        }
        Ok(attrs)
    }

    fn parse_match_pattern(&mut self) -> Result<MatchPattern, String> {
        match self.peek() {
            Token::Underscore => {
                self.advance();
                Ok(MatchPattern::Wildcard)
            }
            Token::Ident(_) => {
                let name = match self.advance() {
                    Token::Ident(n) => n,
                    _ => unreachable!(),
                };
                let (enum_name, variant_name) = if self.match_token(&Token::Colon) {
                    self.expect(&Token::Colon)?;
                    let v = match self.advance() {
                        Token::Ident(v) => v,
                        other => return Err(format!("Expected variant name after '::', got {:?}", other)),
                    };
                    (Some(name), v)
                } else {
                    (None, name)
                };

                let mut bindings = Vec::new();
                if self.match_token(&Token::OpenParen) {
                    while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                        match self.advance() {
                            Token::Ident(b) => bindings.push(b),
                            Token::Underscore => bindings.push("_".to_string()),
                            other => return Err(format!("Expected binding identifier in match arm, got {:?}", other)),
                        }
                        self.match_token(&Token::Comma);
                    }
                    self.expect(&Token::CloseParen)?;
                }
                Ok(MatchPattern::Variant { enum_name, variant_name, bindings })
            }
            _ => {
                let lit = self.parse_expr()?;
                Ok(MatchPattern::Literal(lit))
            }
        }
    }

    fn parse_type_str(&mut self) -> Result<String, String> {
        if self.match_token(&Token::OpenParen) {
            let mut inner = Vec::new();
            while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                inner.push(self.parse_type_str()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
            self.expect(&Token::CloseParen)?;
            return Ok(format!("({})", inner.join(", ")));
        }

        if self.match_token(&Token::OpenBracket) {
            let inner = self.parse_type_str()?;
            let mut count_str = String::new();
            if self.match_token(&Token::Semicolon) {
                match self.advance() {
                    Token::IntLit(n) => count_str = n.to_string(),
                    Token::Ident(n) => count_str = n,
                    other => return Err(format!("Expected array size in type, got {:?}", other)),
                }
            }
            self.expect(&Token::CloseBracket)?;
            if count_str.is_empty() {
                return Ok(format!("[{}]", inner));
            } else {
                return Ok(format!("[{}; {}]", inner, count_str));
            }
        }

        let mut spatial_prefix = String::new();
        if self.match_token(&Token::At) {
            let attr_name = match self.advance() {
                Token::Ident(n) => n,
                other => return Err(format!("Expected spatial memory domain (@sram, @hbm, @noc), got {:?}", other)),
            };
            spatial_prefix = format!("@{}", attr_name);
            if self.match_token(&Token::OpenParen) {
                spatial_prefix.push('(');
                while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                    match self.advance() {
                        Token::Ident(n) => spatial_prefix.push_str(&n),
                        Token::IntLit(n) => spatial_prefix.push_str(&n.to_string()),
                        Token::Assign => spatial_prefix.push('='),
                        Token::Comma => spatial_prefix.push_str(", "),
                        _ => {}
                    }
                }
                self.expect(&Token::CloseParen)?;
                spatial_prefix.push(')');
            }
            spatial_prefix.push(' ');
        }

        let mut type_str = match self.advance() {
            Token::Ident(s) => s,
            Token::Lin => "lin".to_string(),
            other => return Err(format!("Expected type identifier, got {:?}", other)),
        };

        if type_str == "linear" || type_str == "tainted" {
            if let Token::Ident(inner) = self.peek() {
                type_str = format!("{} {}", type_str, inner);
                self.advance();
            }
        }

        if !spatial_prefix.is_empty() {
            type_str = format!("{}{}", spatial_prefix, type_str);
        }

        if self.match_token(&Token::Less) {
            type_str.push('<');
            let mut depth = 1;
            while depth > 0 && !self.check(&Token::Eof) {
                if self.check(&Token::Less) {
                    depth += 1;
                    type_str.push('<');
                    self.advance();
                } else if self.check(&Token::Greater) {
                    depth -= 1;
                    type_str.push('>');
                    self.advance();
                } else {
                    let tok = self.advance();
                    match tok {
                        Token::Ident(s) => type_str.push_str(&s),
                        Token::IntLit(n) => type_str.push_str(&n.to_string()),
                        Token::Comma => type_str.push_str(", "),
                        _ => {}
                    }
                }
            }
        }
        Ok(type_str)
    }

    // =========================================================================
    // Expression Parser with Operator Precedence (Pratt-style)
    // Priority (low→high): or, and, ==/</>/../!=, +/-, *///%,  unary, postfix
    // =========================================================================

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or | Token::PipePipe) {
            self.advance();
            let right = self.parse_and_expr()?;
            expr = Expr::Binary {
                op: "or".to_string(),
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison_expr()?;
        while matches!(self.peek(), Token::And | Token::AmpAmp) {
            self.advance();
            let right = self.parse_comparison_expr()?;
            expr = Expr::Binary {
                op: "and".to_string(),
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_comparison_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_bitwise_expr()?;

        // Check for 'as <type>' cast
        if self.match_token(&Token::As) {
            let target_type = self.parse_type_str()?;
            expr = Expr::Cast {
                expr: Box::new(expr),
                target_type,
            };
        }

        while matches!(self.peek(),
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual
            | Token::EqualEqual | Token::NotEqual
        ) {
            let op = match self.advance() {
                Token::Less => "<",
                Token::LessEqual => "<=",
                Token::Greater => ">",
                Token::GreaterEqual => ">=",
                Token::EqualEqual => "==",
                Token::NotEqual => "!=",
                _ => unreachable!(),
            }.to_string();
            let right = self.parse_bitwise_expr()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        while self.match_token(&Token::DotDot) {
            let right = self.parse_bitwise_expr()?;
            expr = Expr::Binary {
                op: "..".to_string(),
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_bitwise_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_additive_expr()?;
        while matches!(self.peek(), Token::Amp | Token::Pipe | Token::Caret | Token::Shl | Token::Shr) {
            let op = match self.advance() {
                Token::Amp => "&",
                Token::Pipe => "|",
                Token::Caret => "^",
                Token::Shl => "<<",
                Token::Shr => ">>",
                _ => unreachable!(),
            }.to_string();
            let right = self.parse_additive_expr()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_additive_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplicative_expr()?;
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance() {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => unreachable!(),
            }.to_string();
            let right = self.parse_multiplicative_expr()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary_expr()?;
        while matches!(self.peek(), Token::Star | Token::Slash | Token::Percent) {
            let op = match self.advance() {
                Token::Star => "*",
                Token::Slash => "/",
                Token::Percent => "%",
                _ => unreachable!(),
            }.to_string();
            let right = self.parse_unary_expr()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        // 'as' cast after multiplicative
        if self.match_token(&Token::As) {
            let target_type = self.parse_type_str()?;
            expr = Expr::Cast {
                expr: Box::new(expr),
                target_type,
            };
        }

        Ok(expr)
    }

    fn parse_unary_expr(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                match self.peek() {
                    Token::IntLit(_) => {
                        if let Token::IntLit(n) = self.advance() {
                            Ok(Expr::LiteralInt(-n))
                        } else {
                            unreachable!()
                        }
                    }
                    Token::FloatLit(_) => {
                        if let Token::FloatLit(f) = self.advance() {
                            Ok(Expr::LiteralFloat(-f))
                        } else {
                            unreachable!()
                        }
                    }
                    _ => {
                        let operand = self.parse_unary_expr()?;
                        Ok(Expr::Unary {
                            op: "-".to_string(),
                            operand: Box::new(operand),
                        })
                    }
                }
            }
            Token::Plus => {
                self.advance();
                match self.peek() {
                    Token::IntLit(_) => {
                        if let Token::IntLit(n) = self.advance() {
                            Ok(Expr::LiteralInt(n))
                        } else {
                            unreachable!()
                        }
                    }
                    Token::FloatLit(_) => {
                        if let Token::FloatLit(f) = self.advance() {
                            Ok(Expr::LiteralFloat(f))
                        } else {
                            unreachable!()
                        }
                    }
                    _ => {
                        let operand = self.parse_unary_expr()?;
                        Ok(Expr::Unary {
                            op: "+".to_string(),
                            operand: Box::new(operand),
                        })
                    }
                }
            }
            Token::Bang | Token::Not => {
                self.advance();
                let operand = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: "not".to_string(),
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_postfix_expr(),
        }
    }

    fn parse_postfix_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary_expr()?;

        loop {
            if self.match_token(&Token::Dot) {
                // Field access: expr.field or expr.method(...)
                let field = match self.advance() {
                    Token::Ident(f) => f,
                    other => return Err(format!("Expected field name after '.', got {:?}", other)),
                };

                if self.match_token(&Token::OpenParen) {
                    // Method call: expr.method(args...)
                    let mut args = Vec::new();
                    while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                        let arg = self.parse_call_arg()?;
                        args.push(arg);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    self.expect(&Token::CloseParen)?;
                    expr = Expr::MethodCall {
                        object: Box::new(expr),
                        method: field,
                        args,
                    };
                } else {
                    expr = Expr::FieldAccess {
                        object: Box::new(expr),
                        field,
                    };
                }
            } else if self.match_token(&Token::OpenBracket) {
                // Index: expr[index]
                let index = self.parse_expr()?;
                self.expect(&Token::CloseBracket)?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&Token::OpenParen) {
                // Function call chaining on expression, e.g. grad(f)(x)
                let _call_span = self.current_span();
                let mut args = Vec::new();
                while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                    let arg = self.parse_call_arg()?;
                    args.push(arg);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
                self.expect(&Token::CloseParen)?;
                match expr {
                    Expr::Grad { callee, wrt, span: g_span } => {
                        expr = Expr::GradCall {
                            callee,
                            wrt,
                            args,
                            span: g_span,
                        };
                    }
                    other => {
                        expr = Expr::Call {
                            callee: match &other {
                                Expr::Ident(name, _) => name.clone(),
                                _ => format!("{:?}", other),
                            },
                            args,
                        };
                    }
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_call_arg(&mut self) -> Result<CallArg, String> {
        if self.peek_offset(1) == &Token::Assign {
            let key = match self.peek() {
                Token::Ident(n) => n.clone(),
                Token::From => "from".to_string(),
                Token::As => "as".to_string(),
                Token::Lin => "lin".to_string(),
                Token::Fallback => "fallback".to_string(),
                Token::Region => "region".to_string(),
                Token::Type => "type".to_string(),
                Token::Export => "export".to_string(),
                other => format!("{:?}", other),
            };
            self.advance(); // name
            self.advance(); // =
            let val = self.parse_expr()?;
            Ok(CallArg {
                name: Some(key),
                value: val,
            })
        } else {
            let val = self.parse_expr()?;
            Ok(CallArg {
                name: None,
                value: val,
            })
        }
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::OpenParen => {
                self.advance(); // (
                if self.match_token(&Token::CloseParen) {
                    return Ok(Expr::Tuple(Vec::new()));
                }
                let first = self.parse_expr()?;
                if self.match_token(&Token::Comma) {
                    let mut elements = vec![first];
                    while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                        elements.push(self.parse_expr()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    self.expect(&Token::CloseParen)?;
                    Ok(Expr::Tuple(elements))
                } else {
                    self.expect(&Token::CloseParen)?;
                    Ok(first)
                }
            }
            Token::IntLit(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::LiteralInt(val))
            }
            Token::HexLit(h) => {
                let val = *h;
                self.advance();
                Ok(Expr::LiteralHex(val))
            }
            Token::FloatLit(f) => {
                let val = *f;
                self.advance();
                Ok(Expr::LiteralFloat(val))
            }
            Token::True => {
                self.advance();
                Ok(Expr::LiteralBool(true))
            }
            Token::False => {
                self.advance();
                Ok(Expr::LiteralBool(false))
            }
            Token::Dollar => {
                self.advance(); // $
                let inner = self.parse_primary_expr()?;
                Ok(Expr::Call {
                    callee: "$trap".to_string(),
                    args: vec![CallArg { name: None, value: inner }],
                })
            }
            Token::StringLit(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::LiteralString(val))
            }
            Token::Axis(a) => {
                let val = a.clone();
                self.advance();
                Ok(Expr::LiteralAxis(val))
            }
            Token::Consume => {
                let span = self.current_span();
                self.advance(); // consume
                self.expect(&Token::OpenParen)?;
                let mut ident = match self.advance() {
                    Token::Ident(n) => n,
                    other => return Err(format!("Expected identifier in consume(...), got {:?}", other)),
                };
                while self.match_token(&Token::Dot) {
                    if let Token::Ident(f) = self.advance() {
                        ident.push('.');
                        ident.push_str(&f);
                    }
                }
                self.expect(&Token::CloseParen)?;
                Ok(Expr::Consume(ident, span))
            }
            Token::Grad => {
                let span = self.current_span();
                self.advance(); // grad
                self.expect(&Token::OpenParen)?;
                let callee = self.parse_expr()?;
                let mut wrt = None;
                if self.match_token(&Token::Comma) {
                    if let Token::Ident(param_key) = self.peek() {
                        if param_key == "wrt" && self.peek_offset(1) == &Token::Colon {
                            self.advance(); // wrt
                            self.advance(); // :
                        }
                    }
                    match self.advance() {
                        Token::StringLit(s) | Token::Ident(s) => wrt = Some(s),
                        other => return Err(format!("Expected parameter name for grad wrt, got {:?}", other)),
                    }
                }
                self.expect(&Token::CloseParen)?;
                Ok(Expr::Grad {
                    callee: Box::new(callee),
                    wrt,
                    span,
                })
            }
            Token::Await => {
                self.advance(); // await
                let inner = self.parse_expr()?;
                Ok(Expr::Await(Box::new(inner)))
            }
            Token::Spawn => {
                let span = self.current_span();
                self.advance(); // spawn
                if self.check(&Token::Ident("at".to_string())) {
                    self.advance(); // at
                    self.expect(&Token::OpenParen)?;
                    let core_id = self.parse_expr()?;
                    self.expect(&Token::CloseParen)?;
                    let inner = self.parse_expr()?;
                    Ok(Expr::SpawnAt {
                        core_id: Box::new(core_id),
                        target: Box::new(inner),
                        span,
                    })
                } else {
                    let inner = self.parse_expr()?;
                    Ok(Expr::Spawn(Box::new(inner)))
                }
            }
            Token::If => {
                self.advance(); // if
                let condition = self.parse_expr()?;
                self.expect(&Token::OpenBrace)?;
                let then_branch = self.parse_block_expr()?;
                self.expect(&Token::CloseBrace)?;
                let else_branch = if self.match_token(&Token::Else) {
                    if self.check(&Token::If) {
                        self.parse_primary_expr()?
                    } else {
                        self.expect(&Token::OpenBrace)?;
                        let e = self.parse_block_expr()?;
                        self.expect(&Token::CloseBrace)?;
                        e
                    }
                } else {
                    Expr::LiteralInt(0)
                };
                Ok(Expr::IfExpr {
                    condition: Box::new(condition),
                    then_branch: Box::new(then_branch),
                    else_branch: Box::new(else_branch),
                })
            }
            Token::OpenBracket => {
                // Array literal [1, 2, 3] or array repeat [val; count]
                self.advance(); // [
                if self.match_token(&Token::CloseBracket) {
                    return Ok(Expr::Array(Vec::new()));
                }
                let first = self.parse_expr()?;
                if self.match_token(&Token::Semicolon) {
                    let count = self.parse_expr()?;
                    self.expect(&Token::CloseBracket)?;
                    return Ok(Expr::ArrayRepeat {
                        value: Box::new(first),
                        count: Box::new(count),
                    });
                }
                let mut elements = vec![first];
                while self.match_token(&Token::Comma) {
                    if self.check(&Token::CloseBracket) {
                        break;
                    }
                    elements.push(self.parse_expr()?);
                }
                self.expect(&Token::CloseBracket)?;
                Ok(Expr::Array(elements))
            }
            Token::Ident(name) => {
                let id_span = self.current_span();
                let id = name.clone();
                self.advance();

                // Check for generic type arguments: e.g. LinearGuard<T> { ... } or call<T>(...)
                let mut full_id = id.clone();
                if self.check(&Token::Less) {
                    let mut lookahead = 1;
                    let mut depth = 1;
                    while lookahead < 20 {
                        match self.peek_offset(lookahead) {
                            Token::Less => depth += 1,
                            Token::Greater => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            Token::Eof | Token::Semicolon => break,
                            _ => {}
                        }
                        lookahead += 1;
                    }
                    if depth == 0 && (self.peek_offset(lookahead + 1) == &Token::OpenBrace || self.peek_offset(lookahead + 1) == &Token::OpenParen) {
                        self.advance(); // <
                        full_id.push('<');
                        let mut first = true;
                        while !self.check(&Token::Greater) && !self.check(&Token::Eof) {
                            if !first {
                                full_id.push_str(", ");
                            }
                            first = false;
                            let t = self.parse_type_str()?;
                            full_id.push_str(&t);
                            self.match_token(&Token::Comma);
                        }
                        self.expect(&Token::Greater)?;
                        full_id.push('>');
                    }
                }

                if self.match_token(&Token::OpenParen) {
                    // Function Call
                    let mut args = Vec::new();
                    while !self.check(&Token::CloseParen) && !self.check(&Token::Eof) {
                        let arg = self.parse_call_arg()?;
                        args.push(arg);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    self.expect(&Token::CloseParen)?;
                    Ok(Expr::Call {
                        callee: full_id,
                        args,
                    })
                } else if self.check(&Token::OpenBrace) && (
                        (matches!(self.peek_offset(1), Token::Ident(_) | Token::Fallback | Token::Region | Token::Type | Token::Export)
                            && matches!(self.peek_offset(2), Token::Colon)
                            && self.peek_offset(3) != &Token::Colon)
                        || matches!(self.peek_offset(1), Token::CloseBrace)
                    ) {
                    // Struct initialization: StructName { field: val, ... }
                    self.advance(); // consume OpenBrace
                    let mut fields = Vec::new();
                    while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
                        let f_name = match self.advance() {
                            Token::Ident(n) => n,
                            Token::Fallback => "fallback".to_string(),
                            Token::Region => "region".to_string(),
                            Token::Type => "type".to_string(),
                            Token::Export => "export".to_string(),
                            Token::From => "from".to_string(),
                            Token::As => "as".to_string(),
                            Token::Lin => "lin".to_string(),
                            other => return Err(format!("Expected field name in struct {}, got {:?}", id, other)),
                        };
                        self.expect(&Token::Colon)?;
                        let f_val = self.parse_expr()?;
                        fields.push((f_name, f_val));
                        self.match_token(&Token::Comma);
                        self.match_token(&Token::Semicolon);
                    }
                    self.expect(&Token::CloseBrace)?;
                    Ok(Expr::StructInit {
                        struct_name: full_id,
                        fields,
                    })
                } else {
                    Ok(Expr::Ident(id, id_span))
                }
            }
            Token::Fallback | Token::Region | Token::Type | Token::Export => {
                let id_span = self.current_span();
                let name = match self.advance() {
                    Token::Fallback => "fallback",
                    Token::Region => "region",
                    Token::Type => "type",
                    Token::Export => "export",
                    _ => unreachable!(),
                }.to_string();
                Ok(Expr::Ident(name, id_span))
            }
            other => {
                println!("Unexpected token in expression at pos {}: {:?}", self.pos, other);
                println!("Previous token: {:?}", if self.pos > 0 { &self.tokens[self.pos - 1].value } else { &Token::Eof });
                println!("Next token: {:?}", if self.pos + 1 < self.tokens.len() { &self.tokens[self.pos + 1].value } else { &Token::Eof });
                Err(format!("Unexpected token in expression at pos {}: {:?}", self.pos, other))
            }
        }
    }

    fn parse_block_expr(&mut self) -> Result<Expr, String> {
        let mut last_expr = Expr::LiteralInt(0);
        while !self.check(&Token::CloseBrace) && !self.check(&Token::Eof) {
            if self.match_token(&Token::Return) {
                if !self.check(&Token::CloseBrace) && !self.check(&Token::Semicolon) {
                    last_expr = self.parse_expr()?;
                }
                self.match_token(&Token::Semicolon);
                break;
            }
            if self.check(&Token::Let) {
                let _ = self.parse_statement()?;
            } else {
                last_expr = self.parse_expr()?;
                self.match_token(&Token::Semicolon);
            }
        }
        Ok(last_expr)
    }
}
