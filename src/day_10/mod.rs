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

    let mut ascii_lengths: Vec<usize> = input[0].chars().map(|c| c as usize).collect();

    ascii_lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    step = 0;
    current_position = 0;
    macramist = Macramist::new(256);

    for _ in 0..64 {
        macramist.tie_with_lengths(&mut step, &mut current_position, &ascii_lengths);
    }

    println!("Day 10, Part 2: {}", macramist.xor_hash());
}
