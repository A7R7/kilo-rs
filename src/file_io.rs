use crate::editor::Editor;

use std::fs::File;
use std::io::{self, BufRead};
use anyhow::{Context, Result};

impl Editor {
    pub fn open_file(&mut self, file_name: &str) -> Result<()>  {
        let file = File::open(file_name).context("Failed to open file")?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let mut line = line?;
            while line.ends_with(['\n', '\r']) {
                line.pop();
            }
            self.append_row(line);
        }
        self.file_name = file_name.to_string();
        Ok(())
    }
}
