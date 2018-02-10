// notes
// * find the particles with the lowest acceleration
// * if there is more that one particle that share that acceleration
//   * find the minimum number of cycles to stablize the velocity
//   * move all particles by that number
//   * find the minimum number of cycles to stablize the position to have same parity as velocity
//   * move all particles by that number
//   * find the closest to <0, 0, 0> position
mod gpu;
mod particle;

use self::gpu::GPU;
use file_reader::to_string_vector;

pub fn run_day_20() {
    let input = to_string_vector("inputs/day_20.txt").unwrap();
    let gpu = GPU::new(&input).unwrap();

    println!("Day 20, Part 1: {}", gpu.closest_particle());
}
