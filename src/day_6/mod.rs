mod memory_bank;

use file_reader::to_string_vector;
use self::memory_bank::MemoryBank;

pub fn run_day_6() {
    let day_6_input = to_string_vector("inputs/day_6.txt").unwrap();
    let memory_bank = MemoryBank::new(&day_6_input[0]);

    println!("Day 6, Part 1: {:?}", memory_bank.number_of_redistribution_cycles().0);
    println!("Day 6, Part 2: {:?}", memory_bank.length_of_first_redistribution_loop());
}