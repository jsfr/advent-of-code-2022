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
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissor,
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

fn calculate_fight(fight: (Hand, Hand)) -> u32 {
    use Hand::*;

    return match fight {
        (Rock, Scissor) | (Paper, Rock) | (Scissor, Paper) => 6,
        (you, other) if you == other => 3,
        _ => 0,
    };
}

fn main() {
    let total_score: u32 = read_to_string("./input")
        .expect("file not found")
        .lines()
        .map(|line| {
            let mut hands = line.split_whitespace();
            let other = hands.next().map(Hand::new).expect("invalid strategy");
            let you = hands.next().map(Hand::new).expect("invalid strategy");

            let fight = (you, other);
            let fight_value = calculate_fight(fight);
            let hand_value = you.value();

            fight_value + hand_value
        })
        .sum();

    println!("Total score: {}", total_score);
}
