#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Set(String, String),
    Add(String, String),
    Mod(String, String),
    Mutliply(String, String),
    Snd(String),
    Recieve(String),
    Jump(String, String),
}

impl Instruction {
    pub fn new(instruction: &str) -> Instruction {
        let info: Vec<&str> = instruction.split_whitespace().collect();

        match info[0] {
            "set" => Instruction::Set(String::from(info[1]), String::from(info[2])),
            "add" => Instruction::Add(String::from(info[1]), String::from(info[2])),
            "mod" => Instruction::Mod(String::from(info[1]), String::from(info[2])),
            "mul" => Instruction::Mutliply(String::from(info[1]), String::from(info[2])),
            "rcv" => Instruction::Recieve(String::from(info[1])),
            "snd" => Instruction::Snd(String::from(info[1])),
            "jgz" => Instruction::Jump(String::from(info[1]), String::from(info[2])),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let test_instructions: Vec<Instruction> = to_string_vector("test_inputs/day_18_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|s| Instruction::new(&s))
            .collect();

        let expected = vec![
            Instruction::Set(String::from("a"), String::from("1")),
            Instruction::Add(String::from("a"), String::from("2")),
            Instruction::Mutliply(String::from("a"), String::from("a")),
            Instruction::Mod(String::from("a"), String::from("5")),
            Instruction::Snd(String::from("a")),
            Instruction::Set(String::from("a"), String::from("0")),
            Instruction::Recieve(String::from("a")),
            Instruction::Jump(String::from("a"), String::from("-1")),
            Instruction::Set(String::from("a"), String::from("1")),
            Instruction::Jump(String::from("a"), String::from("-2")),
        ];

        assert_eq!(test_instructions, expected);
    }
}
