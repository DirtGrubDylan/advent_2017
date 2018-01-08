mod spin_lock;

use self::spin_lock::SpinLock;

pub fn run_day_17() {
    let spinlock = SpinLock::new(348);

    println!(
        "Day 17, Part 1: {}",
        spinlock.short_cicuit_after_value(2017, 2017).unwrap()
    );

    println!(
        "Day 17, Part 2: {}",
        spinlock.short_cicuit_after_value(50_000_000, 0).unwrap()
    );
}
