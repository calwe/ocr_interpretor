use log::{info, warn};

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
                        TokenKind::Symbol(SymbolKind::LeftSqBracket) => {
                            nodes.push(self.parse_array_assign_ind());
                        }
                        TokenKind::Symbol(SymbolKind::Dot) => {
                            nodes.push(self.parse_dot_expr());
                        }
                        _ => unimplemented!("unimplemented ident"),
                    }
                }
                TokenKind::Keyword(KeywordKind::Array) => {
                    nodes.push(self.parse_array());
                }
                TokenKind::Keyword(KeywordKind::If) => {
                    nodes.push(self.parse_if());
                }
                TokenKind::Keyword(KeywordKind::While) => {
                    nodes.push(self.parse_while());
                }
                TokenKind::Keyword(KeywordKind::EndIf)
                | TokenKind::Keyword(KeywordKind::EndWhile) => {
                    warn!("return from block");
                    return Ok(Node::Block(nodes));
                }
                TokenKind::Keyword(KeywordKind::Else) => {
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
        self.get_token(); // consume "endwhile"

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
            _ => self.parse_expr(),
        };
        Node::Assign {
            ident,
            value: Box::new(expr),
        }
    }

    fn parse_array_assign_ind(&mut self) -> Node {
        info!("Parsing assign to array index");

        let token = self.get_token();
        let ident = match token.kind {
            TokenKind::Ident(x) => x,
            _ => panic!("array assign must have ident"),
        };

        let index = match self.get_token().kind {
            TokenKind::Symbol(SymbolKind::LeftSqBracket) => self.parse_expr(),
            _ => panic!("array must be indexed with ["),
        };

        self.get_token(); // consume '['
        let value = match self.get_token().kind {
            TokenKind::Symbol(SymbolKind::Equals) => self.parse_expr(),
            _ => panic!("Must assign array with ="),
        };

        Node::ArrayAssingIndex {
            ident,
            index: Box::new(index),
            value: Box::new(value),
        }
    }

    fn parse_array(&mut self) -> Node {
        info!("Parsing array");

        self.get_token(); // consume 'array'
        let ident = match self.get_token().kind {
            TokenKind::Ident(x) => x,
            _ => panic!("array must have ident"),
        };

        let size = match self.get_token().kind {
            TokenKind::Symbol(SymbolKind::LeftSqBracket) => self.parse_expr(),
            _ => panic!("array must have ["),
        };

        self.get_token(); // consume final '['

        Node::ArrayAssign {
            ident,
            size: Box::new(size),
        }
    }

    fn parse_dot_expr(&mut self) -> Node {
        info!("Parsing dot expr");

        let left = match self.get_token().kind {
            TokenKind::Ident(x) => x,
            _ => panic!("Dot expressions only supports idents currently"),
        };

        self.get_token(); // consume '.'
        let right = match self.get_token().kind {
            TokenKind::Ident(x) => x,
            _ => panic!("Dot expression rvalue must be ident"),
        };

        Node::DotExpr { left, right }
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
        let left = self.parse_factor();
        let optok = self.peek_token();
        if let Some(x) = optok {
            let operator = match x.kind {
                TokenKind::Symbol(SymbolKind::Multiply) => Op::Multiply,
                TokenKind::Symbol(SymbolKind::Divide) => Op::Divide,
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

    fn parse_factor(&mut self) -> Node {
        info!("Parsing factor");
        let token = self.peek_token().unwrap();
        match token.kind {
            TokenKind::Number(x) => {
                self.get_token();
                Node::Primary(Value::Number(x))
            }
            TokenKind::String(x) => {
                self.get_token();
                Node::Primary(Value::String(x))
            }
            TokenKind::Ident(x) => {
                let mut peekpeek = self.tokens.clone();
                peekpeek.pop();
                if let Some(x) = peekpeek.pop() {
                    if x.kind == TokenKind::Symbol(SymbolKind::LeftBracket) {
                        return self.parse_func_call();
                    } else if x.kind == TokenKind::Symbol(SymbolKind::LeftSqBracket) {
                        info!("Array ref as factor");
                        return self.parse_array_ref();
                    } else if x.kind == TokenKind::Symbol(SymbolKind::Dot) {
                        info!("Dot as factor");
                        return self.parse_dot_expr();
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

    fn parse_array_ref(&mut self) -> Node {
        let ident = match self.get_token().kind {
            TokenKind::Ident(x) => x,
            _ => panic!("array ref must have ident"),
        };

        let index = match self.get_token().kind {
            TokenKind::Symbol(SymbolKind::LeftSqBracket) => self.parse_expr(),
            _ => panic!("array index must be specified with square brackets"),
        };

        self.get_token(); // consume final ']'

        Node::ArrayRef {
            ident,
            index: Box::new(index),
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

    #[cfg(test)]
    pub fn parse_from_list(token_kinds: Vec<TokenKind>) -> Result<Node, ParserError> {
        use crate::Position;

        let tokens = token_kinds
            .iter()
            .map(|x| Token::new(x.clone(), Position::new(0, 0), 0))
            .collect();

        let mut parser = Self::new(tokens, String::new());

        parser.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_assign() {
        let input = vec![
            TokenKind::Ident("num".to_string()),
            TokenKind::Symbol(SymbolKind::Equals),
            TokenKind::Number(10),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::Assign {
                ident: "num".to_string(),
                value: Box::new(Node::Primary(Value::Number(10)))
            }])
        );
    }

    #[test]
    fn binary_assign() {
        let input = vec![
            TokenKind::Ident("num".to_string()),
            TokenKind::Symbol(SymbolKind::Equals),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::Plus),
            TokenKind::Number(5),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::Assign {
                ident: "num".to_string(),
                value: Box::new(Node::BinaryExpr {
                    left: Box::new(Node::Primary(Value::Number(10))),
                    operator: Op::Plus,
                    right: Box::new(Node::Primary(Value::Number(5)))
                })
            }])
        );
    }

    #[test]
    fn nested_binary_assign() {
        let input = vec![
            TokenKind::Ident("num".to_string()),
            TokenKind::Symbol(SymbolKind::Equals),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::Plus),
            TokenKind::Number(5),
            TokenKind::Symbol(SymbolKind::Multiply),
            TokenKind::Number(2),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::Assign {
                ident: "num".to_string(),
                value: Box::new(Node::BinaryExpr {
                    left: Box::new(Node::Primary(Value::Number(10))),
                    operator: Op::Plus,
                    right: Box::new(Node::BinaryExpr {
                        left: Box::new(Node::Primary(Value::Number(5))),
                        operator: Op::Multiply,
                        right: Box::new(Node::Primary(Value::Number(2)))
                    })
                })
            }])
        );
    }

    #[test]
    fn nested_binary_assign_with_parens() {
        let input = vec![
            TokenKind::Ident("num".to_string()),
            TokenKind::Symbol(SymbolKind::Equals),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::Plus),
            TokenKind::Number(5),
            TokenKind::Symbol(SymbolKind::RightBracket),
            TokenKind::Symbol(SymbolKind::Multiply),
            TokenKind::Number(2),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::Assign {
                ident: "num".to_string(),
                value: Box::new(Node::BinaryExpr {
                    left: Box::new(Node::BinaryExpr {
                        left: Box::new(Node::Primary(Value::Number(10))),
                        operator: Op::Plus,
                        right: Box::new(Node::Primary(Value::Number(5)))
                    }),
                    operator: Op::Multiply,
                    right: Box::new(Node::Primary(Value::Number(2)))
                })
            }])
        );
    }

    #[test]
    fn string_assign() {
        let input = vec![
            TokenKind::Ident("str".to_string()),
            TokenKind::Symbol(SymbolKind::Equals),
            TokenKind::String("hello world".to_string()),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::Assign {
                ident: "str".to_string(),
                value: Box::new(Node::Primary(Value::String("hello world".to_string())))
            }])
        );
    }

    #[test]
    fn function_call() {
        let input = vec![
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::String("hello world".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::FuncCall {
                ident: "print".to_string(),
                args: vec![Node::Primary(Value::String("hello world".to_string()))]
            }])
        );
    }

    #[test]
    fn function_call_with_variable_ref() {
        let input = vec![
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::Ident("str".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::FuncCall {
                ident: "print".to_string(),
                args: vec![Node::VariableRef("str".to_string())]
            }])
        );
    }

    #[test]
    fn function_call_with_return_val() {
        let input = vec![
            TokenKind::Ident("in".to_string()),
            TokenKind::Symbol(SymbolKind::Equals),
            TokenKind::Ident("input".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::Symbol(SymbolKind::RightBracket),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::Assign {
                ident: "in".to_string(),
                value: Box::new(Node::FuncCall {
                    ident: "input".to_string(),
                    args: vec![]
                })
            }])
        );
    }

    #[test]
    fn if_statement() {
        let input = vec![
            TokenKind::Keyword(KeywordKind::If),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::Greater),
            TokenKind::Number(5),
            TokenKind::Keyword(KeywordKind::Then),
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::String("hello world".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
            TokenKind::Keyword(KeywordKind::EndIf),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::IfExpr {
                expr: Box::new(Node::BinaryExpr {
                    left: Box::new(Node::Primary(Value::Number(10))),
                    operator: Op::Greater,
                    right: Box::new(Node::Primary(Value::Number(5)))
                }),
                then: Box::new(Node::Block(vec![Node::FuncCall {
                    ident: "print".to_string(),
                    args: vec![Node::Primary(Value::String("hello world".to_string()))]
                }])),
                els: Box::new(Node::Block(vec![]))
            }])
        );
    }

    #[test]
    fn if_else_statement() {
        let input = vec![
            TokenKind::Keyword(KeywordKind::If),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::Greater),
            TokenKind::Number(5),
            TokenKind::Keyword(KeywordKind::Then),
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::String("hello world".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
            TokenKind::Keyword(KeywordKind::Else),
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::String("goodbye world".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
            TokenKind::Keyword(KeywordKind::EndIf),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::IfExpr {
                expr: Box::new(Node::BinaryExpr {
                    left: Box::new(Node::Primary(Value::Number(10))),
                    operator: Op::Greater,
                    right: Box::new(Node::Primary(Value::Number(5)))
                }),
                then: Box::new(Node::Block(vec![Node::FuncCall {
                    ident: "print".to_string(),
                    args: vec![Node::Primary(Value::String("hello world".to_string()))]
                }])),
                els: Box::new(Node::Block(vec![Node::FuncCall {
                    ident: "print".to_string(),
                    args: vec![Node::Primary(Value::String("goodbye world".to_string()))]
                }]))
            }])
        );
    }

    #[test]
    fn while_loop() {
        let input = vec![
            TokenKind::Keyword(KeywordKind::While),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::Greater),
            TokenKind::Number(5),
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::String("hello world".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
            TokenKind::Keyword(KeywordKind::EndWhile),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::WhileStmt {
                expr: Box::new(Node::BinaryExpr {
                    left: Box::new(Node::Primary(Value::Number(10))),
                    operator: Op::Greater,
                    right: Box::new(Node::Primary(Value::Number(5)))
                }),
                body: Box::new(Node::Block(vec![Node::FuncCall {
                    ident: "print".to_string(),
                    args: vec![Node::Primary(Value::String("hello world".to_string()))]
                }]))
            }])
        );
    }

    #[test]
    fn conditional_order_of_operations() {
        let input = vec![
            TokenKind::Keyword(KeywordKind::If),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::Plus),
            TokenKind::Number(5),
            TokenKind::Symbol(SymbolKind::Greater),
            TokenKind::Number(5),
            TokenKind::Keyword(KeywordKind::Then),
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::String("hello world".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
            TokenKind::Keyword(KeywordKind::EndIf),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::IfExpr {
                expr: Box::new(Node::BinaryExpr {
                    left: Box::new(Node::BinaryExpr {
                        left: Box::new(Node::Primary(Value::Number(10))),
                        operator: Op::Plus,
                        right: Box::new(Node::Primary(Value::Number(5)))
                    }),
                    operator: Op::Greater,
                    right: Box::new(Node::Primary(Value::Number(5)))
                }),
                then: Box::new(Node::Block(vec![Node::FuncCall {
                    ident: "print".to_string(),
                    args: vec![Node::Primary(Value::String("hello world".to_string()))]
                }])),
                els: Box::new(Node::Block(vec![]))
            }])
        );
    }

    #[test]
    fn conditional_order_of_operations_2() {
        let input = vec![
            TokenKind::Keyword(KeywordKind::If),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::Greater),
            TokenKind::Number(5),
            TokenKind::Symbol(SymbolKind::Plus),
            TokenKind::Number(5),
            TokenKind::Keyword(KeywordKind::Then),
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::String("hello world".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
            TokenKind::Keyword(KeywordKind::EndIf),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::IfExpr {
                expr: Box::new(Node::BinaryExpr {
                    left: Box::new(Node::Primary(Value::Number(10))),
                    operator: Op::Greater,
                    right: Box::new(Node::BinaryExpr {
                        left: Box::new(Node::Primary(Value::Number(5))),
                        operator: Op::Plus,
                        right: Box::new(Node::Primary(Value::Number(5)))
                    })
                }),
                then: Box::new(Node::Block(vec![Node::FuncCall {
                    ident: "print".to_string(),
                    args: vec![Node::Primary(Value::String("hello world".to_string()))]
                }])),
                els: Box::new(Node::Block(vec![]))
            }])
        );
    }

    #[test]
    fn array_decl() {
        let input = vec![
            TokenKind::Keyword(KeywordKind::Array),
            TokenKind::Ident("arr".to_string()),
            TokenKind::Symbol(SymbolKind::LeftSqBracket),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::RightSqBracket),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::ArrayAssign {
                ident: "arr".to_string(),
                size: Box::new(Node::Primary(Value::Number(10)))
            }])
        );
    }

    #[test]
    fn array_assign_index() {
        let input = vec![
            TokenKind::Ident("arr".to_string()),
            TokenKind::Symbol(SymbolKind::LeftSqBracket),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::RightSqBracket),
            TokenKind::Symbol(SymbolKind::Equals),
            TokenKind::Number(5),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::ArrayAssingIndex {
                ident: "arr".to_string(),
                index: Box::new(Node::Primary(Value::Number(10))),
                value: Box::new(Node::Primary(Value::Number(5)))
            }])
        );
    }

    #[test]
    fn array_ref_index() {
        let input = vec![
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::Ident("arr".to_string()),
            TokenKind::Symbol(SymbolKind::LeftSqBracket),
            TokenKind::Number(10),
            TokenKind::Symbol(SymbolKind::RightSqBracket),
            TokenKind::Symbol(SymbolKind::RightBracket),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::FuncCall {
                ident: "print".to_string(),
                args: vec![Node::ArrayRef {
                    ident: "arr".to_string(),
                    index: Box::new(Node::Primary(Value::Number(10)))
                }]
            }])
        );
    }

    #[test]
    fn variable_ref() {
        let input = vec![
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::Ident("foo".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::FuncCall {
                ident: "print".to_string(),
                args: vec![Node::VariableRef("foo".to_string())]
            }])
        );
    }

    #[test]
    fn dot_expr() {
        let input = vec![
            TokenKind::Ident("print".to_string()),
            TokenKind::Symbol(SymbolKind::LeftBracket),
            TokenKind::Ident("foo".to_string()),
            TokenKind::Symbol(SymbolKind::Dot),
            TokenKind::Ident("length".to_string()),
            TokenKind::Symbol(SymbolKind::RightBracket),
        ];

        assert_eq!(
            Parser::parse_from_list(input).unwrap(),
            Node::Block(vec![Node::FuncCall {
                ident: "print".to_string(),
                args: vec![Node::DotExpr {
                    left: "foo".to_string(),
                    right: "length".to_string()
                }]
            }])
        );
    }
}
