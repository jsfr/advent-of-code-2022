use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;

use crate::solution::Solution;

pub struct Day {}

struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn apply_9000(&mut self, instruction: &Instruction) -> Result<()> {
        let Instruction { from, to, quantity } = *instruction;

        for _ in 1..=quantity {
            let krate = self
                .stack(from)
                .pop()
                .context(format!("No crate to move at stack {from}"))?;

            self.stack(to).push(krate);
        }

        Ok(())
    }

    fn apply_9001(&mut self, instruction: &Instruction) {
        let Instruction { from, to, quantity } = *instruction;
        let from_stack = self.stack(from);
        let split = from_stack.len() - quantity;
        let mut krates = from_stack.split_off(split);

        self.stack(to).append(&mut krates);
    }

    fn top_crates(&self) -> Vec<char> {
        self.0
            .iter()
            .filter_map(|stack| stack.last())
            .copied()
            .collect()
    }

    fn stack(&mut self, stack: usize) -> &mut Vec<char> {
        &mut self.0[stack]
    }
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let stacks: Vec<&str> = s.lines().rev().collect();

        let i = stacks[0].len();
        let j = stacks.len();

        let stacks: Vec<char> = stacks.join("").chars().collect();
        let mut transposed_stacks = vec![' '; stacks.len()];

        transpose::transpose(&stacks, &mut transposed_stacks, i, j);

        let re = Regex::new(r"^[\[\] ]+$")?;
        let transposed_stacks: Vec<Vec<char>> = transposed_stacks
            .chunks(j)
            .filter(|chars| {
                let s = String::from_iter(*chars);
                !re.is_match(&s)
            })
            .map(|chars| {
                chars
                    .iter()
                    .filter(|c| !c.is_whitespace())
                    .copied()
                    .collect::<Vec<char>>()
            })
            .collect();

        Ok(Self(transposed_stacks))
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"^ *move +(\d+) +from +(\d+) +to +(\d+) *$")?;
        let cap = re
            .captures(s)
            .context("Line did not match 'move [quantity] from [old_stack] to [new_stack]")?;

        Ok(Self {
            quantity: cap[1].parse()?,
            from: cap[2].parse::<usize>()? - 1,
            to: cap[3].parse::<usize>()? - 1,
        })
    }
}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let (stacks, instructions) = input
            .split_once("\n\n")
            .context("Failed to split in stacks and instructions")?;

        let mut stacks: Stacks = stacks.parse()?;

        let instructions: Vec<Instruction> = instructions
            .lines()
            .map(str::parse)
            .collect::<Result<_>>()?;

        for instruction in instructions {
            stacks.apply_9000(&instruction)?;
        }

        let answer: String = stacks.top_crates().iter().collect();

        Ok(answer)
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let (stacks, instructions) = input
            .split_once("\n\n")
            .context("Failed to split in stacks and instructions")?;

        let mut stacks: Stacks = stacks.parse()?;

        let instructions: Vec<Instruction> = instructions
            .lines()
            .map(str::parse)
            .collect::<Result<_>>()?;

        for instruction in instructions {
            stacks.apply_9001(&instruction);
        }

        let answer: String = stacks.top_crates().iter().collect();

        Ok(answer)
    }
}
