use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::execute;
use crossterm::cursor::MoveTo;
use std::io::stdout;

pub struct Editor {
    quitting: bool, 
}

impl Editor {

    pub fn default() -> Self {
        Editor{
            quitting: false, 
        }
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))
    }

    fn draw_tildes() -> Result<(), std::io::Error> {
        let (cols, rows) = terminal::size()?;
        for i in 0..rows {
            print!("~");
            if i < rows - 1{
                print!("\n");
            }
        }
        Ok(())
    }

    fn move_cursor(x: u16, y: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(x, y))
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(0, 0)?;
        Ok(())
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.quitting {
            Self::clear_screen()?;
            println!("End of editing! Byebye~");
        } else {
            Self::draw_tildes()?;
            Self::move_cursor(0, 0)?;
        }
        Ok(())
    }
    
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        Self::terminate().unwrap();
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code: Char('q'), 
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event {
            self.quitting = true;
        }
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.quitting {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        disable_raw_mode()?;
        Ok(())
    }
}