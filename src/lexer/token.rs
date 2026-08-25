

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Palabras clave
    Fn, Let, Mut, Const, If, Else, While, For, Return,
    Impl, Struct, Enum, Trait, Match, Async, Await,
    Use, Mod, Pub, Crate, Import, Export, Type, New,
    SelfKeyword, SuperKeyword, True, False, Null,
    Loop, Break, Continue, Extern, Unsafe, Move, Static,
    Abstract, Final, Override, Try, Catch, Throw, Finally,
    
    // Operadores
    Plus, Minus, Star, Slash, Percent,
    Eq, EqEq, Not, NotEq, Lt, Gt, LtEq, GtEq,
    And, Or, AndAnd, OrOr,
    Arrow, FatArrow,
    Dot, DotDot, DotDotDot,
    Comma, Semicolon, Colon, ColonColon,
    Pound, At, Dollar,
    
    // Delimitadores
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
}
