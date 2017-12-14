use std::str::Chars;

pub struct StreamReader {
    inner: String,
}

impl StreamReader {
    pub fn new(stream: &str) -> StreamReader {
        StreamReader {
            inner: String::from(stream),
        }
    }

    pub fn total_group_score(&self) -> usize {
        let mut depth = 1;
        let mut score = 0;
        let mut is_garbage = false;
        let mut skip = false;

        for c in self.inner.chars() {
            match c {
                '{' => {
                    if !is_garbage {
                        score += depth;
                        depth += 1;
                    }
                }
                '<' => {
                    is_garbage = true;
                }
                '>' => {
                    is_garbage = false;
                }
                _ => {}
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_total_group_score() {
        let test_data = to_string_vector("test_inputs/day_9_part_1.txt").unwrap();

        let test_scores: Vec<usize> = test_data
            .iter()
            .map(|s| StreamReader::new(s).total_group_score())
            .collect();
        let expected = vec![1, 6, 5, 16, 1, 9, 9, 3];

        assert_eq!(test_scores, expected);
    }
}
