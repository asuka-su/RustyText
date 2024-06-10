use super::terminal::Terminal;
use std::io::Error;
use std::fs::read_to_string;

#[derive(Default)]
pub struct View {
    buffer: Buffer, 
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
    fn render_welcome() -> Result<(), Error> {
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
                Terminal::print(&welcome)?;
            } else {
                Terminal::print("~")?;
            }
            if i < (rows - 1) {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn render_buffer(&self) -> Result<(), Error> {
        let (.., rows) = Terminal::size()?;
        for i in 0..rows {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(i as usize) {
                Terminal::print(line)?;
            } else {
                Terminal::print("~")?;
            }
            if i < (rows - 1) {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.empty() {
            Self::render_welcome()?;
        } else {
            self.render_buffer()?;
        }
        Ok(())
    }

    pub fn load(&mut self, file_name: &str){
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
        }
    }
}