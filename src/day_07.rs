use std::str::FromStr;

use anyhow::{bail, Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u64,
    combinator::{all_consuming, map, rest},
    sequence::{preceded, tuple},
    Finish, IResult,
};

use crate::solution::Solution;

pub struct Day {}

#[derive(Debug)]
enum ConsoleLine {
    Command(Command),
    Output(Output),
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(Direction),
    ListFiles,
}

#[derive(Debug)]
enum Direction {
    Down(String),
    Up,
    Root,
}

#[derive(Debug)]
enum Output {
    File(usize, String),
    Directory(String),
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
    calculated_size: Option<usize>,
}

#[derive(Debug)]
struct File {
    size: usize,
    _name: String,
}

impl Directory {
    fn size(&mut self) -> usize {
        match self.calculated_size {
            Some(size) => size,
            None => {
                let file_size: usize = self.files.iter().map(|f| f.size).sum();
                let dir_size: usize = self.directories.iter_mut().map(Directory::size).sum();
                let total_size = file_size + dir_size;

                self.calculated_size = Some(total_size);

                total_size
            }
        }
    }

    fn directory_sizes(&mut self) -> Vec<usize> {
        let own_size = self.size();
        let mut sizes: Vec<usize> = self
            .directories
            .iter_mut()
            .flat_map(Directory::directory_sizes)
            .collect();

        sizes.push(own_size);

        sizes
    }
}

const MAX_SIZE: usize = 100000;
const REQUIRED_SIZE: usize = 30000000;
const AVAILABLE_SPACE: usize = 70000000;

fn parse_cmd(i: &str) -> IResult<&str, Command> {
    let cd = map(preceded(tag("$ cd"), rest), |s: &str| {
        let directory = s.trim();
        let change_directory = match directory {
            "/" => Direction::Root,
            ".." => Direction::Up,
            _ => Direction::Down(directory.to_string()),
        };

        Command::ChangeDirectory(change_directory)
    });
    let ls = map(tag("$ ls"), |_| Command::ListFiles);

    alt((cd, ls))(i)
}

fn parse_output(i: &str) -> IResult<&str, Output> {
    let dir = map(preceded(tag("dir"), rest), |s: &str| {
        Output::Directory(s.trim().to_string())
    });
    let file = map(tuple((u64, rest)), |(size, name): (u64, &str)| {
        Output::File(size as usize, name.trim().to_string())
    });

    alt((dir, file))(i)
}

fn parse_line(i: &str) -> IResult<&str, ConsoleLine> {
    let cmd = map(parse_cmd, ConsoleLine::Command);
    let output = map(parse_output, ConsoleLine::Output);

    alt((cmd, output))(i)
}

impl FromStr for ConsoleLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parsed_line = all_consuming(parse_line)(s).finish();

        match parsed_line {
            Ok((_, console_line)) => Ok(console_line),
            Err(err) => bail!("Failed to parse line {s} with error: {err}"),
        }
    }
}

#[derive(Debug)]
struct State {
    current: Vec<usize>,
    tree: Directory,
}

impl State {
    fn new() -> Self {
        let tree = Directory {
            name: "/".to_string(),
            files: vec![],
            directories: vec![],
            calculated_size: None,
        };
        let current = vec![];

        Self { tree, current }
    }

    fn get_current(&mut self) -> &mut Directory {
        let mut current = &mut self.tree;

        for i in &self.current {
            current = &mut current.directories[*i];
        }

        current
    }

    fn apply(&mut self, line: ConsoleLine) -> Result<()> {
        match line {
            ConsoleLine::Command(Command::ChangeDirectory(direction)) => {
                match direction {
                    Direction::Up => {
                        self.current.pop();
                    }
                    Direction::Down(name) => {
                        let current = self.get_current();

                        let position = current
                            .directories
                            .iter()
                            .position(|d| d.name == name)
                            .context(format!(
                                "failed to find directory {} in {:?}",
                                name, current.name
                            ))?;

                        self.current.push(position);
                    }
                    Direction::Root => {
                        self.current = vec![];
                    }
                };
            }
            ConsoleLine::Command(Command::ListFiles) => {}
            ConsoleLine::Output(Output::Directory(name)) => {
                self.get_current().directories.push(Directory {
                    name,
                    files: vec![],
                    directories: vec![],
                    calculated_size: None,
                })
            }
            ConsoleLine::Output(Output::File(size, name)) => {
                self.get_current().files.push(File { size, _name: name });
            }
        };

        Ok(())
    }
}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let lines: Result<Vec<ConsoleLine>> = input.lines().map(str::parse).collect();

        let mut state = State::new();

        for line in lines?.into_iter() {
            state.apply(line)?;
        }

        let answer: usize = state
            .tree
            .directory_sizes()
            .into_iter()
            .filter(|size| *size <= MAX_SIZE)
            .sum();

        Ok(answer.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let lines: Result<Vec<ConsoleLine>> = input.lines().map(str::parse).collect();

        let mut state = State::new();

        for line in lines?.into_iter() {
            state.apply(line)?;
        }

        let root_size = state.tree.size();
        let space_to_free = REQUIRED_SIZE - (AVAILABLE_SPACE - root_size);
        let mut sizes = state.tree.directory_sizes();

        sizes.sort();

        let answer = sizes
            .into_iter()
            .find(|size| *size >= space_to_free)
            .context("No directory found")?;

        Ok(answer.to_string())
    }
}
