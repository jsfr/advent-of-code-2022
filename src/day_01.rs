use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) {
        let snacks_by_elf = input
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

        println!("Fattest elf: {}", fattest_elf);
    }

    fn compute_2(&self, input: &str) {
        let snacks_by_elf = input
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

        calories_by_elf.sort_unstable();
        calories_by_elf.reverse();

        println!(
            "Top 3 fatties: {}, {}, {}",
            calories_by_elf[0], calories_by_elf[1], calories_by_elf[2]
        );

        println!(
            "Total calories by top 3: {}",
            calories_by_elf[0] + calories_by_elf[1] + calories_by_elf[2]
        );
    }
}
