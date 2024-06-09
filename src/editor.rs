use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    quitting: bool, 
}

impl Editor {

    pub fn default() -> Self {
        Editor{
            quitting: false, 
        }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        println!("End of editing! Byebye~\r\n");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent{ code, modifiers, .. }) = read()? {
                println!("Code:{code:?}, Modifiers: {modifiers:?}\n");
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.quitting = true; 
                    }, 
                    _ => (),
                }
            }
            if self.quitting {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}