use ocr_language::{interpretor::Interpretor, lexer::Lexer, parser::Parser};

pub fn main() {
    let input = r#"print("hello ocr!")"#;

    println!("Input program:");
    println!("{}", input);

    println!();

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
    let ast = parser.parse();
    println!("{:#?}", ast);

    println!();

    println!("Running program:");
    let mut interpretor = Interpretor::new(ast);
    interpretor.run();
}
