mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_13;
mod day_14;
mod day_16;
mod day_22;
mod solution;
mod tests;

use std::fs::read_to_string;

use anyhow::{bail, Context};
use argh::FromArgs;
use solution::Solution;

#[derive(FromArgs)]
/// AOC 2022
struct Args {
    #[argh(positional)]
    day: String,

    #[argh(positional)]
    part: String,
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    let Args { day, part } = args;

    let file = format!("./input/{}", day.as_str());
    let input = read_to_string(&file).context(format!("Failed to read {file}"))?;

    if let Some(day) = get_day(day.as_str()) {
        let answer = match part.as_str() {
            "01" => day.compute_1(input.as_str())?,
            "02" => day.compute_2(input.as_str())?,
            _ => bail!("Part {part} was not found"),
        };

        println!("The answer is:\n{answer}");
    } else {
        bail!("Day {day} was not found");
    }

    Ok(())
}

fn get_day(day: &str) -> Option<Box<dyn Solution>> {
    let solution: Box<dyn Solution> = match day {
        "01" => Box::new(day_01::Day {}),
        "02" => Box::new(day_02::Day {}),
        "03" => Box::new(day_03::Day {}),
        "04" => Box::new(day_04::Day {}),
        "05" => Box::new(day_05::Day {}),
        "06" => Box::new(day_06::Day {}),
        "07" => Box::new(day_07::Day {}),
        "08" => Box::new(day_08::Day {}),
        "09" => Box::new(day_09::Day {}),
        "10" => Box::new(day_10::Day {}),
        "11" => Box::new(day_11::Day {}),
        "13" => Box::new(day_13::Day {}),
        "14" => Box::new(day_14::Day {}),
        "16" => Box::new(day_16::Day {}),
        "22" => Box::new(day_22::Day {}),
        _ => return None,
    };

    Some(solution)
}
