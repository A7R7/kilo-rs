mod editor;
mod terminal;
mod input;
mod output;
mod file_io;
mod row_operations;

use editor::Editor;
use anyhow::{Result, Context};

use std::env;

fn main() -> Result<()> {
    let mut editor = Editor::new().context("Failed to initiate editor")?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        editor.open_file(args[1].as_str())?;
    }

    editor.set_status_msg(String::from("HELP: Ctrl-Q = quit"));

    loop {
        editor.refresh_screen();
        editor.process_keypress()?;
    }
}
