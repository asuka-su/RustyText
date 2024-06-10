mod terminal;

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use terminal::Terminal;
use std::io::Error;

pub struct Editor {
    quitting: bool, 
}

impl Editor {
    pub fn default() -> Self {
        Editor{
            quitting: false, 
        }
    }

    fn draw_tildes() -> Result<(), Error> {
        let (cols, rows) = Terminal::size()?;
        for i in 0..rows {
            Terminal::clear_line()?;
            if i == rows / 3 {
                let s = "Welcome to RustyText!";
                let len = s.len();
                let cols = cols as usize;
                let padding = if cols > len {
                    " ".repeat((cols - len) / 2)
                } else {
                    String::new()
                };
                let mut welcome = format!("~{padding}{s}");
                welcome.truncate(cols);
                Terminal::print(welcome)?;
            } else {
                Terminal::print("~")?;
            }
            if i < (rows - 1) {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.quitting {
            Terminal::clear_screen()?;
            Terminal::print("End of editing! Byebye~\r\n")?;
        } else {
            Self::draw_tildes()?;
            Terminal::move_cursor(0, 0)?;
        }
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }
    
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        Terminal::terminate().unwrap();
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

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.quitting {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
}