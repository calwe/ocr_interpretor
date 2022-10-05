use ocr_language::{lexer::Lexer, parser::Parser};

pub fn main() {
    let input = r#"
attempts = 10 + 8
test = 7
long = 10 + (9 - 10) + 18
"#;

    let mut lexer = Lexer::new(input.to_string());
    lexer.lex();
    let tokens = lexer.tokens;

    println!("Tokens:");
    for token in tokens.clone() {
        println!("{:?}", token.kind);
    }

    println!();

    println!("AST:");
    let mut parser = Parser::new(tokens);
    println!("{:#?}", parser.parse());
}
