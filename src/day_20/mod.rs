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

pub fn run_day_20() {}
