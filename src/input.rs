use crate::editor::Editor;

use std::process::exit;
use anyhow::{Result, Context};

macro_rules! ctrl_key {
    ($k:expr) => {($k as u8 & 0x1f) as u8};
}

impl Editor {
    pub fn move_cursor(&mut self, key: char) {
        match key {
            'w' => self.cy -= 1,
            'a' => self.cx -= 1,
            's' => self.cy += 1,
            'd' => self.cx += 1,
            _ => {}
        }
    }

    pub fn process_keypress(&mut self) -> Result<()> {
        let key = self.read_key()?;
        match key {
            c if c == ctrl_key!('q') => {
                Self::clear_screen();
                exit(0);
            }
            b'w' | b'a' | b's' | b'd' => {
                self.move_cursor(key as char);
            }
            _ => {},
        };
        Ok(())
    }
}
