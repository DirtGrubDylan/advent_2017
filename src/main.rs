#![feature(entry_and_modify)]

pub mod file_reader;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;

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

    day_7::run_day_7();
    print_seperator();

    day_8::run_day_8();
    print_seperator();

    day_9::run_day_9();
    print_seperator();

    day_10::run_day_10();
    print_seperator();

    day_11::run_day_11();
    print_seperator();

    day_12::run_day_12();
    print_seperator();

    day_13::run_day_13();
    print_seperator();

    day_14::run_day_14();
    print_seperator();

    day_15::run_day_15();
    print_seperator();

    day_16::run_day_16();
    print_seperator();
}
