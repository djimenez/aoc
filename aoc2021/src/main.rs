use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
/*
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
*/
mod day21;
mod day22;
/*
mod day23;
mod day24;
mod day25;
*/

fn main() {
    let mut args = env::args().skip(1);
    let day = args
        .next()
        .expect("Please specify a day argument, e.g. day1");

    match day.as_str() {
        "day1" => day1::run(),
        "day2" => day2::run(),
        "day3" => day3::run(),
        "day4" => day4::run(),
        "day5" => day5::run(),
        "day6" => day6::run(),
        "day7" => day7::run(),
        "day8" => day8::run(),
        /*
        "day9" => day9::run(),
        "day10" => day10::run(),
        "day11" => day11::run(),
        "day12" => day12::run(),
        "day13" => day13::run(),
        "day14" => day14::run(),
        "day15" => day15::run(),
        "day16" => day16::run(),
        "day17" => day17::run(),
        "day18" => day18::run(),
        "day19" => day19::run(),
        "day20" => day20::run(),
        */
        "day21" => day21::run(),
        "day22" => day22::run(),
        /*
        "day23" => day23::run(),
        "day24" => day24::run(),
        "day25" => day25::run(),
        */
        other => println!("{} was not recognized or is unimplemented", other),
    }
}
