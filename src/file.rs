use std::fs;
use std::io::{Result, Write};

pub struct Archivo {
    pub path: String,
    pub file: fs::File,
    pub buffer: Vec<Vec<char>>, // Change the type to Vec<Vec<char>>
    pub width: usize,
    pub height: usize,
}

impl Archivo {
    pub fn new(path: &str, initial_capacity: usize) -> Archivo {
        let file = fs::File::open(path).unwrap();
        let content = fs::read_to_string(path).unwrap();

        // Convert each line to Vec<char> instead of String
        let buffer: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

        // Calculate width and height based on Vec<Vec<char>>
        let width = buffer.iter().map(|line| line.len()).max().unwrap_or(0);
        let height = buffer.len();

        Archivo {
            path: path.to_string(),
            file,
            buffer,
            width,
            height,
        }
    }

    pub fn save(&self) {
        let flattened: String = self
            .buffer
            .iter()
            .flat_map(|line| {
                line.iter().cloned().chain(std::iter::once('\n')) // Add '\n' after each line
            })
            .collect();

        // Write the flattened string to the file
        fs::write(&self.path, flattened).expect("Error writing to file");
    }
}
