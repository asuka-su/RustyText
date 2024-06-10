mod terminal;

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use terminal::Terminal;

pub struct Editor {
    quitting: bool, 
}

impl Editor {
    pub fn default() -> Self {
        Editor{
            quitting: false, 
        }
    }

    fn draw_tildes() -> Result<(), std::io::Error> {
        let (_cols, rows) = Terminal::size()?;
        for i in 0..rows {
            print!("~");
            if i < (rows - 1) {
                print!("\r\n");
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.quitting {
            Terminal::clear_screen()?;
            println!("End of editing! Byebye~");
        } else {
            Self::draw_tildes()?;
            Terminal::move_cursor(0, 0)?;
        }
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

    fn repl(&mut self) -> Result<(), std::io::Error> {
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