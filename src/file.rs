use std::fs;

#[derive(Debug)]
pub struct Archivo {
    pub path: String,
    pub file: fs::File,
    pub buffer: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Archivo {
    pub fn new(path: &str) -> Archivo {
        let file = fs::File::open(path).unwrap();
        let content = fs::read_to_string(path).unwrap();

        let buffer: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

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
            .flat_map(|line| line.iter().cloned().chain(std::iter::once('\n')))
            .collect();

        fs::write(&self.path, flattened).expect("Error writing to file");
    }
}
