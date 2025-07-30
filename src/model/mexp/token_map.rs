use std::collections::HashMap;
use std::collections::VecDeque;

use crate::model::mexp::token::Token;

pub struct TokenMap {
    map: HashMap<String, usize>,
    tokens: Vec<Token>,
    counter: Vec<usize>,
    length: usize,
    frees: VecDeque<usize>,
}

impl TokenMap {
    pub fn new() -> Self {
        TokenMap {
            map: HashMap::new(),
            tokens: Vec::new(),
            counter: Vec::new(),
            length: 0,
            frees: VecDeque::new(),
        }
    }

    pub fn insert(&mut self, token: String) -> usize {
        if let Some(index) = self.map.get(&token) {
            self.counter[*index] += 1;
            return *index;
        } else if let Some(index) = self.frees.pop_front() {
            self.map.insert(token.clone(), index);
            self.tokens[index] = Token::new(token);
            self.counter[index] = 1;
            return index;
        } else {
            let index = self.length;
            self.map.insert(token.clone(), index);
            self.tokens.push(Token::new(token));
            self.counter.push(1);
            self.length += 1;
            return index;
        }
    }

    pub fn release(&mut self, token: String) {
        if let Some(&index) = self.map.get(&token) { 
            self.counter[index] -= 1;
            if self.counter[index] == 0 {
                self.map.remove(&token);
                self.tokens.remove(index);
                self.counter.remove(index);
                self.length -= 1;
            }
        }
    }

    pub fn delete_token(&mut self, token: String) { 
        if let Some(index) = self.map.remove(&token) {
            self.frees.push_back(index);
        }
    }

    pub fn delete_index(&mut self, index: usize) { 
        if let Some(token) = self.tokens.get(index) {
            self.map.remove(token.get_token().as_str());
        }
        self.frees.push_back(index);
    }
    
    pub fn get(&self, index: usize) -> Option<&Token> { 
        self.tokens.get(index)
    }

    pub fn find(&self, token: String) -> Option<usize> { 
        self.map.get(&token).cloned()
    }
}