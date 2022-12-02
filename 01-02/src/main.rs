use std::fs::read_to_string;

fn main() {
    let snacks_by_elf = read_to_string("./input")
        .expect("file not found")
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| line.parse::<u32>().expect("line is not a positive number"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut calories_by_elf = snacks_by_elf
        .iter()
        .map(|snacks| snacks.iter().sum::<u32>())
        .collect::<Vec<u32>>();

    calories_by_elf.sort();
    calories_by_elf.reverse();

    println!(
        "Top 3 fatties: {}, {}, {}",
        calories_by_elf[0], calories_by_elf[1], calories_by_elf[2]
    );

    println!(
        "Total calories by top 3: {}",
        calories_by_elf[0] + calories_by_elf[1] + calories_by_elf[2]
    )
}
