use std::collections::HashSet;

use anyhow::Result;

use crate::solution::Solution;

fn item_value(item: char) -> u32 {
    let offset = if item.is_lowercase() { 96 } else { 38 };
    let value = item as u32;

    value - offset
}

fn find_shared_item_simple(a: &str, b: &str) -> Option<char> {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    let shared_item = set_a.intersection(&set_b).next().copied();

    shared_item
}

fn find_shared_item_advanced(lines: &[&str]) -> Option<char> {
    let shared_item = lines
        .iter()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(|acc, next| acc.intersection(&next).copied().collect::<HashSet<char>>())
        .and_then(|set| set.iter().next().copied());

    shared_item
}

pub struct Day {}
impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let answer: u32 = input
            .lines()
            .map(|rucksack| {
                let mid = rucksack.len() / 2;
                let (compartment_1, compartment_2) = rucksack.split_at(mid);
                let shared_item = find_shared_item_simple(compartment_1, compartment_2)
                    .expect("found no shared item");

                item_value(shared_item)
            })
            .sum();

        Ok(answer.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let answer: u32 = input
            .lines()
            .collect::<Vec<&str>>()
            .chunks_exact(3)
            .map(|rucksacks| {
                let shared_item =
                    find_shared_item_advanced(rucksacks).expect("found no shared item");

                item_value(shared_item)
            })
            .sum();

        Ok(answer.to_string())
    }
}
