use std::collections::HashMap;

use super::program::Program;

pub struct Tower {
    pub programs: HashMap<String, Program>,
}

impl Tower {
    pub fn new(data: &[String]) -> Tower {
        let mut map = HashMap::new();

        for program_data in data {
            let program = Program::new(program_data);

            map.insert(program.name.clone(), program);
        }

        Tower { programs: map }
    }

    pub fn lowest_program(&self) -> Program {
        self.programs
            .values()
            .max_by_key(|program| program.combined_weight(&self.programs))
            .unwrap()
            .clone()
    }

    pub fn unbalanced_program(&self) -> (Program, isize) {
        let mut current_program = self.lowest_program();
        let mut current_offset = 0;
        let mut unbalanced_info = current_program.unbalanced_tower(&self.programs);

        while let Some((temp_program, temp_offset)) = unbalanced_info {
            current_program = temp_program;
            current_offset = temp_offset;
            unbalanced_info = current_program.unbalanced_tower(&self.programs);
        }

        (current_program, current_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_lowest_program() {
        let test_tower = Tower::new(&(to_string_vector("test_inputs/day_7_part_1.txt").unwrap()));
        let expected = "tknk";

        assert_eq!(test_tower.lowest_program().name, expected);
    }

    #[test]
    fn test_unbalanced_program() {
        let test_tower = Tower::new(&(to_string_vector("test_inputs/day_7_part_1.txt").unwrap()));
        let expected = 60;
        let tested = test_tower.unbalanced_program();
        let tested = (tested.0.weight as isize) + tested.1;

        assert_eq!(tested, expected);
    }
}
