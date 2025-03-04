use crate::editor::Editor;
use anyhow::{Context, Result};

impl Editor {
    pub fn clear_screen() {
        print!("\x1b[2J");
    }

    pub fn reposition_cursor() {
        print!("\x1b[H");
    }

    fn draw_rows(&self) {
        Self::reposition_cursor();
        for _ in 1..= (self.screenrows - 1) {
            print!("~\r\n");
        }
        print!("~");
        Self::reposition_cursor();
    }

    pub fn refresh_screen(&self) {
        Self::clear_screen();
        self.draw_rows();
    }
}
