use crate::editor::Editor;
use anyhow::{Context, Result};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const CLEAR_SCREEN_CMD: &str = "\x1b[2J";
const CLEAR_LINE_CMD: &str = "\x1b[K";
const REPOSITION_CURSOR_CMD: &str = "\x1b[H";
const HIDE_CURSOR_CMD: &str = "\x1b[?25l";
const SHOW_CURSOR_CMD: &str = "\x1b[?25h";
const INVERT_COLOR_CMD: &str = "\x1b[7m";
const NORMAL_COLOR_CMD: &str = "\x1b[m";

impl Editor {
    fn draw_rows_str(&self) -> String {
        let mut buf = String::new();
        for y in 0..=(self.screenrows - 1) {
            let file_row = y + self.row_off;
            if file_row < self.rows.len() {
                let row_str = &self.rows[file_row].render;
                if row_str.chars().count() > self.col_off {
                    let (start, _) = row_str.char_indices().nth(self.col_off).unwrap();
                    let end = if let Some((idx, _)) =
                        row_str.char_indices().nth(self.col_off + self.screencols)
                    {
                        idx
                    } else {
                        row_str.len()
                    };
                    buf.push_str(&row_str[start..end]);
                }
            } else {
                buf.push_str("~");
            }

            buf.push_str(CLEAR_LINE_CMD);
            buf.push_str("\r\n")
        }
        buf
    }

    fn draw_status_bar(&self) -> String {
        let mut bar = String::new();
        bar.push_str(INVERT_COLOR_CMD);
        let status_left = format!(
            " {:.20} - {} lines{}",
            self.file_name,
            self.rows.len(),
            if self.dirty { " modified" } else { "" }
        );
        let status_right = format!(" {}:{} ", self.cy, self.cx);
        let space_len = self.screencols - status_left.len() - status_right.len() - 2;
        let space_len = if space_len > 0 { space_len } else { 0 };
        let space = " ".repeat(space_len);
        let status = format!("{status_left}{space}{status_right}");
        bar.push_str(&status);
        bar.push_str(NORMAL_COLOR_CMD);
        bar.push_str("\r\n");
        bar
    }

    pub fn move_cursor_str(&self) -> String {
        format!(
            "\x1b[{};{}H",
            self.cy - self.row_off + 1,
            self.rx - self.col_off + 1
        )
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
        self.rx = 0;
        if self.cy < self.rows.len() {
            self.rx = Self::row_cx_to_rx(&self.rows[self.cy].chars, self.cx).unwrap();
        }

        if self.cy < self.row_off {
            self.row_off = self.cy;
        }
        if self.cy >= self.row_off + self.screenrows {
            self.row_off = self.cy - self.screenrows + 1;
        }
        if self.rx < self.col_off {
            self.col_off = self.rx;
        }
        if self.rx >= self.col_off + self.screencols {
            self.col_off = self.rx - self.screencols + 1;
        }
    }

    pub fn set_status_msg(&mut self, msg: String) {
        self.status_msg = msg;
        self.status_msg_time = SystemTime::now();
    }

    pub fn draw_msg_bar_str(&self) -> String {
        let mut buf = String::new();
        buf.push_str(CLEAR_LINE_CMD);
        if SystemTime::now()
            .duration_since(self.status_msg_time)
            .unwrap()
            .as_secs()
            < 5
        {
            buf.push_str(&self.status_msg);
        }
        buf
    }

    pub fn refresh_screen(&mut self) -> Result<()> {
        self.scroll();
        let mut buf = String::new();
        buf.push_str(HIDE_CURSOR_CMD);
        buf.push_str(REPOSITION_CURSOR_CMD);
        buf.push_str(&self.draw_rows_str());
        buf.push_str(&self.draw_status_bar());
        buf.push_str(&self.draw_msg_bar_str());
        buf.push_str(&self.move_cursor_str());
        buf.push_str(SHOW_CURSOR_CMD);

        let mut stdout = io::stdout().lock();
        stdout
            .write_all(buf.as_bytes())
            .context("Failed to write to stdout")?;
        stdout.flush().context("Failed to flush")?;
        Ok(())
    }
}
