use std::str::FromStr;
use std::vec::Vec;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::utils::read_data_lines;

struct ZoneRange {
    from: i32,
    to: i32
}

struct ZonePair {
    first: ZoneRange,
    second: ZoneRange
}

#[derive(Debug)]
struct ZoneError {
    details: String
}

type ZoneResult<T> = Result<T, ZoneError>;

impl From<std::io::Error> for ZoneError {
    fn from(e: std::io::Error) -> Self {
        ZoneError { details: e.to_string() }
    }
}

impl From<std::num::ParseIntError> for ZoneError {
    fn from(e: std::num::ParseIntError) -> Self {
        ZoneError { details: e.to_string() }
    }
}

impl Display for ZoneError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ZoneError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl ZoneError {
    fn new(msg: &str) -> Self {
        Self{details: msg.to_string()}
    }
}

impl FromStr for ZoneRange {
    type Err = ZoneError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split('-').collect::<Vec<&str>>();
        if tokens.len() != 2 {
            return Err(ZoneError::new("Bad range"));
        }
        let left = tokens[0].parse::<i32>()?;
        let right = tokens[1].parse::<i32>()?;
        Ok(ZoneRange{from: left, to: right})
    }
}

impl FromStr for ZonePair {
    type Err = ZoneError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(',').collect::<Vec<&str>>();
        if tokens.len() != 2 {
            return Err(ZoneError::new("Bad pair"));
        }
        let left = tokens[0].parse::<ZoneRange>()?;
        let right = tokens[1].parse::<ZoneRange>()?;
        Ok(ZonePair{first: left, second: right})
    }
}

impl ZoneRange {
    fn inside(&self, point: i32) -> bool {
        self.from <= point && point <= self.to
    }
    fn starts_inside(&self, that: &ZoneRange) -> bool {
        that.inside(self.from)
    }
    fn ends_inside(&self, that: &ZoneRange) -> bool {
        that.inside(self.to)
    }
    fn contains(&self, that: &ZoneRange) -> bool {
        that.starts_inside(self) && that.ends_inside(self)
    }
    fn intersects(&self, that: &ZoneRange) -> bool {
        self.starts_inside(that) || self.ends_inside(that) || self.contains(that)
    }
}

impl Display for ZoneRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.from, self.to)
    }
}

impl ZonePair {
    fn fully_overlapping(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }
    fn intersecting(&self) -> bool {
        self.first.intersects(&self.second)
    }
}

impl Display for ZonePair {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{},{}", self.first, self.second)
    }
}

fn read_day4_input() -> ZoneResult<Vec<ZonePair>> {
    let lines = read_data_lines("day4.txt")?;
    lines.iter()
        .map(|x| x.trim().to_owned())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<ZonePair>() )
        .collect()
}

fn number_of_fully_overlapping_pairs() -> ZoneResult<i32> {
    let input = read_day4_input()?;
    Ok(input.iter().filter(|x| x.fully_overlapping()).count() as i32)
}

fn number_of_intersecting_pairs() -> ZoneResult<i32> {
    let input = read_day4_input()?;
    Ok(input.iter().filter(|x| x.intersecting()).count() as i32)
}

pub fn day4() {
    let num = number_of_fully_overlapping_pairs().unwrap();
    println!("Number of fully overlapping pairs: {}", num);
    let num2 = number_of_intersecting_pairs().unwrap();
    println!("Number of intersecting pairs: {}", num2);
}