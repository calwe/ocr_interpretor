type Num = u64;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Ident(String),
    String(String),
    Number(Num),
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
                '=' | '<' | '>' | '(' | ')' | '+' | '-' => {
                    let peek = self.peek_char();
                    match peek {
                        '=' => {
                            self.symbol(c, peek);
                            self.input.pop(); // consume symbol
                        }
                        _ => self.symbol(c, ' '),
                    }
                }
                '"' => self.string(),
                '0'..='9' => self.numeric(c),
                'a'..='z' | 'A'..='Z' | '_' => self.ident_or_keyword(c),
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
            "( " => self.push_symbol(SymbolKind::LeftBracket),
            ") " => self.push_symbol(SymbolKind::RightBracket),
            "+ " => self.push_symbol(SymbolKind::Plus),
            "+=" => self.push_symbol(SymbolKind::PlusEqual),
            "- " => self.push_symbol(SymbolKind::Minus),
            "-=" => self.push_symbol(SymbolKind::MinusEqual),
            _ => unimplemented!(),
        }
    }

    fn string(&mut self) {
        let mut string = String::new();
        while self.peek_char() != '"' && self.peek_char() != '\0' {
            // PANIC: Unwrap should be safe as we verify the character exists
            string.push(self.input.pop().unwrap());
        }
        self.input.pop(); // consume '"'
        self.push_string(string);
    }

    /// Lexes a multi-digit number, but requires the first digit of the number
    /// as it is already consumed
    fn numeric(&mut self, start: char) {
        let mut strnum = String::new();
        strnum.push(start);
        while self.peek_char().is_numeric() {
            // PANIC: Unwrap should be safe as we verify the character is numeric
            strnum.push(self.input.pop().unwrap());
        }
        // PANIC: I think we should be fine here, as all of the characters in strnum
        //          should be verified as being numeric
        let number = strnum
            .parse::<Num>()
            .expect(format!("strnum is not a number! strnum: {}", strnum).as_str());
        self.push_number(number);
    }

    fn ident_or_keyword(&mut self, first: char) {
        let mut ident = String::new();
        ident.push(first);
        while self.peek_char().is_alphanumeric() || self.peek_char() == '_' {
            // PANIC: Unwrap should be safe as we verify the character exists
            ident.push(self.input.pop().unwrap());
        }
        match ident.as_str() {
            "do" => self.push_keyword(KeywordKind::Do),
            "while" => self.push_keyword(KeywordKind::While),
            "endwhile" => self.push_keyword(KeywordKind::EndWhile),
            "if" => self.push_keyword(KeywordKind::If),
            "else" => self.push_keyword(KeywordKind::Else),
            "endif" => self.push_keyword(KeywordKind::EndIf),
            "break" => self.push_keyword(KeywordKind::Break),
            _ => self.push_ident(ident),
        }
    }

    /// Pushes a symbol token onto our list of tokens
    fn push_symbol(&mut self, symbol: SymbolKind) {
        self.tokens.push(Token::new(TokenKind::Symbol(symbol)));
    }

    /// Pushes a string token onto our list of tokens
    fn push_string(&mut self, string: String) {
        self.tokens.push(Token::new(TokenKind::String(string)));
    }

    /// Pushes a number token onto our list of tokens
    fn push_number(&mut self, number: Num) {
        self.tokens.push(Token::new(TokenKind::Number(number)));
    }

    /// Pushes a keyword token onto our list of tokens
    fn push_keyword(&mut self, keyword: KeywordKind) {
        self.tokens.push(Token::new(TokenKind::Keyword(keyword)));
    }

    /// Pushes an indentifier token onto our list of tokens
    fn push_ident(&mut self, ident: String) {
        self.tokens.push(Token::new(TokenKind::Ident(ident)));
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
    /// A test utility function that will return a vector of all of
    /// the token kinds. This is because for basic lexing tests we
    /// can simply ignore any debug infomation bundled in our tokens.
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

    #[test]
    fn brackets() {
        let mut lexer = Lexer::new("()(()".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::Symbol(SymbolKind::LeftBracket),
                TokenKind::Symbol(SymbolKind::RightBracket),
                TokenKind::Symbol(SymbolKind::LeftBracket),
                TokenKind::Symbol(SymbolKind::LeftBracket),
                TokenKind::Symbol(SymbolKind::RightBracket),
            ]
        );
    }

    #[test]
    fn arithmetic() {
        let mut lexer = Lexer::new("+= + - -=".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::Symbol(SymbolKind::PlusEqual),
                TokenKind::Symbol(SymbolKind::Plus),
                TokenKind::Symbol(SymbolKind::Minus),
                TokenKind::Symbol(SymbolKind::MinusEqual),
            ]
        );
    }

    #[test]
    fn string() {
        let mut lexer = Lexer::new("\"this is a test string\" + 7".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::String("this is a test string".to_string()),
                TokenKind::Symbol(SymbolKind::Plus),
                TokenKind::Number(7),
            ]
        )
    }

    #[test]
    fn numeric() {
        let mut lexer = Lexer::new("325".to_string());
        lexer.lex();
        assert_eq!(lexer.token_kinds(), vec![TokenKind::Number(325)])
    }

    #[test]
    fn multi_numeric() {
        let mut lexer = Lexer::new("100 27".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![TokenKind::Number(100), TokenKind::Number(27)]
        )
    }

    #[test]
    fn symbol_numeric() {
        let mut lexer = Lexer::new("420 >= 3158".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::Number(420),
                TokenKind::Symbol(SymbolKind::GreaterEquals),
                TokenKind::Number(3158)
            ]
        )
    }

    #[test]
    fn keyword_while() {
        let mut lexer = Lexer::new("do while break endwhile".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::Keyword(KeywordKind::Do),
                TokenKind::Keyword(KeywordKind::While),
                TokenKind::Keyword(KeywordKind::Break),
                TokenKind::Keyword(KeywordKind::EndWhile),
            ]
        )
    }

    #[test]
    fn keyword_if() {
        let mut lexer = Lexer::new("if else endif".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::Keyword(KeywordKind::If),
                TokenKind::Keyword(KeywordKind::Else),
                TokenKind::Keyword(KeywordKind::EndIf),
            ]
        )
    }

    #[test]
    fn ident() {
        let mut lexer = Lexer::new("apples".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![TokenKind::Ident("apples".to_string())]
        )
    }

    #[test]
    fn ident_mixed() {
        let mut lexer = Lexer::new("attempts = 17".to_string());
        lexer.lex();
        assert_eq!(
            lexer.token_kinds(),
            vec![
                TokenKind::Ident("attempts".to_string()),
                TokenKind::Symbol(SymbolKind::Equals),
                TokenKind::Number(17),
            ]
        )
    }
}
