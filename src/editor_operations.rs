use crate::editor::Editor;
use anyhow::Result;

impl Editor {
    pub fn insert_char(&mut self, c: char) {
        if self.cy == self.rows.len() {
            self.append_row("".to_string());
        }
        self.rows[self.cy].insert_char(self.cx, c);
        self.cx += 1;
        self.dirty = true;
    }
    
    pub fn del_char(&mut self) {
        if self.cy == self.rows.len() { return }
        if self.cx == 0 && self.cy == 0 { return }
        if self.cx > 0 {
            self.rows[self.cy].del_char(self.cx - 1);
            self.cx -= 1;
        } else {
            self.cx = self.rows[self.cy - 1].chars.chars().count();
            let str = self.rows[self.cy].chars.clone();
            self.rows[self.cy - 1].append_string(str.as_str());
            self.del_row(self.cy);
            self.cy -= 1;
        }
        self.dirty = true;
    }
}