use crate::{ast::Node, Value};

pub struct Interpretor {
    ast: Box<Node>,
}

impl Interpretor {
    pub fn new(ast: Box<Node>) -> Self {
        Self { ast }
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
            Node::FuncCall { .. } => {
                self.run_func(node);
            }
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
            _ => unimplemented!("cannot print {:?}", args[0]),
        }
    }
}
