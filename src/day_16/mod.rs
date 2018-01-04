mod dance;
mod dance_moves;

use file_reader::to_string_vector;

use self::dance::Dance;

pub fn run_day_16() {
    let input = &to_string_vector("inputs/day_16.txt").unwrap()[0];

    let moves: Vec<&str> = input.split(',').collect();
    let dancers: Vec<char> = "abcdefghijklmnop".chars().collect();

    let mut dance = Dance::new(&moves, &dancers);

    dance.dance();

    println!("Day 16, Part 1: {}", dance.dancers.iter().collect::<String>());

    dance.dance_n_times(999_999_999);

    println!("Day 16, Part 2: {}", dance.dancers.iter().collect::<String>());
}
