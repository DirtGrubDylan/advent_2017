mod judge;
mod generator;

use file_reader::to_string_vector;

use self::judge::Judge;
use self::generator::Generator;

pub fn run_day_15() {
    let input = to_string_vector("inputs/day_15.txt").unwrap();

    let gen_seeds: Vec<usize> = input
        .iter()
        .map(|s| {
            let temp_v: Vec<&str> = s.split_whitespace().collect();

            temp_v.last().unwrap().parse().unwrap()
        })
        .collect();

    let gen_a = Generator::new(gen_seeds[0], 16807);
    let gen_b = Generator::new(gen_seeds[1], 48271);

    let mut part_1_judge = Judge::new(&[gen_a, gen_b], 65535);

    println!(
        "Day 15, Part 1: {}",
        part_1_judge.number_of_matching_values(40_000_000)
    );
}
