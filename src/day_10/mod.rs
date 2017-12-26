mod macramist;

use file_reader::to_string_vector;
use self::macramist::Macramist;

pub fn run_day_10() {
    let input = to_string_vector("inputs/day_10.txt").unwrap();
    let lengths: Vec<usize> = input[0].split(',').map(|s| s.parse().unwrap()).collect();

    let mut step = 0;
    let mut current_position = 0;
    let mut macramist = Macramist::new(256);

    macramist.tie_with_lengths(&mut step, &mut current_position, &lengths);

    println!("Day 10, Part 1: {}", macramist.hash_from_first_two());

    macramist.reset();

    println!("Day 10, Part 2: {}", macramist.hash(&input[0]));
}
