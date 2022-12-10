use std::{fmt::Debug, str::FromStr};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

use crate::solution::Solution;

type Point = (isize, isize);

struct Rope {
    knots: Vec<Point>,
}

impl Debug for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xs: Vec<isize> = self.knots.iter().map(|(x, _)| *x).collect();
        let ys: Vec<isize> = self.knots.iter().map(|(_, y)| *y).collect();

        let max_x = xs.iter().max().unwrap();
        let min_x = xs.iter().min().unwrap();
        let max_y = ys.iter().max().unwrap();
        let min_y = ys.iter().min().unwrap();

        let mut lines: Vec<String> = vec![];
        for x in *min_x..=*max_x {
            let mut line: String = String::new();
            for y in *min_y..=*max_y {
                match (x, y) {
                    (0, 0) if !self.knots.contains(&(x, y)) => {
                        line.push('s');
                    }
                    (x, y) if self.knots.contains(&(x, y)) => {
                        let (index, _) = self
                            .knots
                            .iter()
                            .find_position(|point| **point == (x, y))
                            .unwrap();
                        let index_str = index.to_string();

                        line.push_str(if index == 0 { "H" } else { index_str.as_str() });
                    }
                    _ => {
                        line.push('.');
                    }
                }
            }
            lines.push(line);
        }

        write!(f, "\n{}", lines.join("\n"))
    }
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            knots: vec![(0, 0); length],
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) -> Vec<Point> {
        let mut visited_points = vec![];
        for _ in 0..instruction.1 {
            let visited_point = self.apply_direction(instruction.0);
            visited_points.push(visited_point);
        }

        visited_points
    }

    fn apply_direction(&mut self, direction: Direction) -> Point {
        let (dx, dy): Point = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        self.knots[0] = (self.knots[0].0 + dx, self.knots[0].1 + dy);

        for i in 1..self.knots.len() {
            let head = self.knots[i - 1];
            let tail = self.knots[i];

            let diff = (head.0.abs_diff(tail.0), head.1.abs_diff(tail.1));

            let new_pos: Point = match diff {
                (2, 0) => {
                    if head.0 > tail.0 {
                        (tail.0 + 1, tail.1)
                    } else {
                        (tail.0 - 1, tail.1)
                    }
                }
                (0, 2) => {
                    if head.1 > tail.1 {
                        (tail.0, tail.1 + 1)
                    } else {
                        (tail.0, tail.1 - 1)
                    }
                }
                (2, 1 | 2) | (1, 2) => {
                    if head.0 > tail.0 && head.1 > tail.1 {
                        (tail.0 + 1, tail.1 + 1)
                    } else if head.0 > tail.0 && head.1 < tail.1 {
                        (tail.0 + 1, tail.1 - 1)
                    } else if head.0 < tail.0 && head.1 < tail.1 {
                        (tail.0 - 1, tail.1 - 1)
                    } else {
                        (tail.0 - 1, tail.1 + 1)
                    }
                }
                (0 | 1, 0 | 1) => tail,
                _ => panic!("failed to move {tail:?}, got {diff:?}"),
            };

            self.knots[i] = new_pos;
        }

        self.knots[self.knots.len() - 1]
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction(Direction, usize);

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::{Down, Left, Right, Up};

        s.split_once(' ')
            .context("failed to parse as an instruction")
            .and_then(|(d, n)| {
                let direction = match d {
                    "U" => Up,
                    "D" => Down,
                    "L" => Left,
                    "R" => Right,
                    _ => bail!("'{d}' is not one of U/D/L/R"),
                };
                let steps: usize = n.parse()?;

                Ok(Instruction(direction, steps))
            })
    }
}

pub struct Day {}
impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let instructions: Result<Vec<Instruction>> = input.lines().map(str::parse).collect();
        let instructions = instructions?;

        let mut rope = Rope::new(2);
        let mut all_points = vec![];

        for instruction in instructions {
            let mut points = rope.apply_instruction(&instruction);
            all_points.append(&mut points);
        }

        let answer = all_points.into_iter().unique().count();

        Ok(answer.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let instructions: Result<Vec<Instruction>> = input.lines().map(str::parse).collect();
        let instructions = instructions?;

        let mut rope = Rope::new(10);
        let mut all_points = vec![];

        for instruction in instructions {
            let mut points = rope.apply_instruction(&instruction);
            all_points.append(&mut points);
        }

        let answer = all_points.into_iter().unique().count();

        Ok(answer.to_string())
    }
}
