use std::io::{self, Read, Write};
use std::process::exit;
use anyhow::{Result, Context};

macro_rules! ctrl_key {
    ($k:expr) => {($k as u8 & 0x1f) as u8};
}

fn read_key() -> Result<u8> {
    let mut buffer = [0u8; 1];
    loop {
        match io::stdin().read(&mut buffer) {
            Ok(_) => return Ok(buffer[0]),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e).context("Failed to read key from stdin"),
        }
    }
}

pub fn process_keypress() -> Result<()>{
    let c = read_key()?;
    match c {
        c if c == ctrl_key!('q') => exit(0),
        _ => {},
    };
    Ok(())
}

pub fn refresh_screen() {
    io::stdout().write_all(b"\x1b[2J").unwrap();
}
