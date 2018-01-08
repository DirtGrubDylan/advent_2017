mod location;
mod disk_grid;

use file_reader::to_string_vector;

use self::disk_grid::DiskGrid;

pub fn run_day_14() {
    let input = to_string_vector("inputs/day_14.txt").unwrap();

    let disk_grid = DiskGrid::new(&input[0]);

    println!("Day 14, Part 1: {}", disk_grid.number_of_used_squares());
    println!("Day 14, Part 2: {}", disk_grid.regions().len());
}
