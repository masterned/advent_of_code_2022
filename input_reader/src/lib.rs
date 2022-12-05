use std::{fs, io};

pub fn read_file_as_lines(path: &str) -> io::Result<Vec<String>> {
    let contents = fs::read_to_string(path)?;

    let lines = contents.trim().split('\n').map(String::from).collect();

    Ok(lines)
}
