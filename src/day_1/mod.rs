pub mod captcha;

use file_reader::to_string_vector;
use self::captcha::Captcha;

pub fn run_day_1() {
    let day_1_input = to_string_vector("inputs/day_1.txt").unwrap();

    for input in day_1_input {
        let captcha = Captcha::new(&input);
        let step_size = captcha.value.len() / 2;

        println!("Day 1, Part 1: {}", captcha.sum(1));
        println!("Day 1, Part 2: {}", captcha.sum(step_size));
    }
}
