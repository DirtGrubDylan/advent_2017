mod disk_grid;

use self::disk_grid::DiskGrid;

pub fn run_day_14() {
    let input = "hwlqcszp";
    let disk_grid = DiskGrid::new(input);

    println!("Day 14, Part 1: {}", disk_grid.number_of_used_squares());
}
