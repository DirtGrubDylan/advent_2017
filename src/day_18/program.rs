use super::registry::Registry;
use super::instruction::Instruction;

#[derive(Debug)]
pub struct Program {
    pub first_registry: Registry,
    pub second_registry: Registry,
    pub deadlocked: bool,
}

impl Program {
    pub fn new(instructions: &[Instruction]) -> Program {
        Program {
            first_registry: Registry::new_with_instructions(0, instructions),
            second_registry: Registry::new_with_instructions(1, instructions),
            deadlocked: false,
        }
    }

    pub fn is_deadlocked(first_registry: &Registry, second_registry: &Registry) -> bool {
        let first_is_deadlocked = first_registry.waiting && second_registry.to_send.is_empty();
        let second_is_deadlocked = second_registry.waiting && first_registry.to_send.is_empty();

        first_is_deadlocked && second_is_deadlocked
    }

    pub fn execute(&mut self) {
        let mut first_registry_finished = self.first_registry.next().is_none();
        let mut second_registry_finished = self.second_registry.next().is_none();

        while !first_registry_finished || !second_registry_finished {
            if Self::is_deadlocked(&self.first_registry, &self.second_registry) {
                self.deadlocked = true;

                break;
            }

            while self.first_registry.waiting && !self.second_registry.to_send.is_empty() {
                self.first_registry
                    .recieve_from(&mut self.second_registry.to_send);

                first_registry_finished = self.first_registry.next().is_none();
            }

            while self.second_registry.waiting && !self.first_registry.to_send.is_empty() {
                self.second_registry
                    .recieve_from(&mut self.first_registry.to_send);

                second_registry_finished = self.second_registry.next().is_none();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_execute() {
        let test_instructions: Vec<Instruction> = to_string_vector("test_inputs/day_18_part_2.txt")
            .unwrap()
            .into_iter()
            .map(|s| Instruction::new(&s))
            .collect();
        let mut test_program = Program::new(&test_instructions);

        assert!(!test_program.deadlocked);
        assert_eq!(test_program.first_registry.value_of("b"), 0);
        assert_eq!(test_program.second_registry.value_of("b"), 0);
        assert_eq!(test_program.second_registry.all_sent, 0);

        test_program.execute();

        assert!(test_program.deadlocked);
        assert_eq!(test_program.first_registry.value_of("b"), 4);
        assert_eq!(test_program.second_registry.value_of("b"), 4);
        assert_eq!(test_program.second_registry.all_sent, 1);
    }

    #[test]
    fn test_is_deadlocked() {
        let test_instructions: Vec<Instruction> = to_string_vector("test_inputs/day_18_part_2.txt")
            .unwrap()
            .into_iter()
            .map(|s| Instruction::new(&s))
            .collect();
        let mut test_registry_0 = Registry::new_with_instructions(0, &test_instructions);
        let mut test_registry_1 = Registry::new_with_instructions(1, &test_instructions);

        test_registry_0.next();
        test_registry_1.next();

        assert!(!Program::is_deadlocked(&test_registry_0, &test_registry_1));

        test_registry_0.recieve_from(&mut test_registry_1.to_send);
        test_registry_1.recieve_from(&mut test_registry_0.to_send);

        test_registry_0.next();
        test_registry_1.next();

        assert!(Program::is_deadlocked(&test_registry_0, &test_registry_1));
    }
}
