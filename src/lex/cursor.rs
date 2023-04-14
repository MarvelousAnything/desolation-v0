use std::str::Chars;

// Taken from the rustc compiler.
struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
    prev: char
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            len_remaining: source.len(),
            chars: source.chars(),
            prev: EOF_CHAR
        }
    }

    pub(crate) fn prev(&mut self) -> char {
        self.prev
    }

    pub(crate) fn first(&mut self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn second(&mut self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}