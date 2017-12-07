mod grid;
mod square;
mod element;
mod direction;

use self::grid::Grid;
use self::square::Square;

pub fn run_day_3() {
    println!("Day 3, Part 1: {}", Square::new(368078).distance_from_center_of_memory());

    let mut current_value = 1;
    let mut grid = Grid::new();

    while current_value <= 368078 {
        current_value = grid.next().value;
    }

    println!("Day 3, Part 1: {}", current_value);
}
