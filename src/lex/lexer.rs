use crate::lex::token::{Token, TokenType};
use crate::lex::types::{KeywordToken, LiteralToken};
use anyhow::{bail, ensure, Result};
use log::debug;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    index: usize,
    curr_char: char,
    line_no: usize,
    col_no: usize,
    length: usize,
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unexpected character: {0:?} at {1}:{2}")]
    InvalidCharacter(char, usize, usize),
    #[error("Invalid string literal at {0}:{1}")]
    InvalidStringLiteral(usize, usize),
    #[error("Invalid character literal at {0}:{1}")]
    InvalidCharacterLiteral(usize, usize),
    #[error("Invalid integer literal")]
    InvalidIntegerLiteral(#[from] std::num::ParseIntError),
    #[error("Invalid identifier at {0}:{1}")]
    InvalidIdentifier(usize, usize),
    #[error("Invalid comment at {0}:{1}")]
    InvalidComment(usize, usize),
    #[error("Unexpected EOF at {0}:{1}")]
    InvalidEOF(usize, usize),
    #[error("Unexpected EOL at {0}:{1}")]
    InvalidEOL(usize, usize),
    #[error("Unknown lexer error at {0}:{1}")]
    Unknown(usize, usize),
}

#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            writeln!(f, "{:?}", token)?;
        }
        Ok(())
    }
}

impl TokenStream {
    pub fn get_keywords(&self) -> Self {
        self.tokens
            .iter()
            .filter(|t| t.is_keyword())
            .cloned()
            .collect::<TokenStream>()
    }

    pub fn get_identifiers(&self) -> Self {
        self.tokens
            .iter()
            .filter(|t| t.is_identifier())
            .cloned()
            .collect::<TokenStream>()
    }

    pub fn get_integer_literals(&self) -> Self {
        self.tokens
            .iter()
            .filter(|t| t.is_integer_literal())
            .cloned()
            .collect::<TokenStream>()
    }

    pub fn get_string_literals(&self) -> Self {
        self.tokens
            .iter()
            .filter(|t| t.is_string_literal())
            .cloned()
            .collect::<TokenStream>()
    }

