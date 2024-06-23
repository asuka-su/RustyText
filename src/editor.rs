mod terminal;
mod view;

use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use terminal::Terminal;
use view::View;
use std::{
    io::Error, 
    env, 
    panic::{set_hook, take_hook}, 
};

pub struct Editor {
    quitting: bool, 
    view: View, 
}

impl Editor {

    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            quitting: false, 
            view, 
        })
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor(self.view.get_location().0, self.view.get_location().1);
        let _ = Terminal::show_cursor();
        let _ = Terminal::flush();
    }
    
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.quitting {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(&event), 
                Err(err) => { // panic only on Debug
                    #[cfg(debug_assertions)]
                    { panic!("Could not read event: {err:?}"); }
                }
            }
        }
    }

    fn evaluate_event(&mut self, event: &Event) {
        match event {
            Key(KeyEvent{
                code, modifiers, kind: KeyEventKind::Press, ..
            }) => {
                match code {
                    KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                        self.quitting = true;
                    },
                    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                        self.view.move_cursor_press(*code);
                    }, 
                    KeyCode::Char(character) if *modifiers == KeyModifiers::NONE || *modifiers == KeyModifiers::SHIFT => {
                        self.view.insert(*character);
                    }
                    _ => (),                 
                }
            }, 
            Event::Resize(..) => {
                self.view.resize();
            }, 
            _ => (), 
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.quitting {
            let _ = Terminal::print("ByeBye~\r\nPresented by RUSTYTEXT\r\n");
        }
    }
}