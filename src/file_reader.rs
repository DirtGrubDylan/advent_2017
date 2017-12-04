use std::fs::File;
use std::io::{BufRead, BufReader};


fn to_i32_vector(file_name: &str) -> Result<Vec<i32>, String> {
    let file = BufReader::new(File::open(file_name).expect("File not found!"));

    Ok(
        file.lines()
            .map(|line| {
                line.expect("The file is bad!")
                    .parse::<i32>()
                    .expect("Expected a number!")
            })
            .collect(),
    )
}
