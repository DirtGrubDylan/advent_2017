pub struct Macramist {
    pub inner: Vec<usize>,
}

impl Macramist {
    pub fn new(rope_length: usize) -> Macramist {
        Macramist {
            inner: (0..rope_length).collect(),
        }
    }

    pub fn tie_with_lengths(
        &mut self,
        step: &mut usize,
        current_position: &mut usize,
        lengths: &[usize],
    ) {
        for &length in lengths {
            let mut temp_vec = Vec::new();

            for mut index in 0..length {
                index = (*current_position + index) % self.inner.len();
                temp_vec.push(self.inner[index]);
            }

            for mut index in 0..length {
                index = (*current_position + index) % self.inner.len();
                self.inner[index] = temp_vec.pop().unwrap();
            }

            *current_position += length + *step;
            *step += 1;
        }
    }

    pub fn hash_from_first_two(&self) -> usize {
        self.inner[0] * self.inner[1]
    }

    pub fn xor_hash(&self) -> String {
        let mut hash = String::new();

        for chunk in self.inner.chunks(16) {
            let mut temp_hash = format!("0{:x}", chunk.iter().fold(0, |acc, x| acc ^ x));

            if temp_hash.len() == 3 {
                hash += &temp_hash[1..3];
            } else {
                hash += temp_hash.as_str();
            }
        }

        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tie_with_lengths() {
        let mut step = 0;
        let mut current_position = 0;
        let mut test_macramist = Macramist::new(5);

        test_macramist.tie_with_lengths(&mut step, &mut current_position, &[3, 4, 1, 5]);

        assert_eq!(test_macramist.inner, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn test_hash_from_first_two() {
        let mut step = 0;
        let mut current_position = 0;
        let mut test_macramist = Macramist::new(5);

        assert_eq!(test_macramist.hash_from_first_two(), 0);

        test_macramist.tie_with_lengths(&mut step, &mut current_position, &[3, 4, 1, 5]);

        assert_eq!(test_macramist.hash_from_first_two(), 12);
    }

    #[test]
    fn test_xor_hash() {
        let mut step = 0;
        let mut current_position = 0;
        let mut test_macramist = Macramist::new(256);

        for _ in 0..64 {
            test_macramist.tie_with_lengths(
                &mut step,
                &mut current_position,
                &[49, 44, 50, 44, 51, 17, 31, 73, 47, 23],
            );
        }

        assert_eq!(
            test_macramist.xor_hash(),
            String::from("3efbe78a8d82f29979031a4aa0b16a9d")
        );
    }
}
