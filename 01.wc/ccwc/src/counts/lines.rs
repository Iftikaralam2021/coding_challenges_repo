use std::fs;

pub fn count_lines(file_path: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents.lines().count())
}