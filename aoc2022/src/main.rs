use std::env;

mod day00;
mod day01;

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

        other => println!("{} was not recognized or is unimplemented", other),
    }
}
