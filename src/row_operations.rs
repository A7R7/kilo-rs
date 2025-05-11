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

    pub fn append_row(&mut self, chars: String) {
        let render = Self::update_row(chars.as_str()).unwrap();
        self.rows.push(
            EditorRow {
                chars,
                render
            }
        );
    }

}

impl EditorRow {
    pub fn insert_char(&mut self, at: usize, c: char) {
        if let Some(pos) = self.chars.char_indices().nth(at).map(|(i, _)| i) {
            self.chars.insert(pos, c);
        } else {
            self.chars.push(c);
        }
        self.render = Editor::update_row(self.chars.as_str()).unwrap();
    }
}