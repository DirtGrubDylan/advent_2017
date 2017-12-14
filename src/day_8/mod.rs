mod registers;
mod conditional;
mod instruction;

use file_reader::to_string_vector;
use self::registers::Registers;

pub fn run_day_8() {
    let input = to_string_vector("inputs/day_8.txt").unwrap();
    let mut registers = Registers::new(&input);

    registers.run_instructions();

    println!("Day 8, Part 1: {}", registers.largest_register());
}
