mod terminal;
mod editor;

use anyhow::Result;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    loop {
        editor::refresh_screen()?;
        editor::process_keypress()?;
    }
}
