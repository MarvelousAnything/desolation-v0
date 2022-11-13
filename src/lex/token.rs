use crate::lex::keyword::Keyword;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    Keyword(Keyword),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Assign,
    Comma,
    Dot,
    Minus,
    Not,
    Plus,
    Times,
    Slash,
    And,
    Or,
    Xor,
    Mod,
    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geq,
    LShift,
    RShift,
    CharacterLiteral(char),
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    Unknown(char),
    Eof,
    NL
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
        match c {
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            ':' => TokenType::Assign,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '!' => TokenType::Not,
            '+' => TokenType::Plus,
            '*' => TokenType::Times,
            '/' => TokenType::Slash,
            '&' => TokenType::And,
            '|' => TokenType::Or,
            '^' => TokenType::Xor,
            '%' => TokenType::Mod,
            '<' => TokenType::Lt,
            '>' => TokenType::Gt,
            _ => TokenType::Unknown(c),
        }
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
        matches!(self.token_type, TokenType::Identifier(_))
    }

    pub fn is_integer_literal(&self) -> bool {
        matches!(self.token_type, TokenType::IntegerLiteral(_))
    }

    pub fn is_string_literal(&self) -> bool {
        matches!(self.token_type, TokenType::StringLiteral(_))
    }

    pub fn is_character_literal(&self) -> bool {
        matches!(self.token_type, TokenType::CharacterLiteral(_))
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
