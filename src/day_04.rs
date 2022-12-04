use std::str::FromStr;

use anyhow::Context;

use crate::solution::Solution;

struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn contains_point(&self, point: usize) -> bool {
        self.start <= point && self.end >= point
    }

    fn contains(&self, other: &Interval) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.contains_point(other.start) || self.contains_point(other.end)
    }
}

impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (start, end) = s
            .split_once('-')
            .context(format!("Failed to parse interval {s}"))?;
        let start: usize = start.parse()?;
        let end: usize = end.parse()?;

        Ok(Self { start, end })
    }
}

struct Pair {
    first: Interval,
    second: Interval,
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (first, second) = s
            .split_once(',')
            .context(format!("Failed to parse pair {s}"))?;
        let first: Interval = first.parse()?;
        let second: Interval = second.parse()?;

        Ok(Pair { first, second })
    }
}

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> anyhow::Result<()> {
        let intervals = input
            .lines()
            .map(|line| line.parse::<Pair>())
            .collect::<anyhow::Result<Vec<_>>>()?;

        let contained_intervals = intervals
            .into_iter()
            .filter(|Pair { first, second }| first.contains(second) || second.contains(first))
            .count();

        dbg!(contained_intervals);

        Ok(())
    }

    fn compute_2(&self, input: &str) -> anyhow::Result<()> {
        let intervals = input
            .lines()
            .map(|line| line.parse::<Pair>())
            .collect::<anyhow::Result<Vec<_>>>()?;

        let overlapping_intervals = intervals
            .into_iter()
            .filter(|Pair { first, second }| first.overlaps(second) || second.overlaps(first))
            .count();

        dbg!(overlapping_intervals);

        Ok(())
    }
}
