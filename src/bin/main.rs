use ocr_language::{interpretor::Interpretor, lexer::Lexer, parser::Parser};

pub fn main() {
    env_logger::init();

    // TODO: Parse function with return value
    let input = r#"
n = int(input("How many fibonacci numbers: "))
a = 0
b = 1
count = 2
print(a)
print(b)
while count < n
    next = a + b
    print(next)
    a = b
    b = next
    count = count + 1
endwhile
"#;
    println!("Input program:");
    println!("{}", input);

    println!();

    let mut lexer = Lexer::new(input.to_string());
    if let Err(e) = lexer.lex() {
        println!("Error while lexing:");
        println!("{}", e);
        return;
    }
    let tokens = lexer.tokens;

    println!("Tokens:");
    for token in tokens.clone() {
        println!("{:?}", token.kind);
    }

    println!();

    println!("AST:");
    let mut parser = Parser::new(tokens, input.clone().to_string());
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("{:#?}", ast);

    println!();

    println!("Running program:");
    let mut interpretor = Interpretor::new(Box::new(ast));
    interpretor.run();
}
