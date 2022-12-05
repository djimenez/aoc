use std::env;

mod day00;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let mut args = env::args().skip(1);
    let day = args
        .next()
        .expect("Please specify a day argument, e.g. 01")
        .parse::<u64>()
        .unwrap();

    match day {
        0 => day00::run(),
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),

        other => println!("{} was not recognized or is unimplemented", other),
    }
}
