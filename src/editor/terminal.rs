use crossterm::{
    queue, 
    terminal::{size, Clear, ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, 
    cursor::{MoveTo, Hide, Show}, 
    style::Print, 
};
use std::io::{stdout, Error, Write};

#[derive(Default)]
pub struct Terminal {

}

impl Terminal {
    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor(x: u16, y: u16) -> Result<(), Error> {
        queue!(stdout(), MoveTo(x, y))
    }

    pub fn size() ->Result<(u16, u16), Error> {
        size()
    }

    pub fn initialize() -> Result<(), Error> {
        Self::flush()?;
        enable_raw_mode()?;
        queue!(stdout(), EnterAlternateScreen)?;
        Self::clear_screen()?;
        Self::move_cursor(0, 0)
    }

    pub fn terminate() -> Result<(), Error> {
        queue!(stdout(), LeaveAlternateScreen)?;
        Self::show_cursor()?;
        Self::flush()?;
        disable_raw_mode()
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)
    }

    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn print(s: &str) -> Result<(), Error> {
        queue!(stdout(), Print(s))
    }
}