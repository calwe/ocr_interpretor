use crate::{Op, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Block(Vec<Node>),
    Assign {
        ident: String,
        value: Box<Node>,
    },
    IfExpr {
        expr: Box<Node>,
        then: Box<Node>,
        els: Box<Node>,
    },
    FuncCall {
        ident: String,
        args: Vec<Node>,
    },
    VariableRef(String),
    BinaryExpr {
        left: Box<Node>,
        operator: Op,
        right: Box<Node>,
    },
    Primary(Value),
}
