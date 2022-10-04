#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Ident(String),
    String(String),
    Number(u64),
    Keyword(KeywordKind),
    Symbol(SymbolKind),
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeywordKind {
    Do,
    While,
    EndWhile,
    If,
    Then,
    Else,
    EndIf,
    Break,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolKind {
    // assignment / arithemtic
    Equals,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    // comparison
    DoubleEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    // other
    LeftBracket,
    RightBracket,
    Quote,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Debug)]
pub struct Lexer {
    input: String,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().rev().collect(), // reverse the input, as we pop from the end
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        // popping from a vector mutates the vector, meaning we can loop until its empty
        while !self.input.is_empty() {
            // PANIC: We cam safely unwrap as we check if the string is empty
            let c = self.input.pop().unwrap();
            match c {
                '\0' | ' ' | '\n' | '\r' => continue,
                '=' | '<' | '>' => {
                    let peek = self.peek_char();
                    match peek {
                        '=' => {
                            self.symbol(c, peek);
                            self.input.pop(); // consume symbol
                        }
                        _ => self.symbol(c, ' '),
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    /// Pushes a symbol token based on 2 characters.
    /// If the symbol only contains 1 character, the second should be passed as a space.
    fn symbol(&mut self, first: char, second: char) {
        let joined = format!("{}{}", first, second);
        match joined.as_str() {
            "==" => self.push_symbol(SymbolKind::DoubleEquals),
            "= " => self.push_symbol(SymbolKind::Equals),
            ">=" => self.push_symbol(SymbolKind::GreaterEquals),
            "> " => self.push_symbol(SymbolKind::Greater),
            "<=" => self.push_symbol(SymbolKind::LessEquals),
            "< " => self.push_symbol(SymbolKind::Less),
            _ => unimplemented!(),
        }
    }

    fn push_symbol(&mut self, symbol: SymbolKind) {
        self.tokens.push(Token::new(TokenKind::Symbol(symbol)));
    }

    /// Peeks the next character
    /// WARN: Returns a null byte if the character doesn't exist.
    fn peek_char(&self) -> char {
        match self.input.clone().pop() {
            Some(x) => x,
            None => '\0',
        }
    }

    #[cfg(test)]
    fn token_kinds(&self) -> Vec<TokenKind> {
        self.tokens.iter().map(|x| x.kind.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals() {
        let mut lexer = Lexer::new("== =".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::Symbol(SymbolKind::DoubleEquals),
                TokenKind::Symbol(SymbolKind::Equals)
            ]
        );
    }

    #[test]
    fn greater() {
        let mut lexer = Lexer::new(">= >".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::Symbol(SymbolKind::GreaterEquals),
                TokenKind::Symbol(SymbolKind::Greater)
            ]
        );
    }

    #[test]
    fn less() {
        let mut lexer = Lexer::new("<= <".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::Symbol(SymbolKind::LessEquals),
                TokenKind::Symbol(SymbolKind::Less)
            ]
        );
    }
}
