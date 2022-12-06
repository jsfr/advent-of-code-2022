use itertools::Itertools;

use crate::solution::Solution;

pub struct Day {}

fn find_marker(input: &str, length: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut answer = length;

    for window in chars.windows(length) {
        if window.iter().all_unique() {
            break;
        }
        answer += 1;
    }

    answer
}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> anyhow::Result<()> {
        let answer = find_marker(input, 4);

        dbg!(answer);

        Ok(())
    }

    fn compute_2(&self, input: &str) -> anyhow::Result<()> {
        let answer = find_marker(input, 14);

        dbg!(answer);

        Ok(())
    }
}
