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
    
    pub fn length(&self) -> usize {
        match self {
            KeywordToken::Var => 3,
            KeywordToken::Fun => 3,
            KeywordToken::If => 2,
            KeywordToken::Else => 4,
            KeywordToken::Until => 5,
            KeywordToken::Loop => 4,
            KeywordToken::Return => 6,
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
    Mod,
    And,
    Or,
    Xor,
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
    pub fn from_char(c: char, next: Option<char>) -> Option<Self> {
        trace!("SyntaxToken::from_char: c = {:?}, next = {:?}", c, next);
        let token = match (c, next) {
            ('{', _) => Some(SyntaxToken::LBrace),
            ('}', _) => Some(SyntaxToken::RBrace),
            ('(', _) => Some(SyntaxToken::LParen),
            (')', _) => Some(SyntaxToken::RParen),
            (':', _) => Some(SyntaxToken::Assign),
            (',', _) => Some(SyntaxToken::Comma),
            ('.', _) => Some(SyntaxToken::Dot),
            ('-', _) => Some(SyntaxToken::Minus),
            ('!', Some('=')) => Some(SyntaxToken::Neq),
            ('!', _) => Some(SyntaxToken::Not),
            ('+', _) => Some(SyntaxToken::Plus),
            ('*', _) => Some(SyntaxToken::Times),
            ('/', _) => Some(SyntaxToken::Slash),
            ('%', _) => Some(SyntaxToken::Mod),
            ('&', _) => Some(SyntaxToken::And),
            ('|', _) => Some(SyntaxToken::Or),
            ('^', _) => Some(SyntaxToken::Xor),
            ('=', Some('=')) => Some(SyntaxToken::Eq),
            ('<', Some('=')) => Some(SyntaxToken::Leq),
            ('>', Some('=')) => Some(SyntaxToken::Geq),
            ('<', Some('<')) => Some(SyntaxToken::LShift),
            ('>', Some('>')) => Some(SyntaxToken::RShift),
            ('<', _) => Some(SyntaxToken::Lt),
            ('>', _) => Some(SyntaxToken::Gt),
            _ => None,
        };
        if token.is_some() {
            trace!("Found token {:?}", token);
        } else {
            trace!("No token found");
        }
        token
    }
    
    pub fn length(&self) -> usize {
        match self {
            SyntaxToken::LBrace => 1,
            SyntaxToken::RBrace => 1,
            SyntaxToken::LParen => 1,
            SyntaxToken::RParen => 1,
            SyntaxToken::Assign => 1,
            SyntaxToken::Comma => 1,
            SyntaxToken::Dot => 1,
            SyntaxToken::Minus => 1,
            SyntaxToken::Not => 1,
            SyntaxToken::Plus => 1,
            SyntaxToken::Times => 1,
            SyntaxToken::Slash => 1,
            SyntaxToken::Mod => 1,
            SyntaxToken::And => 1,
            SyntaxToken::Or => 1,
            SyntaxToken::Xor => 1,
            SyntaxToken::Eq => 2,
            SyntaxToken::Neq => 2,
            SyntaxToken::Lt => 1,
            SyntaxToken::Leq => 2,
            SyntaxToken::Gt => 1,
            SyntaxToken::Geq => 2,
            SyntaxToken::LShift => 2,
            SyntaxToken::RShift => 2,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LiteralToken {
    Character(char),
    Integer(i64),
    String(String),
}
