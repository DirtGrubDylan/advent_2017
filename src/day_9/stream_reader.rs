use std::str::Chars;

pub struct StreamReader<'a> {
    inner: Chars<'a>,
}

impl<'a> StreamReader<'a> {
    pub fn new(stream: &str) -> StreamReader {
        StreamReader {
            inner: stream.chars(),
        }
    }

    pub fn total_group_score(&self) -> usize {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_total_group_score() {
        let test_data = to_string_vector("test_inputs/day_9_part_1.txt").unwrap();

        let test_scores: Vec<usize> =
            test_data.iter().map(|s| StreamReader::new(s).total_group_score()).collect();
        let expected = vec![1, 6, 5, 16, 1, 9, 9 ,3];

        assert_eq!(test_scores, expected);
    }
}
