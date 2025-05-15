use crate::editor::Editor;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use anyhow::{anyhow, Context, Result};

impl Editor {
    pub fn open_file(&mut self, file_name: &str) -> Result<()>  {
        let file = File::open(file_name).context("Failed to open file")?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let mut line = line?;
            while line.ends_with(['\n', '\r']) {
                line.pop();
            }
            self.append_row(line.as_str());
        }
        self.file_name = file_name.to_string();
        Ok(())
    }

    pub fn rows_to_string(&self) -> String {
        let mut buf = String::new();
        for row in &self.rows {
            buf.push_str(row.chars.as_str());
            buf.push_str("\n");
        }        
        buf
    }

    pub fn save_file(&mut self) -> Result<()>{
        if self.file_name.is_empty() {
            return Err(anyhow!("Empty file name"));
        }
        let buf = self.rows_to_string();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.file_name.as_str())
            .context("Failed to open file for writing")?;
        file.write_all(buf.as_bytes()).context("Failed to write to file")?;
        self.set_status_msg(format!("{} bytes written to disk", buf.len()));
        Ok(())
    }
}
