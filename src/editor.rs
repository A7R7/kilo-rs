use nix::sys::termios::Termios;
use anyhow::{Result, Context};

pub struct EditorRow {
    pub chars: String,
    pub render: String
}

pub struct Editor {
    pub cx: i32,
    pub cy: i32,
    pub row_off: i32,
    pub screenrows: i32,
    pub screencols: i32,
    pub rows: Vec<EditorRow>,
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
            row_off: 0,
            screenrows,
            screencols,
            rows: Vec::new(),
            ori_termios,
            termios
        })
    }
}
