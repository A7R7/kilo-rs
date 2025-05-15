use crate::editor::{Editor, EditorRow};
use anyhow::Result;

impl Editor {
    pub fn row_cx_to_rx(chars: &str, cx: usize) ->Result<usize> {
        let mut rx = 0usize;
        for i in 0..cx {
            if chars.chars().nth(i) == Some('\t') {
                rx += (8 - 1) - (rx % 8)
            }
            rx += 1;
        }
        Ok(rx)
    }

    pub fn update_row(chars: &str) -> Result<String>{
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
        Ok(render)
    }

    pub fn append_row(&mut self, chars: &str) {
        self.rows.push(EditorRow::new(chars));
    }

    pub fn insert_row(&mut self, at: usize, chars: &str) {
        self.rows.insert(at, EditorRow::new(chars));
    }

    pub fn del_row(&mut self, at: usize) {
        if at > self.rows.len() { return }
        self.rows.remove(at);
    }


}

impl EditorRow {
    pub fn new(chars: &str) -> Self {
        let mut row = Self {
            chars: chars.to_string(),
            render: String::new(),
        };
        row.update_render();
        row
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
        let tabs_num = self.chars.matches('\t').count();
        self.render = String::with_capacity(self.chars.chars().count() + tabs_num * 7 + 1);
        for c in self.chars.chars() {
            if c == '\t' {
                self.render.push(' ');
                while self.render.chars().count() % 8 != 0 {
                    self.render.push(' ');
                }
            } else {
                self.render.push(c);
            }
        }
    }
}