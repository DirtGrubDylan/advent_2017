mod hexagon;

use file_reader::to_string_vector;
use self::hexagon::Hexagon;

pub fn run_day_11() {
    let input = &to_string_vector("inputs/day_11.txt").unwrap()[0];
    let directions: Vec<&str> = input.split(',').collect();

    let origin = Hexagon::new(0, 0, 0);

    let destination = origin.offset_by_direction_list(&directions).unwrap();

    println!("Day 11, Part 1: {}", origin.distance_to(&destination));

    let mut max_distance = 0;
    let mut temp_hex = origin.clone();

    for direction in &directions {
        match temp_hex.offset_by_direction(direction) {
            Err(err) => panic!(err),
            Ok(hex) => {
                temp_hex = hex;
                max_distance = max_distance.max(origin.distance_to(&temp_hex));
            },
        }
    }

    println!("Day 11, Part 2: {}", max_distance);
}
