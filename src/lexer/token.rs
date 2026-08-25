// src/lexer/token.rs - Definición de todos los tokens de AXIOM

use std::fmt;

/// Representa todos los tokens posibles en el lenguaje AXIOM
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Palabras clave
    Fn,
    Let,
    Mut,
    Const,
    If,
    Else,
    While,
    For,
    Return,
    Impl,
    Struct,
    Enum,
    Trait,
    Match,
    Async,
    Await,
    Use,
    Mod,
    Pub,
    Crate,
    Import,
    Export,
    Type,
    New,
    SelfKeyword,
    SuperKeyword,
    True,
    False,
    Null,
    
    // Operadores aritméticos
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    
    // Operadores de comparación
    Eq,
    EqEq,
    Not,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    
    // Operadores lógicos
    And,
    Or,
    AndAnd,
    OrOr,
    
    // Operadores especiales
    Arrow,
    FatArrow,
    Dot,
    DotDot,
    DotDotDot,
    Comma,
    Semicolon,
    Colon,
    ColonColon,
    Pound,
    At,
    Dollar,
    
    // Delimitadores
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    
    // Literales
    Ident(String),
    IntLiteral(String),
    FloatLiteral(String),
    StringLiteral(String),
    CharLiteral(char),
    
    // Comentarios
    Comment(String),
    DocComment(String),
    
    // Especiales
    Whitespace,
    Newline,
    EOF,
    Error(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Mut => write!(f, "mut"),
            Token::Const => write!(f, "const"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::For => write!(f, "for"),
            Token::Return => write!(f, "return"),
            Token::Impl => write!(f, "impl"),
            Token::Struct => write!(f, "struct"),
            Token::Enum => write!(f, "enum"),
            Token::Trait => write!(f, "trait"),
            Token::Match => write!(f, "match"),
            Token::Async => write!(f, "async"),
            Token::Await => write!(f, "await"),
            Token::Use => write!(f, "use"),
            Token::Mod => write!(f, "mod"),
            Token::Pub => write!(f, "pub"),
            Token::Crate => write!(f, "crate"),
            Token::Import => write!(f, "import"),
            Token::Export => write!(f, "export"),
            Token::Type => write!(f, "type"),
            Token::New => write!(f, "new"),
            Token::SelfKeyword => write!(f, "self"),
            Token::SuperKeyword => write!(f, "super"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Eq => write!(f, "="),
            Token::EqEq => write!(f, "=="),
            Token::Not => write!(f, "!"),
            Token::NotEq => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::LtEq => write!(f, "<="),
            Token::GtEq => write!(f, ">="),
            Token::And => write!(f, "&"),
            Token::Or => write!(f, "|"),
            Token::AndAnd => write!(f, "&&"),
            Token::OrOr => write!(f, "||"),
            Token::Arrow => write!(f, "->"),
            Token::FatArrow => write!(f, "=>"),
            Token::Dot => write!(f, "."),
            Token::DotDot => write!(f, ".."),
            Token::DotDotDot => write!(f, "..."),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::ColonColon => write!(f, "::"),
            Token::Pound => write!(f, "#"),
            Token::At => write!(f, "@"),
            Token::Dollar => write!(f, "$"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Ident(name) => write!(f, "{}", name),
            Token::IntLiteral(val) => write!(f, "{}", val),
            Token::FloatLiteral(val) => write!(f, "{}", val),
            Token::StringLiteral(val) => write!(f, "\"{}\"", val),
            Token::CharLiteral(val) => write!(f, "'{}'", val),
            Token::Comment(val) => write!(f, "//{}", val),
            Token::DocComment(val) => write!(f, "///{}", val),
            Token::Whitespace => write!(f, " "),
            Token::Newline => write!(f, "\n"),
            Token::EOF => write!(f, "EOF"),
            Token::Error(val) => write!(f, "ERROR: {}", val),
        }
    }
}

/// Posición en el código fuente
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }
}

/// Token con su posición
#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
}

impl TokenWithSpan {
    pub fn new(token: Token, span: Span) -> Self {
        Self { token, span }
    }
          }
