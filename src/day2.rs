use std::vec::Vec;
use std::io;
use std::str::FromStr;

use crate::utils::read_data_lines;

#[derive(Clone, Copy)]
enum RockPaperScissors {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(Clone, Copy)]
enum RockPaperScissorsOutcome {
    WIN,
    LOSE,
    DRAW,
}

struct RockPaperScissorsRound {
    me: RockPaperScissors,
    thee: RockPaperScissors,
    target: RockPaperScissorsOutcome,
}

impl FromStr for RockPaperScissors {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RockPaperScissors::ROCK),
            "B" => Ok(RockPaperScissors::PAPER),
            "C" => Ok(RockPaperScissors::SCISSORS),
            "X" => Ok(RockPaperScissors::ROCK),
            "Y" => Ok(RockPaperScissors::PAPER),
            "Z" => Ok(RockPaperScissors::SCISSORS),
            _ => Err(Self::Err::new(io::ErrorKind::InvalidData, "Bad character")),
        }
    }
}

impl FromStr for RockPaperScissorsOutcome {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RockPaperScissorsOutcome::LOSE),
            "Y" => Ok(RockPaperScissorsOutcome::DRAW),
            "Z" => Ok(RockPaperScissorsOutcome::WIN),
            _ => Err(Self::Err::new(io::ErrorKind::InvalidData, "Bad character")),
        }
    }
}

impl FromStr for RockPaperScissorsRound {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        if tokens.len() != 2 {
            return Err(Self::Err::new(io::ErrorKind::InvalidData, "Bad round"));
        }
        let me = tokens[1].parse::<RockPaperScissors>()?;
        let thee = tokens[0].parse::<RockPaperScissors>()?;
        let target = tokens[1].parse::<RockPaperScissorsOutcome>()?;
        Ok(RockPaperScissorsRound { me: me, thee: thee, target: target})
    }
}

fn action_score(x: RockPaperScissors) -> i32 {
    match x {
        RockPaperScissors::ROCK => 1,
        RockPaperScissors::PAPER => 2,
        RockPaperScissors::SCISSORS => 3,
    }
}

fn outcome_score(x: RockPaperScissorsOutcome) -> i32 {
    match x {
        RockPaperScissorsOutcome::LOSE => 0,
        RockPaperScissorsOutcome::DRAW => 3,
        RockPaperScissorsOutcome::WIN => 6
    }
}

fn outcome(me: RockPaperScissors, thee: RockPaperScissors) -> RockPaperScissorsOutcome {
    match me {
        RockPaperScissors::ROCK => match thee {
            RockPaperScissors::ROCK => RockPaperScissorsOutcome::DRAW,
            RockPaperScissors::PAPER => RockPaperScissorsOutcome::LOSE,
            RockPaperScissors::SCISSORS => RockPaperScissorsOutcome::WIN,
        },
        RockPaperScissors::PAPER => match thee {
            RockPaperScissors::ROCK => RockPaperScissorsOutcome::WIN,
            RockPaperScissors::PAPER => RockPaperScissorsOutcome::DRAW,
            RockPaperScissors::SCISSORS => RockPaperScissorsOutcome::LOSE,
        },
        RockPaperScissors::SCISSORS => match thee {
            RockPaperScissors::ROCK => RockPaperScissorsOutcome::LOSE,
            RockPaperScissors::PAPER => RockPaperScissorsOutcome::WIN,
            RockPaperScissors::SCISSORS => RockPaperScissorsOutcome::DRAW,
        },
    }
}

fn achieve(thee: RockPaperScissors, target: RockPaperScissorsOutcome) -> RockPaperScissors {
    match thee {
        RockPaperScissors::ROCK => match target {
            RockPaperScissorsOutcome::LOSE => RockPaperScissors::SCISSORS,
            RockPaperScissorsOutcome::DRAW => RockPaperScissors::ROCK,
            RockPaperScissorsOutcome::WIN => RockPaperScissors::PAPER,
        },
        RockPaperScissors::PAPER => match target {
            RockPaperScissorsOutcome::LOSE => RockPaperScissors::ROCK,
            RockPaperScissorsOutcome::DRAW => RockPaperScissors::PAPER,
            RockPaperScissorsOutcome::WIN => RockPaperScissors::SCISSORS,
        },
        RockPaperScissors::SCISSORS => match target {
            RockPaperScissorsOutcome::LOSE => RockPaperScissors::PAPER,
            RockPaperScissorsOutcome::DRAW => RockPaperScissors::SCISSORS,
            RockPaperScissorsOutcome::WIN => RockPaperScissors::ROCK,
        },
    }
}

impl RockPaperScissorsRound {
    fn score(&self) -> i32 {
        action_score(self.me) + outcome_score(outcome(self.me, self.thee))
    }
    fn score2(&self) -> i32 {
        outcome_score(self.target) + action_score(achieve(self.thee, self.target))
    }
}

fn read_day2_input() -> io::Result<Vec<RockPaperScissorsRound>> {
    let lines = read_data_lines("day2.txt")?;
    lines.iter()
        .map(|x| x.trim().to_owned())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<RockPaperScissorsRound>() )
        .collect()
}

pub fn day2() {
    let rounds = read_day2_input().unwrap();
    let score: i32 = rounds.iter().map(RockPaperScissorsRound::score).sum();
    println!("Rock paper scissors score: {}", score);
    let score2: i32 = rounds.iter().map(RockPaperScissorsRound::score2).sum();
    println!("Rock paper scissors score2: {}", score2);
}