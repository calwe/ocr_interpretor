use std::fmt::Display;

pub type Num = u64;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(Num),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{}", x),
            Self::String(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
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
