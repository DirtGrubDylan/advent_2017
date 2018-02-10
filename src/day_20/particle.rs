use regex::Regex;

type Coords = (i32, i32, i32);

pub fn manhattan_distance(coords: &Coords) -> u32 {
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

    pub fn new_from_str(info: &str) -> Result<Particle, String> {
        let re = Regex::new(r"<(-?\d*),(-?\d*),(-?\d*)>").unwrap();

        let temp_v: Vec<Coords> = re.captures_iter(info)
            .map(|capture| {
                let x = (&capture[1]).parse::<i32>().expect("Bad x-value!");
                let y = (&capture[2]).parse::<i32>().expect("Bad y-value!");
                let z = (&capture[3]).parse::<i32>().expect("Bad z-value!");

                (x, y, z)
            })
            .collect();

        if temp_v.len() != 3 {
            Err(String::from("Bad information sent!"))
        } else {
            Ok(Particle::new(temp_v[0], temp_v[1], temp_v[2]))
        }
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

    pub fn position_at(&self, cycles: u32) -> Coords {
        let v_multiplier = cycles as i32;
        let a_multiplier = v_multiplier * (v_multiplier + 1) / 2;

        (
            self.position.0 + v_multiplier * self.velocity.0 + a_multiplier * self.acceleration.0,
            self.position.1 + v_multiplier * self.velocity.1 + a_multiplier * self.acceleration.1,
            self.position.2 + v_multiplier * self.velocity.2 + a_multiplier * self.acceleration.2,
        )
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

    #[test]
    fn test_new_from_str() {
        let test_particle_0 =
            Particle::new_from_str("p=<-11104,1791,5208>, v=<-6,36,-84>, a=<19,-5,-4>");
        let test_particle_1 =
            Particle::new_from_str("p=<-11104,1791>, v=<-6,36,-84>, a=<19,-5,-4>");

        assert!(test_particle_0.is_ok());
        assert!(test_particle_1.is_err());
    }

    #[test]
    fn test_position_at() {
        let test_particle_0 = Particle::new((3, 0, 0), (2, 0, 0), (-1, 0, 0));
        let test_particle_1 = Particle::new((4, 0, 0), (0, 0, 0), (-2, 0, 0));

        let stable_0 = test_particle_0.cycles_until_stable();
        let stable_1 = test_particle_1.cycles_until_stable();

        assert_eq!(test_particle_0.position_at(stable_0), (-2, 0, 0));
        assert_eq!(test_particle_1.position_at(stable_1), (-2, 0, 0));
    }
}
