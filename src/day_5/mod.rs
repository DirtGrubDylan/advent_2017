mod jump_instructions;

use file_reader::to_string_vector;
use self::jump_instructions::JumpInstructioins;

pub fn run_day_5() {
    let day_5_input = to_string_vector("inputs/day_5.txt").unwrap();

    let instructions = JumpInstructioins::new(day_5_input);

    println!("Day 5, Part 1: {}", instructions.steps_to_exit(|x| { x + 1 }));
    println!("Day 5, Part 2: {}", instructions.steps_to_exit(|x| { 
        if 3 <= x {
            x - 1
        } else {
            x + 1
        }
     }));
}
