mod terminal;

use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use terminal::Terminal;
use std::io::Error;

#[derive(Copy, Clone)]
struct Location {
    x: usize, 
    y: usize, 
}

pub struct Editor {
    quitting: bool, 
    location: Location, 
}

impl Editor {
    pub fn default() -> Self {
        Editor{
            quitting: false, 
            location: Location{ x: 0, y: 0 }, 
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

    fn move_cursor_press(&mut self, code: KeyCode) -> Result<(), Error> {
        let (cols, rows) = Terminal::size()?;
        let cols = cols as usize;
        let rows = rows as usize;
        let Location { mut x, mut y } = self.location;

        // window resizing?
        match code {
            KeyCode::Up => {
                y = if y > 0 { y - 1 } else { 0 }
            }, 
            KeyCode::Down => {
                y = if y < (rows - 1) { y + 1 } else { rows - 1 } 
            }, 
            KeyCode::Left => {
                x = if x > 0 { x - 1 } else { 0 }
            }, 
            KeyCode::Right => {
                x = if x < (cols - 1) { x + 1 } else { cols - 1 }
            }, 
            _ => (), 
        }

        self.location = Location { x, y };
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor(0, 0)?;
        if self.quitting {
            Terminal::clear_screen()?;
            Terminal::print("End of editing! Byebye~\r\n")?;
        } else {
            Self::draw_tildes()?;
            // currently can be done like this.
            Terminal::move_cursor(self.location.x.try_into().unwrap(), self.location.y.try_into().unwrap())?;
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

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, modifiers, kind: KeyEventKind::Press, 
            ..
        }) = event {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.quitting = true;
                },
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                    self.move_cursor_press(*code)?;
                }, 
                _ => (), 
            }
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.quitting {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }
}