#[derive(Debug)]
pub struct Layer {
    pub id: usize,
    pub depth: usize,
}

impl Layer {
    pub fn new(id: usize, depth: usize) -> Layer {
        Layer {
            id: id,
            depth: depth,
        }
    }

    pub fn security_scanner_location_at(&self, picosecond: usize) -> Option<usize> {
        let offset = self.depth.saturating_sub(2);
        let modulus = self.depth + offset;

        if self.depth != 0 {
            let temp_loc = (((picosecond + offset) % modulus) as isize) - (offset as isize);

            Some(temp_loc.abs() as usize)
        } else {
            None
        }
    }

    pub fn capture_severity_at(&self, picosecond: usize) -> usize {
        if self.caught_by_scanner_at(picosecond) {
            self.id * self.depth
        } else {
            0
        }
    }

    pub fn caught_by_scanner_at(&self, picosecond: usize) -> bool {
        match self.security_scanner_location_at(picosecond) {
            Some(location) if location == 0 => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_scanner_location_at() {
        let test_layers: Vec<Layer> = (0..5).map(|i| Layer::new(i, i)).collect();

        assert_eq!(test_layers[0].security_scanner_location_at(17), None);

        assert_eq!(test_layers[1].security_scanner_location_at(1), Some(0));
        assert_eq!(test_layers[1].security_scanner_location_at(2), Some(0));

        assert_eq!(test_layers[2].security_scanner_location_at(3), Some(1));
        assert_eq!(test_layers[2].security_scanner_location_at(4), Some(0));

        assert_eq!(test_layers[3].security_scanner_location_at(3), Some(1));
        assert_eq!(test_layers[3].security_scanner_location_at(6), Some(2));
        assert_eq!(test_layers[3].security_scanner_location_at(8), Some(0));
        assert_eq!(test_layers[3].security_scanner_location_at(9), Some(1));

        assert_eq!(test_layers[4].security_scanner_location_at(3), Some(3));
        assert_eq!(test_layers[4].security_scanner_location_at(6), Some(0));
        assert_eq!(test_layers[4].security_scanner_location_at(8), Some(2));
        assert_eq!(test_layers[4].security_scanner_location_at(9), Some(3));
        assert_eq!(test_layers[4].security_scanner_location_at(11), Some(1));
    }

    #[test]
    fn test_capture_severity_at() {
        let test_layers: Vec<Layer> = (0..5).map(|i| Layer::new(i, i)).collect();

        assert_eq!(test_layers[0].capture_severity_at(17), 0);

        assert_eq!(test_layers[1].capture_severity_at(1), 1);
        assert_eq!(test_layers[1].capture_severity_at(2), 1);

        assert_eq!(test_layers[2].capture_severity_at(3), 0);
        assert_eq!(test_layers[2].capture_severity_at(4), 4);

        assert_eq!(test_layers[3].capture_severity_at(3), 0);
        assert_eq!(test_layers[3].capture_severity_at(6), 0);
        assert_eq!(test_layers[3].capture_severity_at(8), 9);
        assert_eq!(test_layers[3].capture_severity_at(9), 0);

        assert_eq!(test_layers[4].capture_severity_at(3), 0);
        assert_eq!(test_layers[4].capture_severity_at(6), 16);
        assert_eq!(test_layers[4].capture_severity_at(8), 0);
        assert_eq!(test_layers[4].capture_severity_at(9), 0);
        assert_eq!(test_layers[4].capture_severity_at(11), 0);
    }

    #[test]
    fn test_caught_by_scanner_at() {
        let test_layers: Vec<Layer> = (0..5).map(|i| Layer::new(i, i)).collect();

        assert_eq!(test_layers[0].caught_by_scanner_at(17), false);

        assert_eq!(test_layers[1].caught_by_scanner_at(1), true);
        assert_eq!(test_layers[1].caught_by_scanner_at(2), true);

        assert_eq!(test_layers[2].caught_by_scanner_at(3), false);
        assert_eq!(test_layers[2].caught_by_scanner_at(4), true);

        assert_eq!(test_layers[3].caught_by_scanner_at(3), false);
        assert_eq!(test_layers[3].caught_by_scanner_at(6), false);
        assert_eq!(test_layers[3].caught_by_scanner_at(8), true);
        assert_eq!(test_layers[3].caught_by_scanner_at(9), false);

        assert_eq!(test_layers[4].caught_by_scanner_at(3), false);
        assert_eq!(test_layers[4].caught_by_scanner_at(6), true);
        assert_eq!(test_layers[4].caught_by_scanner_at(8), false);
        assert_eq!(test_layers[4].caught_by_scanner_at(9), false);
        assert_eq!(test_layers[4].caught_by_scanner_at(11), false);
    }
}
