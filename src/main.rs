mod day_01;
mod day_02;
mod day_03;
mod solution;

use std::fs::read_to_string;

use argh::FromArgs;
use solution::Solution;

#[derive(FromArgs)]
/// AOC 2022
struct WithPositional {
    #[argh(positional)]
    day: String,

    #[argh(positional)]
    part: String,
}

fn main() {
    let args: WithPositional = argh::from_env();

    let input = read_to_string(format!("./input/{}", args.day)).expect("failed to read input");

    let day = get_day(args.day.as_str());

    match args.part.as_str() {
        "01" => day.compute_1(input.as_str()),
        "02" => day.compute_2(input.as_str()),
        _ => panic!("Part not found"),
    }
}

fn get_day(day: &str) -> Box<dyn Solution> {
    match day {
        "01" => Box::new(day_01::Day {}),
        "02" => Box::new(day_02::Day {}),
        "03" => Box::new(day_03::Day {}),
        _ => panic!("Day not found"),
    }
}
