pub mod file_reader;
mod day_1;
mod day_2;

use day_1::run_day_1;
use day_2::run_day_2;

fn print_seperator() {
    println!("-------------------------------" );
}

fn main() {
    run_day_1();
    print_seperator();

    run_day_2();
    print_seperator();

    // run_day_2();
    // print_seperator();
}
