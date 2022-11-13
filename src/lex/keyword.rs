#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Keyword {
    Var,
    Fun,
    If,
    Else,
    Until,
    Loop,
    Return,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "var" => Some(Keyword::Var),
            "fun" => Some(Keyword::Fun),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "until" => Some(Keyword::Until),
            "loop" => Some(Keyword::Loop),
            "return" => Some(Keyword::Return),
            _ => None,
        }
    }
}
