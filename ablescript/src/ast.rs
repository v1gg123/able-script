//! AbleScript's Abstract Syntax tree
//!
//! Statements are the type which is AST made of, as they
//! express an effect.
//!
//! Expressions are just operations and they cannot be
//! used as statements. Functions in AbleScript are in fact
//! just plain subroutines and they do not return any value,
//! so their calls are statements.

use std::hash::Hash;

use crate::variables::Value;

type Span = std::ops::Range<usize>;

#[derive(Debug, Clone)]
pub struct Iden {
    pub iden: String,
    pub span: Span,
}

impl Iden {
    pub fn new(iden: String, span: Span) -> Self {
        Self { iden, span }
    }
}

impl PartialEq for Iden {
    fn eq(&self, other: &Self) -> bool {
        self.iden == other.iden
    }
}

impl Hash for Iden {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iden.hash(state)
    }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Block {
    pub block: Vec<Stmt>,
}

/// A syntactic unit expressing an effect.
#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl PartialEq for Stmt {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Hash for Stmt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state)
    }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum StmtKind {
    // Control flow
    If {
        cond: Expr,
        body: Block,
    },
    Loop {
        body: Block,
    },
    Break,
    HopBack,

    Var {
        iden: Iden,
        init: Option<Expr>,
    },
    Assign {
        iden: Iden,
        value: Expr,
    },

    Functio {
        iden: Iden,
        params: Vec<Iden>,
        body: Block,
    },
    BfFunctio {
        iden: Iden,
        tape_len: Option<Expr>,
        code: Vec<u8>,
    },
    Call {
        expr: Expr,
        args: Vec<Expr>,
    },
    Print(Expr),
    Read(Iden),
    Melo(Iden),
    Rlyeh,
    Rickroll,
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Expression is parse unit which do not cause any effect,
/// like math and logical operations or values.
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state)
    }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum ExprKind {
    BinOp {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        kind: BinOpKind,
    },
    Not(Box<Expr>),
    Literal(Value),
    Cart(Vec<(Expr, Expr)>),
    Index {
        cart: Box<Expr>,
        index: Box<Expr>,
    },
    Variable(String),
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum BinOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    Less,
    Equal,
    NotEqual,
    And,
    Or,
}

impl BinOpKind {
    pub fn from_token(t: crate::lexer::Token) -> Result<Self, crate::error::ErrorKind> {
        use crate::lexer::Token;

        match t {
            Token::Plus => Ok(Self::Add),
            Token::Minus => Ok(Self::Subtract),
            Token::Star => Ok(Self::Multiply),
            Token::FwdSlash => Ok(Self::Divide),
            Token::GreaterThan => Ok(Self::Greater),
            Token::LessThan => Ok(Self::Less),
            Token::EqualEqual => Ok(Self::Equal),
            Token::NotEqual => Ok(Self::NotEqual),
            Token::And => Ok(Self::And),
            Token::Or => Ok(Self::Or),
            t => Err(crate::error::ErrorKind::UnexpectedToken(t)),
        }
    }
}
