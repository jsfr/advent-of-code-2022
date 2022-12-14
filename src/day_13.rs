use std::{cmp::Ordering, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

use crate::solution::Solution;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Number(usize),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.cmp(b),
            (n @ Value::Number(_), Value::List(_)) => Value::List(vec![n.clone()]).cmp(other),
            (Value::List(_), n @ Value::Number(_)) => self.cmp(&Value::List(vec![n.clone()])),
            (Value::List(vec_a), Value::List(vec_b)) => vec_a.cmp(vec_b),
        }
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Value> {
        let mut value = vec![];
        let mut number: Option<String> = None;

        for c in s.chars() {
            match c {
                '[' => {
                    value.push(Value::List(vec![]));
                }
                ']' => {
                    if let Some(n) = &number {
                        if let Some(Value::List(list)) = value.last_mut() {
                            let n_parsed =
                                n.parse::<usize>().context(format!("failed to parse {n}"))?;
                            list.push(Value::Number(n_parsed));
                            number = None
                        }
                    }
                    if value.len() > 1 {
                        let last = value.pop().unwrap();
                        if let Some(Value::List(list)) = value.last_mut() {
                            list.push(last)
                        }
                    }
                }
                ',' => {
                    if let Some(n) = &number {
                        if let Some(Value::List(list)) = value.last_mut() {
                            let n_parsed =
                                n.parse::<usize>().context(format!("failed to parse {n}"))?;
                            list.push(Value::Number(n_parsed));
                            number = None
                        }
                    }
                }
                _ if c.is_digit(10) => {
                    if let Some(n) = &mut number {
                        n.push(c)
                    } else {
                        number = Some(c.to_string())
                    }
                }
                _ => bail!("unrecognized character in packet: {c}"),
            };
        }

        value
            .pop()
            .context(format!("value was not well-formed: {value:?}"))
    }
}

#[derive(Debug)]
struct Pair {
    first: Value,
    second: Value,
}

impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Pair> {
        let mut values: Vec<Value> = s.lines().map(str::parse).collect::<Result<_>>()?;
        let second = values.pop().context("No second value in pair")?;
        let first = values.pop().context("No first value in pair")?;
        Ok(Pair { first, second })
    }
}

pub struct Day {}
impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let pairs: Vec<Pair> = input.split("\n\n").map(str::parse).collect::<Result<_>>()?;

        let right_pairs: Vec<usize> = pairs
            .into_iter()
            .enumerate()
            .filter_map(|(i, pair)| {
                if pair.first <= pair.second {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .collect();

        let answer: usize = right_pairs.into_iter().sum();

        Ok(answer.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let dividers = vec![
            Value::List(vec![Value::List(vec![Value::Number(2)])]),
            Value::List(vec![Value::List(vec![Value::Number(6)])]),
        ];
        let mut packets: Vec<Value> = input
            .lines()
            .filter_map(|l| if l.is_empty() { None } else { Some(l.parse()) })
            .collect::<Result<_>>()?;

        packets.append(&mut dividers.clone());
        packets.sort();

        let (index_1, _) = packets
            .iter()
            .find_position(|p| **p == dividers[0])
            .context("could not find divider")?;
        let (index_2, _) = packets
            .iter()
            .find_position(|p| **p == dividers[1])
            .context("could not find divider")?;

        let answer = index_1 + index_2 + 2;

        Ok(answer.to_string())
    }
}
