use crate::editor::Editor;
use std::io::{self, Write};
use anyhow::{Context, Result};

const CLEAR_SCREEN_CMD: &str = "\x1b[2J";
const CLEAR_LINE_CMD: &str = "\x1b[K";
const REPOSITION_CURSOR_CMD: &str = "\x1b[H";
const HIDE_CURSOR_CMD: &str = "\x1b[?25l";
const SHOW_CURSOR_CMD: &str = "\x1b[?25h";

impl Editor {
    fn draw_rows_str(&self) -> String {
        let mut str = String::new();
        for _ in 1..= (self.screenrows - 1) {
            str.push_str("~");
            str.push_str(CLEAR_LINE_CMD);
            str.push_str("\r\n")
        }
        str.push_str("~");
        str.push_str(CLEAR_LINE_CMD);
        str
    }

    pub fn move_cursor_str(&self) -> String {
        format!("\x1b[{};{}H", self.cy + 1, self.cx + 1)
    }

    pub fn clear_screen() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(CLEAR_SCREEN_CMD.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    pub fn reposition_cursor() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(REPOSITION_CURSOR_CMD.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }


    pub fn refresh_screen(&self) {
        let mut buf  = String::new();
        buf.push_str(HIDE_CURSOR_CMD);
        buf.push_str(REPOSITION_CURSOR_CMD);
        buf.push_str(self.draw_rows_str().as_str());
        buf.push_str(self.move_cursor_str().as_str());
        buf.push_str(SHOW_CURSOR_CMD);

        let mut stdout = io::stdout().lock();
        stdout.write_all(buf.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}
