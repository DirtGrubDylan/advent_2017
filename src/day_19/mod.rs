mod grid;
mod mouse;
mod direction;

use file_reader::to_string_vector;
use self::grid::Grid;
use self::mouse::Mouse;

pub fn run_day_19() {
    let input = to_string_vector("inputs/day_19.txt").unwrap();
    let grid = Grid::new(&input);
    let mut mouse = Mouse::new(&grid);

    println!("Day 19, Part 1: {}", mouse.find_all_letters());
    println!("Day 19, Part 2: {}", mouse.steps_taken);
}
