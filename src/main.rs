use std::io::{self, Read};
use std::os::unix::io::AsFd;
use nix::sys::termios::{tcgetattr, tcsetattr, LocalFlags, SetArg};

fn enable_raw_mode() -> io::Result<()> {
    let stdin = io::stdin();
    let fd = stdin.as_fd();
    let mut termios = tcgetattr(fd).map_err(io::Error::from)?;

    termios.local_flags.remove(LocalFlags::ECHO);

    tcsetattr(fd, SetArg::TCSAFLUSH, &termios).map_err(io::Error::from)?;
    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut buffer = [0; 1];
    while io::stdin().read(&mut buffer)? == 1 && buffer[0] != b'q' {}
    Ok(())
}
