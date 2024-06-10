use crossterm::{
    execute, 
    terminal::{size, Clear, ClearType, disable_raw_mode, enable_raw_mode}, 
    cursor::MoveTo, 
};
use std::io::stdout;

pub struct Terminal {

}

impl Terminal {
    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))
    }

    pub fn move_cursor(x: u16, y: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(x, y))
    }

    pub fn size() ->Result<(u16, u16), std::io::Error> {
        size()
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(0, 0)
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
}