use std::collections::{HashMap, VecDeque};

use super::dance_moves::DanceMove;

#[derive(Debug)]
pub struct Dance {
    pub moves: Vec<DanceMove>,
    pub dancers: VecDeque<char>,
}

impl Dance {
    pub fn new(moves: &[&str], dancers: &[char]) -> Dance {
        Dance {
            moves: moves.iter().map(|s| DanceMove::new(s)).collect(),
            dancers: dancers.iter().map(|c| *c).collect(),
        }
    }

    pub fn dance(&mut self) {
        for m in &self.moves {
            match m {
                &DanceMove::Spin(num) => for _ in 0..num {
                    let temp_element = self.dancers.pop_back().unwrap();

                    self.dancers.push_front(temp_element);
                },
                &DanceMove::Exchange(first_index, second_index) => {
                    self.dancers.swap(first_index, second_index);
                }
                &DanceMove::Parter(first_partner, second_partner) => {
                    let first_index = self.dancers
                        .iter()
                        .position(|&c| c == first_partner)
                        .unwrap();
                    let second_index = self.dancers
                        .iter()
                        .position(|&c| c == second_partner)
                        .unwrap();

                    self.dancers.swap(first_index, second_index);
                }
            }
        }
    }

    pub fn reoccurance_start_length(&mut self) -> (usize, usize) {
        let mut n: usize = 0;
        let mut seen_patterns = HashMap::new();
        let temp_v = self.dancers.clone();

        loop {
            match seen_patterns.get(&self.dancers) {
                None => {
                    seen_patterns.insert(self.dancers.clone(), n);
                    n += 1;
                    self.dance();
                }
                Some(&x) => {
                    self.dancers = temp_v;

                    return (x + 1, n - x);
                }
            }
        }
    }

    pub fn dance_n_times(&mut self, mut n: usize) {
        let (rec_start, rec_len) = self.reoccurance_start_length();

        if rec_start <= n {
            n = (n - rec_start) % rec_len + rec_start;
        }

        for _ in 0..n {
            self.dance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_dance() {
        let test_input = &to_string_vector("test_inputs/day_16_part_1.txt").unwrap()[0];
        let temp_moves_info: Vec<&str> = test_input.split(',').collect();
        let temp_dancers: Vec<char> = "abcde".chars().collect();

        let mut test_dance = Dance::new(&temp_moves_info, &temp_dancers);

        test_dance.dance();

        assert_eq!(test_dance.dancers, "baedc".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_dance_n_times() {
        let test_input = &to_string_vector("test_inputs/day_16_part_1.txt").unwrap()[0];
        let temp_moves_info: Vec<&str> = test_input.split(',').collect();
        let temp_dancers: Vec<char> = "abcde".chars().collect();

        let mut test_dance = Dance::new(&temp_moves_info, &temp_dancers);

        test_dance.dance_n_times(2);

        assert_eq!(test_dance.dancers, "ceadb".chars().collect::<Vec<char>>());
    }
}
