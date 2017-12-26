use std::collections::HashMap;

use super::layer::Layer;

#[derive(Debug)]
pub struct Firewall {
    pub layers: HashMap<usize, Layer>,
}

impl Firewall {
    pub fn new(layer_info: &[String]) -> Firewall {
        let temp_map: HashMap<usize, Layer> = layer_info
            .iter()
            .map(|info| {
                let temp: Vec<usize> = info.split(": ").map(|s| s.parse().unwrap()).collect();

                (temp[0], Layer::new(temp[0], temp[1]))
            })
            .collect();

        Firewall { layers: temp_map }
    }

    pub fn capture_severity_leaving_at(&self, start_time: usize) -> usize {
        self.layers.iter().fold(0, |acc, (&id, layer)| {
            acc + layer.capture_severity_at(id + start_time)
        })
    }

    pub fn is_vulnerable_at(&self, start_time: usize) -> bool {
        for layer in self.layers.values() {
            if layer.caught_by_scanner_at(layer.id + start_time) {
                return false;
            }
        }

        true
    }

    pub fn vulerable_time(&self) -> usize {
        let mut start_time = 0;

        while !self.is_vulnerable_at(start_time) {
            start_time += 1;
        }

        start_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_capture_severity_leaving_at() {
        let test_firewall =
            Firewall::new(&to_string_vector("test_inputs/day_13_part_1.txt").unwrap());

        assert_eq!(test_firewall.capture_severity_leaving_at(0), 24);
    }

    #[test]
    fn test_is_vulnerable_at() {
        let test_firewall =
            Firewall::new(&to_string_vector("test_inputs/day_13_part_1.txt").unwrap());

        assert_eq!(test_firewall.is_vulnerable_at(0), false);
        assert_eq!(test_firewall.is_vulnerable_at(4), false);
        assert_eq!(test_firewall.is_vulnerable_at(10), true);
    }

    #[test]
    fn test_vulerable_time() {
        let test_firewall =
            Firewall::new(&to_string_vector("test_inputs/day_13_part_1.txt").unwrap());

        assert_eq!(test_firewall.vulerable_time(), 10);
    }
}
