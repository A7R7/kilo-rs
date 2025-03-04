mod editor;
mod terminal;
mod input;
mod output;

use editor::Editor;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let editor = Editor::new().context("Failed to initiate editor")?;

    loop {
        editor.refresh_screen();
        editor.process_keypress()?;
    }
}
