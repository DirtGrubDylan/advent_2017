mod pass_phrase;

use file_reader::to_string_vector;
use self::pass_phrase::PassPhrase;

pub fn run_day_4() {
    let inputs = to_string_vector("inputs/day_4.txt").unwrap();

    let number_of_valid_inputs_old = inputs.iter().fold(0, |acc, input| {
        if PassPhrase::new(input).is_valid_old() {
            acc + 1
        } else {
            acc
        }
    });

    let number_of_valid_inputs = inputs.iter().fold(0, |acc, input| {
        if PassPhrase::new(input).is_valid() {
            acc + 1
        } else {
            acc
        }
    });

    println!("Day 4, Part 1: {}", number_of_valid_inputs_old);
    println!("Day 4, Part 2: {}", number_of_valid_inputs);
}
