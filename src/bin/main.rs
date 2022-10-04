use ocr_language::lexer::Lexer;

pub fn main() {
    let mut lexer = Lexer::new("===".to_string());
    lexer.lex();
    let tokens = lexer.tokens;

    println!("Tokens:");
    for token in tokens {
        println!("{:?}", token.kind);
    }
}
