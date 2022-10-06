use ocr_language::{lexer::Lexer, parser::Parser};

pub fn main() {
    let input = r#"
attempts = 5
test = attempts - 18
print("hello world!")
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
