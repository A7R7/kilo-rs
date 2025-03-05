use crate::editor::{Editor, EditorRow};

impl Editor {
    pub fn append_row(&mut self, chars: String) {
        let render = String::new();
        self.rows.push(
            EditorRow {
                chars,
                render
            }
        );
    }
}
