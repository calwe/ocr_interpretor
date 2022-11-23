use std::io::{self, Write};

use log::info;

use crate::{ast::Node, symbol_table::SymbolTable, Num, Op, Value};

pub struct Interpretor {
    ast: Box<Node>,
    symbol_table: SymbolTable,
}

impl Interpretor {
    pub fn new(ast: Box<Node>) -> Self {
        Self {
            ast,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn run(&mut self) {
        info!("Running program");
        match *self.ast.clone() {
            Node::Block(nodes) => {
                for node in nodes {
                    self.run_node(node);
                }
            }
            _ => panic!("Code must be in a block"),
        }
    }

    fn run_node(&mut self, node: Node) {
        info!("Running node");
        match node {
            // TODO: Variables - requires symbol table
            Node::FuncCall { .. } => {
                self.run_func(node);
            }
            Node::Assign { .. } => self.run_assign(node),
            Node::IfExpr { .. } => self.run_if(node),
            Node::Block(nodes) => self.run_block(nodes),
            _ => todo!("more node types"),
        }
    }

    fn run_block(&mut self, nodes: Vec<Node>) {
        info!("Running block");
        for node in nodes {
            self.run_node(node);
        }
    }

    fn run_if(&mut self, node: Node) {
        info!("Running if");
        let (expr, then, els) = match node {
            Node::IfExpr { expr, then, els } => (expr, then, els),
            _ => panic!("Not if statement"),
        };

        let condition = self.run_expr(*expr);
        match condition {
            Value::Boolean(true) => {
                info!("If expression is true!");
                self.run_node(*then);
            }
            Value::Boolean(false) => {
                info!("If expression is false.");
                self.run_node(*els);
            }
            _ => panic!("Unsupported expression as condition"),
        }
    }

    fn run_func(&mut self, node: Node) -> Option<Value> {
        info!("Running function");
        let (ident, args) = match node {
            Node::FuncCall { ident, args } => (ident, args),
            _ => panic!("Not a function"),
        };
        // built in functions
        match ident.as_str() {
            "print" => {
                self.builtin_print(args);
                None
            }
            "input" => Some(self.builtin_input(args)),
            "int" => Some(self.builtin_casti(args)),
            _ => todo!("Implement custom functions"),
        }
    }

    fn run_assign(&mut self, node: Node) {
        info!("Assigning value");
        let (ident, rexpr) = match node {
            Node::Assign { ident, value } => (ident, value),
            _ => panic!("Not an assign"),
        };
        // get value to put in symbol table
        match *rexpr.clone() {
            Node::BinaryExpr { .. } => {
                let rvalue = self.run_expr(*rexpr);
                self.symbol_table.assign_variable(ident, rvalue);
            }
            Node::FuncCall { .. } => {
                let rvalue = self.run_func(*rexpr).expect("function has no return value");
                self.symbol_table.assign_variable(ident, rvalue);
            }
            Node::Primary(x) => self.symbol_table.assign_variable(ident, x),
            _ => panic!("unsupported rvalue for assign"),
        }
    }

    fn run_expr(&mut self, node: Node) -> Value {
        info!("Running expression");
        let (left, op, right) = match node {
            Node::BinaryExpr {
                left,
                operator,
                right,
            } => (left, operator, right),
            _ => panic!("Not an expression"),
        };

        let lvalue = self.get_expr_val(*left);
        let rvalue = self.get_expr_val(*right);
        match op {
            Op::Plus => Value::Number(lvalue + rvalue),
            Op::Minus => Value::Number(lvalue - rvalue),
            Op::EqualTo => Value::Boolean(lvalue == rvalue),
            Op::Less => Value::Boolean(lvalue < rvalue),
            Op::LessEqual => Value::Boolean(lvalue <= rvalue),
            Op::Greater => Value::Boolean(lvalue > rvalue),
            Op::GreaterEqual => Value::Boolean(lvalue >= rvalue),
            _ => panic!("Invalid arithmetic expression"),
        }
    }

    fn get_expr_val(&mut self, node: Node) -> Num {
        info!("Getting numeric value from expression");
        match node {
            Node::BinaryExpr { .. } => match self.run_expr(node) {
                Value::Number(x) => x,
                _ => panic!("Expression only supports numbers"),
            },
            Node::VariableRef(x) => {
                let var = self.symbol_table.get_variable(x);
                match var {
                    Value::Number(x) => x,
                    _ => panic!("Expression only supports numbers"),
                }
            }
            Node::FuncCall { .. } => match self.run_func(node) {
                Some(Value::Number(x)) => x,
                _ => panic!("Expression only supports numbers"),
            },
            Node::Primary(Value::Number(x)) => x,
            _ => unimplemented!("Unsupported value for expression side"),
        }
    }

    fn builtin_print(&mut self, args: Vec<Node>) {
        info!("Function was built-in: print");
        // verify arguments
        if args.len() == 0 {
            println!();
            return;
        } else if args.len() > 1 {
            panic!("print cannot accept more than 1 arg!");
        }

        match &args[0] {
            Node::Primary(x) => {
                println!("{}", x);
            }
            Node::BinaryExpr { .. } => {
                let expr = self.run_expr(args[0].clone());
                println!("{}", expr);
            }
            Node::VariableRef(x) => {
                let var = self.symbol_table.get_variable(x.to_string());
                println!("{}", var);
            }
            _ => unimplemented!("cannot print {:?}", args[0]),
        }
    }

    fn builtin_input(&mut self, args: Vec<Node>) -> Value {
        info!("Function was built-in: input");
        if args.len() > 1 {
            panic!("print cannot accept more than 1 arg!");
        }

        let mut input = String::new();
        if args.len() == 1 {
            match &args[0] {
                Node::Primary(x) => {
                    print!("{}", x);
                }
                Node::BinaryExpr { .. } => {
                    let expr = self.run_expr(args[0].clone());
                    print!("{}", expr);
                }
                Node::VariableRef(x) => {
                    let var = self.symbol_table.get_variable(x.to_string());
                    print!("{}", var);
                }
                _ => unimplemented!("cannot print {:?}", args[0]),
            }
            let _ = io::stdout().flush();
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");
        input.pop(); // consume newline
        Value::String(input)
    }

    fn builtin_casti(&mut self, args: Vec<Node>) -> Value {
        info!("FUnction was built-in: int");
        if args.len() > 1 {
            panic!("int cannot accept more than 1 arg");
        }

        match &args[0] {
            Node::Primary(Value::String(x)) => {
                info!("Casting primary ({}) to int", x);

                Value::Number(x.parse().unwrap())
            }
            Node::VariableRef(x) => {
                let var = self.symbol_table.get_variable(x.to_string());
                info!("Casting variable ({} = {}) to int", x, var);
                match var {
                    Value::String(x) => Value::Number(x.parse().unwrap()),
                    _ => panic!("Invalid variable type for cast"),
                }
            }
            _ => panic!("Invald cast"),
        }
    }
}
