use crate::editor::{Editor, EditorRow};
use anyhow::Result;

impl Editor {
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
}
