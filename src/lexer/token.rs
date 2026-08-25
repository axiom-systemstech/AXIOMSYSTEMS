// src/lexer/token.rs - Definición de todos los tokens de AXIOM

use std::fmt;

/// Representa todos los tokens posibles en el lenguaje AXIOM
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // ============ PALABRAS CLAVE ============
    // Declaraciones
    Fn, Let, Mut, Const, Struct, Enum, Union, Trait, Impl, Type,
    Mod, Use, Pub, Crate, Import, Export, New,
    SelfKeyword, SuperKeyword,
    
    // Control de flujo
    If, Else, While, For, Loop, Match, Return, Break, Continue,
    
    // Concurrencia
    Async, Await, Spawn, Channel, Select,
    
    // Visibilidad
    PubCrate, PubSuper,
    
    // Especiales
    Extern, Unsafe, Move, Static, Abstract, Final, Override,
    
    // Manejo de errores
    Try, Catch, Throw, Finally,
    
    // Literales
    True, False, Null,
    
    // ============ OPERADORES ============
    // Aritméticos
    Plus, Minus, Star, Slash, Percent,
    
    // Comparación
    Eq, EqEq, Not, NotEq, Lt, Gt, LtEq, GtEq,
    
    // Lógicos
    And, Or, AndAnd, OrOr,
    
    // Especiales
    Arrow, FatArrow,
    Dot, DotDot, DotDotDot,
    Comma, Semicolon, Colon, ColonColon,
    Pound, At, Dollar,
    
    // Asignación compuesta
    PlusEq, MinusEq, StarEq, SlashEq, PercentEq,
    
    // ============ DELIMITADORES ============
    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,
    
    // ============ LITERALES ============
    Ident(String),
    IntLiteral(String),
    FloatLiteral(String),
    StringLiteral(String),
    CharLiteral(char),
    
    // ============ COMENTARIOS ============
    Comment(String),
    DocComment(String),
    
    // ============ ESPECIALES ============
    Whitespace,
    Newline,
    EOF,
    Error(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Palabras clave
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Mut => write!(f, "mut"),
            Token::Const => write!(f, "const"),
            Token::Struct => write!(f, "struct"),
            Token::Enum => write!(f, "enum"),
            Token::Union => write!(f, "union"),
            Token::Trait => write!(f, "trait"),
            Token::Impl => write!(f, "impl"),
            Token::Type => write!(f, "type"),
            Token::Mod => write!(f, "mod"),
            Token::Use => write!(f, "use"),
            Token::Pub => write!(f, "pub"),
            Token::Crate => write!(f, "crate"),
            Token::Import => write!(f, "import"),
            Token::Export => write!(f, "export"),
            Token::New => write!(f, "new"),
            Token::SelfKeyword => write!(f, "self"),
            Token::SuperKeyword => write!(f, "super"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::For => write!(f, "for"),
            Token::Loop => write!(f, "loop"),
            Token::Match => write!(f, "match"),
            Token::Return => write!(f, "return"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::Async => write!(f, "async"),
            Token::Await => write!(f, "await"),
            Token::Spawn => write!(f, "spawn"),
            Token::Channel => write!(f, "channel"),
            Token::Select => write!(f, "select"),
            Token::PubCrate => write!(f, "pub(crate)"),
            Token::PubSuper => write!(f, "pub(super)"),
            Token::Extern => write!(f, "extern"),
            Token::Unsafe => write!(f, "unsafe"),
            Token::Move => write!(f, "move"),
            Token::Static => write!(f, "static"),
            Token::Abstract => write!(f, "abstract"),
            Token::Final => write!(f, "final"),
            Token::Override => write!(f, "override"),
            Token::Try => write!(f, "try"),
            Token::Catch => write!(f, "catch"),
            Token::Throw => write!(f, "throw"),
            Token::Finally => write!(f, "finally"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
            
            // Operadores
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
            Token::PlusEq => write!(f, "+="),
            Token::MinusEq => write!(f, "-="),
            Token::StarEq => write!(f, "*="),
            Token::SlashEq => write!(f, "/="),
            Token::PercentEq => write!(f, "%="),
            
            // Delimitadores
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            
            // Literales
            Token::Ident(name) => write!(f, "{}", name),
            Token::IntLiteral(val) => write!(f, "{}", val),
            Token::FloatLiteral(val) => write!(f, "{}", val),
            Token::StringLiteral(val) => write!(f, "\"{}\"", val),
            Token::CharLiteral(val) => write!(f, "'{}'", val),
            
            // Comentarios
            Token::Comment(val) => write!(f, "//{}", val),
            Token::DocComment(val) => write!(f, "///{}", val),
            
            // Especiales
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
