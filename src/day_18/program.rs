use std::collections::VecDeque;

use super::registry::Registry;
use super::instruction::Instruction;

#[derive(Debug)]
pub struct Program {
    pub first_registry: Registry,
    pub second_registry: Registry,
    pub first_registry_sent_data: VecDeque<isize>,
    pub second_registry_sent_data: VecDeque<isize>,
}

impl Program {
    pub fn new(instructions: &[Instruction]) -> Program {
        Program {
            first_registry: Registry::new_with_instructions(0, instructions),
            second_registry: Registry::new_with_instructions(1, instructions),
            first_registry_sent_data: VecDeque::new(),
            second_registry_sent_data: VecDeque::new(),
        }
    }

    pub fn execute(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let test_instructions: Vec<Instruction> = to_string_vector("test_inputs/day_18_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|s| Instruction::new(&s))
            .collect();
        let mut test_program = Program::new(&test_instructions);

        unimplemented!();
    }
}
