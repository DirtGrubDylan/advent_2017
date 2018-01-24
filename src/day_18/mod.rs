mod program;
mod registry;
mod instruction;

use file_reader::to_string_vector;

use self::program::Program;
use self::registry::Registry;
use self::instruction::Instruction;

pub fn run_day_18() {
    let instructions: Vec<Instruction> = to_string_vector("inputs/day_18.txt")
        .unwrap()
        .into_iter()
        .map(|s| Instruction::new(&s))
        .collect();
    let mut registry = Registry::new_with_instructions(0, &instructions);
    let mut program = Program::new(&instructions);

    println!(
        "Day 18, Part 1: {}",
        registry.last_recovered_sound().unwrap()
    );

    program.execute();

    println!("Day 18, Part 2: {}", program.second_registry.all_sent);
}
