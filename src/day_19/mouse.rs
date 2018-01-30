use super::direction::Direction;
use super::grid::{Grid, Location};

#[derive(Debug)]
pub struct Mouse<'a> {
    pub grid: &'a Grid,
    pub location: Location,
    pub direction: Direction,
    pub letters_found: Vec<char>,
}

impl<'a> Mouse<'a> {
    pub fn new(grid: &Grid) -> Mouse {
        Mouse {
            grid: grid,
            location: grid.entrance_location(),
            direction: Direction::Down,
            letters_found: Vec::new(),
        }
    }

    pub fn move_to_next_corner(&mut self) {
        let offset = match self.direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };

        loop {
            let temp_location = (self.location.0 + offset.0, self.location.1 + offset.1);

            if temp_location.0 < 0 || temp_location.1 < 0 || s
                break
            }
        }
        unimplemented!();
    }

    pub fn get_next_direction(&self) -> Option<Direction> {
        match self.grid.get(self.location) {
            None => None,
            Some(c) if c != '+' => None,
            Some(_) => Some(self.get_new_direction()),
        }
    }

    pub fn get_new_direction(&self) -> Direction {
        if self.direction == Direction::Up || self.direction == Direction::Down {
            match self.get_non_empty_char_from_offset((0, -1)) {
                Some(_) => Direction::Left,
                None => Direction::Right,
            }
        } else {
            match self.get_non_empty_char_from_offset((-1, 0)) {
                Some(_) => Direction::Up,
                None => Direction::Down,
            }
        }
    }

    fn get_non_empty_char_from_offset(&self, offset: Location) -> Option<char> {
        let temp_location = (self.location.0 + offset.0, self.location.1 + offset.1);

        match self.grid.get(temp_location) {
            None => None,
            Some(c) => {
                if c == ' ' {
                    None
                } else {
                    Some(c)
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_move_to_next_corner() {
        let test_input = to_string_vector("test_inputs/day_19_part_1.txt").unwrap();
        let test_grid = Grid::new(&test_input);
        let mut test_mouse = Mouse::new(&test_grid);

        test_mouse.move_to_next_corner();

        assert_eq!(test_mouse.location, (5, 5));
        assert_eq!(test_mouse.direction, Direction::Right);
        assert_eq!(test_mouse.letters_found, vec!['A']);
    }

    #[test]
    fn get_next_direction() {
        let test_input = to_string_vector("test_inputs/day_19_part_1.txt").unwrap();
        let test_grid = Grid::new(&test_input);
        let mut test_mouse = Mouse::new(&test_grid);

        assert_eq!(test_mouse.get_next_direction(), None);

        test_mouse.location = (5, 5);

        assert_eq!(test_mouse.get_next_direction(), Some(Direction::Right));

        test_mouse.location = (6, 6);

        assert_eq!(test_mouse.get_next_direction(), None);
    }

    #[test]
    fn test_get_new_direction() {
        let test_input = to_string_vector("test_inputs/day_19_part_1.txt").unwrap();
        let test_grid = Grid::new(&test_input);
        let mut test_mouse = Mouse::new(&test_grid);

        test_mouse.location = (5, 5);

        assert_eq!(test_mouse.get_new_direction(), Direction::Right);

        test_mouse.location = (5, 8);
        test_mouse.direction = Direction::Right;

        assert_eq!(test_mouse.get_new_direction(), Direction::Up);

        test_mouse.location = (1, 11);

        assert_eq!(test_mouse.get_new_direction(), Direction::Down);

        test_mouse.location = (3, 14);
        test_mouse.direction = Direction::Up;

        assert_eq!(test_mouse.get_new_direction(), Direction::Left);
    }
}
