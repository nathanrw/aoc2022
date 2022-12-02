use std::fs::File;
use std::vec::Vec;
use std::io::{self, BufRead};
use std::str::FromStr;

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

#[derive(Debug)]
struct RockPaperScissorsError {
    details: String
}

impl RockPaperScissorsError {
    fn new(details: &str) -> RockPaperScissorsError {
        RockPaperScissorsError{details: details.to_string()}
    }
}

impl FromStr for RockPaperScissors {
    type Err = RockPaperScissorsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RockPaperScissors::ROCK),
            "B" => Ok(RockPaperScissors::PAPER),
            "C" => Ok(RockPaperScissors::SCISSORS),
            "X" => Ok(RockPaperScissors::ROCK),
            "Y" => Ok(RockPaperScissors::PAPER),
            "Z" => Ok(RockPaperScissors::SCISSORS),
            _ => Err(Self::Err::new("Bad character")),
        }
    }
}

impl FromStr for RockPaperScissorsOutcome {
    type Err = RockPaperScissorsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RockPaperScissorsOutcome::LOSE),
            "Y" => Ok(RockPaperScissorsOutcome::DRAW),
            "Z" => Ok(RockPaperScissorsOutcome::WIN),
            _ => Err(Self::Err::new("Bad character")),
        }
    }
}

struct RockPaperScissorsRound {
    me: RockPaperScissors,
    thee: RockPaperScissors,
    target: RockPaperScissorsOutcome,
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

impl FromStr for RockPaperScissorsRound {
    type Err = RockPaperScissorsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        if tokens.len() != 2 {
            return Err(Self::Err::new("Bad round"));
        }
        let me = tokens[1].parse::<RockPaperScissors>()?;
        let thee = tokens[0].parse::<RockPaperScissors>()?;
        let target = tokens[1].parse::<RockPaperScissorsOutcome>()?;
        Ok(RockPaperScissorsRound { me: me, thee: thee, target: target})
    }
}

fn read_day2_input() -> Result<Vec<RockPaperScissorsRound>, RockPaperScissorsError> {
    let file = File::open("./inputs/day2.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let rounds = lines
        .map(|x| x.unwrap().trim().to_owned())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<RockPaperScissorsRound>() );
    let mut ret = Vec::new();
    for round in rounds {
        ret.push(round?);
    }
    Ok(ret)
}

pub fn day2() {
    let rounds = read_day2_input().unwrap();
    let score: i32 = rounds.iter().map(RockPaperScissorsRound::score).sum();
    println!("Rock paper scissors score: {}", score);
    let score2: i32 = rounds.iter().map(RockPaperScissorsRound::score2).sum();
    println!("Rock paper scissors score2: {}", score2);
}