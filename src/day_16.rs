use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use regex::Regex;

use crate::solution::Solution;

const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

pub struct Day {}
impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let input = INPUT;
        let cave: Cave = input.parse()?;

        let steps = 30;
        let start = "AA".to_string();

        let mut runs = vec![Run::new(start, steps)];

        for _ in 1..=steps {
            runs = runs
                .into_iter()
                .flat_map(|run| {
                    let moves = run.valid_moves(&cave);
                    moves
                        .into_iter()
                        .map(|mmove| {
                            let mut run = run.clone();
                            run.make_move(&cave, mmove).unwrap(); // TODO get rid of unwrap
                            run
                        })
                        .collect::<Vec<Run>>()
                })
                .collect::<Vec<Run>>()
        }

        let answer = runs.into_iter().max_by_key(|run| run.acc_flow).unwrap();

        Ok(answer.acc_flow.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

type Id = String;

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
}

impl Valve {
    fn new(flow_rate: usize) -> Self {
        Self { flow_rate }
    }
}

#[derive(Debug)]
struct Cave {
    valves: HashMap<Id, Valve>,
    tunnels: HashMap<Id, Vec<Id>>,
}

impl Cave {
    fn new(valves: HashMap<Id, Valve>, tunnels: HashMap<Id, Vec<Id>>) -> Self {
        Self { valves, tunnels }
    }
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (valves, tunnels) =
            s.lines()
                .try_fold((HashMap::new(), HashMap::new()), |(mut valves, mut tunnels), line| {
                    let Some((valve_str, tunnels_str)) = line.split_once("; ") else { bail!("failed to split line") };
                    let (id, valve) = parse_valve_str(valve_str)?;
                    let new_tunnels = parse_tunnels_str(tunnels_str)?;

                    valves.insert(id.clone(), valve);
                    tunnels.insert(id, new_tunnels);

                    Ok((valves, tunnels))
                })?;

        Ok(Self::new(valves, tunnels))
    }
}

fn parse_valve_str(s: &str) -> Result<(String, Valve)> {
    let re = Regex::new(r"^Valve ([A-Z][A-Z]) has flow rate=(\d+)$")?;
    let cap = re.captures(s).context("failed to match valve line")?;

    Ok((cap[1].parse()?, Valve::new(cap[2].parse()?)))
}

fn parse_tunnels_str(s: &str) -> Result<Vec<String>> {
    let re = Regex::new(r"^tunnels? leads? to valves? (.*)$")?;
    let cap = re.captures(s).context("failed to match tunnels line")?;
    let tunnels: Vec<String> = cap[1].split(", ").map(|s| s.to_string()).collect();

    Ok(tunnels)
}

#[derive(Debug, Clone)]
struct Run {
    previous_move: Option<Move>,
    current: Id,
    is_open: Vec<Id>,
    flow_rate: usize,
    acc_flow: usize,
    time_left: usize,
}

#[derive(Debug, Clone)]
enum Move {
    Go { from: Id, to: Id },
    OpenValve,
}

impl Run {
    fn new(start: Id, steps: usize) -> Self {
        Self {
            previous_move: None,
            current: start,
            is_open: vec![],
            flow_rate: 0,
            acc_flow: 0,
            time_left: steps,
        }
    }

    fn is_current_open(&self) -> bool {
        self.is_open.contains(&self.current)
    }

    fn valid_moves(&self, cave: &Cave) -> Vec<Move> {
        let mut moves = vec![];

        match cave.valves.get(&self.current) {
            Some(Valve { flow_rate }) if *flow_rate > 0 && !self.is_current_open() => {
                moves.push(Move::OpenValve);
            }
            _ => {}
        }

        if let Some(tunnels) = cave.tunnels.get(&self.current) {
            tunnels
                .iter()
                .filter_map(|tunnel| match &self.previous_move {
                    Some(Move::Go { from, .. }) if from == tunnel => None,
                    _ => Some(Move::Go {
                        from: self.current.clone(),
                        to: tunnel.clone(),
                    }),
                })
                .for_each(|mmove| {
                    moves.push(mmove);
                });
        }

        moves
    }

    fn make_move(&mut self, cave: &Cave, mmove: Move) -> Result<()> {
        self.time_left -= 1;

        match mmove {
            Move::Go { ref to, .. } => {
                self.current = to.clone();
            }
            Move::OpenValve => {
                let valve = cave
                    .valves
                    .get(&self.current)
                    .context("failed to find valve")?;
                self.is_open.push(self.current.clone());
                self.flow_rate += valve.flow_rate;
                self.acc_flow += valve.flow_rate * self.time_left;
            }
        };

        self.previous_move = Some(mmove);

        Ok(())
    }
}
