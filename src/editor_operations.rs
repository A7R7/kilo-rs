use crate::editor::Editor;
use anyhow::Result;

impl Editor {
    pub fn insert_char(&mut self, c: char) {
        if self.cy == self.rows.len() {
            self.append_row("");
        }
        self.rows[self.cy].insert_char(self.cx, c);
        self.cx += 1;
        self.dirty = true;
    }
    
    pub fn insert_newline(&mut self) {
        if self.cx == 0 {
            self.insert_row(self.cy, "");
        } else if self.cx == self.rows[self.cy].chars.chars().count() {
            self.insert_row(self.cy + 1, "");
        } else {
            let row = &mut self.rows[self.cy];
            let mut chars = std::mem::take(&mut row.chars);
            let split_at = chars.char_indices().nth(self.cx).map(|(i, _)| i).unwrap();
            let right_chars = chars.split_off(split_at);
            self.insert_row(self.cy + 1, &right_chars);
            self.rows[self.cy].chars = chars;
            self.rows[self.cy].update_render();
        }
        self.cx = 0;
        self.cy += 1;
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
            let row = self.rows.remove(self.cy);
            self.rows[self.cy - 1].append_string(&row.chars);
            self.cy -= 1;
        }
        self.dirty = true;
    }
}