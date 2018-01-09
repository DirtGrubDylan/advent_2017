use std::collections::HashMap;

use super::instruction::Instruction;

#[derive(Debug)]
pub struct Registry {
    pub registers: HashMap<String, isize>,
}

impl Registry {
    pub fn new() -> Registry {
        let mut temp_map = HashMap::new();

        temp_map.insert(String::from("sound"), 0);
        temp_map.insert(String::from("index"), 0);

        Registry {
            registers: temp_map,
        }
    }

    pub fn first_recovered_sound(&mut self, instructions: &[Instruction]) -> Option<isize> {
        let mut recovered_sound = None;
        let mut index = self.value_of("index");

        while 0 <= index && index < (instructions.len() as isize) {
            let instruction = &instructions[index as usize];

            recovered_sound = self.execute_instruction(instruction);

            match instruction {
                &Instruction::Recover(_) if recovered_sound.is_some() => break,
                _ => {}
            }

            index = self.value_of("index") + 1;
            self.set_register_value("index", index);
        }

        recovered_sound
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) -> Option<isize> {
        match instruction {
            &Instruction::Set(ref id, ref value) => {
                let temp_value = self.value_of(value);

                self.set_register_value(id, temp_value);

                None
            }
            &Instruction::Add(ref id, ref value) => {
                let temp_value = self.value_of(id) + self.value_of(value);

                self.set_register_value(id, temp_value);

                None
            }
            &Instruction::Mod(ref id, ref value) => {
                let temp_value = self.value_of(id) % self.value_of(value);

                self.set_register_value(id, temp_value);

                None
            }
            &Instruction::Mutliply(ref id, ref value) => {
                let temp_value = self.value_of(id) * self.value_of(value);

                self.set_register_value(id, temp_value);

                None
            }
            &Instruction::Recover(ref id) => {
                let temp_value = self.value_of(id);

                if temp_value != 0 {
                    Some(self.value_of("sound"))
                } else {
                    None
                }
            }
            &Instruction::PlaySound(ref value) => {
                let temp_value = self.value_of(value);

                self.set_register_value("sound", temp_value);

                Some(self.value_of("sound"))
            }
            &Instruction::Jump(ref id, ref value) => {
                let temp_reg_value = self.value_of(id);
                let temp_offset_value = self.value_of("index") + self.value_of(value);

                if 0 < temp_reg_value {
                    self.set_register_value("index", temp_offset_value - 1);
                }

                None
            }
        }
    }

    pub fn value_of(&self, x: &str) -> isize {
        match x.parse::<isize>() {
            Err(_) => match self.registers.get(x) {
                None => 0,
                Some(&num) => num,
            },
            Ok(num) => num,
        }
    }

    pub fn set_register_value(&mut self, register_id: &str, value: isize) {
        self.registers.insert(String::from(register_id), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use file_reader::to_string_vector;

    #[test]
    fn test_execute_instruction_set() {
        let mut test_registry = Registry::new();
        let set = Instruction::Set(String::from("a"), String::from("1"));

        assert_eq!(test_registry.value_of("a"), 0);
        assert_eq!(test_registry.execute_instruction(&set), None);
        assert_eq!(test_registry.value_of("a"), 1);
    }

    #[test]
    fn test_execute_instruction_add() {
        let mut test_registry = Registry::new();
        let add = Instruction::Add(String::from("a"), String::from("3"));

        assert_eq!(test_registry.execute_instruction(&add), None);
        assert_eq!(test_registry.value_of("a"), 3);
    }

    #[test]
    fn test_execute_instruction_mul() {
        let mut test_registry = Registry::new();
        let set = Instruction::Set(String::from("a"), String::from("3"));
        let mul = Instruction::Mutliply(String::from("a"), String::from("a"));

        assert_eq!(test_registry.execute_instruction(&set), None);
        assert_eq!(test_registry.execute_instruction(&mul), None);
        assert_eq!(test_registry.value_of("a"), 9);
    }

    #[test]
    fn test_execute_instruction_mod() {
        let mut test_registry = Registry::new();
        let set = Instruction::Set(String::from("a"), String::from("9"));
        let modd = Instruction::Mod(String::from("a"), String::from("5"));

        assert_eq!(test_registry.execute_instruction(&set), None);
        assert_eq!(test_registry.execute_instruction(&modd), None);
        assert_eq!(test_registry.value_of("a"), 4);
    }

    #[test]
    fn test_execute_instruction_snd() {
        let mut test_registry = Registry::new();
        let set = Instruction::Set(String::from("a"), String::from("4"));
        let snd = Instruction::PlaySound(String::from("a"));

        assert_eq!(test_registry.execute_instruction(&set), None);
        assert_eq!(test_registry.execute_instruction(&snd), Some(4));
        assert_eq!(test_registry.value_of("sound"), 4);
    }

    #[test]
    fn test_execute_instruction_rcv() {
        let mut test_registry = Registry::new();
        let set = Instruction::Set(String::from("a"), String::from("4"));
        let snd = Instruction::PlaySound(String::from("a"));
        let rcv_a = Instruction::Recover(String::from("a"));
        let rcv_b = Instruction::Recover(String::from("b"));
        let rcv_0 = Instruction::Recover(String::from("0"));

        assert_eq!(test_registry.execute_instruction(&set), None);
        assert_eq!(test_registry.execute_instruction(&snd), Some(4));
        assert_eq!(test_registry.execute_instruction(&rcv_a), Some(4));
        assert_eq!(test_registry.execute_instruction(&rcv_b), None);
        assert_eq!(test_registry.execute_instruction(&rcv_0), None);
    }

    #[test]
    fn test_execute_instruction_jgz() {
        let mut test_registry = Registry::new();
        let set_a_0 = Instruction::Set(String::from("a"), String::from("0"));
        let set_a_1 = Instruction::Set(String::from("a"), String::from("1"));
        let jgz_neg_1 = Instruction::Jump(String::from("a"), String::from("-1"));
        let jgz_neg_2 = Instruction::Jump(String::from("a"), String::from("-2"));

        assert_eq!(test_registry.execute_instruction(&set_a_0), None);
        assert_eq!(test_registry.execute_instruction(&jgz_neg_1), None);
        assert_eq!(test_registry.execute_instruction(&jgz_neg_2), None);
        assert_eq!(test_registry.value_of("index"), 0);
        assert_eq!(test_registry.execute_instruction(&set_a_1), None);
        assert_eq!(test_registry.execute_instruction(&jgz_neg_1), None);
        assert_eq!(test_registry.execute_instruction(&jgz_neg_2), None);
        assert_eq!(test_registry.value_of("index"), -5);
    }

    #[test]
    fn test_value_of() {
        let mut test_registry = Registry::new();

        assert_eq!(test_registry.value_of("a"), 0);
        assert_eq!(test_registry.value_of("133"), 133);
        assert_eq!(test_registry.value_of("-133"), -133);

        test_registry.set_register_value("a", -133);

        assert_eq!(test_registry.value_of("a"), -133);
    }

    #[test]
    fn test_first_recovered_sound() {
        let test_instructions: Vec<Instruction> = to_string_vector("test_inputs/day_18_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|s| Instruction::new(&s))
            .collect();
        let mut test_registry = Registry::new();

        assert_eq!(
            test_registry.first_recovered_sound(&test_instructions),
            Some(4)
        );
    }
}
