use std::collections::{HashMap, VecDeque};

use super::instruction::Instruction;

#[derive(Debug)]
pub struct Registry {
    pub registers: HashMap<String, isize>,
    instructions: Vec<Instruction>,
    to_send: VecDeque<isize>,
    waiting: bool,
}

impl Registry {
    pub fn new(id: isize) -> Registry {
        let mut temp_map = HashMap::new();

        temp_map.insert(String::from("sound"), 0);
        temp_map.insert(String::from("index"), 0);
        temp_map.insert(String::from("p"), id);

        Registry {
            registers: temp_map,
            instructions: Vec::new(),
            to_send: VecDeque::new(),
            waiting: false,
        }
    }

    pub fn new_with_instructions(id: isize, instructions: &[Instruction]) -> Registry {
        let mut temp_map = HashMap::new();

        temp_map.insert(String::from("sound"), 0);
        temp_map.insert(String::from("index"), 0);
        temp_map.insert(String::from("p"), id);

        Registry {
            registers: temp_map,
            instructions: instructions.clone().to_owned(),
            to_send: VecDeque::new(),
            waiting: false,
        }
    }

    pub fn first_recovered_sound(&mut self) -> Option<isize> {
        match self.next() {
            None => None,
            Some(v) => match v.front() {
                None => None,
                Some(&num) => Some(num),
            },
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            &Instruction::Set(ref id, ref value) => {
                let temp_value = self.value_of(value);

                self.set_register_value(id, temp_value);
            }
            &Instruction::Add(ref id, ref value) => {
                let temp_value = self.value_of(id) + self.value_of(value);

                self.set_register_value(id, temp_value);
            }
            &Instruction::Mod(ref id, ref value) => {
                let temp_value = self.value_of(id) % self.value_of(value);

                self.set_register_value(id, temp_value);
            }
            &Instruction::Mutliply(ref id, ref value) => {
                let temp_value = self.value_of(id) * self.value_of(value);

                self.set_register_value(id, temp_value);
            }
            &Instruction::Snd(ref value) => {
                let temp_value = self.value_of(value);

                self.set_register_value("sound", temp_value);
                self.to_send.push_back(temp_value);
            }
            &Instruction::Recieve(_) => {}
            &Instruction::Jump(ref id, ref value) => {
                let temp_reg_value = self.value_of(id);
                let temp_offset_value = self.value_of("index") + self.value_of(value);

                if 0 < temp_reg_value {
                    self.set_register_value("index", temp_offset_value - 1);
                }
            }
        }
    }

