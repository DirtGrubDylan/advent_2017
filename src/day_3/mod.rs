mod grid;
mod square;
mod element;
mod direction;

use file_reader::to_string_vector;

use self::grid::Grid;
use self::square::Square;

pub fn run_day_3() {
    let data = to_string_vector("inputs/day_3.txt").unwrap();

    let input = data[0].parse().unwrap();

    println!("Day 3, Part 1: {}",
             Square::new(input).distance_from_center_of_memory());

    let mut current_value = 1;
    let mut grid = Grid::new();

    while current_value <= input {
        current_value = grid.next().value;
    }

    println!("Day 3, Part 1: {}", current_value);
}
