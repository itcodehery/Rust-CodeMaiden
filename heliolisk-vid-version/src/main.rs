mod editor;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

pub struct Line {
    text: String,
}

impl Line {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }
}

pub struct Document {
    pub lines: Vec<Line>,
    pub file_format: String,
}

impl Document {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            file_format: ".txt".to_string(),
        }
    }

    pub fn line_length(&self, line_index: usize) -> usize {
        self.lines
            .get(line_index)
            .map(|l| l.text.len())
            .unwrap_or(0)
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn char_count(&self) -> usize {
        self.lines.iter().map(|l| l.text.len()).sum()
    }
}

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
