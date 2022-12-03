use std::{collections::HashSet, fs::read_to_string};

fn item_value(item: &char) -> u32 {
    let offset = if item.is_lowercase() { 96 } else { 38 };
    let value = *item as u32;

    return value - offset;
}

fn find_shared_item(lines: &[&str]) -> Option<char> {
    let shared_item = lines
        .iter()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(|acc, next| acc.intersection(&next).copied().collect::<HashSet<char>>())
        .map(|set| set.iter().next().copied())
        .flatten();

    shared_item
}

fn main() {
    let rucksack_value: u32 = read_to_string("./input")
        .expect("file not found")
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|rucksacks| {
            let shared_item = find_shared_item(rucksacks).expect("found no shared item");

            println!("Shared item: {}", shared_item);

            item_value(&shared_item)
        })
        .sum();

    println!("Total sum of priorities: {}", rucksack_value);
}
