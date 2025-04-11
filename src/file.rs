use std::{fs, os::unix::fs::PermissionsExt};

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

        let mut buffer: Vec<Vec<char>> =
            content.lines().map(|line| line.chars().collect()).collect();
        if buffer.is_empty() {
            buffer.push(Vec::<char>::new());
        }

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

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let flattened: String = self
            .buffer
            .iter()
            .flat_map(|line| line.iter().cloned().chain(std::iter::once('\n')))
            .collect();

        fs::write(&self.path, flattened)?;

        Ok(())
    }
}

pub fn format_permissions(permissions: fs::Permissions, is_directory: bool) -> String {
    let mode = permissions.mode();
    let file_type_char = if is_directory { 'd' } else { '-' };

    let owner_read = if mode & 0o400 != 0 { 'r' } else { '-' };
    let owner_write = if mode & 0o200 != 0 { 'w' } else { '-' };
    let owner_execute = if mode & 0o100 != 0 { 'x' } else { '-' };

    let group_read = if mode & 0o040 != 0 { 'r' } else { '-' };
    let group_write = if mode & 0o020 != 0 { 'w' } else { '-' };
    let group_execute = if mode & 0o010 != 0 { 'x' } else { '-' };

    let other_read = if mode & 0o004 != 0 { 'r' } else { '-' };
    let other_write = if mode & 0o002 != 0 { 'w' } else { '-' };
    let other_execute = if mode & 0o001 != 0 { 'x' } else { '-' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type_char,
        owner_read,
        owner_write,
        owner_execute,
        group_read,
        group_write,
        group_execute,
        other_read,
        other_write,
        other_execute
    )
}

pub fn is_file(path: impl AsRef<str>) -> bool {
    let path = std::path::Path::new(path.as_ref());
    path.is_file()
}
