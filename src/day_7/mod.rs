mod tower;
mod program;

use file_reader::to_string_vector;
use self::tower::Tower;

pub fn run_day_7() {
    let input = to_string_vector("inputs/day_7.txt").unwrap();
    let tower = Tower::new(&input);

    println!("Day 7, Part 1: {}", tower.lowest_program().name);

    let (unblanced_program, offset) = tower.unbalanced_program();

    let unblanced_program_new_weight = (unblanced_program.weight as isize) + offset;

    println!("Day 7, Part 2: {}", unblanced_program_new_weight);
}
