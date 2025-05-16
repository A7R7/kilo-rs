use crate::editor::{Editor, EditorRow};
use anyhow::Result;

impl Editor {
    pub fn append_row(&mut self, chars: &str) {
        self.rows.push(EditorRow::new(chars));
    }

    pub fn insert_row(&mut self, at: usize, chars: &str) {
        self.rows.insert(at, EditorRow::new(chars));
    }

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

impl EditorRow {
    pub fn new(chars: &str) -> Self {
        Self {
            chars: chars.to_string(),
            render: Self::render_from_chars(chars),
        }
    }

    pub fn insert_char(&mut self, at: usize, c: char) {
        if let Some(pos) = self.chars.char_indices().nth(at).map(|(i, _)| i) {
            self.chars.insert(pos, c);
        } else {
            self.chars.push(c);
        }
        self.update_render();
    }

    pub fn del_char(&mut self, at: usize) {
        if at > self.chars.chars().count() { return }
        if let Some((start, c)) = self.chars.char_indices().nth(at) {
            let end = start + c.len_utf8();
            self.chars.replace_range(start..end, "");
        }
        self.update_render();
    }

    pub fn append_string(&mut self, str: &str) {
        self.chars.push_str(str);
        self.update_render();
    }

    pub fn update_render(&mut self) {
        self.render = Self::render_from_chars(&self.chars);
    }

    fn render_from_chars(chars: &str) -> String {
        let tabs_num = chars.matches('\t').count();
        let mut render = String::with_capacity(chars.chars().count() + tabs_num * 7 + 1);
        for c in chars.chars() {
            if c == '\t' {
                render.push(' ');
                while render.chars().count() % 8 != 0 {
                    render.push(' ');
                }
            } else {
                render.push(c);
            }
        }
        render
    }

    pub fn cx_to_rx(&self, cx: usize) -> usize {
        let mut rx = 0usize;
        for i in 0..cx {
            if self.chars.chars().nth(i) == Some('\t') {
                rx += (8 - 1) - (rx % 8)
            }
            rx += 1;
        }
        rx
    }

}