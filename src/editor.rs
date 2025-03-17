use nix::sys::termios::Termios;
use anyhow::{Result, Context};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct EditorRow {
    pub chars: String,
    pub render: String
}

pub struct Editor {
    pub cx: usize,
    pub cy: usize,
    pub rx: usize,
    pub row_off: usize,
    pub col_off: usize,
    pub screenrows: usize,
    pub screencols: usize,
    pub rows: Vec<EditorRow>,
    pub file_name: String,
    pub status_msg: String,
    pub status_msg_time: SystemTime,
    pub ori_termios: Termios,
    pub termios: Termios,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (ori_termios, termios) = Self::enable_raw_mode()
            .context("Failed to enable raw mode")?;
        let (screencols, screenrows) = Self::get_window_size()
            .context("Failed to get window size")?;
        Ok(Self {
            cx: 0,
            cy: 0,
            rx: 0,
            row_off: 0,
            col_off: 0,
            screenrows: screenrows - 2, // leave 2 line for status and msg bar
            screencols,
            rows: Vec::new(),
            file_name: String::new(),
            status_msg: String::new(),
            status_msg_time: UNIX_EPOCH,
            ori_termios,
            termios
        })
    }
}
