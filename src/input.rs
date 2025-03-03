use crate::terminal;
use crate::output;

use std::process::exit;
use anyhow::{Result, Context};

macro_rules! ctrl_key {
    ($k:expr) => {($k as u8 & 0x1f) as u8};
}


pub fn process_keypress() -> Result<()> {
    let c = terminal::read_key()?;
    match c {
        c if c == ctrl_key!('q') => {
            output::clear_screen()?;
            exit(0);
        }
        _ => {},
    };
    Ok(())
}
