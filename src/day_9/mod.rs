mod stream_reader;
mod stream_reader_info;

use file_reader::to_string_vector;
use self::stream_reader::StreamReader;

pub fn run_day_9() {
    let input = to_string_vector("inputs/day_9.txt").unwrap();

    let mut stream_reader = StreamReader::new();

    stream_reader.read_stream(&input[0]);

    let stream_info = stream_reader.info.to_tuple();

    println!("Day 9, Part 1: {}", stream_info.1);
    println!("Day 9, Part 2: {}", stream_info.2);
}
