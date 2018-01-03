use super::generator::Generator;

#[derive(Debug)]
pub struct Judge {
    generators: Vec<Generator>,
    mask: usize,
}

impl Judge {
    pub fn new(generators: &[Generator], mask: usize) -> Judge {
        Judge {
            generators: generators.iter().map(|g| g.clone()).collect(),
            mask: mask,
        }
    }

    pub fn number_of_matching_values(&mut self, number_of_iterations: usize) -> usize {
        let mask = self.mask;
        let mut number_of_matching_values = 0;

        for iteration in 0..number_of_iterations {
            let mut temp_values: Vec<usize> = self.generators
                .iter_mut()
                .map(|g| g.next().unwrap() & mask)
                .collect();

            temp_values.dedup_by(|a, b| a == b);

            if temp_values.len() == 1 {
                number_of_matching_values += 1;
            }
        }

        number_of_matching_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_matching_values() {
        let test_gens = vec![Generator::new(65, 16807), Generator::new(8921, 48271)];
        let mut test_judge = Judge::new(&test_gens, 65535);

        assert_eq!(test_judge.number_of_matching_values(40_000_000), 588);
    }
}
