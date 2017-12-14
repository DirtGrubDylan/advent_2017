use std::cmp;
use std::collections::HashMap;

use super::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub struct Registers {
    pub inner: HashMap<String, isize>,
    pub highest_register_value_achieved: isize,
    instructions: Vec<Instruction>,
}

impl Registers {
    pub fn new(instructions: &[String]) -> Registers {
        Registers {
            inner: HashMap::new(),
            highest_register_value_achieved: 0,
            instructions: instructions.iter().map(|s| Instruction::new(s)).collect(),
        }
    }

    pub fn run_instructions(&mut self) {
        for instruction in &self.instructions {
            instruction.execute(&mut self.inner);

            self.highest_register_value_achieved = cmp::max(
                self.largest_register(),
                self.highest_register_value_achieved,
            );
        }
    }

    pub fn largest_register(&self) -> isize {
        if self.inner.is_empty() {
            0
        } else {
            *self.inner.values().max().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_run_instructions() {
        let inputs = to_string_vector("test_inputs/day_8_part_1.txt").unwrap();
        let mut test_registers = Registers::new(&inputs);
        let mut expected_map = HashMap::new();

        expected_map.insert(String::from("a"), 1);
        expected_map.insert(String::from("c"), -10);

        assert!(test_registers.inner.is_empty());

        test_registers.run_instructions();

        assert_eq!(test_registers.inner, expected_map);
    }

    #[test]
    fn test_largest_register() {
        let inputs = to_string_vector("test_inputs/day_8_part_1.txt").unwrap();
        let mut test_registers = Registers::new(&inputs);

        assert_eq!(test_registers.largest_register(), 0);

        test_registers.run_instructions();

        assert_eq!(test_registers.largest_register(), 1);
    }

    #[test]
    fn test_highest_value_achieved() {
        let inputs = to_string_vector("test_inputs/day_8_part_1.txt").unwrap();
        let mut test_registers = Registers::new(&inputs);

        assert_eq!(test_registers.highest_register_value_achieved, 0);

        test_registers.run_instructions();

        assert_eq!(test_registers.highest_register_value_achieved, 10);
    }
}
