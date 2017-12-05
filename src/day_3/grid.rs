pub type Grid = Vec<Vec<usize>>;


pub fn new_grid(size: usize) -> Grid {
    let mut grid = Grid::new();

    for _ in 0..size {
        let mut temp_vec = Vec::new();

        for _ in 0..size {
            temp_vec.push(0);
        }

        grid.push(temp_vec);
    }

    let mid = size / 2;

    grid[mid][mid] = 1;

    grid
}


pub fn next_number_larger_in_grid(number: usize, grid: &Grid) -> usize {
    let mut layer = 1;
    let mut location = 1;
    let mut coords = [0, 0];
    let mid = grid.len() / 2;
    let mut next_number = grid[mid][mid];

    while next_number <= number {
        if layer == (location * location) {
            coords[1] += 1;
        }
    }

    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid() {
        let expected_grid: Vec<Vec<usize>> = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        assert_eq!(new_grid(5), expected_grid);
    }

    #[test]
    fn test_next_number_larger_in_grid() {
        let test_grid = new_grid(5);

        assert_eq!(next_number_larger_in_grid(2, &test_grid), 4);
        assert_eq!(next_number_larger_in_grid(3, &test_grid), 4);
        assert_eq!(next_number_larger_in_grid(6, &test_grid), 10);
        assert_eq!(next_number_larger_in_grid(750, &test_grid), 806);
    }
}
