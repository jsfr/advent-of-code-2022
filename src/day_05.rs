use std::str::FromStr;

use anyhow::Context;
use regex::Regex;

use crate::solution::Solution;

pub struct Day {}

struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn move_one_at_a_time(&mut self, next_move: &Move) -> anyhow::Result<()> {
        let Move { from, to, quantity } = *next_move;

        for _ in 1..=quantity {
            let value = self
                .stack(from)
                .pop()
                .context(format!("No crate to move at stack {from}"))?;

            self.stack(to).push(value);
        }

        Ok(())
    }

    fn move_all_at_once(&mut self, next_move: &Move) {
        let Move { from, to, quantity } = *next_move;
        let from_stack = self.stack(from);
        let split = from_stack.len() - quantity;
        let mut values = from_stack.split_off(split);

        self.stack(to).append(&mut values);
    }

    fn all_tops(&self) -> Vec<char> {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .copied()
            .collect()
    }

    fn stack(&mut self, stack: usize) -> &mut Vec<char> {
        &mut self.stacks[stack - 1]
    }
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
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

        Ok(Self {
            stacks: transposed_stacks,
        })
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let re = Regex::new(r"^ *move +(\d+) +from +(\d+) +to +(\d+) *$")?;
        let cap = re
            .captures(s)
            .context("Line did not match 'move [quantity] from [old_stack] to [new_stack]")?;

        Ok(Self {
            quantity: cap[1].parse()?,
            from: cap[2].parse()?,
            to: cap[3].parse()?,
        })
    }
}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> anyhow::Result<()> {
        let (stacks, moves) = input
            .split_once("\n\n")
            .context("Failed to split in stacks and moves")?;

        let mut stacks: Stacks = stacks.parse()?;

        let moves: Vec<Move> = moves
            .lines()
            .map(str::parse)
            .collect::<anyhow::Result<_>>()?;

        for next_move in moves {
            stacks.move_one_at_a_time(&next_move)?;
        }

        let all_tops: String = stacks.all_tops().iter().collect();

        dbg!(all_tops);

        Ok(())
    }

    fn compute_2(&self, input: &str) -> anyhow::Result<()> {
        let (stacks, moves) = input
            .split_once("\n\n")
            .context("Failed to split in stacks and moves")?;

        let mut stacks: Stacks = stacks.parse()?;

        let moves: Vec<Move> = moves
            .lines()
            .map(str::parse)
            .collect::<anyhow::Result<_>>()?;

        for next_move in moves {
            stacks.move_all_at_once(&next_move);
        }

        let all_tops: String = stacks.all_tops().iter().collect();

        dbg!(all_tops);

        Ok(())
    }
}
