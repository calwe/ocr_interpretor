use std::fs;

use clap::Parser as CParser;
use ocr_language::{interpretor::Interpretor, lexer::Lexer, parser::Parser};

#[derive(CParser)]
#[command(name = "OCR Interpretor")]
#[command(author = "Callum W. <spikywebb@gmail.com>")]
#[command(version = "0.1a")]
#[command(about = "An interpretor built for the OCR Reference Language", long_about = None)]
struct Cli {
    /// Display debug info such as the AST
    #[arg(short, long)]
    debug: bool,

    /// The program that should be run
    program: String,
}

pub fn main() {
    env_logger::init();
    let cli = Cli::parse()

    let input = fs::read_to_string(cli.program).unwrap();

    if cli.debug {
        println!("Input program:");
        println!("{}", input);
        println!();
    }

    let mut lexer = Lexer::new(input.to_string());
    if let Err(e) = lexer.lex() {
        println!("Error while lexing:");
        println!("{}", e);
        return;
    }
    let tokens = lexer.tokens;

    if cli.debug {
        println!("Tokens:");
        for token in tokens.clone() {
            println!("{:?}", token.kind);
        }
        println!();
        println!("AST:");
    }

    let mut parser = Parser::new(tokens, input.clone().to_string());
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if cli.debug {
        println!("{:#?}", ast);
        println!();
        println!("Running program:");
    }

    let mut interpretor = Interpretor::new(Box::new(ast));
    interpretor.run();
}
