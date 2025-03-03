use std::io::{self, Read};
use std::os::unix::io::AsFd;
use nix::sys::termios::{tcgetattr, tcsetattr, LocalFlags, SetArg};
use anyhow::{Context, Result};

fn enable_raw_mode() -> Result<()> {
    let stdin = io::stdin();
    let fd = stdin.as_fd();
    let mut termios = tcgetattr(fd)
        .context("Failed to get terminal attributes")?;

    termios.local_flags.remove(LocalFlags::ECHO);
    termios.local_flags.remove(LocalFlags::ICANON);

    tcsetattr(fd, SetArg::TCSAFLUSH, &termios)
        .context("Failed to set terminal attributes")?;
    Ok(())
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut buffer = [0; 1];
    while io::stdin().read(&mut buffer)? == 1 && buffer[0] != b'q' {}
    Ok(())
}
