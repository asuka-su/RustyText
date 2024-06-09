use std::io::{self, Read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {

}

impl Editor {

    pub fn default() -> Self {
        Editor{}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;
                    println!("Binary: {0:08b} Character: {1:#?}\r", b, c);
                    if c == 'q' {
                        break;
                    }
                }, 
                Err(err) => println!("Error: {}", err), 
            }
        }
    }
}