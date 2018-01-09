#[derive(Debug, PartialEq)]
pub enum Instruction {
    Set(char, String),
    Add(char, String),
    Mod(char, String),
    Mutliply(char, String),
    Recover(String),
    PlaySound(String),
    Jump(String, String),
}

impl Instruction {
    pub fn new(instruction: &str) -> Instruction {
        let info: Vec<&str> = instruction.split_whitespace().collect();

        match info[0] {
            "set" => Instruction::Set(info[1].chars().nth(0).unwrap(), String::from(info[2])),
            "add" => Instruction::Add(info[1].chars().nth(0).unwrap(), String::from(info[2])),
            "mod" => Instruction::Mod(info[1].chars().nth(0).unwrap(), String::from(info[2])),
            "mul" => Instruction::Mutliply(info[1].chars().nth(0).unwrap(), String::from(info[2])),
            "rcv" => Instruction::Recover(String::from(info[1])),
            "snd" => Instruction::PlaySound(String::from(info[1])),
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
            Instruction::Set('a', String::from("1")),
            Instruction::Add('a', String::from("2")),
            Instruction::Mutliply('a', String::from("a")),
            Instruction::Mod('a', String::from("5")),
            Instruction::PlaySound(String::from("a")),
            Instruction::Set('a', String::from("0")),
            Instruction::Recover(String::from("a")),
            Instruction::Jump(String::from("a"), String::from("-1")),
            Instruction::Set('a', String::from("1")),
            Instruction::Jump(String::from("a"), String::from("-2")),
        ];

        assert_eq!(test_instructions, expected);
    }
}