    pub fn get_character_literals(&self) -> Self {
        self.tokens
            .iter()
            .filter(|t| t.is_character_literal())
            .cloned()
            .collect::<TokenStream>()
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        let tokens = iter.into_iter().collect();
        TokenStream { tokens }
    }
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            source: String::new(),
            index: 0,
            curr_char: '\0',
            line_no: 1,
            col_no: 1,
            length: 0,
        }
    }

    pub fn lex(&mut self, source: String) -> Result<TokenStream> {
        self.source = source;
        self.length = self.source.len();
        self.curr_char = self.get_curr()?;
        let mut tokens = Vec::new();
        while self.has_next() {
            tokens.push(self.get_next_token()?);
        }
        if !self.has_next() {
            debug!("No more tokens to lex");
            tokens.push(TokenType::Eof.at(self.index, self.line_no, self.col_no));
        }

        info!("Lexed {} tokens", tokens.len());

        // Post processing.
        // fold consecutive NL tokens into one.
        // find a NL token, then delete all the NL tokens until the next non-NL token.
        let indices = tokens
            .iter()
            .enumerate()
            .filter(|(_, t)| t.token_type() == TokenType::NL)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        let indices_folded = indices
            .iter()
            .fold((Vec::new(), Vec::new()), |mut acc, i| {
                if let Some(last) = acc.0.last() {
                    if i - last == 1 {
                        acc.1.push(*last);
                        acc.0.pop();
                    }
                }
                acc.0.push(*i);
                acc
            })
            .1;

        for i in indices_folded.iter().rev() {
            tokens.remove(*i);
        }

        info!("Folded {} NL tokens", indices_folded.len());
        info!("Now {} tokens", tokens.len());

        Ok(TokenStream::from_iter(tokens))
    }

    fn has_next(&self) -> bool {
        self.index < self.length
    }

    fn reset(&mut self) {
        self.index = 0;
        self.curr_char = self.source.chars().next().unwrap();
        self.line_no = 1;
        self.col_no = 1;
        self.length = self.source.chars().count();
    }

    fn get_curr(&self) -> Result<char> {
        ensure!(
            self.has_next(),
            LexerError::InvalidEOF(self.line_no, self.col_no)
        );
        let c = self.source.chars().nth(self.index).unwrap();
        trace!(
            "Got current character: {:?} at {}:{}[{}]",
            c,
            self.line_no,
            self.col_no,
            self.index
        );
        Ok(c)
    }

    fn advance(&mut self) -> Result<()> {
        self.index += 1;
        self.col_no += 1;
        if self.has_next() {
            self.curr_char = self.get_curr()?;
            // trace!("Advanced to {}:{}[{}]", self.line_no, self.col_no, self.index);
        }
        Ok(())
    }

    fn advance_n(&mut self, n: usize) -> Result<()> {
        for _ in 0..n {
            self.advance()?;
        }
        Ok(())
    }

    fn advance_until(&mut self, pred: Box<dyn Fn() -> bool>) -> Result<()> {
        while self.has_next() && !pred() {
            self.advance()?;
        }
        Ok(())
    }

    fn advance_eol(&mut self) -> Result<()> {
        if let Some(n) = self.find_next('\n') {
            self.advance_n(n)?;
        } else {
            bail!(LexerError::InvalidEOL(self.line_no, self.col_no));
        }
        Ok(())
    }

    fn consume(&mut self) -> Result<char> {
        let c = self.curr_char;
        self.advance()?;
        Ok(c)
    }

    fn collect(&mut self, n: usize) -> Result<&str> {
        let start = self.index;
        let end = self.index + n;
        self.advance_n(n)?;
        Ok(&self.source[start..end])
    }

    fn find_next(&mut self, c: char) -> Option<usize> {
        let mut i = self.index;
        while i < self.length {
            if self.source.chars().nth(i).unwrap() == c {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    // Advance the index until the current character is not a whitespace character. This excludes \n
    fn skip_whitespace(&mut self) -> Result<()> {
        while self.is_whitespace() && self.has_next() {
            self.advance()?;
        }
        Ok(())
    }

    fn collect_string(&mut self) -> Result<String> {
        let mut result = String::new();
        while self.curr_char != '"' {
            result.push(self.consume()?);
        }
        self.advance()?;
        Ok(result)
    }

    fn collect_identifier(&mut self) -> Result<String> {
        let mut result = String::new();
        while self.curr_char.is_alphanumeric() {
            result.push(self.consume()?);
        }
        Ok(result)
    }

    fn collect_integer(&mut self) -> Result<i64> {
        let mut result = String::new();
        while self.curr_char.is_numeric() && self.has_next() {
            result.push(self.consume()?);
        }
        let parsed = result.parse::<i64>()?;
        Ok(parsed)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self.curr_char, ' ' | '\t' | '\r' | '\x0B' | '\x0C')
    }

    fn get_next_token(&mut self) -> Result<Token> {
        let start = (self.index, self.line_no, self.col_no);
        let token = match self.curr_char {
            n if self.is_whitespace() => {
                trace!(
                    "Found whitespace {:?} at {}:{}[{}]",
                    n,
                    self.line_no,
                    self.col_no,
                    self.index
                );
                self.skip_whitespace()?;
                debug!(
                    "Skipped {} whitespace from {}:{}[{}] to {}:{}[{}]",
                    self.index - start.0,
                    start.1,
                    start.2,
                    start.0,
                    self.line_no,
                    self.col_no,
                    self.index
                );
                return self.get_next_token();
            }
            '\n' => {
                trace!(
                    "Found newline at {}:{}[{}]",
                    self.line_no,
                    self.col_no,
                    self.index
                );
                // fold newlines into a single token
                self.col_no = 0;
                self.line_no += 1;
                self.advance()?;
                TokenType::NL
            }
            // is alphabetic ensures the the identifies starts with a letter. This may not be the behaviour I want.
            // TODO: Look into this.
            n if n.is_alphabetic() => {
                let identifier = self.collect_identifier()?;
                if let Some(keyword) = KeywordToken::from_str(&identifier) {
                    debug!(
                        "Found keyword {:?} at {}:{}[{}]",
                        keyword, self.line_no, self.col_no, self.index
                    );
                    TokenType::Keyword(keyword)
                } else {
                    debug!(
                        "Found identifier {:?} at {}:{}[{}]",
                        identifier, self.line_no, self.col_no, self.index
                    );
                    TokenType::IdentifierToken(identifier)
                }
            }
            n if n.is_numeric() => {
                let integer = self.collect_integer()?;
                debug!(
                    "Collected integer: {} at {}:{}[{}] to {}:{}[{}]",
                    integer, start.1, start.2, start.0, self.line_no, self.col_no, self.index
                );
                TokenType::Literal(LiteralToken::Integer(integer))
            }
            '"' => {
                self.advance()?;
                let string = self.collect_string()?;
                debug!(
                    "Collected string: \"{}\" at {}:{}[{}] to {}:{}[{}]",
                    string, start.1, start.2, start.0, self.line_no, self.col_no, self.index
                );
                TokenType::Literal(LiteralToken::String(string))
            }
            '\'' => {
                self.advance()?;
                let character = self.consume()?;
                ensure!(
                    self.consume()? == '\'',
                    LexerError::InvalidCharacterLiteral(self.line_no, self.col_no)
                );
                debug!(
                    "Collected character literal: {:?} at {}:{}[{}] to {}:{}[{}]",
                    character, start.1, start.2, start.0, self.line_no, self.col_no, self.index
                );
                TokenType::Literal(LiteralToken::Character(character))
            }
            '#' => {
                debug!(
                    "Found comment at {}:{}[{}]",
                    self.line_no, self.col_no, self.index
                );
                self.advance_eol()?;
                return self.get_next_token();
            }
            _ => TokenType::from_char(self.consume()?),
        }
        .at(start.0, start.1, start.2);
        Ok(token)
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}
