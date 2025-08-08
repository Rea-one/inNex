// memory expression

use std::fs;

use crate::model::words::token_map;
use crate::model::words::piece_table;
use crate::model::words::token::{WordType, word_type};

pub struct Mexp {
    // 片段表
    piken: Vec<piece_table::PieceTable>,
    // 符号表
    tomap: token_map::TokenMap,
    at: usize,
    // 请求长度
    req_len: usize,
    // 文件路径
    file_path: Option<String>,
}

impl Mexp {
    pub fn new(req_len: usize) -> Self {
        Mexp {
            piken: Vec::new(),
            tomap: token_map::TokenMap::new(),
            at: 0,
            req_len,
            file_path: None,
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
        // 保存文件路径
        self.file_path = Some(file_path.to_string());
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

    pub fn dequest(&mut self, sidex: usize) -> String { 
        let index = self.piken[sidex].request();
        self.tomap.get_string(index)
    }
    
    pub fn request(&mut self, at: usize) -> Vec<String> { 
        let mut result: Vec<String> = Vec::new();
        self.at = at;
        
        // 确保不会越界访问
        let end: usize = std::cmp::min(self.at + self.req_len, self.piken.len());
        
        // 从指定位置开始，获取指定数量的 token 字符串
        for i in self.at..end {
            let token_str = self.dequest(i);
            result.push(token_str);
        }
        
        result
    }

    pub fn aquest(&mut self, at: usize, offset: i32) -> Vec<String> { 
        let mut begin = at as i32 + offset;
        if begin < 0 {
            begin = 0;
        } else if begin >= self.piken.len() as i32 { 
            begin = self.piken.len() as i32 - 1;
        }
        let mut result = Vec::new();
        let begin = begin as usize;
        let end = begin + self.req_len;
        let last_end = self.at + self.req_len;
        if begin >= self.at {
            for idx in (last_end).max(begin)..end {
                result.push(self.dequest(idx));
            }
        } else {
            for idx in self.at..last_end.min(self.at) {
                result.push(self.dequest(idx));
            }
        }
        self.at = begin;
        result
    }
    
    // 保存内容到文件
    pub fn save(&self, _at: usize, _content: Vec<String>) -> Result<(), String> {
        // 检查是否有文件路径
        if let Some(ref file_path) = self.file_path {
            // TODO: 实现保存逻辑
            // 这里需要根据 Tauri 框架的要求来实现文件保存功能
            // 可能需要通过 Tauri 的命令系统来处理文件操作
            println!("Saving to file: {}", file_path);
            Ok(())
        } else {
            Err("No file path specified".to_string())
        }
    }
    
    // 获取总行数
    pub fn total_lines(&self) -> usize {
        self.piken.len()
    }
    
    // 插入新内容
    pub fn insert(&mut self, token: String, position: usize) -> usize {
        let index = self.register_token(token);
        if position < self.piken.len() {
            self.piken.insert(position, piece_table::PieceTable::desig(index));
        } else {
            self.piken.push(piece_table::PieceTable::desig(index));
        }
        index
    }
    
    // 删除内容
    pub fn remove(&mut self, position: usize) -> Option<piece_table::PieceTable> {
        if position < self.piken.len() {
            Some(self.piken.remove(position))
        } else {
            None
        }
    }
    
    // 设置文件路径
    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = Some(file_path);
    }
    
    // 获取文件路径
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }
}