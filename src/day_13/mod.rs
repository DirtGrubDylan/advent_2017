mod layer;
mod firewall;

use file_reader::to_string_vector;
use self::firewall::Firewall;

pub fn run_day_13() {
    let input = to_string_vector("inputs/day_13.txt").unwrap();
    let firewall = Firewall::new(&input);

    println!("Day 13, Part 1: {}", firewall.capture_severity_leaving_at(0));
    println!("Day 13, Part 2: {}", firewall.vulerable_time());
}