    pub fn recieve_from(&mut self, sent_data: &mut VecDeque<isize>) {
        let index = self.value_of("index");
        let instruction = self.instructions[index as usize].clone();

        match instruction {
            Instruction::Recieve(ref id) => if let Some(&value) = sent_data.front() {
                self.set_register_value(id, value);
                self.set_register_value("index", index + 1);
                self.waiting = false;
            },
            _ => {}
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

impl Iterator for Registry {
    type Item = VecDeque<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = self.value_of("index");
        let number_of_instructions = self.instructions.len() as isize;

        while 0 <= index && index < number_of_instructions {
            let instruction = self.instructions[index as usize].clone();

            self.execute_instruction(&instruction);

            match instruction {
                Instruction::Recieve(_) => break,
                _ => {}
            }

            index = self.value_of("index") + 1;
            self.set_register_value("index", index);
        }

        println!("{:?}", self.registers);

        if 0 <= index && index < number_of_instructions {
            self.waiting = true;

            Some(self.to_send.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use file_reader::to_string_vector;

    #[test]
    fn test_execute_instruction_set() {
        let mut test_registry = Registry::new(0);
        let set = Instruction::Set(String::from("a"), String::from("1"));

        assert_eq!(test_registry.value_of("a"), 0);

        test_registry.execute_instruction(&set);

        assert_eq!(test_registry.value_of("a"), 1);
    }

    #[test]
    fn test_execute_instruction_add() {
        let mut test_registry = Registry::new(0);
        let add = Instruction::Add(String::from("a"), String::from("3"));

        test_registry.execute_instruction(&add);

        assert_eq!(test_registry.value_of("a"), 3);
    }

    #[test]
    fn test_execute_instruction_mul() {
        let mut test_registry = Registry::new(0);
        let set = Instruction::Set(String::from("a"), String::from("3"));
        let mul = Instruction::Mutliply(String::from("a"), String::from("a"));

        test_registry.execute_instruction(&set);
        test_registry.execute_instruction(&mul);

        assert_eq!(test_registry.value_of("a"), 9);
    }

    #[test]
    fn test_execute_instruction_mod() {
        let mut test_registry = Registry::new(0);
        let set = Instruction::Set(String::from("a"), String::from("9"));
        let modd = Instruction::Mod(String::from("a"), String::from("5"));

        test_registry.execute_instruction(&set);
        test_registry.execute_instruction(&modd);

        assert_eq!(test_registry.value_of("a"), 4);
    }

    #[test]
    fn test_execute_instruction_snd() {
        let mut test_registry = Registry::new(0);
        let set = Instruction::Set(String::from("a"), String::from("4"));
        let snd = Instruction::Snd(String::from("a"));

        test_registry.execute_instruction(&set);
        test_registry.execute_instruction(&snd);

        assert_eq!(test_registry.value_of("sound"), 4);
    }

    #[test]
    fn test_execute_instruction_jgz() {
        let mut test_registry = Registry::new(0);
        let set_a_0 = Instruction::Set(String::from("a"), String::from("0"));
        let set_a_1 = Instruction::Set(String::from("a"), String::from("1"));
        let jgz_neg_1 = Instruction::Jump(String::from("a"), String::from("-1"));
        let jgz_neg_2 = Instruction::Jump(String::from("a"), String::from("-2"));

        test_registry.execute_instruction(&set_a_0);
        test_registry.execute_instruction(&jgz_neg_1);
        test_registry.execute_instruction(&jgz_neg_2);

        assert_eq!(test_registry.value_of("index"), 0);

        test_registry.execute_instruction(&set_a_1);
        test_registry.execute_instruction(&jgz_neg_1);
        test_registry.execute_instruction(&jgz_neg_2);

        assert_eq!(test_registry.value_of("index"), -5);
    }

    #[test]
    fn test_value_of() {
        let mut test_registry = Registry::new(0);

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
        let mut test_registry = Registry::new_with_instructions(0, &test_instructions);

        assert_eq!(test_registry.first_recovered_sound(), Some(4));
    }

    #[test]
    fn test_next() {
        let test_instructions: Vec<Instruction> = to_string_vector("test_inputs/day_18_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|s| Instruction::new(&s))
            .collect();
        let mut test_registry = Registry::new_with_instructions(0, &test_instructions);

        assert_eq!(test_registry.next().unwrap().front(), Some(&4));
        assert!(test_registry.waiting);
    }

    #[test]
    fn test_recieve_from() {
        let test_instructions: Vec<Instruction> = to_string_vector("test_inputs/day_18_part_2.txt")
            .unwrap()
            .into_iter()
            .map(|s| Instruction::new(&s))
            .collect();
        let mut test_registry_0 = Registry::new_with_instructions(0, &test_instructions);
        let mut test_registry_1 = Registry::new_with_instructions(1, &test_instructions);

        let mut v_0 = test_registry_0.next().unwrap();
        test_registry_1.next().unwrap();

        assert_eq!(test_registry_0.value_of("a"), 0);
        assert_eq!(test_registry_1.value_of("a"), 0);
        assert_eq!(test_registry_0.value_of("b"), 0);
        assert_eq!(test_registry_1.value_of("b"), 0);

        test_registry_1.recieve_from(&mut v_0);

        assert_eq!(test_registry_0.value_of("b"), 0);
        assert_eq!(test_registry_1.value_of("b"), 4);
    }
}
