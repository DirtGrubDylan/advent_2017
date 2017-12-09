use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn to_string_vector(file_name: &str) -> Result<Vec<String>, String> {
    let file = BufReader::new(File::open(file_name).expect("File not found!"));

    Ok(file.lines()
        .map(|line| line.expect("The file is bad!"))
        .collect())
}
