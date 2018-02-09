type Coords = (i32, i32, i32);

fn manhattan_distance(coords: &Coords) -> u32 {
    (coords.0.abs() + coords.1.abs() + coords.2.abs()) as u32
}

fn has_same_parity(first: &Coords, second: &Coords) -> bool {
    vec![
        (first.0, second.0),
        (first.1, second.1),
        (first.2, second.2),
    ].into_iter()
        .all(|(x, y)| x.signum() == y.signum() || x == 0 || y == 0)
}

#[derive(Debug)]
pub struct Particle {
    position: Coords,
    velocity: Coords,
    acceleration: Coords,
}

impl Particle {
    pub fn new(position: Coords, velocity: Coords, acceleration: Coords) -> Particle {
        Particle {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }

    pub fn new_from_str(info: &str) -> Particle {
        unimplemented!();
    }

    pub fn distance_to_center(&self) -> u32 {
        manhattan_distance(&self.position)
    }

    pub fn acceleration_strength(&self) -> u32 {
        manhattan_distance(&self.acceleration)
    }

    pub fn cycles_until_stable(&self) -> u32 {
        let mut cycles = 0;
        let mut temp_velocity = self.velocity;
        let mut temp_position = self.position;
        let mut finished = has_same_parity(&temp_position, &temp_velocity)
            && has_same_parity(&temp_position, &self.acceleration)
            && has_same_parity(&temp_velocity, &self.acceleration);

        while !finished {
            temp_velocity = (
                temp_velocity.0 + self.acceleration.0,
                temp_velocity.1 + self.acceleration.1,
                temp_velocity.2 + self.acceleration.2,
            );

            temp_position = (
                temp_position.0 + temp_velocity.0,
                temp_position.1 + temp_velocity.1,
                temp_position.2 + temp_velocity.2,
            );

            cycles += 1;
            finished = has_same_parity(&temp_position, &temp_velocity)
                && has_same_parity(&temp_position, &self.acceleration)
                && has_same_parity(&temp_velocity, &self.acceleration);
        }

        cycles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(&(0, 0, 0)), 0);
        assert_eq!(manhattan_distance(&(-1, 0, 1)), 2);
        assert_eq!(manhattan_distance(&(1, 1, 1)), 3);
        assert_eq!(manhattan_distance(&(-2, -1, -1)), 4);
    }

    #[test]
    fn test_has_same_parity() {
        assert!(has_same_parity(&(0, 0, 0), &(0, 0, 0)));
        assert!(has_same_parity(&(0, 0, 0), &(1, 1, -1)));
        assert!(has_same_parity(&(1, 1, -2), &(1, 1, -1)));
        assert!(has_same_parity(&(1, 3, 2), &(1, 1, 1)));
        assert!(has_same_parity(&(-1, -3, -2), &(-1, -1, -1)));

        assert!(!has_same_parity(&(1, 1, 2), &(1, 1, -1)));
        assert!(!has_same_parity(&(1, -3, 2), &(1, 1, -1)));
        assert!(!has_same_parity(&(4, -3, 2), &(1, 1, -1)));
        assert!(!has_same_parity(&(-4, -3, 2), &(1, 1, -1)));
    }

    #[test]
    fn test_cycles_until_stable() {
        let test_particle_0 = Particle::new((3, 0, 0), (2, 0, 0), (-1, 0, 0));
        let test_particle_1 = Particle::new((4, 0, 0), (0, 0, 0), (-2, 0, 0));

        assert_eq!(test_particle_0.cycles_until_stable(), 5);
        assert_eq!(test_particle_1.cycles_until_stable(), 2);
    }
}
