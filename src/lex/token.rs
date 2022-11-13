use crate::lex::types::{KeywordToken, LiteralToken, SyntaxToken};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    Keyword(KeywordToken),
    Syntax(SyntaxToken),
    IdentifierToken(String),
    Literal(LiteralToken),
    Unknown(char),
    Eof,
    NL,
}

impl TokenType {
    pub fn at(self, index: usize, line_no: usize, col_no: usize) -> Token {
        Token {
            token_type: self,
            index,
            line_no,
            col_no,
        }
    }

    pub fn from_char(c: char) -> Self {
        SyntaxToken::from_char(c)
            .map(TokenType::Syntax)
            .unwrap_or_else(|| TokenType::Unknown(c))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    index: usize,
    line_no: usize,
    col_no: usize,
}

impl Token {
    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self.token_type, TokenType::Keyword(_))
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.token_type, TokenType::IdentifierToken(_))
    }

    pub fn is_integer_literal(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::Literal(LiteralToken::Integer(_))
        )
    }

    pub fn is_string_literal(&self) -> bool {
        matches!(self.token_type, TokenType::Literal(LiteralToken::String(_)))
    }

    pub fn is_character_literal(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::Literal(LiteralToken::Character(_))
        )
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn line_no(&self) -> usize {
        self.line_no
    }

    pub fn col_no(&self) -> usize {
        self.col_no
    }
}
