use super::particle::{manhattan_distance, Particle};

#[derive(Debug)]
pub struct GPU {
    pub buffer: Vec<Particle>,
}

impl GPU {
    pub fn new(info: &[String]) -> Result<GPU, String> {
        let mut temp_v = Vec::new();

        for s in info {
            temp_v.push(Particle::new_from_str(s)?);
        }

        Ok(GPU { buffer: temp_v })
    }

    pub fn closest_particle(&self) -> usize {
        let min_acceleration = self.buffer
            .iter()
            .min_by_key(|particle| particle.acceleration_strength())
            .unwrap()
            .acceleration_strength();

        let lowest_accelerating_particles: Vec<(usize, &Particle)> = self.buffer
            .iter()
            .enumerate()
            .filter(|&(_, particle)| particle.acceleration_strength() == min_acceleration)
            .collect();

        let maximum_number_of_cycles_until_stable = lowest_accelerating_particles
            .iter()
            .max_by_key(|&&(_, particle)| particle.cycles_until_stable())
            .unwrap()
            .1
            .cycles_until_stable();

        lowest_accelerating_particles
            .iter()
            .min_by_key(|&&(_, particle)| {
                let temp_position = particle.position_at(maximum_number_of_cycles_until_stable);

                manhattan_distance(&temp_position)
            })
            .unwrap()
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let test_good_input = to_string_vector("test_inputs/day_20_part_1.txt").unwrap();
        let test_bad_input = to_string_vector("test_inputs/day_19_part_1.txt").unwrap();
        let test_good_gpu = GPU::new(&test_good_input);
        let test_bad_gpu = GPU::new(&test_bad_input);

        assert!(test_good_gpu.is_ok());
        assert!(test_bad_gpu.is_err());
    }

    #[test]
    fn test_closest_particle() {
        let test_input = to_string_vector("test_inputs/day_20_part_1.txt").unwrap();
        let test_gpu = GPU::new(&test_input).unwrap();

        assert_eq!(test_gpu.closest_particle(), 0);
    }
}
