pub mod file_reader;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

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

    day_4::run_day_4();
    print_seperator();

    day_5::run_day_5();
    print_seperator();

    day_6::run_day_6();
    print_seperator();
}
