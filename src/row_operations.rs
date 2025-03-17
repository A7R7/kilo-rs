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

    fn update_row(chars: &str) -> Result<String>{
        let tabs_num = chars.matches('\t').count();
        let mut render = String::with_capacity(chars.len() + tabs_num * 7 + 1);
        for c in chars.chars() {
            if c == '\t' {
                render.push(' ');
                while render.len() % 8 != 0 {
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

    pub fn row_insert_car(row: &mut EditorRow, at: usize, c: char) {
        if at > row.chars.len() {
            row.chars.push(c);
        } else {
            row.chars.insert(at, c);
        }
        row.render = Self::update_row(row.chars.as_str()).unwrap();
    }
}
