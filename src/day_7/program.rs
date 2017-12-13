use std::collections::HashMap;

type Disc = Vec<String>;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub name: String,
    pub weight: usize,
    pub disc: Disc,
}

impl Program {
    pub fn new(info: &str) -> Program {
        let parsed_info: Vec<Vec<&str>> = info.split(" -> ")
            .map(|s| s.split_whitespace().collect::<Vec<&str>>())
            .collect();

        let weight_str = &parsed_info[0][1];
        let weight_str = &weight_str[1..(weight_str.len() - 1)];

        let weight = weight_str.parse().unwrap();
        let disc: Disc = if parsed_info.len() == 2 {
            Program::disc_from(&(parsed_info[1]))
        } else {
            Disc::new()
        };

        Program {
            name: parsed_info[0][0].to_owned(),
            weight: weight,
            disc: disc,
        }
    }

    pub fn combined_weight(&self, tower: &HashMap<String, Program>) -> usize {
        if self.disc.is_empty() {
            self.weight
        } else {
            let mut sub_sums = self.weight;

            for program_name in &self.disc {
                sub_sums += tower.get(program_name).unwrap().combined_weight(tower);
            }

            sub_sums
        }
    }

    pub fn unbalanced_tower(&self, tower: &HashMap<String, Program>) -> Option<(Program, isize)> {
        let mut counter: HashMap<isize, (Program, usize)> = HashMap::new();

        for program_name in &self.disc {
            let temp_program = tower.get(program_name).unwrap();

            counter
                .entry(temp_program.combined_weight(tower) as isize)
                .and_modify(|e| e.1 += 1)
                .or_insert((temp_program.clone(), 1));
        }

        let (mut majority_weight, mut minority_weight) = (0, 0);
        let mut unbalanced_program = None;

        for (weight, (program, count)) in counter {
            if count == 1 {
                unbalanced_program = Some(program);
                minority_weight = weight;
            } else {
                majority_weight = weight;
            }
        }

        if let Some(program) = unbalanced_program {
            Some((program, majority_weight - minority_weight))
        } else {
            None
        }
    }

    fn disc_from(program_names: &[&str]) -> Disc {
        program_names
            .iter()
            .map(|&s| {
                if s.ends_with(',') {
                    s[..(s.len() - 1)].to_owned()
                } else {
                    s.to_owned()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let test_data = vec!["pbga (66)", "fwft (72) -> ktlj, cntj, xhth"]
            .into_iter()
            .map(|s| Program::new(s))
            .collect::<Vec<Program>>();

        let expected = vec![
            Program {
                name: "pbga".to_owned(),
                weight: 66,
                disc: Disc::new(),
            },
            Program {
                name: "fwft".to_owned(),
                weight: 72,
                disc: vec!["ktlj".to_owned(), "cntj".to_owned(), "xhth".to_owned()],
            },
        ];

        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_combined_weight() {
        let test_data: Vec<Program> = to_string_vector("test_inputs/day_7_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|s| Program::new(&s))
            .collect();
        let mut test_map: HashMap<String, Program> = HashMap::new();

        for program in &test_data {
            test_map.insert(program.name.clone(), program.clone());
        }

        let test_weights: Vec<usize> = test_data
            .iter()
            .map(|program| program.combined_weight(&test_map))
            .collect();
        let expected = vec![66, 57, 61, 66, 57, 243, 66, 243, 778, 61, 251, 61, 57];

        assert_eq!(test_weights, expected);
    }

    #[test]
    fn test_unbalanced_tower() {
        let mut test_map: HashMap<String, Program> = HashMap::new();

        for program_data in to_string_vector("test_inputs/day_7_part_1.txt").unwrap() {
            let program = Program::new(&program_data);

            test_map.insert(program.name.clone(), program);
        }

        let expected = Some((Program::new("ugml (68) -> gyxo, ebii, jptl"), -8));

        assert_eq!(
            test_map.get("ugml").unwrap().unbalanced_tower(&test_map),
            None
        );
        assert_eq!(
            test_map.get("tknk").unwrap().unbalanced_tower(&test_map),
            expected
        );
    }

    #[test]
    fn test_disc_from() {
        let test_info = ["ktlj,", "cntj,", "xhth"];

        assert_eq!(Program::disc_from(&test_info), vec!["ktlj", "cntj", "xhth"]);
    }
}
