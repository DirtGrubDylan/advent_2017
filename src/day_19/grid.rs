pub type Location = (isize, isize);

#[derive(Debug)]
pub struct Grid {
    pub inner: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(map: &[String]) -> Grid {
        let temp_v = map.iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();

        Grid { inner: temp_v }
    }

    pub fn get(&self, location: Location) -> Option<char> {
        if location.0 < 0 || location.1 < 0 {
            None
        } else {
            match self.inner.get(location.0 as usize) {
                None => None,
                Some(row) => row.get(location.1 as usize).cloned(),
            }
        }
    }

    pub fn entrance_location(&self) -> Location {
        let mut temp_loc = (0, 0);

        while let Some(c) = self.get(temp_loc) {
            if c == '|' {
                break;
            } else {
                temp_loc.1 += 1;
            }
        }

        temp_loc
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_get() {
        let test_input = to_string_vector("test_inputs/day_19_part_1.txt").unwrap();

        let test_grid = Grid::new(&test_input);

        assert_eq!(test_grid.get((-1, -3)), None);
        assert_eq!(test_grid.get((-1, 3)), None);
        assert_eq!(test_grid.get((1, -3)), None);
        assert_eq!(test_grid.get((0, 6)), None);
        assert_eq!(test_grid.get((6, 0)), None);
        assert_eq!(test_grid.get((0, 0)), Some(' '));
        assert_eq!(test_grid.get((0, 5)), Some('|'));
        assert_eq!(test_grid.get((3, 10)), Some('E'));
    }

    #[test]
    fn test_entrance_location() {
        let test_input = to_string_vector("test_inputs/day_19_part_1.txt").unwrap();

        let test_grid = Grid::new(&test_input);

        assert_eq!(test_grid.entrance_location(), (0, 5));
    }
}
