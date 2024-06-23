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

#[derive(Copy, Clone, Default)]
struct Location {
    x: u16, 
    y: u16, 
}

pub struct Editor {
    quitting: bool, 
    location: Location, 
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
            location: Location::default(), 
            view, 
        })
    }

    fn move_cursor_press(&mut self, code: KeyCode) {
        let (cols, rows) = Terminal::size().unwrap_or_default();
        let Location { mut x, mut y } = self.location;

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
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor(self.location.x, self.location.y);
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
                        self.move_cursor_press(*code);
                    }, 
                    _ => (),                 
                }
            }, 
            Event::Resize(..) => {
                self.view.redraw = true;
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