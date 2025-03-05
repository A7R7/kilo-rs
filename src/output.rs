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
        let mut buf = String::new();
        for y in 0..= (self.screenrows - 1) {
            let file_row = y + self.row_off;
            if (file_row as usize) < self.rows.len() {
                let row_str = self.rows[file_row as usize].chars.as_str();
                let len = if row_str.len() > self.screencols as usize {
                    self.screencols as usize
                } else {
                    row_str.len()
                };
                buf.push_str(&row_str[0..len]);
            } else {
                buf.push_str("~");
            }

            buf.push_str(CLEAR_LINE_CMD);
            if y < self.screenrows - 1 {
                buf.push_str("\r\n")
            }
        }
        buf
    }

    pub fn move_cursor_str(&self) -> String {
        format!("\x1b[{};{}H", self.cy - self.row_off + 1 , self.cx + 1)
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

    pub fn scroll(&mut self) {
        if self.cy < self.row_off {
            self.row_off = self.cy;
        }
        if self.cy >= self.row_off + self.screenrows {
            self.row_off = self.cy - self.screenrows + 1;
        }
    }

    pub fn refresh_screen(&mut self) {
        self.scroll();
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
