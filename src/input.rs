use crate::editor::Editor;

use std::process::exit;
use anyhow::{Result, Context};

macro_rules! ctrl_key {
    ($k:expr) => {($k as u8 & 0x1f) as u8};
}

impl Editor {
    pub fn process_keypress(&self) -> Result<()> {
        let c = self.read_key()?;
        match c {
            c if c == ctrl_key!('q') => {
                Self::clear_screen();
                exit(0);
            }
            _ => {},
        };
        Ok(())
    }
}
