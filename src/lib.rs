use std::fmt::Display;

use lexer::TokenKind;

pub type Num = u64;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(Num),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{}", x),
            Self::String(x) => write!(f, "{}", x),
            Self::Boolean(x) => write!(f, "{}", x),
            Self::Array(x) => write!(f, "{:?}", x),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    EqualTo,
}

impl From<TokenKind> for Op {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Symbol(lexer::SymbolKind::Plus) => Op::Plus,
            TokenKind::Symbol(lexer::SymbolKind::Minus) => Op::Minus,
            TokenKind::Symbol(lexer::SymbolKind::Greater) => Op::Greater,
            TokenKind::Symbol(lexer::SymbolKind::GreaterEquals) => Op::GreaterEqual,
            TokenKind::Symbol(lexer::SymbolKind::Less) => Op::Less,
            TokenKind::Symbol(lexer::SymbolKind::LessEquals) => Op::LessEqual,
            TokenKind::Symbol(lexer::SymbolKind::DoubleEquals) => Op::EqualTo,
            _ => panic!("Cannot create Operator from Token: {:?}", kind),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.line, self.col)
    }
}

pub mod ast;
pub mod error;
pub mod interpretor;
pub mod lexer;
pub mod parser;
pub mod symbol_table;
