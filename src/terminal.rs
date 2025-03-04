use crate::output;

use std::io::{self, Read, Write};
use std::os::unix::io::AsFd;
use nix::sys::termios::{tcgetattr, tcsetattr,
    LocalFlags, InputFlags, OutputFlags, ControlFlags, SpecialCharacterIndices, SetArg};
use anyhow::{Context, Result};

pub fn enable_raw_mode() -> Result<()> {
    let stdin = io::stdin();
    let fd = stdin.as_fd();
    let mut termios = tcgetattr(fd)
        .context("Failed to get terminal attributes")?;

    termios.local_flags.remove(
          LocalFlags::ECHO    // avoid each key typed printed to the terminal
        | LocalFlags::ICANON  // read input byte-by byte
        | LocalFlags::ISIG    // turn off Ctrl-C and Ctrl-Z signals
        | LocalFlags::IEXTEN  // disable Ctrl-V function
    );

    termios.input_flags.remove(
          InputFlags::BRKINT  // break condition causes SIGINT signal sent to the program
        | InputFlags::ICRNL   // make Ctrl-M return 13 instead of 10
        | InputFlags::INPCK   // enable parity checking
        | InputFlags::ISTRIP  // causes the 8th bit of each input byte to be stripped
        | InputFlags::IXON    // disable Ctrl-S and Ctrl-Q
    );

    termios.output_flags.remove(OutputFlags::OPOST); // Turn off all output processing

    termios.control_flags.insert(ControlFlags::CS8); // set the character size to 8 bits per byte

    termios.control_chars[SpecialCharacterIndices::VMIN as usize] = 0;
    termios.control_chars[SpecialCharacterIndices::VTIME as usize] = 1;

    tcsetattr(fd, SetArg::TCSAFLUSH, &termios)
        .context("Failed to set terminal attributes")?;
    Ok(())
}

pub fn read_key() -> Result<u8> {
    let mut buffer = [0u8; 1];
    loop {
        match io::stdin().read(&mut buffer) {
            Ok(_) => return Ok(buffer[0]),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e).context("Failed to read key from stdin"),
        }
    }
}

fn get_cursor_position() -> Result<(u32, u32)> {
    print!("\x1b[6n\r\n");

    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    let mut x: u32 = 0;
    let mut y: u32 = 0;
    if let Some(buf) = buf.strip_prefix("\x1b[") {
        if let Some(buf) = buf.strip_suffix("R") {
            let parts: Vec<&str> = buf.split(';').collect();
            if parts.len() == 2 {
                y = parts[0].parse()?;
                x = parts[1].parse()?;
            }
        }
    }
    Ok((x, y))
}

pub fn get_window_size() -> Result<(u32, u32)> {
    print!("\x1b[999C\x1b[999B");
    let ret = get_cursor_position();
    output::reposition_cursor();
    return ret;
}
