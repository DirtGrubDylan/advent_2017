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

    let gen_a_1 = Generator::new(gen_seeds[0], 16807, 1);
    let gen_b_1 = Generator::new(gen_seeds[1], 48271, 1);

    let mut part_1_judge = Judge::new(&[gen_a_1, gen_b_1], 65535);

    println!(
        "Day 15, Part 1: {}",
        part_1_judge.number_of_matching_values(40_000_000)
    );

    let gen_a_4 = Generator::new(gen_seeds[0], 16807, 4);
    let gen_b_8 = Generator::new(gen_seeds[1], 48271, 8);

    let mut part_2_judge = Judge::new(&[gen_a_4, gen_b_8], 65535);

    println!(
        "Day 15, Part 2: {}",
        part_2_judge.number_of_matching_values(5_000_000)
    );
}
