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

    let fattest_elf = snacks_by_elf
        .iter()
        .map(|snacks| snacks.iter().sum::<u32>())
        .max()
        .expect("no fattest elf?");

    println!("Fattest elf: {}", fattest_elf)
}
