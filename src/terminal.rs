use crate::editor::Editor;
use crate::input::*;

use std::io::{self, Read, Write};
use std::os::unix::io::AsFd;
use nix::sys::termios::{Termios, tcgetattr, tcsetattr,
    LocalFlags, InputFlags, OutputFlags, ControlFlags, SpecialCharacterIndices, SetArg};
use anyhow::{Context, Result};

impl Editor {
    pub fn enable_raw_mode() -> Result<(Termios, Termios)> {
        let stdin = io::stdin();
        let fd = stdin.as_fd();
        let ori_termios = tcgetattr(fd).context("Failed to get terminal attributes")?;
        let mut termios = ori_termios.clone();

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

        termios.control_flags.insert(ControlFlags::CS8); // 8 bits per byte

        termios.control_chars[SpecialCharacterIndices::VMIN as usize] = 0;
        termios.control_chars[SpecialCharacterIndices::VTIME as usize] = 1;

        tcsetattr(fd, SetArg::TCSAFLUSH, &termios).context("Failed to set terminal attributes")?;
        Ok((ori_termios, termios))
    }

    pub fn read_key(&self) -> Result<i32> {
        let mut stdin = io::stdin().lock();
        let mut buffer = [0u8; 1];
        loop {
            match stdin.read(&mut buffer) {
                Ok(_) => break,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e).context("Failed to read key from stdin"),
            }
        }
        if buffer[0] == b'\x1b' {
            let mut seq = [0u8; 3];
            if stdin.read_exact(&mut seq[0..1]).is_err() ||
               stdin.read_exact(&mut seq[1..2]).is_err() {
                   return Ok(b'\x1b' as i32);
            }
            if seq[0] == b'[' {
                match seq[1] {
                    x if b'0' < x && x < b'9' => {
                        if stdin.read_exact(&mut seq[2..3]).is_err() {
                            return Ok(b'\x1b' as i32)
                        }
                        if seq[2] == b'~' {
                            match seq[1] {
                                b'5' => return Ok(PAGE_UP),
                                b'6' => return Ok(PAGE_DOWN),
                                _ => {}
                            }
                        }
                    }
                    b'A' => return Ok(ARROW_UP),
                    b'B' => return Ok(ARROW_DOWN),
                    b'C' => return Ok(ARROW_RIGHT),
                    b'D' => return Ok(ARROW_LEFT),
                    _ => {}
                }
            }
            return Ok(b'\x1b' as i32)
        }
        Ok(buffer[0] as i32)
    }

    fn get_cursor_position() -> Result<(i32, i32)> {
        let mut stdout = io::stdout().lock();
        stdout.write_all(b"\x1b[6n\r\n")?;
        stdout.flush()?;

        let mut buf = String::new();
        let _ = io::stdin().lock().read_to_string(&mut buf);

        let mut x: i32 = 0;
        let mut y: i32 = 0;
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

    pub fn get_window_size() -> Result<(i32, i32)> {
        let mut stdout = io::stdout().lock();
        stdout.write_all(b"\x1b[999C\x1b[999B")?;
        stdout.flush()?;

        let ret = Self::get_cursor_position();
        Self::reposition_cursor();
        return ret;
    }
}
