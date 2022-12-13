use std::ops::Div;

use anyhow::Result;

use crate::solution::Solution;

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    is_divisible_by: usize,
    if_true: usize,
    if_false: usize,
    inspection_count: usize,
}

impl Monkey {}

struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    worry_factor: Box<dyn Fn(usize) -> usize>,
}

impl MonkeyGroup {
    fn exec_turn(&mut self, index: usize) -> Result<()> {
        let monkey = &mut self.monkeys[index];

        let new_items: Vec<_> = monkey
            .items
            .iter()
            .map(|item| {
                let new_item = (self.worry_factor)((monkey.operation)(*item));

                (
                    if new_item % monkey.is_divisible_by == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    },
                    new_item,
                )
            })
            .collect();

        monkey.inspection_count += monkey.items.len();
        monkey.items = vec![];

        new_items.into_iter().for_each(|(new_index, new_item)| {
            self.monkeys[new_index].items.push(new_item);
        });

        Ok(())
    }

    fn exec_round(&mut self) -> Result<()> {
        for i in 0..self.monkeys.len() {
            self.exec_turn(i)?;
        }

        Ok(())
    }
}

pub struct Day {}
impl Solution for Day {
    fn compute_1(&self, _input: &str) -> anyhow::Result<String> {
        // TODO parse the input instead of hard-coding
        let mut monkey_group = MonkeyGroup {
            monkeys: vec![
                Monkey {
                    items: vec![75, 75, 98, 97, 79, 97, 64],
                    operation: Box::new(|item| item * 13),
                    is_divisible_by: 19,
                    if_true: 2,
                    if_false: 7,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![50, 99, 80, 84, 65, 95],
                    operation: Box::new(|item| item + 2),
                    is_divisible_by: 3,
                    if_true: 4,
                    if_false: 5,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![96, 74, 68, 96, 56, 71, 75, 53],
                    operation: Box::new(|item| item + 1),
                    is_divisible_by: 11,
                    if_true: 7,
                    if_false: 3,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![83, 96, 86, 58, 92],
                    operation: Box::new(|item| item + 8),
                    is_divisible_by: 17,
                    if_true: 6,
                    if_false: 1,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![99],
                    operation: Box::new(|item| item * item),
                    is_divisible_by: 5,
                    if_true: 0,
                    if_false: 5,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![60, 54, 83],
                    operation: Box::new(|item| item + 4),
                    is_divisible_by: 2,
                    if_true: 2,
                    if_false: 0,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![77, 67],
                    operation: Box::new(|item| item * 17),
                    is_divisible_by: 13,
                    if_true: 4,
                    if_false: 1,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![95, 65, 58, 76],
                    operation: Box::new(|item| item + 5),
                    is_divisible_by: 7,
                    if_true: 3,
                    if_false: 6,
                    inspection_count: 0,
                },
            ],
            worry_factor: Box::new(|item| item.div(3)),
        };

        for _ in 1..=20 {
            monkey_group.exec_round()?;
        }

        let mut inspection_counts: Vec<usize> = monkey_group
            .monkeys
            .into_iter()
            .map(|m| m.inspection_count)
            .collect();

        inspection_counts.sort_unstable();
        inspection_counts.reverse();

        let answer = inspection_counts[0] * inspection_counts[1];

        Ok(answer.to_string())
    }

    fn compute_2(&self, _input: &str) -> anyhow::Result<String> {
        let mut monkey_group = MonkeyGroup {
            monkeys: vec![
                Monkey {
                    items: vec![75, 75, 98, 97, 79, 97, 64],
                    operation: Box::new(|item| item * 13),
                    is_divisible_by: 19,
                    if_true: 2,
                    if_false: 7,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![50, 99, 80, 84, 65, 95],
                    operation: Box::new(|item| item + 2),
                    is_divisible_by: 3,
                    if_true: 4,
                    if_false: 5,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![96, 74, 68, 96, 56, 71, 75, 53],
                    operation: Box::new(|item| item + 1),
                    is_divisible_by: 11,
                    if_true: 7,
                    if_false: 3,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![83, 96, 86, 58, 92],
                    operation: Box::new(|item| item + 8),
                    is_divisible_by: 17,
                    if_true: 6,
                    if_false: 1,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![99],
                    operation: Box::new(|item| item * item),
                    is_divisible_by: 5,
                    if_true: 0,
                    if_false: 5,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![60, 54, 83],
                    operation: Box::new(|item| item + 4),
                    is_divisible_by: 2,
                    if_true: 2,
                    if_false: 0,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![77, 67],
                    operation: Box::new(|item| item * 17),
                    is_divisible_by: 13,
                    if_true: 4,
                    if_false: 1,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![95, 65, 58, 76],
                    operation: Box::new(|item| item + 5),
                    is_divisible_by: 7,
                    if_true: 3,
                    if_false: 6,
                    inspection_count: 0,
                },
            ],
            worry_factor: Box::new(|item| item % 9699690),
        };

        for _ in 1..=10_000 {
            monkey_group.exec_round()?;
        }

        let mut inspection_counts: Vec<usize> = monkey_group
            .monkeys
            .into_iter()
            .map(|m| m.inspection_count)
            .collect();

        inspection_counts.sort_unstable();
        inspection_counts.reverse();

        let answer = inspection_counts[0] * inspection_counts[1];

        Ok(answer.to_string())
    }
}
