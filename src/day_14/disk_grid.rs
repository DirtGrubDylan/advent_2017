use day_10::macramist::Macramist;

#[derive(Debug)]
pub struct DiskGrid {
    pub key: String,
}

impl DiskGrid {
    pub fn new(key: &str) -> DiskGrid {
        DiskGrid {
            key: key.to_owned(),
        }
    }

    pub fn number_of_used_squares(&self) -> usize {
        let mut used_squares = 0;
        let mut hasher = Macramist::new(256);

        for row in 0..128 {
            let temp_key = self.key.clone() + "-" +  &row.to_string();

            let temp_knot_hash = hasher.hash(&temp_key);
            hasher.reset();

            used_squares += temp_knot_hash
                .chars()
                .fold(0, |acc, c| acc + c.to_digit(16).unwrap().count_ones());
        }

        used_squares as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_used_squares() {
        let test_grid = DiskGrid::new("flqrgnkx");

        assert_eq!(test_grid.number_of_used_squares(), 8108);
    }
}
