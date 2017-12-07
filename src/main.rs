pub mod file_reader;
mod day_1;
mod day_2;
mod day_3;

fn print_seperator() {
    println!("-------------------------------");
}

fn main() {
    print_seperator();

    day_1::run_day_1();
    print_seperator();

    day_2::run_day_2();
    print_seperator();

    day_3::run_day_3();
    print_seperator();
}
