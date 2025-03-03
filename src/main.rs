use std::io::{self, Read};
use std::os::unix::io::AsFd;
use nix::sys::termios::{tcgetattr, tcsetattr,
    LocalFlags, InputFlags, OutputFlags, ControlFlags, SetArg};
use anyhow::{Context, Result};

fn enable_raw_mode() -> Result<()> {
    let stdin = io::stdin();
    let fd = stdin.as_fd();
    let mut termios = tcgetattr(fd)
        .context("Failed to get terminal attributes")?;

    termios.local_flags.remove(
          LocalFlags::ECHO
        | LocalFlags::ICANON
        | LocalFlags::ISIG
        | LocalFlags::IEXTEN
    );

    termios.input_flags.remove(
          InputFlags::BRKINT
        | InputFlags::ICRNL
        | InputFlags::INPCK
        | InputFlags::ISTRIP
        | InputFlags::IXON
    );

    termios.output_flags.remove(OutputFlags::OPOST);

    termios.control_flags.insert(ControlFlags::CS8);

    tcsetattr(fd, SetArg::TCSAFLUSH, &termios)
        .context("Failed to set terminal attributes")?;
    Ok(())
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut buffer = [0; 1];
    while io::stdin().read(&mut buffer)? == 1 && buffer[0] != b'q' {
        let byte = buffer[0] as char;
        if byte.is_control() {
            print!("{}\r\n", byte as u8);
        } else {
            print!("{} ('{}')\r\n", byte as u8, byte);
        }
    }
    Ok(())
}
