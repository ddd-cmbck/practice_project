use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{queue, Command};
use crossterm::style::{Print};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};
use core::fmt::Display;




#[derive(Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

/// Represents the Terminal.
/// Edge Case for platforms where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller.
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`)
/// And should you attempt to set the caret out of these bounds, it will also be truncated.

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error>{
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position{col: 0, row: 0})?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// Returns the current size of this Terminal.
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        // clippy::as_conversions: See doc above
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        // clippy::as_conversions: See doc above
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok( Size {height, width} )
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    /// Moves the caret to the given Position.
    /// # Arguments
    /// * `Position` - the  `Position` to move the caret to. Will be truncated to `u16::MAX` if bigger.
    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        // clippy::as_conversions: See doc above
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
}