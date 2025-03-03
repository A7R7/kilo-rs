mod raw_mode;
mod editor;

use anyhow::Result;

fn main() -> Result<()> {
    raw_mode::enable_raw_mode()?;

    loop {
        editor::refresh_screen()?;
        editor::process_keypress()?
    }
}
