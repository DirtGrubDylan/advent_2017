mod pipe;
mod pipe_system;

use file_reader::to_string_vector;
use self::pipe_system::PipeSystem;

pub fn run_day_12() {
    let input = to_string_vector("inputs/day_12.txt").unwrap();
    let pipe_system = PipeSystem::new(&input);

    println!("Day 12, Part 1: {}", pipe_system.group_containing(0).len());
    println!("Day 12, Part 2: {}", pipe_system.groups().len());
}
