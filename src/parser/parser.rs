// src/parser/parser.rs - Implementación del Parser de AXIOM

use crate::lexer::{Lexer, Token, TokenWithSpan, Span};
use crate::parser::ast::*;
use std::iter::Peekable;

/// Error del parser
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

/// Resultado del parser
pub type ParseResult<T> = Result<T, ParseError>;

/// El parser de AXIOM
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    current: Option<TokenWithSpan>,
}

impl<'a> Parser<'a> {
    /// Crea un nuevo parser
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        let mut peeking = lexer.peekable();
        let current = peeking.next();
        Self {
            lexer: peeking,
            current,
        }
    }

    /// Avanza al siguiente token
    fn advance(&mut self) -> Option<TokenWithSpan> {
        let current = self.current.take();
        self.current = self.lexer.next();
        current
    }

    /// Mira el token actual sin consumirlo
    fn peek(&self) -> Option<&TokenWithSpan> {
        self.current.as_ref()
    }

    /// Verifica si el token actual es el esperado
    fn check(&self, expected: Token) -> bool {
        self.peek()
            .map(|t| t.token == expected)
            .unwrap_or(false)
    }

    /// Espera un token específico o devuelve error
    fn expect(&mut self, expected: Token) -> ParseResult<TokenWithSpan> {
        if let Some(token) = self.peek() {
            if token.token == expected {
                return Ok(self.advance().unwrap());
            }
        }
        Err(ParseError {
            message: format!("Se esperaba {:?}, encontrado {:?}", expected, self.peek().map(|t| &t.token)),
            span: self.peek().map(|t| t.span).unwrap_or(Span::new(0, 0, 0, 0)),
        })
    }

    /// Espera un identificador
    fn expect_ident(&mut self) -> ParseResult<String> {
        if let Some(token) = self.peek() {
            if let Token::Ident(name) = &token.token {
                let name = name.clone();
                self.advance();
                return Ok(name);
            }
        }
        Err(ParseError {
            message: format!("Se esperaba un identificador, encontrado {:?}", self.peek().map(|t| &t.token)),
            span: self.peek().map(|t| t.span).unwrap_or(Span::new(0, 0, 0, 0)),
        })
    }

    fn check_ident(&self, expected: &str) -> bool {
        matches!(self.peek().map(|token| &token.token), Some(Token::Ident(name)) if name == expected)
    }

    fn expect_ident_value(&mut self, expected: &str) -> ParseResult<String> {
        if self.check_ident(expected) {
            return self.expect_ident();
        }
        Err(ParseError {
            message: format!("Se esperaba '{}', encontrado {:?}", expected, self.peek().map(|t| &t.token)),
            span: self.peek().map(|t| t.span).unwrap_or(Span::new(0, 0, 0, 0)),
        })
    }

    fn skip_trivia(&mut self) {
        while matches!(self.peek().map(|token| &token.token), Some(Token::Whitespace | Token::Newline | Token::Comment(_) | Token::DocComment(_))) {
            self.advance();
        }
    }

    /// Espera un token y devuelve su span
    fn expect_span(&mut self, expected: Token) -> ParseResult<Span> {
        let token = self.expect(expected)?;
        Ok(token.span)
    }

    /// Parsea un programa completo
    pub fn parse_program(&mut self) -> ParseResult<Program> {
        self.skip_trivia();
        let mut items = Vec::new();
        let start_span = self.peek().map(|t| t.span).unwrap_or(Span::new(0, 0, 0, 0));
        
        loop {
            self.skip_trivia();
            let Some(token) = self.peek() else { break; };
            if matches!(token.token, Token::EOF) {
                break;
            }
            items.push(self.parse_item()?);
        }
        
        let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
        Ok(Program {
            items,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea un item
    fn parse_item(&mut self) -> ParseResult<Item> {
        match self.peek() {
            Some(token) => match &token.token {
                Token::Fn => self.parse_function().map(Item::Function),
                Token::Struct => self.parse_struct().map(Item::Struct),
                Token::Enum => self.parse_enum().map(Item::Enum),
                Token::Trait => self.parse_trait().map(Item::Trait),
                Token::Impl => self.parse_impl().map(Item::Impl),
                Token::Use => self.parse_use().map(Item::Use),
                Token::Mod => self.parse_mod().map(Item::Mod),
                Token::Const => self.parse_const().map(Item::Const),
                Token::Type => self.parse_type_alias().map(Item::Type),
                _ => {
                    let expr = self.parse_expr(0)?;
                    Ok(Item::Expr(expr))
                }
            },
            None => Err(ParseError {
                message: "Fin de archivo inesperado".to_string(),
                span: Span::new(0, 0, 0, 0),
            }),
        }
    }

    /// Parsea una función
    fn parse_function(&mut self) -> ParseResult<Function> {
        let start_span = self.expect_span(Token::Fn)?;
        let name = self.expect_ident()?;
        
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        
        if !self.check(Token::RParen) {
            loop {
                let param_name = self.expect_ident()?;
                self.expect(Token::Colon)?;
                let param_type = self.parse_type()?;
                params.push(Param {
                    name: param_name,
                    ty: param_type,
                    span: Span::new(0, 0, 0, 0), // TODO: calcular span
                });
                
                if self.check(Token::Comma) {
                    self.advance();
                    continue;
                }
                break;
            }
        }
        self.expect(Token::RParen)?;
        
        let return_type = if self.check(Token::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        let body = self.parse_block()?;
        
        let end_span = body.span;
        Ok(Function {
            name,
            params,
            return_type,
            body,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea una estructura
    fn parse_struct(&mut self) -> ParseResult<Struct> {
        let start_span = self.expect_span(Token::Struct)?;
        let name = self.expect_ident()?;
        
        self.expect(Token::LBrace)?;
        let mut fields = Vec::new();
        
        while !self.check(Token::RBrace) {
            let field_name = self.expect_ident()?;
            self.expect(Token::Colon)?;
            let field_type = self.parse_type()?;
            fields.push(Field {
                name: field_name,
                ty: field_type,
                span: Span::new(0, 0, 0, 0),
            });
            
            if self.check(Token::Comma) {
                self.advance();
            }
        }
        let end_span = self.expect_span(Token::RBrace)?;
        
        Ok(Struct {
            name,
            fields,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea un enum
    fn parse_enum(&mut self) -> ParseResult<Enum> {
        let start_span = self.expect_span(Token::Enum)?;
        let name = self.expect_ident()?;
        
        self.expect(Token::LBrace)?;
        let mut variants = Vec::new();
        
        while !self.check(Token::RBrace) {
            let variant_name = self.expect_ident()?;
            let mut fields = Vec::new();
            
            if self.check(Token::LParen) {
                self.advance();
                while !self.check(Token::RParen) {
                    fields.push(self.parse_type()?);
                    if self.check(Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(Token::RParen)?;
            }
            
            variants.push(Variant {
                name: variant_name,
                fields,
                span: Span::new(0, 0, 0, 0),
            });
            
            if self.check(Token::Comma) {
                self.advance();
            }
        }
        let end_span = self.expect_span(Token::RBrace)?;
        
        Ok(Enum {
            name,
            variants,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea un trait
    fn parse_trait(&mut self) -> ParseResult<Trait> {
        let start_span = self.expect_span(Token::Trait)?;
        let name = self.expect_ident()?;
        
        self.expect(Token::LBrace)?;
        let mut methods = Vec::new();
        
        while !self.check(Token::RBrace) {
            if self.check(Token::Fn) {
                methods.push(self.parse_function()?);
            } else {
                return Err(ParseError {
                    message: "Se esperaba una función en el trait".to_string(),
                    span: self.peek().map(|t| t.span).unwrap_or(Span::new(0, 0, 0, 0)),
                });
            }
        }
        let end_span = self.expect_span(Token::RBrace)?;
        
        Ok(Trait {
            name,
            methods,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea una implementación
    fn parse_impl(&mut self) -> ParseResult<Impl> {
        let start_span = self.expect_span(Token::Impl)?;
        
        let trait_name = if self.peek().map(|t| matches!(t.token, Token::Ident(_))).unwrap_or(false) {
            let name = self.expect_ident()?;
            self.expect(Token::For)?;
            Some(name)
        } else {
            None
        };
        
        let type_name = self.expect_ident()?;
        self.expect(Token::LBrace)?;
        let mut methods = Vec::new();
        
        while !self.check(Token::RBrace) {
            if self.check(Token::Fn) {
                methods.push(self.parse_function()?);
            } else {
                return Err(ParseError {
                    message: "Se esperaba una función en el impl".to_string(),
                    span: self.peek().map(|t| t.span).unwrap_or(Span::new(0, 0, 0, 0)),
                });
            }
        }
        let end_span = self.expect_span(Token::RBrace)?;
        
        Ok(Impl {
            trait_name,
            type_name,
            methods,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea una importación
    fn parse_use(&mut self) -> ParseResult<Use> {
        let start_span = self.expect_span(Token::Use)?;
        let mut path_parts = Vec::new();
        
        loop {
            let name = self.expect_ident()?;
            path_parts.push(name);
            
            if self.check(Token::ColonColon) {
                self.advance();
                continue;
            }
            break;
        }
        
        let path = path_parts.join("::");
        let alias = if self.check_ident("as") {
            self.advance();
            Some(self.expect_ident()?)
        } else {
            None
        };
        
        let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
        Ok(Use {
            path,
            alias,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea un módulo
    fn parse_mod(&mut self) -> ParseResult<Mod> {
        let start_span = self.expect_span(Token::Mod)?;
        let name = self.expect_ident()?;
        
        let items = if self.check(Token::LBrace) {
            self.advance();
            let mut items = Vec::new();
            while !self.check(Token::RBrace) {
                items.push(self.parse_item()?);
            }
            self.expect(Token::RBrace)?;
            items
        } else {
            Vec::new()
        };
        
        let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
        Ok(Mod {
            name,
            items,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea una constante
    fn parse_const(&mut self) -> ParseResult<Const> {
        let start_span = self.expect_span(Token::Const)?;
        let name = self.expect_ident()?;
        
        let ty = if self.check(Token::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect(Token::Eq)?;
        let value = self.parse_expr(0)?;
        
        let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
        Ok(Const {
            name,
            ty,
            value,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea un type alias
    fn parse_type_alias(&mut self) -> ParseResult<TypeAlias> {
        let start_span = self.expect_span(Token::Type)?;
        let name = self.expect_ident()?;
        self.expect(Token::Eq)?;
        let ty = self.parse_type()?;
        
        let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
        Ok(TypeAlias {
            name,
            ty,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea un bloque
    fn parse_block(&mut self) -> ParseResult<Block> {
        let start_span = self.expect_span(Token::LBrace)?;
        let mut stmts = Vec::new();
        
        while !self.check(Token::RBrace) {
            self.skip_trivia();
            if self.check(Token::RBrace) { break; }
            stmts.push(self.parse_stmt()?);
        }
        let end_span = self.expect_span(Token::RBrace)?;
        
        Ok(Block {
            stmts,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    /// Parsea una sentencia
    fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        match self.peek() {
            Some(token) => match &token.token {
                Token::Let => {
                    let start_span = self.expect_span(Token::Let)?;
                    let mutable = self.check(Token::Mut);
                    if mutable {
                        self.advance();
                    }
                    let name = self.expect_ident()?;
                    let ty = if self.check(Token::Colon) {
                        self.advance();
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    
                    if self.check(Token::Eq) {
                        self.advance();
                        let value = self.parse_expr(0)?;
                        let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
                        Ok(Stmt::Let(Let {
                            name,
                            mutable,
                            ty,
                            value,
                            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
                        }, Span::new(start_span.start, end_span.end, start_span.line, start_span.column)))
                    } else {
                        Err(ParseError {
                            message: "Se esperaba '=' en la declaración let".to_string(),
                            span: self.peek().map(|t| t.span).unwrap_or(start_span),
                        })
                    }
                }
                _ => {
                    let expr = self.parse_expr(0)?;
                    let span = expr.span();
                    // Opcionalmente esperar ';'
                    if self.check(Token::Semicolon) {
                        self.advance();
                    }
                    Ok(Stmt::Expr(expr, span))
                }
            },
            None => Err(ParseError {
                message: "Fin de archivo inesperado".to_string(),
                span: Span::new(0, 0, 0, 0),
            }),
        }
    }

    /// Parsea un tipo
    fn parse_type(&mut self) -> ParseResult<Type> {
        let start_span = self.peek().map(|t| t.span).unwrap_or(Span::new(0, 0, 0, 0));
        
        match self.peek() {
            Some(token) => match &token.token {
                Token::Ident(name) => {
                    let name = name.clone();
                    self.advance();
                    
                    // Array type: [T; N]
                    if self.check(Token::LBracket) {
                        self.advance();
                        let elem_type = self.parse_type()?;
                        self.expect(Token::Semicolon)?;
                        let size = self.parse_expr(0)?;
                        self.expect(Token::RBracket)?;
                        let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
                        return Ok(Type::Array(Box::new(elem_type), Box::new(size), 
                            Span::new(start_span.start, end_span.end, start_span.line, start_span.column)));
                    }
                    
                    // Slice type: [T]
                    if self.check(Token::LBracket) && self.peek_two().map(|t| t.token == Token::RBracket).unwrap_or(false) {
                        self.advance();
                        let elem_type = self.parse_type()?;
                        self.expect(Token::RBracket)?;
                        let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
                        return Ok(Type::Slice(Box::new(elem_type),
                            Span::new(start_span.start, end_span.end, start_span.line, start_span.column)));
                    }
                    
                    // Generic type
                    if self.check(Token::Lt) {
                        self.advance();
                        let mut params = Vec::new();
                        while !self.check(Token::Gt) {
                            params.push(self.parse_type()?);
                            if self.check(Token::Comma) {
                                self.advance();
                            }
                        }
                        self.expect(Token::Gt)?;
                        let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
                        return Ok(Type::Generic(Box::new(Type::Ident(name, start_span)), params,
                            Span::new(start_span.start, end_span.end, start_span.line, start_span.column)));
                    }
                    
                    let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
                    Ok(Type::Ident(name, Span::new(start_span.start, end_span.end, start_span.line, start_span.column)))
                }
                Token::LParen => {
                    self.advance();
                    let mut types = Vec::new();
                    while !self.check(Token::RParen) {
                        types.push(self.parse_type()?);
                        if self.check(Token::Comma) {
                            self.advance();
                        }
                    }
                    let end_span = self.expect_span(Token::RParen)?;
                    Ok(Type::Tuple(types, Span::new(start_span.start, end_span.end, start_span.line, start_span.column)))
                }
                _ => {
                    Err(ParseError {
                        message: "Tipo esperado".to_string(),
                        span: token.span,
                    })
                }
            },
            None => Err(ParseError {
                message: "Tipo esperado".to_string(),
                span: Span::new(0, 0, 0, 0),
            }),
        }
    }

    /// Parsea una expresión (con precedencia)
    fn parse_expr(&mut self, min_prec: u8) -> ParseResult<Expr> {
        let mut left = self.parse_primary()?;
        
        while let Some((op, prec)) = self.peek_binary_op() {
            if prec < min_prec {
                break;
            }
            self.advance();
            
            let right = self.parse_expr(prec + 1)?;
            let span = Span::new(
                left.span().start,
                right.span().end,
                left.span().line,
                left.span().column,
            );
            left = Expr::Binary(op, Box::new(left), Box::new(right), span);
        }
        
        Ok(left)
    }

    /// Parsea una expresión primaria
    fn parse_primary(&mut self) -> ParseResult<Expr> {
        match self.peek() {
            Some(token) => match &token.token {
                Token::IntLiteral(val) => {
                    let val = val.parse().map_err(|_| ParseError {
                        message: format!("Número inválido: {}", val),
                        span: token.span,
                    })?;
                    let span = self.advance().unwrap().span;
                    Ok(Expr::Int(val, span))
                }
                Token::FloatLiteral(val) => {
                    let val = val.parse().map_err(|_| ParseError {
                        message: format!("Número flotante inválido: {}", val),
                        span: token.span,
                    })?;
                    let span = self.advance().unwrap().span;
                    Ok(Expr::Float(val, span))
                }
                Token::StringLiteral(val) => {
                    let val = val.clone();
                    let span = self.advance().unwrap().span;
                    Ok(Expr::String(val, span))
                }
                Token::CharLiteral(val) => {
                    let val = *val;
                    let span = self.advance().unwrap().span;
                    Ok(Expr::Char(val, span))
                }
                Token::True => {
                    let span = self.advance().unwrap().span;
                    Ok(Expr::Bool(true, span))
                }
                Token::False => {
                    let span = self.advance().unwrap().span;
                    Ok(Expr::Bool(false, span))
                }
                Token::Null => {
                    let span = self.advance().unwrap().span;
                    Ok(Expr::None(span))
                }
                Token::Ident(name) => {
                    let name = name.clone();
                    let span = self.advance().unwrap().span;
                    
                    // Llamada a función: ident(...)
                    if self.check(Token::LParen) {
                        self.advance();
                        let mut args = Vec::new();
                        while !self.check(Token::RParen) {
                            args.push(self.parse_expr(0)?);
                            if self.check(Token::Comma) {
                                self.advance();
                            }
                        }
                        let end_span = self.expect_span(Token::RParen)?;
                        return Ok(Expr::Call(
                            Box::new(Expr::Ident(name, span)),
                            args,
                            Span::new(span.start, end_span.end, span.line, span.column),
                        ));
                    }
                    
                    Ok(Expr::Ident(name, span))
                }
                Token::LParen => {
                    self.advance();
                    let expr = self.parse_expr(0)?;
                    self.expect(Token::RParen)?;
                    Ok(expr)
                }
                Token::LBrace => {
                    let block = self.parse_block()?;
                    Ok(Expr::Block(block.stmts, block.span))
                }
                Token::If => {
                    self.parse_if_expr()
                }
                Token::While => {
                    let start_span = self.expect_span(Token::While)?;
                    let condition = self.parse_expr(0)?;
                    let body = self.parse_block()?;
                    let end = body.span.end;
                    Ok(Expr::While(
                        Box::new(condition),
                        Box::new(body),
                        Span::new(start_span.start, end, start_span.line, start_span.column),
                    ))
                }
                Token::For => {
                    let start_span = self.expect_span(Token::For)?;
                    let name = self.expect_ident()?;
                    self.expect_ident_value("in")?;
                    let iterable = self.parse_expr(0)?;
                    let body = self.parse_block()?;
                    let end = body.span.end;
                    Ok(Expr::For(
                        name,
                        Box::new(iterable),
                        Box::new(body),
                        Span::new(start_span.start, end, start_span.line, start_span.column),
                    ))
                }
                Token::Loop => {
                    let start_span = self.expect_span(Token::Loop)?;
                    let body = self.parse_block()?;
                    let end = body.span.end;
                    Ok(Expr::Loop(
                        Box::new(body),
                        Span::new(start_span.start, end, start_span.line, start_span.column),
                    ))
                }
                Token::Return => {
                    let start_span = self.advance().unwrap().span;
                    let value = if !self.check(Token::Semicolon) && !self.check(Token::RBrace) && !self.check(Token::EOF) {
                        Some(Box::new(self.parse_expr(0)?))
                    } else {
                        None
                    };
                    let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
                    Ok(Expr::Return(
                        value,
                        Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
                    ))
                }
                Token::Break => {
                    let start_span = self.advance().unwrap().span;
                    let value = if !self.check(Token::Semicolon) && !self.check(Token::RBrace) && !self.check(Token::EOF) {
                        Some(Box::new(self.parse_expr(0)?))
                    } else {
                        None
                    };
                    let end_span = self.peek().map(|t| t.span).unwrap_or(start_span);
                    Ok(Expr::Break(
                        value,
                        Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
                    ))
                }
                Token::Continue => {
                    let span = self.advance().unwrap().span;
                    Ok(Expr::Continue(span))
                }
                Token::Match => {
                    self.parse_match_expr()
                }
                Token::Await => {
                    let start_span = self.expect_span(Token::Await)?;
                    let expr = self.parse_expr(0)?;
                    let end = expr.span().end;
                    Ok(Expr::Await(
                        Box::new(expr),
                        Span::new(start_span.start, end, start_span.line, start_span.column),
                    ))
                }
                _ => {
                    Err(ParseError {
                        message: format!("Expresión inesperada: {:?}", token.token),
                        span: token.span,
                    })
                }
            },
            None => {
                Err(ParseError {
                    message: "Fin de archivo inesperado en expresión".to_string(),
                    span: Span::new(0, 0, 0, 0),
                })
            }
        }
    }

    /// Parsea una expresión if
    fn parse_if_expr(&mut self) -> ParseResult<Expr> {
        let start_span = self.expect_span(Token::If)?;
        let condition = self.parse_expr(0)?;
        let then_branch = self.parse_block()?;
        
        let else_branch = if self.check(Token::Else) {
            self.advance();
            if self.check(Token::If) {
                Some(Box::new(self.parse_if_expr()?))
            } else {
                Some(Box::new(Expr::Block(self.parse_block()?.stmts, then_branch.span)))
            }
        } else {
            None
        };
        
        let end_span = if let Some(else_expr) = &else_branch {
            else_expr.span()
        } else {
            then_branch.span
        };
        
        Ok(Expr::If(
            Box::new(condition),
            Box::new(then_branch),
            else_branch,
            Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        ))
    }

    /// Parsea una expresión match
    fn parse_match_expr(&mut self) -> ParseResult<Expr> {
        let start_span = self.expect_span(Token::Match)?;
        let value = self.parse_expr(0)?;
        self.expect(Token::LBrace)?;
        let mut arms = Vec::new();
        
        while !self.check(Token::RBrace) {
            let pattern = self.parse_pattern()?;
            let guard = if self.check(Token::If) {
                self.advance();
                Some(self.parse_expr(0)?)
            } else {
                None
            };
            self.expect(Token::FatArrow)?;
            let body = self.parse_expr(0)?;
            if self.check(Token::Comma) {
                self.advance();
            }
            arms.push(MatchArm {
                pattern,
                guard,
                body,
                span: Span::new(0, 0, 0, 0),
            });
        }
        let end_span = self.expect_span(Token::RBrace)?;
        
        Ok(Expr::Match(
            Box::new(value),
            arms,
            Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        ))
    }

    /// Parsea un patrón
    fn parse_pattern(&mut self) -> ParseResult<Pattern> {
        let token = self.advance().ok_or(ParseError {
            message: "Patrón esperado".to_string(),
            span: Span::new(0, 0, 0, 0),
        })?;
        let span = token.span;
        match token.token {
            Token::Ident(name) if name == "_" => Ok(Pattern::Wildcard(span)),
            Token::Ident(name) => Ok(Pattern::Ident(name, span)),
            Token::IntLiteral(value) => value.parse().map(|value| Pattern::Int(value, span)).map_err(|_| ParseError {
                message: format!("Número inválido en patrón: {}", value), span,
            }),
            Token::LParen => {
                let mut patterns = Vec::new();
                while !self.check(Token::RParen) {
                    patterns.push(self.parse_pattern()?);
                    if self.check(Token::Comma) { self.advance(); } else { break; }
                }
                self.expect(Token::RParen)?;
                Ok(Pattern::Tuple(patterns, span))
            }
            token => Err(ParseError { message: format!("Patrón inesperado: {:?}", token), span }),
        }
    }

    /// Verifica el siguiente operador binario
    fn peek_binary_op(&mut self) -> Option<(BinaryOp, u8)> {
        match self.peek() {
            Some(token) => match &token.token {
                Token::Plus => Some((BinaryOp::Add, 10)),
                Token::Minus => Some((BinaryOp::Sub, 10)),
                Token::Star => Some((BinaryOp::Mul, 20)),
                Token::Slash => Some((BinaryOp::Div, 20)),
                Token::Percent => Some((BinaryOp::Mod, 20)),
                Token::EqEq => Some((BinaryOp::Eq, 5)),
                Token::NotEq => Some((BinaryOp::Neq, 5)),
                Token::Lt => Some((BinaryOp::Lt, 8)),
                Token::Gt => Some((BinaryOp::Gt, 8)),
                Token::LtEq => Some((BinaryOp::Leq, 8)),
                Token::GtEq => Some((BinaryOp::Geq, 8)),
                Token::AndAnd => Some((BinaryOp::And, 4)),
                Token::OrOr => Some((BinaryOp::Or, 3)),
                Token::And => Some((BinaryOp::BitAnd, 7)),
                Token::Or => Some((BinaryOp::BitOr, 6)),
                _ => None,
            },
            None => None,
        }
    }

    /// Método auxiliar para peek_two
    fn peek_two(&mut self) -> Option<&TokenWithSpan> {
        // Esta es una implementación simple
        // En un parser real, necesitarías un buffer de tokens
        self.peek()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function() {
        let input = "fn add(a: Int, b: Int) -> Int { return a + b }";
        let mut parser = Parser::new(input);
        let program = parser.parse_program().unwrap();
        
        assert_eq!(program.items.len(), 1);
        match &program.items[0] {
            Item::Function(func) => {
                assert_eq!(func.name, "add");
                assert_eq!(func.params.len(), 2);
                assert!(func.return_type.is_some());
            }
            _ => panic!("Se esperaba una función"),
        }
    }

    #[test]
    fn test_parse_struct() {
        let input = "struct Point { x: Float, y: Float }";
        let mut parser = Parser::new(input);
        let program = parser.parse_program().unwrap();
        
        assert_eq!(program.items.len(), 1);
        match &program.items[0] {
            Item::Struct(s) => {
                assert_eq!(s.name, "Point");
                assert_eq!(s.fields.len(), 2);
            }
            _ => panic!("Se esperaba una estructura"),
        }
    }

    #[test]
    fn test_parse_if() {
        let input = "if x > 0 { return 1 } else { return 0 }";
        let mut parser = Parser::new(input);
        let expr = parser.parse_expr(0).unwrap();
        
        match expr {
            Expr::If(_, _, _, _) => {}
            _ => panic!("Se esperaba una expresión if"),
        }
    }

    #[test]
    fn test_parse_binary() {
        let input = "1 + 2 * 3";
        let mut parser = Parser::new(input);
        let expr = parser.parse_expr(0).unwrap();
        
        match expr {
            Expr::Binary(op, left, right, _) => {
                assert_eq!(op, BinaryOp::Add);
                match *left {
                    Expr::Int(1, _) => {}
                    _ => panic!("Se esperaba 1"),
                }
                match *right {
                    Expr::Binary(op, left, right, _) => {
                        assert_eq!(op, BinaryOp::Mul);
                        match *left {
                            Expr::Int(2, _) => {}
                            _ => panic!("Se esperaba 2"),
                        }
                        match *right {
                            Expr::Int(3, _) => {}
                            _ => panic!("Se esperaba 3"),
                        }
                    }
                    _ => panic!("Se esperaba una multiplicación"),
                }
            }
            _ => panic!("Se esperaba una expresión binaria"),
        }
    }
}