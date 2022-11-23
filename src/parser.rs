use log::info;

use crate::{
    ast::Node,
    error::ParserError,
    lexer::{KeywordKind, SymbolKind, Token, TokenKind},
    Op, Value,
};

pub struct Parser {
    tokens: Vec<Token>,
    input: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, input: String) -> Self {
        Self {
            tokens: tokens.into_iter().rev().collect(), // reverse tokens, as we pop from the end
            input,
        }
    }

    pub fn parse(&mut self) -> Result<Node, ParserError> {
        info!("Begin parse");

        self.parse_block()
    }

    fn parse_block(&mut self) -> Result<Node, ParserError> {
        info!("Parsing block");

        let mut nodes = Vec::new();
        // loop through every token from our lexer
        while !self.tokens.is_empty() {
            let mut token_clone = self.tokens.clone();
            let token = token_clone.pop().unwrap();
            match token.kind {
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
                TokenKind::Keyword(KeywordKind::If) => {
                    nodes.push(self.parse_if());
                }
                TokenKind::Keyword(KeywordKind::While) => {
                    nodes.push(self.parse_while());
                }
                TokenKind::Keyword(KeywordKind::EndIf)
                | TokenKind::Keyword(KeywordKind::Else)
                | TokenKind::Keyword(KeywordKind::EndWhile) => {
                    return Ok(Node::Block(nodes));
                }
                _ => return Err(ParserError::InvalidTokenInBlock(token, self.input.clone())),
            }
        }
        Ok(Node::Block(nodes))
    }

    fn parse_if(&mut self) -> Node {
        info!("Parsing if statement");

        self.get_token(); // consume "if"
        let expr = self.parse_expr();
        self.get_token(); // consume "then"
        let then = self.parse_block().unwrap();
        let els = match self.get_token().kind {
            TokenKind::Keyword(KeywordKind::Else) => self.parse_block().unwrap(),
            _ => Node::Block(Vec::new()),
        };

        Node::IfExpr {
            expr: Box::new(expr),
            then: Box::new(then),
            els: Box::new(els),
        }
    }

    fn parse_while(&mut self) -> Node {
        info!("Parsing while statement");

        self.get_token(); // consume "while"
        let expr = self.parse_expr();
        let body = self.parse_block().unwrap();

        Node::WhileStmt {
            expr: Box::new(expr),
            body: Box::new(body),
        }
    }

    fn parse_func_call(&mut self) -> Node {
        info!("Parsing func call");

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
        info!("Parsing an argument");

        match self.peek_token().unwrap().kind {
            TokenKind::String(x) => {
                self.get_token(); // consume token
                Node::Primary(Value::String(x))
            }
            _ => self.parse_expr(),
        }
    }

    fn parse_assign(&mut self) -> Node {
        info!("Parsing assign");

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
        info!("Parsing expresion");

        let left = self.parse_term();
        let optok = self.peek_token();
        if let Some(x) = optok {
            let operator = match x.kind {
                TokenKind::Symbol(SymbolKind::Plus) => Op::Plus,
                TokenKind::Symbol(SymbolKind::Minus) => Op::Minus,
                TokenKind::Symbol(SymbolKind::Greater) => Op::Greater,
                TokenKind::Symbol(SymbolKind::GreaterEquals) => Op::GreaterEqual,
                TokenKind::Symbol(SymbolKind::Less) => Op::Less,
                TokenKind::Symbol(SymbolKind::LessEquals) => Op::LessEqual,
                TokenKind::Symbol(SymbolKind::DoubleEquals) => Op::EqualTo,
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
        info!("Parsing term");
        // for now, we will skip this
        self.parse_factor()
    }

    fn parse_factor(&mut self) -> Node {
        info!("Parsing factor");
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
        let tok = self.tokens.pop().unwrap();
        info!("Get token: {:?}", tok);
        tok
    }

    fn peek_token(&mut self) -> Option<Token> {
        let tok = self.tokens.clone().pop();
        info!("Peek token: {:?}", tok);
        tok
    }
}
