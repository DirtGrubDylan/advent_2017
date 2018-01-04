#[derive(Debug, Clone)]
pub struct Generator {
    inner: usize,
    divisor: usize,
    multiplier: usize,
    key: usize,
}

impl Generator {
    pub fn new(starting_value: usize, multiplier: usize, key: usize) -> Generator {
        Generator {
            inner: starting_value,
            multiplier: multiplier,
            divisor: 2147483647,
            key: key,
        }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.inner = (self.inner * self.multiplier) % self.divisor;

            if self.inner % self.key == 0 {
                break;
            }
        }

        Some(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_1() {
        let test_gen_a = Generator::new(65, 16807, 1);
        let expected_a = vec![1092455, 1181022009, 245556042, 1744312007, 1352636452];

        let test_gen_b = Generator::new(8921, 48271, 1);
        let expected_b = vec![430625591, 1233683848, 1431495498, 137874439, 285222916];

        assert_eq!(test_gen_a.take(5).collect::<Vec<usize>>(), expected_a);
        assert_eq!(test_gen_b.take(5).collect::<Vec<usize>>(), expected_b);
    }

    #[test]
    fn test_next_4_8() {
        let test_gen_a = Generator::new(65, 16807, 4);
        let expected_a = vec![1352636452, 1992081072, 530830436, 1980017072, 740335192];

        let test_gen_b = Generator::new(8921, 48271, 8);
        let expected_b = vec![1233683848, 862516352, 1159784568, 1616057672, 412269392];

        assert_eq!(test_gen_a.take(5).collect::<Vec<usize>>(), expected_a);
        assert_eq!(test_gen_b.take(5).collect::<Vec<usize>>(), expected_b);
    }
}
