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

pub mod ast;
pub mod interpretor;
pub mod lexer;
pub mod parser;
