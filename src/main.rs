mod raw_mode;
mod editor;

use std::io::{self, Read};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    raw_mode::enable_raw_mode()?;

    let mut buffer = [0u8; 1];
    loop {
        buffer[0] = '\0' as u8;
        io::stdin().read(&mut buffer)
            .context("Failed to read input")?;
        let byte = buffer[0] as char;
        if byte.is_control() {
            print!("{}\r\n", byte as u8);
        } else {
            print!("{} ('{}')\r\n", byte as u8, byte);
        }
        if byte == 'q' {
            break;
        }
    }
    Ok(())
}
