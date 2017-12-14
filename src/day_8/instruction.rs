use std::collections::HashMap;
use super::conditional::Conditional;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub name_of_register_to_modify: String,
    pub new_value_to_modify_with: isize,
    conditional: Conditional,
}

impl Instruction {
    pub fn new(instruction: &str) -> Instruction {
        let info: Vec<String> = instruction
            .split_whitespace()
            .map(|s| String::from(s))
            .collect();

        let multiplier = if info[1] == "dec" { -1 } else { 1 };

        Instruction {
            name_of_register_to_modify: info[0].clone(),
            new_value_to_modify_with: info[2].parse::<isize>().unwrap() * multiplier,
            conditional: Conditional::new(&info[4], &info[5], (&info[6]).parse().unwrap()),
        }
    }

    pub fn execute(&self, registers: &mut HashMap<String, isize>) {
        if self.conditional.is_true(registers) {
            registers
                .entry(self.name_of_register_to_modify.clone())
                .and_modify(|e| *e += self.new_value_to_modify_with)
                .or_insert(self.new_value_to_modify_with);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let expected = Instruction {
            name_of_register_to_modify: String::from("c"),
            new_value_to_modify_with: 10,
            conditional: Conditional::new("a", ">=", 1),
        };

        assert_eq!(Instruction::new("c dec -10 if a >= 1"), expected);
    }

    #[test]
    fn test_execute() {
        let inputs: Vec<Instruction> = to_string_vector("test_inputs/day_8_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|s| Instruction::new(&s))
            .collect();
        let mut registers: HashMap<String, isize> = HashMap::new();

        inputs[0].execute(&mut registers);

        assert!(registers.is_empty());

        inputs[1].execute(&mut registers);

        assert_eq!(registers.get(&String::from("a")), Some(&1));
        assert_eq!(registers.get(&String::from("b")), None);
        assert_eq!(registers.get(&String::from("c")), None);

        inputs[2].execute(&mut registers);

        assert_eq!(registers.get(&String::from("a")), Some(&1));
        assert_eq!(registers.get(&String::from("b")), None);
        assert_eq!(registers.get(&String::from("c")), Some(&10));

        inputs[3].execute(&mut registers);

        assert_eq!(registers.get(&String::from("a")), Some(&1));
        assert_eq!(registers.get(&String::from("b")), None);
        assert_eq!(registers.get(&String::from("c")), Some(&-10));
    }
}
