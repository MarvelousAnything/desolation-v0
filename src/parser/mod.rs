use thiserror::Error;
use anyhow::{bail, ensure, Result};
use crate::ast;
use crate::ast::Program;
use crate::lex::lexer::TokenStream;
use crate::lex::token::Token;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unexpected token")]
    UnexpectedToken,
    #[error("Unexpected end of file")]
    UnexpectedEOF,
}

pub struct Parser {
    tokens: TokenStream,
    stack: Vec<Token>
}

pub enum ParserActions {
    Shift,
    Reduce(usize),
    Accept,
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens, stack: vec![] }
    }

    // This should parse a token stream into an AST.
    pub fn parse(&mut self) -> Result<Program> {
        todo!("Implement parser");
    }
}
