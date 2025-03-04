use crate::editor::Editor;
use std::io::{self, Write};
use anyhow::{Context, Result};

const CLEAR_SCREEN_CMD: &[u8] = b"\x1b[2J";
const REPOSITION_CURSOR_CMD: &[u8] = b"\x1b[H";
const HIDE_CURSOR_CMD: &[u8] = b"\x1b[?25l";
const SHOW_CURSOR_CMD: &[u8] = b"\x1b[?25h";

impl Editor {
    fn draw_rows_str(&self) -> String {
        let mut str = String::new();
        for _ in 1..= (self.screenrows - 1) {
            str.push_str("~\r\n");
        }
        str.push_str("~");
        str
    }

    pub fn move_cursor_str(&self) -> String {
        format!("\x1b[{};{}H", self.cy + 1, self.cx + 1)
    }

    pub fn clear_screen() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(CLEAR_SCREEN_CMD).unwrap();
        stdout.flush().unwrap();
    }

    pub fn reposition_cursor() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(REPOSITION_CURSOR_CMD).unwrap();
        stdout.flush().unwrap();
    }


    pub fn refresh_screen(&self) {
        let mut buf: Vec<u8> = vec![];
        buf.extend_from_slice(HIDE_CURSOR_CMD);
        buf.extend_from_slice(CLEAR_SCREEN_CMD);
        buf.extend_from_slice(REPOSITION_CURSOR_CMD);
        buf.extend_from_slice(self.draw_rows_str().as_bytes());
        buf.extend_from_slice(REPOSITION_CURSOR_CMD);
        buf.extend_from_slice(SHOW_CURSOR_CMD);

        let mut stdout = io::stdout().lock();
        stdout.write_all(buf.as_slice()).unwrap();
        stdout.flush().unwrap();
    }
}
