use std::fs::File;
use std::vec::Vec;
use std::io::{self, BufRead};
use std::str::FromStr;

fn read_day1_input() -> Vec<i32> {
    let file = File::open("./inputs/day1.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut calories: i32 = 0;
    let mut fruits: i32 = 0;
    let mut calories_array = Vec::new();
    for line in lines {
        let line_string = line.unwrap();
        if line_string.trim().is_empty() {
            calories_array.push(calories);
            calories = 0;
            fruits = 0;
        } else {
            calories += line_string.parse::<i32>().unwrap();
            fruits += 1;
        }
    }
    if fruits > 0 {
        calories_array.push(calories);
    }
    return calories_array;
}

fn day1_part1() {
    let calories_array = read_day1_input();
    let max = calories_array.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
    println!("Most calorific elf: {}", max);
    println!("Most calories on an elf: {}", calories_array[max]);
}

fn day1_part2() {
    let mut calories_array = read_day1_input();
    calories_array.sort();
    calories_array.reverse();
    println!("Calories of top 3 elves: {}", calories_array[0..3].iter().sum::<i32>())
}

enum RockPaperScissors {
    ROCK,
    PAPER,
    SCISSORS,
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

struct RockPaperScissorsRound {
    me: RockPaperScissors,
    thee: RockPaperScissors,
}

impl RockPaperScissorsRound {
    fn score(&self) -> i32 {
        match self.me {
            RockPaperScissors::ROCK => 1 + match self.thee {
                RockPaperScissors::ROCK => 3,
                RockPaperScissors::PAPER => 0,
                RockPaperScissors::SCISSORS => 6,
            },
            RockPaperScissors::PAPER => 2 + match self.thee {
                RockPaperScissors::ROCK => 6,
                RockPaperScissors::PAPER => 3,
                RockPaperScissors::SCISSORS => 0,
            },
            RockPaperScissors::SCISSORS => 3 + match self.thee {
                RockPaperScissors::ROCK => 0,
                RockPaperScissors::PAPER => 6,
                RockPaperScissors::SCISSORS => 3,
            },
        }
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
        Ok(RockPaperScissorsRound { me: me, thee: thee })
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

fn day2() {
    let rounds = read_day2_input().unwrap();
    let score: i32 = rounds.iter().map(RockPaperScissorsRound::score).sum();
    println!("Rock paper scissors score: {}", score);
}

fn main() {
    day1_part1();
    day1_part2();
    day2();
}