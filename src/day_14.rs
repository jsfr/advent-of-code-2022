use std::collections::HashSet;

use anyhow::{Context, Result};
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day {}
impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let stones: Vec<Stone> = input
            .lines()
            .flat_map(|line| {
                line.split(" -> ").tuple_windows().map(|(start, end)| {
                    let (x, y) = start.split_once(',').context("")?;
                    let start: Coord = (x.parse()?, y.parse()?);

                    let (x, y) = end.split_once(',').context("")?;
                    let end: Coord = (x.parse()?, y.parse()?);

                    Ok(Stone { start, end })
                })
            })
            .collect::<Result<_>>()?;
        let entry: Coord = (500, 0);
        let mut cave = Cave::new(stones, entry);

        while cave.drop_sand() {}

        let answer = cave.sand.len();

        Ok(answer.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Stone {
    start: Coord,
    end: Coord,
}

#[derive(Debug)]
struct Cave {
    sand_entry: Coord,
    stones: Vec<Stone>,
    sand: HashSet<Coord>,
    bottom_y: usize,
}

impl Stone {
    fn intersects(&self, coord: &Coord) -> bool {
        let (xs, ys) = self.start;
        let (xe, ye) = self.end;
        let (xc, yc) = *coord;

        if xs == xe && xe == xc {
            (ys <= yc && yc <= ye) || (ye <= yc && yc <= ys)
        } else if ys == ye && ye == yc {
            (xs <= xc && xc <= xe) || (xe <= xc && xc <= xs)
        } else {
            false
        }
    }
}

impl Cave {
    fn new(stones: Vec<Stone>, sand_entry: Coord) -> Self {
        // TODO remove unwrap
        let bottom_y = stones
            .iter()
            .map(|stone| stone.start.1.max(stone.end.1))
            .max()
            .unwrap();

        Self {
            sand_entry,
            stones,
            sand: HashSet::new(),
            bottom_y,
        }
    }

    fn valid_move(&self, from: Coord) -> Option<Coord> {
        let down = (from.0, from.1 + 1);
        if !self.sand.contains(&down) && !self.stones.iter().any(|s| s.intersects(&down)) {
            return Some(down);
        }

        let left = (from.0 - 1, from.1 + 1);
        if !self.sand.contains(&left) && !self.stones.iter().any(|s| s.intersects(&left)) {
            return Some(left);
        }

        let right = (from.0 + 1, from.1 + 1);
        if !self.sand.contains(&right) && !self.stones.iter().any(|s| s.intersects(&right)) {
            return Some(right);
        }

        None
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand = self.sand_entry;

        while let Some(next_pos) = self.valid_move(sand) {
            if next_pos.1 <= self.bottom_y {
                sand = next_pos;
            } else {
                // Sand fell out
                return false;
            }
        }

        self.sand.insert(sand)
    }
}
