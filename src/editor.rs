use nix::sys::termios::Termios;
use anyhow::{Result, Context};

pub struct Editor {
    pub cx: i32,
    pub cy: i32,
    pub screenrows: i32,
    pub screencols: i32,
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
            screenrows,
            screencols,
            ori_termios,
            termios
        })
    }
}
