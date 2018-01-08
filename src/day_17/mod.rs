mod spin_lock;

use file_reader::to_string_vector;

use self::spin_lock::SpinLock;

pub fn run_day_17() {
    let data = to_string_vector("inputs/day_17.txt").unwrap();

    let input = data[0].parse().unwrap();

    let spinlock = SpinLock::new(input);

    println!(
        "Day 17, Part 1: {}",
        spinlock.short_cicuit_after_value(2017, 2017).unwrap()
    );

    println!(
        "Day 17, Part 2: {}",
        spinlock.short_cicuit_after_index(50_000_000, 0).unwrap()
    );
}
