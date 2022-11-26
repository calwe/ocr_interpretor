use crate::{Num, Op, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Block(Vec<Node>),
    Assign {
        ident: String,
        value: Box<Node>,
    },
    ArrayAssign {
        ident: String,
        size: Box<Node>,
    },
    ArrayAssingIndex {
        ident: String,
        index: Box<Node>,
        value: Box<Node>,
    },
    IfExpr {
        expr: Box<Node>,
        then: Box<Node>,
        els: Box<Node>,
    },
    WhileStmt {
        expr: Box<Node>,
        body: Box<Node>,
    },
    FuncCall {
        ident: String,
        args: Vec<Node>,
    },
    VariableRef(String),
    ArrayRef {
        ident: String,
        index: Box<Node>,
    },
    BinaryExpr {
        left: Box<Node>,
        operator: Op,
        right: Box<Node>,
    },
    DotExpr {
        left: String,
        right: String,
    },
    Primary(Value),
}
