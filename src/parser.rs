use crate::{
    ast::Node,
    lexer::{SymbolKind, Token, TokenKind},
    Op, Value,
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().rev().collect(), // reverse tokens, as we pop from the end
        }
    }

    pub fn parse(&mut self) -> Box<Node> {
        Box::new(self.parse_block())
    }

    fn parse_block(&mut self) -> Node {
        let mut nodes = Vec::new();
        while !self.tokens.is_empty() {
            let mut token_clone = self.tokens.clone();
            match token_clone.pop().unwrap().kind {
                TokenKind::Ident(_) => {
                    match token_clone
                        .pop()
                        .expect("ident unsupported on its own")
                        .kind
                    {
                        TokenKind::Symbol(SymbolKind::Equals) => {
                            nodes.push(self.parse_assign());
                        }
                        TokenKind::Symbol(SymbolKind::LeftBracket) => {
                            nodes.push(self.parse_func_call());
                        }
                        _ => unimplemented!("unimplemented ident"),
                    }
                }
                _ => unimplemented!("unimplemneted parse branch: {:?}", self.get_token().kind),
            }
        }
        Node::Block(nodes)
    }

    fn parse_func_call(&mut self) -> Node {
        let token = self.get_token();
        let ident = match token.kind {
            TokenKind::Ident(x) => x,
            _ => panic!("assignment must start with ident!"),
        };
        // TODO: parse arguments
        let mut args = Vec::new();
        match self.get_token().kind {
            TokenKind::Symbol(SymbolKind::LeftBracket) => {
                match self.peek_token().unwrap().kind {
                    TokenKind::Symbol(SymbolKind::RightBracket) => (),
                    _ => args.push(self.parse_arg()), // TODO: Multiple args, string args
                }
            }
            _ => panic!("Must have bracket after function!"),
        };
        self.get_token(); // consume final bracket
        Node::FuncCall { ident, args }
    }

    fn parse_arg(&mut self) -> Node {
        match self.peek_token().unwrap().kind {
            TokenKind::String(x) => {
                self.get_token(); // consume token
                Node::Primary(Value::String(x))
            }
            _ => self.parse_expr(),
        }
    }

    fn parse_assign(&mut self) -> Node {
        let token = self.get_token();
        let ident = match token.kind {
            TokenKind::Ident(x) => x,
            _ => panic!("assignment must start with ident!"),
        };
        // TODO: Verify equals
        self.get_token(); // consume '='
        let expr = match self.peek_token().unwrap().kind {
            TokenKind::String(x) => {
                self.get_token(); // consume string
                Node::Primary(Value::String(x))
            }
            _ => self.parse_expr(),
        };
        Node::Assign {
            ident,
            value: Box::new(expr),
        }
    }

    fn parse_expr(&mut self) -> Node {
        let left = self.parse_term();
        let optok = self.peek_token();
        if let Some(x) = optok {
            let operator = match x.kind {
                TokenKind::Symbol(SymbolKind::Plus) => Op::Plus,
                TokenKind::Symbol(SymbolKind::Minus) => Op::Minus,
                _ => return left,
            };
            self.get_token(); // consume token
            let right = self.parse_expr();
            Node::BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        } else {
            left
        }
    }

    fn parse_term(&mut self) -> Node {
        // for now, we will skip this
        self.parse_factor()
    }

    fn parse_factor(&mut self) -> Node {
        let token = self.peek_token().unwrap();
        match token.kind {
            TokenKind::Number(x) => {
                self.get_token();
                Node::Primary(Value::Number(x))
            }
            TokenKind::Ident(x) => {
                let mut peekpeek = self.tokens.clone();
                peekpeek.pop();
                if let Some(x) = peekpeek.pop() {
                    if x.kind == TokenKind::Symbol(SymbolKind::LeftBracket) {
                        return self.parse_func_call();
                    }
                }
                self.get_token();
                Node::VariableRef(x)
            }
            TokenKind::Symbol(SymbolKind::LeftBracket) => {
                self.get_token();
                let expr = self.parse_expr();
                // TODO: verify bracket (needs error handling)
                self.get_token(); // consume end bracket
                expr
            }
            _ => unimplemented!("Unimplemented factor: {:?}", token.kind),
        }
    }

    // TODO: MUST DO ERROR HANDLING - PANICING IS NOT ACCEPTABLE
    fn get_token(&mut self) -> Token {
        self.tokens.pop().unwrap()
    }

    fn peek_token(&mut self) -> Option<Token> {
        self.tokens.clone().pop()
    }
}
