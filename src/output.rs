use std::io::{self, Write};

pub fn clear_screen() -> Result<()> {
    io::stdout().write_all(b"\x1b[2J").context("Failed to refresh screen")?;
    io::stdout().flush()?;
    Ok(())
}

pub fn reposition_cursor() -> Result<()> {
    io::stdout().write_all(b"\x1b[H").context("Failed to reposition cursor")?;
    io::stdout().flush()?;
    Ok(())
}

fn draw_rows() -> Result<()> {
    for _ in 1..=24 {
        io::stdout().write_all(b"~\r\n").context("Failed to draw rows")?;
    }
    io::stdout().flush()?;
    Ok(())
}

pub fn refresh_screen() -> Result<()> {
    clear_screen()?;
    reposition_cursor()?;
    draw_rows()?;
    reposition_cursor()?;
    Ok(())
}
