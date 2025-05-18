use crate::editor::*;
use crate::rope::*;

use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::io::{self, BufRead, Write};
use anyhow::{anyhow, Context, Result};

impl Editor {
    pub fn open_file(&mut self, file_name: &str) -> Result<()>  {
        let file = File::open(file_name).context("Failed to open file")?;
        let reader = io::BufReader::new(file);
        let mut builder = RopeBuilder::<EditorRow>::new();
        for line in reader.lines() {
            let mut line = line?;
            while line.ends_with(['\n', '\r']) {
                line.pop();
            }
            builder.insert(EditorRow::new(&line));
        }
        self.rows = builder.build().unwrap();
        self.file_name = file_name.to_string();
        Ok(())
    }

    pub fn rows_to_string(&self) -> String {
        let mut buf = String::new();
        for row in self.rows.lines() {
            buf.push_str(&row.chars);
            buf.push_str("\n");
        }        
        buf
    }

    pub fn save_file(&mut self) -> Result<()>{
        if self.file_name.is_empty() {
            if let Some(file_name) = self.prompt("Save as")? {
                if !file_name.is_empty() {
                    self.file_name = file_name;
                } else {
                    self.set_status_msg("Empty file name");
                }
            } else {
                self.set_status_msg("Save aborted");
            }
        }
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_name)
            .context("Failed to open file for writing")?;
        let mut writer = BufWriter::new(file);
        for row in self.rows.lines() {
            writeln!(writer, "{}", row.chars)?; // writes a line with a newline
        }
        self.dirty = false;
        Ok(())
    }
}
