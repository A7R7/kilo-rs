use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = [0; 1];
    while io::stdin().read(&mut buffer)? == 1 && buffer[0] != b'q' {}
    Ok(())
}
