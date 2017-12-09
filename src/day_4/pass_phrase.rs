use std::collections::HashSet;

#[derive(Debug)]
pub struct PassPhrase {
    pub phrase: Vec<String>,
}

impl PassPhrase {
    pub fn new(phrase: &str) -> PassPhrase {
        PassPhrase {
            phrase: phrase.split_whitespace().map(|s| s.to_owned()).collect(),
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut seen_words = HashSet::new();
        let mut is_valid = true;

        for word in &self.phrase {
            let mut temp_vec: Vec<char> = word.chars().collect();

            temp_vec.sort();

            if !seen_words.insert(temp_vec) {
                is_valid = false;
                break;
            }
        }

        is_valid
    }

    pub fn is_valid_old(&self) -> bool {
        let mut seen_words = HashSet::new();
        let mut is_valid = true;

        for word in &self.phrase {
            if !seen_words.insert(word) {
                is_valid = false;
                break;
            }
        }

        is_valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_is_valid_old() {
        let test_input: Vec<bool> = to_string_vector("test_inputs/day_4_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|s| PassPhrase::new(&s).is_valid_old())
            .collect();
        let expected_values = vec![true, false, true];

        assert_eq!(test_input, expected_values);
    }

    #[test]
    fn test_is_valid() {
        let test_input: Vec<bool> = to_string_vector("test_inputs/day_4_part_2.txt")
            .unwrap()
            .into_iter()
            .map(|s| PassPhrase::new(&s).is_valid())
            .collect();
        let expected_values = vec![true, false, true, true, false];

        assert_eq!(test_input, expected_values);
    }
}
