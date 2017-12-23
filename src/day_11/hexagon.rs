#[derive(Debug, PartialEq, Clone)]
pub struct Hexagon {
    x: isize,
    y: isize,
    z: isize,
}

impl Hexagon {
    pub fn new(x: isize, y: isize, z: isize) -> Hexagon {
        Hexagon { x: x, y: y, z: z }
    }

    pub fn offset_by(&self, x: isize, y: isize, z: isize) -> Hexagon {
        Hexagon {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }

    pub fn offset_by_direction(&self, direction: &str) -> Result<Hexagon, String> {
        match direction {
            "n" => Ok(self.offset_by(0, 1, -1)),
            "ne" => Ok(self.offset_by(1, 0, -1)),
            "nw" => Ok(self.offset_by(-1, 1, 0)),
            "s" => Ok(self.offset_by(0, -1, 1)),
            "se" => Ok(self.offset_by(1, -1, 0)),
            "sw" => Ok(self.offset_by(-1, 0, 1)),
            _ => Err(format!("Direction \"{}\" is not valid!", direction)),
        }
    }

    pub fn offset_by_direction_list(&self, direction_list: &[&str]) -> Result<Hexagon, String> {
        let mut new_hex = self.clone();

        for direction in direction_list {
            new_hex = match new_hex.offset_by_direction(direction) {
                Err(err) => return Err(err),
                Ok(hex) => hex,
            }
        }

        Ok(new_hex)
    }

    pub fn distance_to(&self, other: &Hexagon) -> isize {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();

        dx.max(dy.max(dz))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_offset_by() {
        let origin = Hexagon::new(0, 0, 0);

        assert_eq!(origin.offset_by(0, 1, -1), Hexagon { x: 0, y: 1, z: -1 });
    }

    #[test]
    fn test_offset_by_direction() {
        let origin = Hexagon::new(0, 0, 0);

        assert_eq!(
            origin.offset_by_direction("n").unwrap(),
            Hexagon { x: 0, y: 1, z: -1 }
        );
        assert_eq!(
            origin.offset_by_direction("ne").unwrap(),
            Hexagon { x: 1, y: 0, z: -1 }
        );
        assert_eq!(
            origin.offset_by_direction("nw").unwrap(),
            Hexagon { x: -1, y: 1, z: 0 }
        );
        assert_eq!(
            origin.offset_by_direction("s").unwrap(),
            Hexagon { x: 0, y: -1, z: 1 }
        );
        assert_eq!(
            origin.offset_by_direction("se").unwrap(),
            Hexagon { x: 1, y: -1, z: 0 }
        );
        assert_eq!(
            origin.offset_by_direction("sw").unwrap(),
            Hexagon { x: -1, y: 0, z: 1 }
        );
        assert!(origin.offset_by_direction("bad_direction").is_err());
    }

    #[test]
    fn test_distance_to() {
        let origin = Hexagon::new(0, 0, 0);
        let other_origin = Hexagon::new(3, 2, -5);

        assert_eq!(origin.distance_to(&Hexagon::new(0, 0, 0)), 0);
        assert_eq!(origin.distance_to(&Hexagon::new(3, 2, -5)), 5);
        assert_eq!(origin.distance_to(&Hexagon::new(-1, -3, 4)), 4);

        assert_eq!(other_origin.distance_to(&Hexagon::new(0, 0, 0)), 5);
        assert_eq!(other_origin.distance_to(&Hexagon::new(3, 2, -5)), 0);
        assert_eq!(other_origin.distance_to(&Hexagon::new(-1, -3, 4)), 9);
    }

    #[test]
    fn test_offset_by_direction_list() {
        let origin = Hexagon::new(0, 0, 0);
        let test_hex: Vec<Hexagon> = to_string_vector("test_inputs/day_11_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|s| {
                origin
                    .offset_by_direction_list(&s.split(',').collect::<Vec<&str>>())
                    .unwrap()
            })
            .collect();
        let expected = vec![
            Hexagon::new(3, 0, -3),
            Hexagon::new(0, 0, 0),
            Hexagon::new(2, -2, 0),
            Hexagon::new(-1, -2, 3),
        ];

        assert_eq!(test_hex, expected);
    }
}
