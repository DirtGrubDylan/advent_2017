mod grid;
mod square;

use self::grid::Grid;
use self::square::Square;

pub fn run_day_3() {
    println!("Day 3, Part 1: {}", Square::new(368078).distance_from_center_of_memory());
}
