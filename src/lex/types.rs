#[derive(Debug, Eq, PartialEq, Clone)]
pub enum KeywordToken {
    Var,
    Fun,
    If,
    Else,
    Until,
    Loop,
    Return,
}

impl KeywordToken {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "var" => Some(KeywordToken::Var),
            "fun" => Some(KeywordToken::Fun),
            "if" => Some(KeywordToken::If),
            "else" => Some(KeywordToken::Else),
            "until" => Some(KeywordToken::Until),
            "loop" => Some(KeywordToken::Loop),
            "return" => Some(KeywordToken::Return),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SyntaxToken {
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
}

impl SyntaxToken {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '{' => Some(SyntaxToken::LBrace),
            '}' => Some(SyntaxToken::RBrace),
            '(' => Some(SyntaxToken::LParen),
            ')' => Some(SyntaxToken::RParen),
            ':' => Some(SyntaxToken::Assign),
            ',' => Some(SyntaxToken::Comma),
            '.' => Some(SyntaxToken::Dot),
            '-' => Some(SyntaxToken::Minus),
            '!' => Some(SyntaxToken::Not),
            '+' => Some(SyntaxToken::Plus),
            '*' => Some(SyntaxToken::Times),
            '/' => Some(SyntaxToken::Slash),
            '&' => Some(SyntaxToken::And),
            '|' => Some(SyntaxToken::Or),
            '^' => Some(SyntaxToken::Xor),
            '%' => Some(SyntaxToken::Mod),
            '=' => Some(SyntaxToken::Eq),
            '<' => Some(SyntaxToken::Lt),
            '>' => Some(SyntaxToken::Gt),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LiteralToken {
    Character(char),
    Integer(i64),
    String(String),
}
