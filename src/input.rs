use crate::editor::Editor;

use std::process::exit;
use anyhow::{Result, Context};

macro_rules! ctrl_key {
    ($k:expr) => {($k as u8 & 0x1f) as i32};
}

const fn ctrl_key(c: char) -> i32 {
    (c as u8 & 0x1f) as i32
}

pub const BACKSPACE: i32 = 127;
pub const ARROW_LEFT: i32 = 1000;
pub const ARROW_RIGHT: i32 = 1001;
pub const ARROW_UP: i32 = 1002;
pub const ARROW_DOWN: i32 = 1003;
pub const DEL_KEY: i32 = 1004;
pub const HOME_KEY: i32 = 1005;
pub const END_KEY: i32 = 1006;
pub const PAGE_UP: i32 = 1007;
pub const PAGE_DOWN: i32 = 1008;

pub const CTRL_Q: i32 = ctrl_key('q');
pub const CTRL_S: i32 = ctrl_key('s');
pub const CTRL_H: i32 = ctrl_key('h');

pub const NEWLINE: i32 = '\r' as i32;
pub const ESCAPE: i32 = '\x1b' as i32;

impl Editor {
    pub fn move_cursor(&mut self, key: i32) {
        match key {
            ARROW_UP => {
                if self.cy > 0 {
                    self.cy -= 1;
                }
            }
            ARROW_DOWN => {
                if self.cy < self.rows.count() - 1 {
                    self.cy += 1;
                }
            }
            ARROW_LEFT => {
                if self.cx > 0 {
                    self.cx -= 1;
                } else if self.cy > 0{
                    self.cy -= 1;
                    self.cx = self.rows.get_line(self.cy).len();
                }
            }
            ARROW_RIGHT => {
                if self.cx < self.rows.get_line(self.cy).len() {
                    self.cx += 1;
                } else if self.cy < self.rows.count() - 1{
                    self.cy += 1;
                    self.cx = 0;
                }
            }
            _ => {}
        }

        if self.cy < self.rows.count() {
            let line = &self.rows.get_line(self.cy);
            if self.cx > line.len() {
                self.cx = line.len();
            }
        }
    }

    pub fn process_keypress(&mut self) -> Result<()> {
        let key = self.read_key()?;
        match key {
            0 => {
                // nothing
            }
            CTRL_Q => {
                Self::clear_screen();
                self.disable_raw_mode();
                exit(0);
            }
            CTRL_S => {
                return self.save_file();
            }
            ARROW_UP | ARROW_DOWN | ARROW_LEFT | ARROW_RIGHT => {
                self.move_cursor(key);
            }
            PAGE_UP | PAGE_DOWN => {
                if key == PAGE_UP {
                    self.cy = self.row_off;
                } else if key == PAGE_DOWN {
                    self.cy = self.row_off + self.screenrows - 1;
                    if self.cy > self.rows.count() {
                        self.cy = self.rows.count();
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
                if self.cy < self.rows.count() {
                    self.cx = self.rows.get_line(self.cy).len();
                }
            }
            NEWLINE => {
                self.insert_newline();
            }
            BACKSPACE | CTRL_H | DEL_KEY => {
                self.del_char();
            }
            _ => {
                if let Some(c) = char::from_u32(key as u32) {
                    self.insert_char(c);    
                }
            }
        };
        Ok(())
    }

    pub fn prompt(&mut self, prompt: &str) -> Result<Option<String>>{
        let mut buf = String::new();
        loop {
            self.set_status_msg(&format!("{}: {}", prompt, buf));
            self.refresh_screen()?;
            let key = self.read_key()?;
            match key {
                0 => {}
                BACKSPACE | CTRL_H | DEL_KEY => {
                    buf.pop();
                }
                ESCAPE => {
                    self.set_status_msg("");
                    return Ok(None);
                }
                NEWLINE => {
                    self.set_status_msg("");
                    return Ok(Some(buf));
                }
                _ => {
                    if let Some(c) = char::from_u32(key as u32) {
                        buf.push(c);    
                    }
                }
            }
        }
    }
}
