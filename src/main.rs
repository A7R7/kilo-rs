mod terminal;
mod input;
mod output;

use anyhow::Result;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    loop {
        output::refresh_screen();
        input::process_keypress()?;
    }
}
