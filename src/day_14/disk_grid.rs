use std::collections::{HashSet, VecDeque};

use day_10::macramist::Macramist;

use super::location::Location;

#[derive(Debug)]
pub struct DiskGrid {
    pub key: String,
    pub grid: Vec<Vec<usize>>,
}

impl DiskGrid {
    pub fn new(key: &str) -> DiskGrid {
        DiskGrid {
            key: key.to_owned(),
            grid: Self::key_to_grid(key),
        }
    }

    pub fn number_of_used_squares(&self) -> usize {
        self.grid
            .iter()
            .fold(0, |acc, v| acc + v.iter().fold(0, |acc, x| acc + *x))
    }

    pub fn element_at(&self, location: Location) -> usize {
        self.grid[location.row as usize][location.col as usize]
    }

    pub fn region_containing(&self, location: Location) -> HashSet<Location> {
        let mut seen_locs = HashSet::new();
        let mut queue = VecDeque::new();

        seen_locs.insert(location);
        queue.push_back(location);

        while let Some(current_loc) = queue.pop_front() {
            for nearby_loc in current_loc.surrounding_locations() {
                if !seen_locs.contains(&nearby_loc) && self.element_at(nearby_loc) == 1 {
                    seen_locs.insert(nearby_loc);
                    queue.push_back(nearby_loc);
                }
            }
        }

        seen_locs
    }

    pub fn regions(&self) -> Vec<HashSet<Location>> {
        let mut regions = Vec::new();
        let mut seen_locs: HashSet<Location> = HashSet::new();

        for row in 0..128 {
            for col in 0..128 {
                let temp_loc = Location::new((row, col));

                if !seen_locs.contains(&temp_loc) && self.element_at(temp_loc) == 1 {
                    let region = self.region_containing(temp_loc);

                    regions.push(region.clone());
                    seen_locs.extend(region);
                }
            }
        }

        regions
    }

    pub fn key_to_grid(key: &str) -> Vec<Vec<usize>> {
        let key = key.to_owned();
        let mut grid = Vec::new();
        let mut hasher = Macramist::new(256);

        for row in 0..128 {
            let temp_key = key.clone() + "-" + &row.to_string();

            let temp_knot_hash = hasher.hash(&temp_key);
            hasher.reset();

            let temp_string = temp_knot_hash.chars().fold(String::from(""), |acc, c| {
                acc + &format!("{:04b}", c.to_digit(16).unwrap())
            });

            grid.push(
                temp_string
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect(),
            );
        }

        grid
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

    #[test]
    fn test_region_containing() {
        let test_grid = DiskGrid::new("flqrgnkx");
        let expected_region_1: HashSet<Location> = [(0, 0), (0, 1), (1, 1)]
            .into_iter()
            .map(|l| Location::new(*l))
            .collect();
        let expected_region_2: HashSet<Location> = [(0, 3), (1, 3)]
            .into_iter()
            .map(|l| Location::new(*l))
            .collect();
        let expected_region_6: HashSet<Location> =
            [(2, 6)].into_iter().map(|l| Location::new(*l)).collect();

        assert_eq!(test_grid.region_containing(Location::new((0, 0))), expected_region_1);
        assert_eq!(test_grid.region_containing(Location::new((0, 3))), expected_region_2);
        assert_eq!(test_grid.region_containing(Location::new((2, 6))), expected_region_6);
    }

    #[test]
    fn test_regions() {
        let test_grid = DiskGrid::new("flqrgnkx");

        assert_eq!(test_grid.regions().len(), 1242);
    }
}
