use std::{slice::Iter, str::FromStr};

use anyhow::Context;

use crate::solution::Solution;

pub struct Day {}

struct Forest(Vec<Vec<Tree>>);

struct Tree {
    height: usize,
    visible: VisibleFrom,
}

impl Tree {
    fn new(height: usize) -> Self {
        Self {
            height,
            visible: VisibleFrom::new(),
        }
    }

    fn set_visible(&mut self, direction: Direction, visible: bool) {
        match direction {
            Direction::North => self.visible.north = Some(visible),
            Direction::South => self.visible.south = Some(visible),
            Direction::East => self.visible.east = Some(visible),
            Direction::West => self.visible.west = Some(visible),
        }
    }

    fn get_visible(&self, direction: Direction) -> Option<bool> {
        match direction {
            Direction::North => self.visible.north,
            Direction::South => self.visible.south,
            Direction::East => self.visible.east,
            Direction::West => self.visible.west,
        }
    }
}

struct VisibleFrom {
    north: Option<bool>,
    east: Option<bool>,
    west: Option<bool>,
    south: Option<bool>,
}

impl VisibleFrom {
    fn new() -> Self {
        Self {
            north: None,
            east: None,
            west: None,
            south: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 4] = [North, South, East, West];
        DIRECTIONS.iter()
    }
}

impl FromStr for Forest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: anyhow::Result<Vec<Vec<Tree>>> = s
            .lines()
            .map(|line| {
                let line_of_trees: anyhow::Result<Vec<Tree>> = line
                    .chars()
                    .map(|c| {
                        let height = c
                            .to_digit(10)
                            .context(format!("Could not convert char '{c}' to digit"))?;

                        Ok(Tree::new(height as usize))
                    })
                    .collect();

                line_of_trees
            })
            .collect();

        Ok(Self(trees?))
    }
}

impl Forest {
    fn is_tree_visible(&mut self, point: (usize, usize)) -> bool {
        for direction in Direction::iterator() {
            if self.is_tree_visible_from(point, *direction) {
                return true;
            }
        }

        false
    }

    fn is_tree_visible_from(&mut self, (row, col): (usize, usize), direction: Direction) -> bool {
        let rows = self.0.len();
        let cols = self.0[0].len();

        let tree = &self.0[row][col];
        if let Some(visible) = tree.get_visible(direction) {
            return visible;
        };

        let (row_n, col_n) = match direction {
            Direction::North if row > 0 => (row - 1, col),
            Direction::South if row < rows - 1 => (row + 1, col),
            Direction::West if col > 0 => (row, col - 1),
            Direction::East if col < cols - 1 => (row, col + 1),
            _ => {
                // This is a point on the edge and will always be visible from this direction
                let tree = &mut self.0[row][col];
                tree.set_visible(direction, true);

                return true;
            }
        };

        let neighbour_tree = &self.0[row_n][col_n];

        if neighbour_tree.height >= tree.height {
            // Neighbour is too tall and the tree is not visible from this direction
            let tree = &mut self.0[row][col];
            tree.set_visible(direction, false);

            return false;
        }

        let is_neighbour_visible = self.is_tree_visible_from((row_n, col_n), direction);
        let tree = &mut self.0[row][col];
        tree.set_visible(direction, is_neighbour_visible);

        is_neighbour_visible
    }
}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> anyhow::Result<()> {
        let mut forest: Forest = input.parse()?;
        let rows = forest.0.len();
        let cols = forest.0[0].len();
        let mut answer = 0;

        for i in 0..rows {
            for j in 0..cols {
                let visible = forest.is_tree_visible((i, j));
                let height = forest.0[i][j].height;
                if visible {
                    answer += 1;
                }
                println!("({i}, {j}) is {visible} and has {height}")
            }
        }

        dbg!(answer);

        Ok(())
    }

    fn compute_2(&self, input: &str) -> anyhow::Result<()> {
        todo!()
    }
}
