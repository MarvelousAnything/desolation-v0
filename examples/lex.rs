use std::fs;
use log::{error, info};
use desolation::lex::Lexer;

fn main() {
    sensible_env_logger::init!();
    let mut lexer = Lexer::new();

    let source = fs::read_to_string("examples/sq.t").unwrap();
    info!("{:#?}", source);
    let tokens = lexer.lex(source);
    match tokens {
        Ok(tokens) => {
            for token in &tokens.tokens {
                info!("{:?}", token);
            }
            println!("Keywords:\n{}", tokens.get_keywords());
            println!("Identifiers:\n{}", tokens.get_identifiers());
            println!("Integers:\n{}", tokens.get_integer_literals());
            println!("Strings:\n{}", tokens.get_string_literals());
            println!("Characters:\n{}", tokens.get_character_literals());
        }
        Err(e) => {
            error!("Failed to lex: {}", e);
        }
    }
}