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
            Node::ArrayAssign { .. } => self.run_array_assign(node),
            Node::ArrayAssingIndex { .. } => self.run_array_assign_ind(node),
            Node::IfExpr { .. } => self.run_if(node),
            Node::WhileStmt { .. } => self.run_while(node),
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
            _ => panic!("Unsupported expression as condition: {}", condition),
        }
    }

    fn run_while(&mut self, node: Node) {
        info!("Running while");
        let (expr, body) = match node {
            Node::WhileStmt { expr, body } => (expr, body),
            _ => panic!("Not a while statement"),
        };

        while self.evaluate_condition(*expr.clone()) {
            self.run_node(*body.clone());
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
            Node::VariableRef(_) => {
                let rvalue = self.get_expr_val(*rexpr.clone());
                self.symbol_table.assign_variable(ident, rvalue);
            }
            Node::ArrayRef { .. } => {
                let rvalue = self.get_array_ref(*rexpr);
                self.symbol_table.assign_variable(ident, rvalue);
            }
            Node::DotExpr { .. } => {
                let rvalue = self.run_dot_expr(*rexpr);
                self.symbol_table.assign_variable(ident, rvalue);
            }
            Node::FuncCall { .. } => {
                let rvalue = self.run_func(*rexpr).expect("function has no return value");
                self.symbol_table.assign_variable(ident, rvalue);
            }
            Node::Primary(x) => self.symbol_table.assign_variable(ident, x),
            _ => panic!("unsupported rvalue for assign: {:?}", *rexpr.clone()),
        }
    }

    fn run_array_assign(&mut self, node: Node) {
        info!("Creating array");
        let (ident, size) = match node {
            Node::ArrayAssign { ident, size } => (ident, size),
            _ => panic!("Not an assign"),
        };

        let numeric_size = match self.get_expr_val(*size) {
            Value::Number(x) => x,
            _ => panic!("Array size must be numeric"),
        };

        // create vector of size, with all parts initialised as 0
        let array = std::iter::repeat(Value::Number(0))
            .take(numeric_size as usize)
            .collect::<Vec<_>>();

        self.symbol_table
            .assign_variable(ident, Value::Array(array));

        info!("Symbol table: {:#?}", self.symbol_table);
    }

    fn run_array_assign_ind(&mut self, node: Node) {
        info!("Assigning array index");
        let (ident, index, value) = match node {
            Node::ArrayAssingIndex {
                ident,
                index,
                value,
            } => (ident, index, value),
            _ => panic!("Not an array index assign"),
        };

        let numeric_index = match self.get_expr_val(*index) {
            Value::Number(x) => x,
            _ => panic!("Index must be numeric"),
        };

        let value = self.get_expr_val(*value);

        let mut vec = match self.symbol_table.get_variable(ident.clone()) {
            Value::Array(x) => x,
            _ => panic!("Cannot index into non array type"),
        };
        vec[numeric_index as usize] = value;
        self.symbol_table.assign_variable(ident, Value::Array(vec));
    }

    fn run_expr(&mut self, node: Node) -> Value {
        info!("Running expression: {:?}", node);
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

        info!("lv: {:?}, op: {:?}, rv: {:?}", lvalue, op, rvalue);

        match lvalue {
            Value::Number(x) => match rvalue {
                Value::Number(y) => match op {
                    Op::Plus => Value::Number(x + y),
                    Op::Minus => Value::Number(x - y),
                    Op::Multiply => Value::Number(x * y),
                    Op::Divide => Value::Number(x / y),
                    Op::Mod => Value::Number(x % y),
                    Op::EqualTo => Value::Boolean(x == y),
                    Op::Less => Value::Boolean(x < y),
                    Op::LessEqual => Value::Boolean(x <= y),
                    Op::Greater => Value::Boolean(x > y),
                    Op::GreaterEqual => Value::Boolean(x >= y),
                    _ => panic!("Invalid arithmetic expression"),
                },
                Value::String(_) => self.concat(lvalue, rvalue),
                _ => panic!(),
            },
            Value::String(_) => self.concat(lvalue, rvalue),
            _ => panic!(),
        }
    }

    fn get_expr_val(&mut self, node: Node) -> Value {
        info!("Getting numeric value from expression: {:?}", node);
        match node {
            Node::BinaryExpr { .. } => self.run_expr(node),
            Node::VariableRef(x) => self.symbol_table.get_variable(x),
            Node::ArrayRef { .. } => self.get_array_ref(node),
            Node::FuncCall { .. } => self.run_func(node).unwrap(),
            Node::DotExpr { .. } => self.run_dot_expr(node),
            Node::Primary(x) => x,
            _ => unimplemented!("Unsupported value for expression side"),
        }
    }

    fn run_dot_expr(&mut self, node: Node) -> Value {
        info!("Running dot expr");

        let (lvalue, rvalue) = match node {
            Node::DotExpr { left, right } => (left, right),
            _ => panic!("Not a dot expr"),
        };

        // test for builtin
        match rvalue.as_str() {
            "length" => self.builtin_length(lvalue),
            _ => panic!("Only builtin lvalues are suported"),
        }
    }

    fn get_array_ref(&mut self, node: Node) -> Value {
        info!("Getting array reference: {:?}", node);
        let (ident, index) = match node {
            Node::ArrayRef { ident, index } => (ident, index),
            _ => panic!("Not an array ref"),
        };

        let numeric_index = match self.get_expr_val(*index) {
            Value::Number(x) => x,
            _ => panic!("Index must be numeric"),
        };

        info!("Array Index: {}", numeric_index);

        let symbol = self.symbol_table.get_variable(ident.to_string());
        let vec = match symbol {
            Value::Array(x) => x,
            _ => panic!("Cannot index into {}", symbol),
        };
        vec[numeric_index as usize].clone()
    }

    fn concat(&mut self, lvalue: Value, rvalue: Value) -> Value {
        Value::String(format!("{}{}", lvalue, rvalue))
    }

    fn evaluate_condition(&mut self, expr: Node) -> bool {
        match self.run_expr(expr) {
            Value::Boolean(x) => x,
            _ => panic!("Invalid expression for while loop condition"),
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

        let to_print = self.get_expr_val(args[0].clone());
        println!("{}", to_print);
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
            Node::FuncCall { .. } => {
                info!("Casting result from func call to int");
                let ret = self.run_func(args[0].clone()).unwrap();
                info!("Got {ret} from func");
                match ret {
                    Value::String(x) => Value::Number(x.parse().unwrap()),
                    _ => panic!("Invalid variable type for cast"),
                }
            }
            _ => panic!("Invald cast"),
        }
    }

    fn builtin_length(&mut self, ident: String) -> Value {
        info!("Built in property: length");

        let vec = match self.symbol_table.get_variable(ident) {
            Value::Array(x) => x,
            _ => panic!("Only arrays have the builtin property: length"),
        };

        Value::Number(vec.len() as Num)
    }
}
