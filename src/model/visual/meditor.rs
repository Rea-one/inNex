pub struct MeditorModel {
    pub at: usize,
    pub total_lines: usize,
    pub offset: i32,
}

impl MeditorModel {
    pub fn new(total_lines: usize) -> Self {
        MeditorModel {
            at: 0,
            total_lines,
            offset: 0,
        }
    }
    
    pub fn set_position(&mut self, position: usize) {
        self.at = position.min(self.total_lines.saturating_sub(1));
    }
    
    pub fn scroll_by(&mut self, offset: i32) {
        let new_pos = if offset < 0 {
            self.at.saturating_sub((-offset) as usize)
        } else {
            self.at.saturating_add(offset as usize)
        };
        self.at = new_pos.min(self.total_lines.saturating_sub(1));
    }
    
    pub fn get_position(&self) -> usize {
        self.at
    }
    
    pub fn set_offset(&mut self, offset: i32) {
        self.offset = offset;
    }
    
    pub fn get_offset(&self) -> i32 {
        self.offset
    }
}