use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum NeededOutcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn score(&self, other: &Hand) -> u32 {
        (*self as u32) + self._win_score(other)
    }

    fn from_char(c: char, is_part_one: bool) -> Option<Self> {
        use Hand::*;

        match (c.to_ascii_uppercase(), is_part_one) {
            ('A', _) | ('X', true) => Some(Rock),
            ('B', _) | ('Y', true) => Some(Paper),
            ('C', _) | ('Z', true) => Some(Scissors),
            _ => None,
        }
    }

    fn score_with_outcome(&self, outcome: NeededOutcome) -> u32 {
        use Hand::*;
        use NeededOutcome::*;

        let other_hand = match (self, &outcome) {
            (&a, Draw) => a,

            (Rock, Loss) => Scissors,
            (Rock, Win) => Paper,

            (Paper, Loss) => Rock,
            (Paper, Win) => Scissors,

            (Scissors, Loss) => Paper,
            (Scissors, Win) => Rock,
        };

        println!("The hand needed for outcome {outcome:?} is {other_hand:?}");

        other_hand as u32 + outcome as u32
    }

    //Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
    fn _win_score(&self, other: &Hand) -> u32 {
        use Hand::*;

        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            (a, b) if a == b => 3,
            (_, _) => 0,
        }
    }
}

impl NeededOutcome {
    fn from_char(c: char) -> Option<Self> {
        use NeededOutcome::*;

        match c {
            'X' => Some(Loss),
            'Y' => Some(Draw),
            'Z' => Some(Win),
            _ => None,
        }
    }
}

fn part_one(input: &Vec<String>) {
    let hands = input
        .iter()
        .map(|line| {
            line.splitn(2, ' ')
                .flat_map(|chunk| chunk.chars().nth(0))
                .flat_map(|c| Hand::from_char(c, true))
                .collect::<Vec<_>>()
        })
        .map(|x| {
            let [a, b] = x[..2] else { unreachable!() };
            (a, b)
        })
        .collect::<Vec<_>>();

    let total_score = hands.iter().map(|(other, me)| me.score(other)).sum::<u32>();

    println!("Total score: {total_score}");
}

fn part_two(input: &Vec<String>) {
    let score = input
        .iter()
        .map(|line| {
            line.splitn(2, ' ')
                .flat_map(|chunk| chunk.chars().nth(0))
                .collect::<Vec<_>>()
        })
        .map(|x| {
            let [a, b] = x[..2] else { unreachable!() };

            let opponents_hand = Hand::from_char(a, false).unwrap();
            let outcome = NeededOutcome::from_char(b).unwrap();

            let score = opponents_hand.score_with_outcome(outcome);

            println!(
                "Opponent played {opponents_hand:?} and the outcome is {outcome:?}. Score: {score}"
            );

            score
        })
        .sum::<u32>();

    println!("Score: {score}");
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(File::open("./task.txt")?);

    let lines = file.lines().flatten().collect();

    part_one(&lines);

    part_two(&lines);

    Ok(())
}
