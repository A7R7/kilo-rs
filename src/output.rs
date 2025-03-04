use crate::editor::Editor;
use anyhow::{Context, Result};

impl Editor {
    pub fn clear_screen(&self) {
        print!("\x1b[2J");
    }

    pub fn reposition_cursor(&self) {
        print!("\x1b[H");
    }

    fn draw_rows(&self) {
        for _ in 1..=24 {
            print!("~\r\n");
        }
    }

    pub fn refresh_screen(&self) {
        self.clear_screen();
        self.reposition_cursor();
        self.draw_rows();
        self.reposition_cursor();
    }
}
