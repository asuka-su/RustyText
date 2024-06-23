use super::terminal::Terminal;
use std::{
    io::Error, 
    fs::read_to_string, 
    cmp::min, 
};
use crossterm::event::KeyCode;

#[derive(Default, Clone, Copy)]
struct Location {
    x: u16, 
    y: u16, 
}

pub struct View {
    buffer: Buffer, 
    redraw: bool, 
    location: Location, 
    offset: Location, 
}

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>, 
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut temp = Vec::new();
        for lines in contents.lines() {
            temp.push(String::from(lines));
        }
        Ok(Self{ lines: temp })
    }

    pub fn empty(&self) -> bool {
        self.lines.is_empty()
    }
}

impl View {
    fn render_line(row: u16, text: &str) {
        let _ = Terminal::move_cursor(0, row);
        let _ = Terminal::clear_line();
        let rst = Terminal::print(text);
        debug_assert!(rst.is_ok(), "Failed to render line!");
    }

    fn render_welcome() {
        let (cols, rows) = Terminal::size().unwrap_or_default();
        for i in 0..rows {
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
                Self::render_line(i, &welcome);
            } else {
                Self::render_line(i, "~");
            }
        }
    }

    fn render_buffer(&self) {
        let (cols, rows) = Terminal::size().unwrap_or_default();
        let top = self.offset.y;
        let left = self.offset.x;
        let right = left + cols;
        for i in 0..rows {
            if let Some(line) = self.buffer.lines.get((i + top) as usize) {
                let truncate_line = if line.len() >= right as usize {
                    &line[min(left as usize, line.len())..right as usize]
                } else { &line[min(left as usize, line.len())..] };
                Self::render_line(i, truncate_line);
            } else {
                Self::render_line(i, "~");
            }
        }
    }

    pub fn render(&mut self) {
        if !self.redraw {
            return;
        }
        if self.buffer.empty() {
            Self::render_welcome();
        } else {
            self.render_buffer();
        }
        self.redraw = false;
    }

    pub fn load(&mut self, file_name: &str){
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
        }
    }

    pub fn move_cursor_press(&mut self, code: KeyCode) {
        let Location { mut x, mut y } = self.location;
    
        match code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }, 
            KeyCode::Down => {
                y = y.saturating_add(1);
            }, 
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }, 
            KeyCode::Right => {
                x = x.saturating_add(1);
            }, 
            _ => (), 
        }
        
        self.location = Location { x, y };
        self.scroll();
    }
    
    pub fn redraw(&mut self) {
        self.scroll();
        self.redraw = true;
    }

    fn scroll(&mut self) {
        let (cols, rows) = Terminal::size().unwrap_or_default();
        let Location { x, y } = self.location;

        if y < self.offset.y {
            self.offset.y = self.offset.y.saturating_sub(1);
            self.redraw = true;
        } else if y >= self.offset.y.saturating_add(rows) {
            self.offset.y = self.offset.y.saturating_add(1);
            self.redraw = true;
        }

        if x < self.offset.x {
            self.offset.x = self.offset.x.saturating_sub(1);
            self.redraw = true;
        } else if x >= self.offset.x.saturating_add(cols) {
            self.offset.x = self.offset.x.saturating_add(1);
            self.redraw = true;
        }
    }

    pub fn get_location(&self) -> (u16, u16) {
        (self.location.x - self.offset.x, self.location.y - self.offset.y)
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(), 
            redraw: true, 
            location: Location::default(), 
            offset: Location::default(),
        }
    }
}