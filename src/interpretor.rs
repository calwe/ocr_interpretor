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
        match node {
            // TODO: Variables - requires symbol table
            Node::FuncCall { .. } => {
                self.run_func(node);
            }
            Node::Assign { .. } => self.run_assign(node),
            _ => todo!("more node types"),
        }
    }

    fn run_func(&mut self, node: Node) {
        let (ident, args) = match node {
            Node::FuncCall { ident, args } => (ident, args),
            _ => panic!("Not a function"),
        };
        // built in functions
        match ident.as_str() {
            "print" => self.builtin_print(args),
            _ => todo!("Implement custom functions"),
        }
    }

    fn run_assign(&mut self, node: Node) {
        let (ident, rexpr) = match node {
            Node::Assign { ident, value } => (ident, value),
            _ => panic!("Not an assign"),
        };
        // get value to put in symbol table
        match *rexpr {
            Node::BinaryExpr { .. } => {
                let rvalue = self.run_expr(*rexpr);
                self.symbol_table
                    .assign_variable(ident, Value::Number(rvalue));
            }
            Node::Primary(x) => self.symbol_table.assign_variable(ident, x),
            _ => panic!("unsupported rvalue for assign"),
        }
    }

    fn run_expr(&mut self, node: Node) -> Num {
        let (left, op, right) = match node {
            Node::BinaryExpr {
                left,
                operator,
                right,
            } => (left, operator, right),
            _ => panic!("Not an expression"),
        };

        let left_val = self.get_expr_val(*left);
        let right_val = self.get_expr_val(*right);
        match op {
            Op::Plus => left_val + right_val,
            Op::Minus => left_val - right_val,
        }
    }

    fn get_expr_val(&mut self, node: Node) -> Num {
        match node {
            Node::BinaryExpr { .. } => self.run_expr(node),
            Node::VariableRef(x) => {
                let var = self.symbol_table.get_variable(x);
                match var {
                    Value::Number(x) => x,
                    _ => panic!("Expression only supports numbers"),
                }
            }
            Node::Primary(Value::Number(x)) => x,
            _ => unimplemented!("Unsupported value for expression side"),
        }
    }

    fn builtin_print(&mut self, args: Vec<Node>) {
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
}
