use std::collections::HashMap;

use super::element::Element;
use super::direction::Direction;

type Location = (isize, isize);

const OFFSETS: [Location; 8] = [
    (0, 1),   // right
    (-1, 1),  // top-right
    (-1, 0),  // top
    (-1, -1), // top-left
    (0, -1),  // left
    (1, -1),  // bottom-left
    (1, 0),   // bottom
    (1, 1),   // bottom-right
];

#[derive(Debug)]
pub struct Grid {
    pub data: HashMap<Location, Element>,
    current_layer: usize,
    current_location: Location,
}

impl Grid {
    pub fn new() -> Grid {
        let mut temp_map = HashMap::new();

        temp_map.insert(
            (0, 0),
            Element {
                position: 1,
                value: 1,
            },
        );

        Grid {
            data: temp_map,
            current_layer: 0,
            current_location: (0, 0),
        }
    }

    fn next_location_for(layer: isize, location: Location) -> Location {
        match Grid::next_direction_for(layer, location) {
            Direction::Up => (location.0 - 1, location.1),
            Direction::Down => (location.0 + 1, location.1),
            Direction::Left => (location.0, location.1 - 1),
            Direction::Right => (location.0, location.1 + 1),
        }
    }

    fn next_direction_for(layer: isize, location: Location) -> Direction {
        match location {
            (x, _) if x == layer => Direction::Right,
            (_, y) if y == -layer => Direction::Down,
            (x, _) if x == -layer => Direction::Left,
            _ => Direction::Up,
        }
    }

    fn sum_of_values_around(location: Location, data: &HashMap<Location, Element>) -> usize {
        OFFSETS.into_iter().fold(0, |acc, &(x_off, y_off)| {
            let temp_location = (location.0 + x_off, location.1 + y_off);

            match data.get(&temp_location) {
                Some(element) => acc + element.value,
                None => acc,
            }
        })
    }

    fn layer_of(location: Location) -> usize {
        [location.0, location.1]
            .into_iter()
            .map(|&x| x.abs() as usize)
            .max()
            .unwrap()
    }

    pub fn next(&mut self) -> Element {
        let next_location =
            Grid::next_location_for(self.current_layer as isize, self.current_location);

        let next_element = Element {
            position: self.data.get(&self.current_location).unwrap().position + 1,
            value: Grid::sum_of_values_around(next_location, &self.data),
        };


        self.data.insert(next_location, next_element.clone());
        self.current_layer = Grid::layer_of(next_location);
        self.current_location = next_location;

        next_element
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_direction_for() {
        // Random cases
        assert_eq!(Grid::next_direction_for(2, (0, 2)), Direction::Up);
        assert_eq!(Grid::next_direction_for(3, (1, -3)), Direction::Down);
        assert_eq!(Grid::next_direction_for(4, (-4, 4)), Direction::Left);
        assert_eq!(Grid::next_direction_for(0, (0, 0)), Direction::Right);

        // Corner cases
        assert_eq!(Grid::next_direction_for(5, (4, 5)), Direction::Up);
        assert_eq!(Grid::next_direction_for(5, (-5, 5)), Direction::Left);
        assert_eq!(Grid::next_direction_for(5, (-5, -5)), Direction::Down);
        assert_eq!(Grid::next_direction_for(5, (5, -5)), Direction::Right);
        assert_eq!(Grid::next_direction_for(5, (5, 5)), Direction::Right);
    }

    #[test]
    fn test_next_location_for() {
        // Random cases
        assert_eq!(Grid::next_location_for(2, (0, 2)), (-1, 2));
        assert_eq!(Grid::next_location_for(3, (1, -3)), (2, -3));
        assert_eq!(Grid::next_location_for(4, (-4, 4)), (-4, 3));
        assert_eq!(Grid::next_location_for(0, (0, 0)), (0, 1));

        // Corner cases
        assert_eq!(Grid::next_location_for(5, (4, 5)), (3, 5));
        assert_eq!(Grid::next_location_for(5, (-5, 5)), (-5, 4));
        assert_eq!(Grid::next_location_for(5, (-5, -5)), (-4, -5));
        assert_eq!(Grid::next_location_for(5, (5, -5)), (5, -4));
        assert_eq!(Grid::next_location_for(5, (5, 5)), (5, 6));
    }

    #[test]
    fn test_sum_of_values_around() {
        let mut test_map = HashMap::new();

        test_map.insert(
            (0, 0),
            Element {
                position: 1,
                value: 1,
            },
        );
        test_map.insert(
            (0, 1),
            Element {
                position: 2,
                value: 1,
            },
        );
        test_map.insert(
            (-1, 1),
            Element {
                position: 3,
                value: 2,
            },
        );

        assert_eq!(Grid::sum_of_values_around((-1, 0), &test_map), 4);

        test_map.insert(
            (-1, 0),
            Element {
                position: 4,
                value: 4,
            },
        );

        assert_eq!(Grid::sum_of_values_around((-1, -1), &test_map), 5);
    }

    #[test]
    fn test_layer_of() {
        assert_eq!(Grid::layer_of((0, 0)), 0);
        assert_eq!(Grid::layer_of((-5, 4)), 5);
        assert_eq!(Grid::layer_of((-4, -5)), 5);
        assert_eq!(Grid::layer_of((5, 4)), 5);
        assert_eq!(Grid::layer_of((4, 5)), 5);
    }

    #[test]
    fn test_next() {
        let mut test_grid = Grid::new();

        assert_eq!(
            test_grid.next(),
            Element {
                position: 2,
                value: 1,
            }
        );
        assert_eq!(
            test_grid.next(),
            Element {
                position: 3,
                value: 2,
            }
        );
        assert_eq!(
            test_grid.next(),
            Element {
                position: 4,
                value: 4,
            }
        );
        assert_eq!(
            test_grid.next(),
            Element {
                position: 5,
                value: 5,
            }
        );
        assert_eq!(
            test_grid.next(),
            Element {
                position: 6,
                value: 10,
            }
        );
    }
}
