use crate::editor::Editor;

use std::process::exit;
use anyhow::{Result, Context};

macro_rules! ctrl_key {
    ($k:expr) => {($k as u8 & 0x1f) as i32};
}

pub const ARROW_LEFT: i32 = 1000;
pub const ARROW_RIGHT: i32 = 1001;
pub const ARROW_UP: i32 = 1002;
pub const ARROW_DOWN: i32 = 1003;
pub const DEL_KEY: i32 = 1004;
pub const HOME_KEY: i32 = 1005;
pub const END_KEY: i32 = 1006;
pub const PAGE_UP: i32 = 1007;
pub const PAGE_DOWN: i32 = 1008;

impl Editor {
    pub fn move_cursor(&mut self, key: i32) {
        match key {
            ARROW_UP => {
                if self.cy > 0 {
                    self.cy -= 1;
                }
            }
            ARROW_DOWN => {
                if self.cy < self.rows.len() - 1 {
                    self.cy += 1;
                }
            }
            ARROW_LEFT => {
                if self.cx > 0 {
                    self.cx -= 1;
                } else if self.cy > 0{
                    self.cy -= 1;
                    self.cx = self.rows[self.cy].chars.len();
                }
            }
            ARROW_RIGHT => {
                if self.cx < self.rows[self.cy].chars.len() {
                    self.cx += 1;
                } else if self.cy < self.rows.len() - 1{
                    self.cy += 1;
                    self.cx = 0;
                }
            }
            _ => {}
        }

        if self.cy < self.rows.len() {
            let line = &self.rows[self.cy].chars;
            if self.cx > line.len() {
                self.cx = line.len();
            }
        }
    }

    pub fn process_keypress(&mut self) -> Result<()> {
        let key = self.read_key()?;
        match key {
            c if c == ctrl_key!('q') => {
                Self::clear_screen();
                exit(0);
            }
            ARROW_UP | ARROW_DOWN | ARROW_LEFT | ARROW_RIGHT => {
                self.move_cursor(key);
            }
            PAGE_UP | PAGE_DOWN => {
                if key == PAGE_UP {
                    self.cy = self.row_off;
                } else if key == PAGE_DOWN {
                    self.cy = self.row_off + self.screenrows - 1;
                    if self.cy > self.rows.len() {
                        self.cy = self.rows.len();
                    }
                }
                for _ in 1..= (self.screenrows - 1) {
                    self.move_cursor(
                        if key == PAGE_UP { ARROW_UP } else { ARROW_DOWN }
                    );
                }
            }
            HOME_KEY => {
                self.cx = 0;
            }
            END_KEY => {
                if self.cy < self.rows.len() {
                    self.cx = self.rows[self.cy].chars.len() - 1;
                }
            }
            _ => {},
        };
        Ok(())
    }
}
