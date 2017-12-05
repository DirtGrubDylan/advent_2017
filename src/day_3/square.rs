pub struct Square {
    pub id: usize,
}


impl Square {
    pub fn new(id: usize) -> Square {
        Square { id: id }
    }

    pub fn layer(&self) -> usize {
        ((self.id as f64).sqrt().ceil() as usize) / 2
    }

    pub fn distance_from_mid_layer(&self) -> usize {
        let temp_root = (self.layer() * 2 + 1) as isize;

        let starting_mid_point = (temp_root - 2).pow(2) + (temp_root / 2);

        let dist = [0, 1, 2, 3]
            .into_iter()
            .map(|&x| {
                ((self.id as isize) - (starting_mid_point + x * (temp_root - 1))).abs()
            })
            .min()
            .unwrap();

        dist as usize
    }

    pub fn distance_from_center_of_memory(&self) -> usize {
        self.layer() + self.distance_from_mid_layer()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer() {
        let test_layers = [1, 12, 23, 1024]
            .into_iter()
            .map(|id| Square::new(*id).layer())
            .collect::<Vec<usize>>();
        let expected_layers = vec![0, 2, 2, 16];

        assert_eq!(test_layers, expected_layers);
    }

    #[test]
    fn test_distance_from_mid_layer() {
        let test_mid_layer_distances = [1, 12, 23, 1024]
            .into_iter()
            .map(|id| Square::new(*id).distance_from_mid_layer())
            .collect::<Vec<usize>>();
        let expected_mid_layer_distances = vec![0, 1, 0, 15];

        assert_eq!(test_mid_layer_distances, expected_mid_layer_distances);
    }

    #[test]
    fn test_distance_from_center_of_memory() {
        let test_memory_distances = [1, 12, 23, 1024]
            .into_iter()
            .map(|id| Square::new(*id).distance_from_center_of_memory())
            .collect::<Vec<usize>>();
        let expected_memory_distances = vec![0, 3, 2, 31];

        assert_eq!(test_memory_distances, expected_memory_distances);
    }
}
