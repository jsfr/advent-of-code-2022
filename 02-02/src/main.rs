use std::fs::read_to_string;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl Hand {
    fn new(input: &str) -> Self {
        use Hand::*;

        return match input {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissor,
            _ => panic!("invalid input"),
        };
    }

    fn value(&self) -> u32 {
        use Hand::*;

        return match self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        };
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn new(input: &str) -> Self {
        use Outcome::*;

        match input {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("invalid input"),
        }
    }

    fn value(&self) -> u32 {
        use Outcome::*;

        return match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        };
    }
}

fn find_hand(outcome: Outcome, hand: Hand) -> Hand {
    use Hand::*;
    use Outcome::*;

    return match (outcome, hand) {
        (Draw, _) => hand,
        (Win, Rock) | (Lose, Scissor) => Paper,
        (Win, Paper) | (Lose, Rock) => Scissor,
        (Win, Scissor) | (Lose, Paper) => Rock,
    };
}

fn main() {
    let total_score: u32 = read_to_string("./input")
        .expect("file not found")
        .lines()
        .map(|line| {
            let mut strategy = line.split_whitespace();
            let other = strategy.next().map(Hand::new).expect("invalid strategy");
            let outcome = strategy.next().map(Outcome::new).expect("invalid strategy");
            let you = find_hand(outcome, other);

            let fight_value = outcome.value();
            let hand_value = you.value();

            fight_value + hand_value
        })
        .sum();

    println!("Total score: {}", total_score);
}
