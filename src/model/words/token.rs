use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum WordType {
    Space,
    Letter,
    Number,
    Symbol,
    Other,
}

pub fn word_type(word: char) -> WordType {
    if word.is_whitespace() {
        return WordType::Space;
    } else if word.is_alphabetic() {
        return WordType::Letter;
    } else if word.is_numeric() {
        return WordType::Number;
    } else if word.is_ascii_punctuation() {
        return WordType::Symbol;
    } else {
        return WordType::Other;
    }
}

pub struct Token {
    token: String,
    length: usize,
}

impl Token {
    pub fn new(the: String) -> Self {
        Token {
            length: the.len(),
            token: the,
        }
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}
