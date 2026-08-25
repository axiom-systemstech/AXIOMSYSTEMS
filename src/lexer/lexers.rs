// src/lexer/lexer.rs - Implementación del Lexer de AXIOM

use crate::lexer::token::{Token, TokenWithSpan, Span};
use std::iter::Peekable;
use std::str::Chars;

/// El lexer de AXIOM - Convierte texto en tokens según la especificación
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    position: usize,
    line: usize,
    column: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.peek().copied();
        Self {
            input,
            chars,
            position: 0,
            line: 1,
            column: 0,
            current_char,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.current_char;
        self.position += 1;
        self.column += 1;
        
        if let Some(c) = c {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            }
        }
        
        self.current_char = self.chars.next();
        c
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn next_token(&mut self) -> TokenWithSpan {
        self.skip_whitespace();
        
        let start_pos = self.position;
        let start_line = self.line;
        let start_col = self.column;
        
        let token = match self.current_char {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let ident = self.collect_identifier();
                self.ident_to_keyword(ident)
            }
            Some(c) if c.is_ascii_digit() => self.collect_number(),
            Some('"') => { self.advance(); self.collect_string() }
            Some('\'') => { self.advance(); self.collect_char() }
            Some('+') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::PlusEq
                } else {
                    Token::Plus
                }
            }
            Some('-') => {
                self.advance();
                if self.current_char == Some('>') {
                    self.advance();
                    Token::Arrow
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::MinusEq
                } else {
                    Token::Minus
                }
            }
            Some('*') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::StarEq
                } else {
                    Token::Star
                }
            }
            Some('/') => {
                self.advance();
                if self.current_char == Some('/') {
                    self.advance();
                    if self.current_char == Some('/') {
                        self.advance();
                        Token::DocComment(self.collect_comment())
                    } else {
                        Token::Comment(self.collect_comment())
                    }
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::SlashEq
                } else {
                    Token::Slash
                }
            }
            Some('%') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::PercentEq
                } else {
                    Token::Percent
                }
            }
            Some('=') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::EqEq
                } else if self.current_char == Some('>') {
                    self.advance();
                    Token::FatArrow
                } else {
                    Token::Eq
                }
            }
            Some('!') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::NotEq
                } else {
                    Token::Not
                }
            }
            Some('<') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            Some('>') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            Some('&') => {
                self.advance();
                if self.current_char == Some('&') {
                    self.advance();
                    Token::AndAnd
                } else {
                    Token::And
                }
            }
            Some('|') => {
                self.advance();
                if self.current_char == Some('|') {
                    self.advance();
                    Token::OrOr
                } else {
                    Token::Or
                }
            }
            Some('.') => {
                self.advance();
                if self.current_char == Some('.') {
                    self.advance();
                    if self.current_char == Some('.') {
                        self.advance();
                        Token::DotDotDot
                    } else {
                        Token::DotDot
                    }
                } else {
                    Token::Dot
                }
            }
            Some(',') => { self.advance(); Token::Comma }
            Some(';') => { self.advance(); Token::Semicolon }
            Some(':') => {
                self.advance();
                if self.current_char == Some(':') {
                    self.advance();
                    Token::ColonColon
                } else {
                    Token::Colon
                }
            }
            Some('#') => { self.advance(); Token::Pound }
            Some('@') => { self.advance(); Token::At }
            Some('$') => { self.advance(); Token::Dollar }
            Some('(') => { self.advance(); Token::LParen }
            Some(')') => { self.advance(); Token::RParen }
            Some('{') => { self.advance(); Token::LBrace }
            Some('}') => { self.advance(); Token::RBrace }
            Some('[') => { self.advance(); Token::LBracket }
            Some(']') => { self.advance(); Token::RBracket }
            None => Token::EOF,
            Some(c) => {
                self.advance();
                Token::Error(format!("Carácter inesperado: '{}'", c))
            }
        };
        
        let span = Span::new(start_pos, self.position, start_line, start_col);
        TokenWithSpan::new(token, span)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn collect_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.current_char {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn ident_to_keyword(&mut self, ident: String) -> Token {
        match ident.as_str() {
            // Declaraciones
            "fn" => Token::Fn,
            "let" => Token::Let,
            "mut" => Token::Mut,
            "const" => Token::Const,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "union" => Token::Union,
            "trait" => Token::Trait,
            "impl" => Token::Impl,
            "type" => Token::Type,
            "mod" => Token::Mod,
            "use" => Token::Use,
            "pub" => Token::Pub,
            "crate" => Token::Crate,
            "import" => Token::Import,
            "export" => Token::Export,
            "new" => Token::New,
            "self" => Token::SelfKeyword,
            "super" => Token::SuperKeyword,
            
            // Control de flujo
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "loop" => Token::Loop,
            "match" => Token::Match,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            
            // Concurrencia
            "async" => Token::Async,
            "await" => Token::Await,
            "spawn" => Token::Spawn,
            "channel" => Token::Channel,
            "select" => Token::Select,
            
            // Visibilidad
            "pub(crate)" => Token::PubCrate,
            "pub(super)" => Token::PubSuper,
            
            // Especiales
            "extern" => Token::Extern,
            "unsafe" => Token::Unsafe,
            "move" => Token::Move,
            "static" => Token::Static,
            "abstract" => Token::Abstract,
            "final" => Token::Final,
            "override" => Token::Override,
            
            // Manejo de errores
            "try" => Token::Try,
            "catch" => Token::Catch,
            "throw" => Token::Throw,
            "finally" => Token::Finally,
            
            // Literales
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            
            _ => Token::Ident(ident),
        }
    }

    fn collect_number(&mut self) -> Token {
        let mut num = String::new();
        let mut is_float = false;
        let mut has_exponent = false;
        let mut has_hex = false;
        
        if self.current_char == Some('0') {
            let next = self.peek();
            if next == Some('x') || next == Some('X') {
                has_hex = true;
                num.push('0');
                self.advance();
                if let Some(c) = self.current_char {
                    num.push(c);
                    self.advance();
                }
            }
        }
        
        while let Some(c) = self.current_char {
            if c.is_ascii_digit() || (has_hex && c.is_ascii_hexdigit()) {
                num.push(c);
                self.advance();
            } else if c == '.' && !is_float && !has_hex {
                is_float = true;
                num.push(c);
                self.advance();
            } else if (c == 'e' || c == 'E') && !has_exponent && !has_hex {
                has_exponent = true;
                num.push(c);
                self.advance();
                if self.current_char == Some('+') || self.current_char == Some('-') {
                    if let Some(sign) = self.current_char {
                        num.push(sign);
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }
        
        if is_float || has_exponent {
            Token::FloatLiteral(num)
        } else {
            Token::IntLiteral(num)
        }
    }

    fn collect_string(&mut self) -> Token {
        let mut s = String::new();
        
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance();
                break;
            }
            if c == '\\' {
                self.advance();
                if let Some(next) = self.current_char {
                    match next {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        'r' => s.push('\r'),
                        '0' => s.push('\0'),
                        '\\' => s.push('\\'),
                        '"' => s.push('"'),
                        '\'' => s.push('\''),
                        'x' => {
                            self.advance();
                            let mut hex = String::new();
                            for _ in 0..2 {
                                if let Some(h) = self.current_char {
                                    if h.is_ascii_hexdigit() {
                                        hex.push(h);
                                        self.advance();
                                    }
                                }
                            }
                            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                s.push(byte as char);
                            }
                        }
                        'u' => {
                            self.advance();
                            if self.current_char == Some('{') {
                                self.advance();
                                let mut hex = String::new();
                                while let Some(h) = self.current_char {
                                    if h == '}' {
                                        self.advance();
                                        break;
                                    }
                                    if h.is_ascii_hexdigit() {
                                        hex.push(h);
                                        self.advance();
                                    }
                                }
                                if let Ok(codepoint) = u32::from_str_radix(&hex, 16) {
                                    if let Some(ch) = char::from_u32(codepoint) {
                                        s.push(ch);
                                    }
                                }
                            }
                        }
                        _ => s.push(next),
                    }
                    self.advance();
                }
            } else {
                s.push(c);
                self.advance();
            }
        }
        
        Token::StringLiteral(s)
    }

    fn collect_char(&mut self) -> Token {
        let mut c = self.current_char.unwrap_or('\0');
        self.advance();
        
        if c == '\\' {
            if let Some(next) = self.current_char {
                c = match next {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '0' => '\0',
                    '\\' => '\\',
                    '\'' => '\'',
                    '"' => '"',
                    _ => next,
                };
                self.advance();
            }
        }
        
        if self.current_char == Some('\'') {
            self.advance();
            Token::CharLiteral(c)
        } else {
            Token::Error("Carácter literal incompleto".to_string())
        }
    }

    fn collect_comment(&mut self) -> String {
        let mut comment = String::new();
        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }
        comment
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = TokenWithSpan;
    
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if matches!(token.token, Token::EOF) {
            None
        } else {
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_basic() {
        let input = "fn main() { return 42 }";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.map(|t| t.token).collect();
        
        assert!(matches!(tokens[0], Token::Fn));
        assert!(matches!(&tokens[1], Token::Ident(name) if name == "main"));
        assert!(matches!(tokens[2], Token::LParen));
        assert!(matches!(tokens[3], Token::RParen));
        assert!(matches!(tokens[4], Token::LBrace));
        assert!(matches!(tokens[5], Token::Return));
        assert!(matches!(&tokens[6], Token::IntLiteral(val) if val == "42"));
        assert!(matches!(tokens[7], Token::RBrace));
    }

    #[test]
    fn test_lexer_keywords_complete() {
        let input = "fn let mut const if else while for return impl struct enum trait match async await use mod pub crate self super";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.map(|t| t.token).collect();
        
        let expected = [
            Token::Fn, Token::Let, Token::Mut, Token::Const,
            Token::If, Token::Else, Token::While, Token::For,
            Token::Return, Token::Impl, Token::Struct, Token::Enum,
            Token::Trait, Token::Match, Token::Async, Token::Await,
            Token::Use, Token::Mod, Token::Pub, Token::Crate,
            Token::SelfKeyword, Token::SuperKeyword,
        ];
        
        for (i, exp) in expected.iter().enumerate() {
            assert_eq!(&tokens[i], exp);
        }
    }

    #[test]
    fn test_lexer_operators_complete() {
        let input = "+ - * / % = == ! != < > <= >= & | && || -> =>";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.map(|t| t.token).collect();
        
        assert!(matches!(tokens[0], Token::Plus));
        assert!(matches!(tokens[1], Token::Minus));
        assert!(matches!(tokens[2], Token::Star));
        assert!(matches!(tokens[3], Token::Slash));
        assert!(matches!(tokens[4], Token::Percent));
        assert!(matches!(tokens[5], Token::Eq));
        assert!(matches!(tokens[6], Token::EqEq));
        assert!(matches!(tokens[7], Token::Not));
        assert!(matches!(tokens[8], Token::NotEq));
        assert!(matches!(tokens[9], Token::Lt));
        assert!(matches!(tokens[10], Token::Gt));
        assert!(matches!(tokens[11], Token::LtEq));
        assert!(matches!(tokens[12], Token::GtEq));
        assert!(matches!(tokens[13], Token::And));
        assert!(matches!(tokens[14], Token::Or));
        assert!(matches!(tokens[15], Token::AndAnd));
        assert!(matches!(tokens[16], Token::OrOr));
        assert!(matches!(tokens[17], Token::Arrow));
        assert!(matches!(tokens[18], Token::FatArrow));
    }

    #[test]
    fn test_lexer_numbers() {
        let input = "42 3.14 0x1A 1e10";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.map(|t| t.token).collect();
        
        assert!(matches!(&tokens[0], Token::IntLiteral(val) if val == "42"));
        assert!(matches!(&tokens[1], Token::FloatLiteral(val) if val == "3.14"));
        assert!(matches!(&tokens[2], Token::IntLiteral(val) if val == "0x1A"));
        assert!(matches!(&tokens[3], Token::FloatLiteral(val) if val == "1e10"));
    }
          }
