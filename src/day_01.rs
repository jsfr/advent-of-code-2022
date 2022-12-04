use anyhow::Context;

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> anyhow::Result<()> {
        let snacks_by_elf = input
            .split("\n\n")
            .map(|chunk| {
                chunk
                    .lines()
                    .map(|line| {
                        line.parse::<usize>()
                            .context(format!("Failed to parse {line} as a usize"))
                    })
                    .collect::<anyhow::Result<Vec<_>>>()
            })
            .collect::<anyhow::Result<Vec<Vec<_>>>>()?;

        let fattest_elf = snacks_by_elf
            .iter()
            .map(|snacks| snacks.iter().sum::<usize>())
            .max();

        dbg!(fattest_elf);

        Ok(())
    }

    fn compute_2(&self, input: &str) -> anyhow::Result<()> {
        let snacks_by_elf = input
            .split("\n\n")
            .map(|chunk| {
                chunk
                    .lines()
                    .map(|line| {
                        line.parse::<usize>()
                            .context(format!("Failed to parse {line} as a usize"))
                    })
                    .collect::<anyhow::Result<Vec<usize>>>()
            })
            .collect::<anyhow::Result<Vec<Vec<usize>>>>()?;

        let mut calories_by_elf = snacks_by_elf
            .iter()
            .map(|snacks| snacks.iter().sum::<usize>())
            .collect::<Vec<usize>>();

        calories_by_elf.sort_unstable();
        calories_by_elf.reverse();

        let top_3_summed: usize = calories_by_elf.into_iter().take(3).sum();

        dbg!(top_3_summed);

        Ok(())
    }
}
