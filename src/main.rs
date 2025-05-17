mod editor;
mod terminal;
mod input;
mod output;
mod file_io;
mod operations;
mod rope;

use editor::Editor;
use anyhow::{Result, Context};

use std::env;

fn main() -> Result<()> {
    let mut editor = Editor::new().context("Failed to initiate editor")?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        editor.open_file(&args[1])?;
    }

    editor.set_status_msg("HELP: Ctrl-S = save, Ctrl-Q = quit");

    loop {
        editor.refresh_screen();
        editor.process_keypress()?;
    }
}
