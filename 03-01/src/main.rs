use std::{collections::HashSet, fs::read_to_string};

fn item_value(item: &char) -> u32 {
    let offset = if item.is_lowercase() { 96 } else { 38 };
    let value = *item as u32;

    return value - offset;
}

fn find_shared_item(a: &str, b: &str) -> Option<char> {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    let shared_item = set_a.intersection(&set_b).next().copied();

    println!("shared item: {:?}", shared_item);

    shared_item
}

fn main() {
    let rucksack_value: u32 = read_to_string("./input")
        .expect("file not found")
        .lines()
        .map(|rucksack| {
            let mid = rucksack.len() / 2;
            let (compartment_1, compartment_2) = rucksack.split_at(mid);
            let shared_item =
                find_shared_item(compartment_1, compartment_2).expect("found no shared item");

            item_value(&shared_item)
        })
        .sum();

    println!("Total sum of priorities: {}", rucksack_value);
}
