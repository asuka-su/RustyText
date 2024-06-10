mod terminal;
mod view;

use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use terminal::Terminal;
use view::View;
use std::io::Error;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize, 
    y: usize, 
}

#[derive(Default)]
pub struct Editor {
    quitting: bool, 
    location: Location, 
    view: View, 
}

impl Editor {

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
            self.view.render()?;
            // currently can be done like this.
            Terminal::move_cursor(self.location.x.try_into().unwrap(), self.location.y.try_into().unwrap())?;
        }
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }
    
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let args: Vec<String> = std::env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
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