use std::collections::HashSet;

type Block = usize;

#[derive(Debug, PartialEq)]
pub struct MemoryBank {
    pub data: Vec<Block>,
}

impl MemoryBank {
    pub fn new(memory_bank: &str) -> MemoryBank {
        MemoryBank {
            data: memory_bank
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }

    pub fn number_of_redistribution_cycles(&self) -> (usize, Vec<Block>) {
        let mut cycle = 0;
        let mut temp_data = self.data.clone();
        let mut seen_patterns = HashSet::new();

        while seen_patterns.insert(temp_data.clone()) {
            cycle += 1;
            temp_data = MemoryBank::redistribute_blocks(&temp_data);
        }

        (cycle, temp_data)
    }

    pub fn length_of_first_redistribution_loop(&self) -> usize {
        let (first_repeat_cycle, first_repeat_pattern) = self.number_of_redistribution_cycles();

        let mut cycle = 0;
        let mut temp_data = self.data.clone();

        while first_repeat_pattern != temp_data {
            cycle += 1;
            temp_data = MemoryBank::redistribute_blocks(&temp_data);
        }

        first_repeat_cycle - cycle
    }

    fn redistribute_blocks_at(mut index: usize, data: &[Block]) -> Vec<Block> {
        let mut temp_data = data.to_vec();
        let mut number_of_blocks = temp_data[index];
        temp_data[index] = 0;

        while number_of_blocks != 0 {
            index = (index + 1) % temp_data.len();

            temp_data[index] += 1;
            number_of_blocks -= 1;
        }

        temp_data
    }

    fn redistribute_blocks(data: &[Block]) -> Vec<Block> {
        let (max_index, _) = data.iter()
            .enumerate()
            .max_by_key(|&(i, x)| (x, -(i as isize)))
            .unwrap();

        MemoryBank::redistribute_blocks_at(max_index, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let input = to_string_vector("test_inputs/day_6_part_1.txt").unwrap();
        let expected = MemoryBank {
            data: vec![0, 2, 7, 0],
        };

        assert_eq!(MemoryBank::new(&input[0]), expected);
    }

    #[test]
    fn test_number_of_redistribution_cycles() {
        let mem_bank =
            MemoryBank::new(&to_string_vector("test_inputs/day_6_part_1.txt").unwrap()[0]);

        assert_eq!(
            mem_bank.number_of_redistribution_cycles(),
            (5, vec![2, 4, 1, 2])
        );
    }

    #[test]
    fn test_redistribute_blocks_at() {
        assert_eq!(
            MemoryBank::redistribute_blocks_at(2, &vec![0, 2, 7, 0]),
            vec![2, 4, 1, 2]
        );
        assert_eq!(
            MemoryBank::redistribute_blocks_at(1, &vec![2, 4, 1, 2]),
            vec![3, 1, 2, 3]
        );
        assert_eq!(
            MemoryBank::redistribute_blocks_at(0, &vec![3, 1, 2, 3]),
            vec![0, 2, 3, 4]
        );
        assert_eq!(
            MemoryBank::redistribute_blocks_at(3, &vec![0, 2, 3, 4]),
            vec![1, 3, 4, 1]
        );
        assert_eq!(
            MemoryBank::redistribute_blocks_at(2, &vec![1, 3, 4, 1]),
            vec![2, 4, 1, 2]
        );
    }

    #[test]
    fn test_redistribute_blocks() {
        assert_eq!(
            MemoryBank::redistribute_blocks(&vec![0, 2, 7, 0]),
            vec![2, 4, 1, 2]
        );
        assert_eq!(
            MemoryBank::redistribute_blocks(&vec![2, 4, 1, 2]),
            vec![3, 1, 2, 3]
        );
        assert_eq!(
            MemoryBank::redistribute_blocks(&vec![3, 1, 2, 3]),
            vec![0, 2, 3, 4]
        );
        assert_eq!(
            MemoryBank::redistribute_blocks(&vec![0, 2, 3, 4]),
            vec![1, 3, 4, 1]
        );
        assert_eq!(
            MemoryBank::redistribute_blocks(&vec![1, 3, 4, 1]),
            vec![2, 4, 1, 2]
        );
    }

    #[test]
    fn test_length_of_first_redistribution_loop() {
        let mem_bank =
            MemoryBank::new(&to_string_vector("test_inputs/day_6_part_1.txt").unwrap()[0]);

        assert_eq!(mem_bank.length_of_first_redistribution_loop(), 4);
    }
}
