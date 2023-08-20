use std::str::FromStr;

use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, u64},
    combinator::{all_consuming, map},
    multi::many1,
    Finish, IResult,
};

use crate::solution::Solution;

pub struct Day {}
impl Solution for Day {
    fn compute_1(&self, input: &str) -> anyhow::Result<String> {
        let (map, instructions) = input.split_once("\n\n").context("failed to split input")?;

        let map: Map = map.parse()?;
        let instructions: Vec<Instruction> = parse_instructions(instructions)?;

        let position = (
            0_usize,
            map.0[0]
                .iter()
                .find_position(|t| **t == Tile::Path)
                .context("no empty path found")?
                .0,
        );
        let person = Person {
            position,
            direction: Direction::Right,
        };

        let person = instructions
            .into_iter()
            .fold(person, apply_instruction(&map));

        Ok(String::new())
    }

    fn compute_2(&self, input: &str) -> anyhow::Result<String> {
        todo!()
    }
}

fn apply_instruction(map: &Map) -> impl Fn(Person, Instruction) -> Person {
    |person: Person, instruction: Instruction| match instruction {
        Instruction::Walk(d) => {
            loop {
                todo!();
            }
            Person {
                position,
                direction: person.direction,
            }
        }
        rotate => Person {
            position: person.position,
            direction: person.direction.rotate(rotate),
        },
    }
}

#[derive(Debug, Clone)]
struct Person {
    position: (usize, usize),
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self, rotate: Instruction) -> Direction {
        match rotate {
            Instruction::RotateLeft => match self {
                Self::Up => Self::Left,
                Self::Down => Self::Right,
                Self::Left => Self::Down,
                Self::Right => Self::Up,
            },
            Instruction::RotateRight => match self {
                Self::Up => Self::Right,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
                Self::Right => Self::Down,
            },
            _ => *self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Path,
    Wall,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug)]
enum Instruction {
    Walk(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug)]
struct Map(Vec<Vec<Tile>>);

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let map: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| line.chars().map(parse_tile).collect::<Result<_>>())
            .collect::<Result<_>>()?;

        Ok(Self(map))
    }
}

fn parse_tile(c: char) -> Result<Tile> {
    use Tile::*;

    match c {
        ' ' => Ok(Empty),
        '#' => Ok(Wall),
        '.' => Ok(Path),
        _ => Err(anyhow!("failed to parse '{c}' as a tile")),
    }
}

fn parse_instructions(s: &str) -> Result<Vec<Instruction>> {
    let s = s.trim();
    let parsed = all_consuming(many1(parse_instruction))(s);

    match parsed.finish() {
        Ok((_, instructions)) => Ok(instructions),
        Err(err) => bail!("Failed to parse instructions {s} with error: {err}"),
    }
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    use Instruction::*;

    let walk = map(u64, |d: u64| Walk(d as usize));
    let left = map(char('L'), |_| RotateLeft);
    let right = map(char('R'), |_| RotateRight);

    alt((walk, left, right))(s)
}
