use std::str::FromStr;

use anyhow::{Context, Result};

use crate::solution::Solution;

pub struct Day {}

struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn new(trees: Vec<Vec<Tree>>) -> Self {
        Self { trees }
    }
}

struct Tree {
    height: usize,
}

impl Tree {
    fn new(height: usize) -> Self {
        Self { height }
    }
}

impl FromStr for Forest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Result<Vec<Vec<Tree>>> = s
            .lines()
            .map(|line| {
                let line_of_trees: Result<Vec<Tree>> = line
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

        Ok(Self::new(trees?))
    }
}

impl Forest {
    fn is_tree_visible(&self, (i, j): (usize, usize)) -> bool {
        let current_height = self.trees[i][j].height;

        let rows = self.trees.len();
        let cols = self.trees[0].len();

        let top = (0..i).rev();
        let bottom = i + 1..rows;
        let left = (0..j).rev();
        let right = j + 1..cols;

        let mut visible = true;
        for i in top {
            let next_height = self.trees[i][j].height;
            if current_height <= next_height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }

        visible = true;
        for i in bottom {
            let next_height = self.trees[i][j].height;
            if current_height <= next_height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }

        visible = true;
        for j in left {
            let next_height = self.trees[i][j].height;
            if current_height <= next_height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }

        visible = true;
        for j in right {
            let next_height = self.trees[i][j].height;
            if current_height <= next_height {
                visible = false;
                break;
            }
        }

        visible
    }

    fn scenic_value(&self, (i, j): (usize, usize)) -> usize {
        let current_height = self.trees[i][j].height;

        let rows = self.trees.len();
        let cols = self.trees[0].len();

        let top = (0..i).rev();
        let bottom = i + 1..rows;
        let left = (0..j).rev();
        let right = j + 1..cols;

        let mut top_value = 0_usize;
        for i in top {
            top_value += 1;
            let next_height = self.trees[i][j].height;
            if current_height <= next_height {
                break;
            }
        }

        let mut bottom_value = 0_usize;
        for i in bottom {
            bottom_value += 1;
            let next_height = self.trees[i][j].height;
            if current_height <= next_height {
                break;
            }
        }

        let mut left_value = 0_usize;
        for j in left {
            left_value += 1;
            let next_height = self.trees[i][j].height;
            if current_height <= next_height {
                break;
            }
        }

        let mut right_value = 0_usize;
        for j in right {
            right_value += 1;
            let next_height = self.trees[i][j].height;
            if current_height <= next_height {
                break;
            }
        }

        let answer = top_value * bottom_value * left_value * right_value;

        answer
    }
}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let forest: Forest = input.parse()?;
        let rows = forest.trees.len();
        let cols = forest.trees[0].len();
        let mut answer = 0_usize;

        for i in 0..rows {
            for j in 0..cols {
                if forest.is_tree_visible((i, j)) {
                    answer += 1;
                }
            }
        }

        Ok(answer.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let forest: Forest = input.parse()?;
        let rows = forest.trees.len();
        let cols = forest.trees[0].len();
        let mut answer = 0_usize;

        for i in 0..rows {
            for j in 0..cols {
                let scenic_value = forest.scenic_value((i, j));

                if scenic_value > answer {
                    answer = scenic_value
                }
            }
        }

        Ok(answer.to_string())
    }
}
