mod spreadsheet;

use file_reader::to_string_vector;
use self::spreadsheet::SpreadSheet;

pub fn run_day_2() {
    let day_2_input = to_string_vector("inputs/day_2.txt").unwrap();

    let spreadsheet = SpreadSheet::new(&day_2_input).unwrap();

    println!("Day 2, Part 1: {}", spreadsheet.min_max_checksum());
}