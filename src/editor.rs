use nix::sys::termios::Termios;
use anyhow::{Result, Context};

pub struct Editor {
    pub ori_termios: Termios,
    pub termios: Termios,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (ori_termios, termios) = Self::enable_raw_mode()
            .context("Failed to enable raw mode")?;
        Ok(Self {
            ori_termios,
            termios
        })
    }
}
