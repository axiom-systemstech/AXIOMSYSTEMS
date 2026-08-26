// src/parser/ast.rs - Árbol de Sintaxis Abstracta de AXIOM

use crate::lexer::Span;

/// Nodo raíz del programa
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
    pub span: Span,
}

/// Items que pueden aparecer en el nivel superior
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Trait(Trait),
    Impl(Impl),
    Use(Use),
    Mod(Mod),
    Const(Const),
    Type(TypeAlias),
    Expr(Expr),
}

/// Función
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub span: Span,
}

/// Parámetro de función
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Estructura
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub span: Span,
}

/// Campo de estructura
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Enum
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>,
    pub span: Span,
}

/// Variante de enum
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<Type>,
    pub span: Span,
}

/// Trait
#[derive(Debug, Clone, PartialEq)]
pub struct Trait {
    pub name: String,
    pub methods: Vec<Function>,
    pub span: Span,
}

/// Implementación de trait
#[derive(Debug, Clone, PartialEq)]
pub struct Impl {
    pub trait_name: Option<String>,
    pub type_name: String,
    pub methods: Vec<Function>,
    pub span: Span,
}

/// Use (importación)
#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    pub path: String,
    pub alias: Option<String>,
    pub span: Span,
}

/// Módulo
#[derive(Debug, Clone, PartialEq)]
pub struct Mod {
    pub name: String,
    pub items: Vec<Item>,
    pub span: Span,
}

/// Constante
#[derive(Debug, Clone, PartialEq)]
pub struct Const {
    pub name: String,
    pub ty: Option<Type>,
    pub value: Expr,
    pub span: Span,
}

/// Type alias
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Tipos
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Ident(String, Span),
    Array(Box<Type>, Box<Expr>, Span),
    Slice(Box<Type>, Span),
    Tuple(Vec<Type>, Span),
    Fn(Vec<Type>, Box<Type>, Span),
    Generic(Box<Type>, Vec<Type>, Span),
    Path(Vec<String>, Span),
    Never(Span),
    Unit(Span),
}

/// Expresiones
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Literales
    Int(i64, Span),
    Float(f64, Span),
    Bool(bool, Span),
    String(String, Span),
    Char(char, Span),
    None(Span),
    
    /// Identificador
    Ident(String, Span),
    
    /// Operadores binarios
    Binary(BinaryOp, Box<Expr>, Box<Expr>, Span),
    
    /// Operadores unarios
    Unary(UnaryOp, Box<Expr>, Span),
    
    /// Llamada a función
    Call(Box<Expr>, Vec<Expr>, Span),
    
    /// Acceso a campo
    Field(Box<Expr>, String, Span),
    
    /// Indexación
    Index(Box<Expr>, Box<Expr>, Span),
    
    /// If/Else
    If(Box<Expr>, Box<Block>, Option<Box<Expr>>, Span),
    
    /// While
    While(Box<Expr>, Box<Block>, Span),
    
    /// For
    For(String, Box<Expr>, Box<Block>, Span),
    
    /// Loop
    Loop(Box<Block>, Span),
    
    /// Match
    Match(Box<Expr>, Vec<MatchArm>, Span),
    
    /// Block
    Block(Vec<Stmt>, Span),
    
    /// Return
    Return(Option<Box<Expr>>, Span),
    
    /// Break
    Break(Option<Box<Expr>>, Span),
    
    /// Continue
    Continue(Span),
    
    /// Let
    Let(String, bool, Option<Type>, Box<Expr>, Span),
    
    /// Asignación
    Assign(Box<Expr>, Box<Expr>, Span),
    
    /// Asignación compuesta
    AssignOp(BinaryOp, Box<Expr>, Box<Expr>, Span),
    
    /// Struct literal
    StructLit(String, Vec<FieldInit>, Span),
    
    /// Tuple
    Tuple(Vec<Expr>, Span),
    
    /// Array
    Array(Vec<Expr>, Span),
    
    /// Repeat array
    RepeatArray(Box<Expr>, Box<Expr>, Span),
    
    /// Lambda
    Lambda(Vec<Param>, Option<Type>, Box<Expr>, Span),
    
    /// Await
    Await(Box<Expr>, Span),
    
    /// Try (operador ?)
    Try(Box<Expr>, Span),
    
    /// Error
    Error(String, Span),
}

/// Operadores binarios
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Neq, Lt, Gt, Leq, Geq,
    And, Or,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
}

/// Operadores unarios
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
    Ref,
    RefMut,
}

/// Bloque de código
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// Sentencias
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr, Span),
    Let(Let, Span),
    Item(Item, Span),
}

/// Declaración Let
#[derive(Debug, Clone, PartialEq)]
pub struct Let {
    pub name: String,
    pub mutable: bool,
    pub ty: Option<Type>,
    pub value: Expr,
    pub span: Span,
}

/// Brazo de match
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
    pub span: Span,
}

/// Patrones
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Ident(String, Span),
    Wildcard(Span),
    Int(i64, Span),
    Float(f64, Span),
    Bool(bool, Span),
    String(String, Span),
    Char(char, Span),
    Tuple(Vec<Pattern>, Span),
    Struct(String, Vec<PatternField>, Span),
    Enum(String, Vec<Pattern>, Span),
    Or(Vec<Pattern>, Span),
    Range(Box<Pattern>, Box<Pattern>, Span),
}

/// Campo de patrón de struct
#[derive(Debug, Clone, PartialEq)]
pub struct PatternField {
    pub name: String,
    pub pattern: Pattern,
    pub span: Span,
}

/// Inicialización de campo de struct
#[derive(Debug, Clone, PartialEq)]
pub struct FieldInit {
    pub name: String,
    pub value: Expr,
    pub span: Span,
}

/// Tipos de expresión
impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Int(_, span) => *span,
            Expr::Float(_, span) => *span,
            Expr::Bool(_, span) => *span,
            Expr::String(_, span) => *span,
            Expr::Char(_, span) => *span,
            Expr::None(span) => *span,
            Expr::Ident(_, span) => *span,
            Expr::Binary(_, _, _, span) => *span,
            Expr::Unary(_, _, span) => *span,
            Expr::Call(_, _, span) => *span,
            Expr::Field(_, _, span) => *span,
            Expr::Index(_, _, span) => *span,
            Expr::If(_, _, _, span) => *span,
            Expr::While(_, _, span) => *span,
            Expr::For(_, _, _, span) => *span,
            Expr::Loop(_, span) => *span,
            Expr::Match(_, _, span) => *span,
            Expr::Block(_, span) => *span,
            Expr::Return(_, span) => *span,
            Expr::Break(_, span) => *span,
            Expr::Continue(span) => *span,
            Expr::Let(_, _, _, _, span) => *span,
            Expr::Assign(_, _, span) => *span,
            Expr::AssignOp(_, _, _, span) => *span,
            Expr::StructLit(_, _, span) => *span,
            Expr::Tuple(_, span) => *span,
            Expr::Array(_, span) => *span,
            Expr::RepeatArray(_, _, span) => *span,
            Expr::Lambda(_, _, _, span) => *span,
            Expr::Await(_, span) => *span,
            Expr::Try(_, span) => *span,
            Expr::Error(_, span) => *span,
        }
    }
}