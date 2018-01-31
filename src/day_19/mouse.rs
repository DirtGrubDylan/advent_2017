use super::direction::Direction;
use super::grid::{Grid, Location};

#[derive(Debug)]
pub struct Mouse<'a> {
    pub grid: &'a Grid,
    pub location: Location,
    pub direction: Direction,
    pub letters_found: Vec<char>,
    pub steps_taken: usize,
}

impl<'a> Mouse<'a> {
    pub fn new(grid: &Grid) -> Mouse {
        Mouse {
            grid: grid,
            location: grid.entrance_location(),
            direction: Direction::Down,
            letters_found: Vec::new(),
            steps_taken: 1,
        }
    }

    pub fn move_to_next_corner(&mut self) -> bool {
        let offset = match self.direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };

        self.location = (self.location.0 + offset.0, self.location.1 + offset.1);

        while let Some(c) = self.grid.get(self.location) {
            self.steps_taken += 1;

            match c {
                '+' | ' ' => break,
                '|' | '-' => {}
                _ => self.letters_found.push(c),
            };

            self.location = (self.location.0 + offset.0, self.location.1 + offset.1);
        }

        match self.get_next_direction() {
            None => {
                self.steps_taken -= 1;

                false
            }
            Some(direction) => {
                self.direction = direction;

                true
            }
        }
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

    pub fn find_all_letters(&mut self) -> String {
        while self.move_to_next_corner() {}

        self.letters_found.iter().collect()
    }

    fn get_non_empty_char_from_offset(&self, offset: Location) -> Option<char> {
        let temp_location = (self.location.0 + offset.0, self.location.1 + offset.1);
        let char_to_find = match self.direction {
            Direction::Up | Direction::Down => '-',
            _ => '|',
        };

        match self.grid.get(temp_location) {
            None => None,
            Some(c) => {
                if c == char_to_find || (c >= 'A' && c <= 'Z') {
                    Some(c)
                } else {
                    None
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
    fn test_find_all_letters() {
        let test_input = to_string_vector("test_inputs/day_19_part_1.txt").unwrap();
        let test_grid = Grid::new(&test_input);
        let mut test_mouse = Mouse::new(&test_grid);

        assert_eq!(test_mouse.find_all_letters(), String::from("ABCDEF"));
        assert_eq!(test_mouse.steps_taken, 38);
    }

    #[test]
    fn test_move_to_next_corner() {
        let test_input = to_string_vector("test_inputs/day_19_part_1.txt").unwrap();
        let test_grid = Grid::new(&test_input);
        let mut test_mouse = Mouse::new(&test_grid);

        assert!(test_mouse.move_to_next_corner());
        assert_eq!(test_mouse.location, (5, 5));
        assert_eq!(test_mouse.direction, Direction::Right);
        assert_eq!(test_mouse.letters_found, vec!['A']);
        assert_eq!(test_mouse.steps_taken, 6);

        assert!(test_mouse.move_to_next_corner());
        assert_eq!(test_mouse.location, (5, 8));
        assert_eq!(test_mouse.direction, Direction::Up);
        assert_eq!(test_mouse.letters_found, vec!['A', 'B']);
        assert_eq!(test_mouse.steps_taken, 9);
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
