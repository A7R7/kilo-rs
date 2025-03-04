use std::io::{self, Write};
use anyhow::{Context, Result};

pub fn clear_screen() {
    print!("\x1b[2J");
}

pub fn reposition_cursor() {
    print!("\x1b[H");
}

fn draw_rows() {
    for _ in 1..=24 {
        print!("~\r\n");
    }
}

pub fn refresh_screen() {
    clear_screen();
    reposition_cursor();
    draw_rows();
    reposition_cursor();
}
