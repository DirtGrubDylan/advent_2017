#[derive(Debug, PartialEq)]
pub struct JumpInstructioins {
    instructions: Vec<isize>,
}

impl JumpInstructioins {
    pub fn new(instructions: Vec<String>) -> JumpInstructioins {
        JumpInstructioins {
            instructions: instructions
                .into_iter()
                .map(|s| s.parse::<isize>().unwrap())
                .collect(),
        }
    }

    pub fn steps_to_exit<F>(&self, rules: F) -> usize
    where
        F: Fn(isize) -> isize,
    {
        let mut steps = 1;
        let mut current_index = 0;
        let mut temp_instructions = self.instructions.clone();

        loop {
            let offset = temp_instructions[current_index];

            temp_instructions[current_index] = rules(temp_instructions[current_index]);

            let temp_index = (current_index as isize) + offset;

            if (0 <= temp_index) && (temp_index < (temp_instructions.len() as isize)) {
                current_index = temp_index as usize;
            } else {
                break;
            }

            steps += 1;
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let test_instruction =
            JumpInstructioins::new(to_string_vector("test_inputs/day_5_part_1.txt").unwrap());
        let expected = JumpInstructioins {
            instructions: vec![0, 3, 0, 1, -3],
        };

        assert_eq!(test_instruction, expected);
    }

    #[test]
    fn test_steps_to_exit() {
        let test_instruction =
            JumpInstructioins::new(to_string_vector("test_inputs/day_5_part_1.txt").unwrap());

        assert_eq!(test_instruction.steps_to_exit(|x| { x + 1 }), 5);
        assert_eq!(test_instruction.steps_to_exit(|x| {
            if 3 <= x {
                x - 1
            } else {
                x + 1
            }
        }), 10);
    }
}
