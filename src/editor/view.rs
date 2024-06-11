use super::terminal::Terminal;
use std::io::Error;
use std::fs::read_to_string;

pub struct View {
    buffer: Buffer, 
    pub redraw: bool, 
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
    fn render_line(row: u16, text: &str) -> Result<(), Error> {
        Terminal::move_cursor(0, row)?;
        Terminal::clear_line()?;
        Terminal::print(text)?;
        Ok(())
    }

    fn render_welcome() -> Result<(), Error> {
        let (cols, rows) = Terminal::size()?;
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
                Self::render_line(i, &welcome)?;
            } else {
                Self::render_line(i, "~")?;
            }
        }
        Ok(())
    }

    fn render_buffer(&self) -> Result<(), Error> {
        let (cols, rows) = Terminal::size()?;
        for i in 0..rows {
            if let Some(line) = self.buffer.lines.get(i as usize) {
                let truncate_line = if line.len() >= cols as usize {
                    &line[0..cols as usize]
                } else { line };
                Self::render_line(i, truncate_line)?;
            } else {
                Self::render_line(i, "~")?;
            }
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.redraw {
            return Ok(());
        }
        if self.buffer.empty() {
            Self::render_welcome()?;
        } else {
            self.render_buffer()?;
        }
        self.redraw = false;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str){
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(), 
            redraw: true, 
        }
    }
}