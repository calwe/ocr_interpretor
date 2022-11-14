use core::fmt;
use std::{error::Error, fmt::Display};

use crate::{lexer::Token, Position};

#[derive(Clone, Debug)]
pub enum LexerError {
    UnrecognisedCharacter(char, Position, String),
}

impl Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnrecognisedCharacter(_, p, i) => {
                let _ = writeln!(f, "Unrecognised Character");
                write_position(f, p, 1, i)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum ParserError {
    InvalidTokenInBlock(Token, String),
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTokenInBlock(t, input) => {
                let _ = writeln!(f, "Invalid statement at the root of block");
                write_position(f, &t.start, t.len, input)
            }
            _ => todo!(),
        }
    }
}

fn write_position(
    f: &mut fmt::Formatter<'_>,
    position: &Position,
    len: usize,
    input: &String,
) -> fmt::Result {
    let (line_num_str, line_num_pad) = line_number_strings(position.line);
    let line = offending_line(position.line, input);
    let pointer = pointer_string(position.col, len);
    let _ = writeln!(f, "{}", line_num_pad);
    let _ = writeln!(f, "{}{}", line_num_str, line);
    write!(f, "{}{}", line_num_pad, pointer)
}

fn offending_line(line: usize, input: &String) -> String {
    let lines = input.clone();
    lines.lines().nth(line - 1).unwrap().to_string()
}

fn pointer_string(col: usize, len: usize) -> String {
    let mut padding = String::new();
    for _ in 1..col {
        padding.push(' ');
    }
    let mut pointer = String::new();
    for _ in 0..len {
        pointer.push('^');
    }
    format!("{}{}", padding, pointer)
}

fn line_number_strings(line: usize) -> (String, String) {
    let line_num_length = line.to_string().len();
    let mut padding = String::new();
    for _ in 0..line_num_length {
        padding.push(' ');
    }
    (format!("{} | ", line), format!("{} | ", padding))
}
