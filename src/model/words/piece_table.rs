pub enum PieceType {
    Original,
    Added,
}

pub struct PieceTable {
    original: usize,
    added: Vec<usize>,
    now: usize,
    history_length: usize,
}

impl PieceTable {
    pub fn new() -> Self {
        PieceTable {
            original: 0,
            added: Vec::new(),
            now: 0,
            history_length: 0,
        }
    }

    // 指定 designate
    pub fn desig(index: usize) -> Self { 
        PieceTable {
            original: index,
            added: Vec::new(),
            now: 0,
            history_length: 0,
        }
    }

    pub fn add(&mut self, index: usize) { 
        self.added.push(index);
        self.history_length += 1;
        self.now = self.history_length - 1;
    }
    
    /// 获取当前 token 索引
    pub fn request(&self) -> usize {
        if self.added.is_empty() || self.now == 0 {
            self.original
        } else {
            self.added[self.now - 1]
        }
    }
}