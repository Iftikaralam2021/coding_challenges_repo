use std::fs;

pub fn count_words(filename: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let count = contents.split_whitespace().count();
    Ok(count)
}