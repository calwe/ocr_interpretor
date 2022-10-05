pub type Num = u64;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(Num),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
}

pub mod ast;
pub mod lexer;
pub mod parser;
