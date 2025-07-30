
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

enum token_state {
    init,
    ana,
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
