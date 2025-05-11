use crate::editor::Editor;
use anyhow::Result;

impl Editor {
    pub fn insert_char(&mut self, c: char) {
        if self.cy == self.rows.len() {
            self.append_row("".to_string());
        }
        self.rows[self.cy].insert_char(self.cx, c);
        self.cx += 1;
    }
}