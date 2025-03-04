use crate::editor::Editor;
use std::io::{self, Write};
use anyhow::{Context, Result};

impl Editor {
    pub fn clear_screen() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(b"\x1b[2J");
        stdout.flush();
    }

    pub fn reposition_cursor() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(b"\x1b[H");
        stdout.flush();
    }

    fn draw_rows(&self) {
        Self::reposition_cursor();
        let mut stdout = io::stdout().lock();
        for _ in 1..= (self.screenrows - 1) {
            stdout.write_all(b"~\r\n");
        }
        stdout.write_all(b"~");
        stdout.flush();
        Self::reposition_cursor();
    }

    pub fn refresh_screen(&self) {
        Self::clear_screen();
        self.draw_rows();
    }
}
