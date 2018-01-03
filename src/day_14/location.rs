#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Location {
    pub row: isize,
    pub col: isize,
}

impl Location {
    pub fn new(location: (isize, isize)) -> Location {
        Location {
            row: location.0,
            col: location.1,
        }
    }

    pub fn surrounding_locations(&self) -> Vec<Location> {
        let mut temp_vec = Vec::new();

        for &offset in [(-1, 0), (0, 1), (1, 0), (0, -1)].into_iter() {
            let (new_row, new_col) = (self.row + offset.0, self.col + offset.1);

            if 0 <= new_row && 0 <= new_col && new_row <= 127 && new_col <= 127 {
                temp_vec.push(Location::new((new_row, new_col)));
            }
        }

        temp_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surrounding_locations() {
        let top_left = Location::new((0, 0));
        let expected_0: Vec<Location> = [(0, 1), (1, 0)]
            .into_iter()
            .map(|l| Location::new(*l))
            .collect();

        let top_right = Location::new((0, 127));
        let expected_1: Vec<Location> = [(1, 127), (0, 126)]
            .into_iter()
            .map(|l| Location::new(*l))
            .collect();

        let bottom_left = Location::new((127, 0));
        let expected_2: Vec<Location> = [(126, 0), (127, 1)]
            .into_iter()
            .map(|l| Location::new(*l))
            .collect();

        let bottom_right = Location::new((127, 127));
        let expected_3: Vec<Location> = [(126, 127), (127, 126)]
            .into_iter()
            .map(|l| Location::new(*l))
            .collect();

        let one_one = Location::new((1, 1));
        let expected_4: Vec<Location> = [(0, 1), (1, 2), (2, 1), (1, 0)]
            .into_iter()
            .map(|l| Location::new(*l))
            .collect();

        assert_eq!(top_left.surrounding_locations(), expected_0);
        assert_eq!(top_right.surrounding_locations(), expected_1);
        assert_eq!(bottom_left.surrounding_locations(), expected_2);
        assert_eq!(bottom_right.surrounding_locations(), expected_3);
        assert_eq!(one_one.surrounding_locations(), expected_4);
    }
}
