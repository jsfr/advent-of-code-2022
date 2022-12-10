use std::str::FromStr;

use anyhow::{bail, Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, rest},
    sequence::preceded,
    Finish, IResult,
};

use crate::solution::Solution;

#[derive(Debug)]
enum Instruction {
    Noop(usize),
    Addx(usize, i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_instruction)(s).finish() {
            Ok((_, instruction)) => Ok(instruction),
            Err(err) => bail!("Failed to parse line {s} with error: {err}"),
        }
    }
}

#[derive(Debug)]
struct State {
    x: i32,
    instructions: Vec<Instruction>,
}

impl State {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self { x: 1, instructions }
    }

    fn tick(&mut self) -> Result<()> {
        let instruction = self
            .instructions
            .last_mut()
            .context("No more instructions left")?;

        match instruction {
            Instruction::Noop(_) => {
                self.instructions.pop();
            }
            Instruction::Addx(cycles, n) => {
                if *cycles == 1 {
                    self.x += *n;
                    self.instructions.pop();
                } else {
                    *instruction = Instruction::Addx(*cycles - 1, *n);
                }
            }
        }

        Ok(())
    }
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    let parse_noop = tag("noop");
    let parse_addx = preceded(tag("addx "), rest);

    alt((
        map(parse_noop, |_| Instruction::Noop(1)),
        map(parse_addx, |n: &str| {
            Instruction::Addx(2, n.parse().unwrap())
        }),
    ))(s)
}

pub struct Day {}
impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<()> {
        let cycle_stops = [20, 60, 100, 140, 180, 220];

        let instructions: Vec<Instruction> =
            input.lines().map(str::parse).rev().collect::<Result<_>>()?;

        let mut state = State::new(instructions);

        let mut answer = vec![];

        for i in 1..=220 {
            if cycle_stops.contains(&i) {
                answer.push(state.x * i);
            }

            state.tick()?;
        }

        let answer: i32 = answer.into_iter().sum();

        dbg!(answer);

        Ok(())
    }

    fn compute_2(&self, input: &str) -> Result<()> {
        let instructions: Vec<Instruction> =
            input.lines().map(str::parse).rev().collect::<Result<_>>()?;

        let mut state = State::new(instructions);

        let mut answer = "".to_string();
        for _ in 1..=6 {
            let mut row = "\n".to_string();
            for pixel in 0..=39 {
                let sprite = state.x - 1..=state.x + 1;
                if sprite.contains(&pixel) {
                    row.push('#');
                } else {
                    row.push('.');
                }
                state.tick()?;
            }

            answer.push_str(&row);
        }

        println!("{answer}");

        Ok(())
    }
}
