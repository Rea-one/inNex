// memory expression

use std::fs;

use crate::model::mexp::token_map;
use crate::model::mexp::piece_table;
use crate::model::mexp::token::{WordType, word_type};

pub struct Mexp {
    // 片段表
    piken: Vec<piece_table::PieceTable>,
    // 符号表
    tomap: token_map::TokenMap,
}

impl Mexp {
    pub fn new() -> Self {
        Mexp {
            piken: Vec::new(),
            tomap: token_map::TokenMap::new(),
        }
    }

    // 读取文件， 生成一个 mexp
    pub fn read(&mut self, file_path: &str) {
        let mut prev_type = WordType::Space;
        let mut mem_token = Vec::new();
        let target = fs::read_to_string(file_path).unwrap();
        for ch in target.chars() {
            let curr_type = word_type(ch);
            if curr_type == WordType::Space || curr_type != prev_type { 
                if mem_token.len() > 0 { 
                    self.add(String::from_iter(mem_token.iter()));
                    mem_token.clear();
                }
            } else {
                mem_token.push(ch);
            }
            prev_type = curr_type;
        }
        if mem_token.len() > 0 { 
            self.add(String::from_iter(mem_token.iter()));
        }
    }


    // 直接注册一个token
    pub fn register_token(&mut self, token: String) -> usize { 
        match self.tomap.find(token.clone()) { 
            Some(index) => { return index },
            None => { return self.tomap.insert(token) },
        }
    }

    // 记录一个token
    pub fn add(&mut self, token: String) -> usize {
        let index = self.register_token(token);
        self.piken.push(piece_table::PieceTable::desig(index));
        index
    }

    pub fn fix(&mut self, val: String, sidex: usize) {
        let index = self.tomap.insert(val);
        self.piken[sidex].add(index);
    }
}