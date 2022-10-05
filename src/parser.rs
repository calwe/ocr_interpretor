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
            nodes.push(self.parse_assign());
        }
        Node::Block(nodes)
    }

    fn parse_assign(&mut self) -> Node {
        let token = self.get_token();
        let ident = match token.kind {
            TokenKind::Ident(x) => x,
            _ => panic!("assignment must start with ident!"),
        };
        // TODO: Verify equals
        self.get_token(); // consume '='
        let expr = self.parse_expr();
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
        // TODO: Brackets
        let token = self.get_token();
        match token.kind {
            TokenKind::Number(x) => Node::Primary(Value::Number(x)),
            TokenKind::Symbol(SymbolKind::LeftBracket) => {
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
