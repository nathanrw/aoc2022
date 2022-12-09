

use crate::utils::{read_data_lines, AocResult, AocError};

use std::io;
use std::io::BufRead;
use std::fs::File;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::Read;
use core::num::ParseIntError;
use core::str::FromStr;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Clone, Copy)]
struct Move {
    amount: i32,
    direction: Direction
}

impl Move {
    fn displacement(&self) -> (i32, i32) {
        match self.direction {
            Direction::Left => (-self.amount, 0),
            Direction::Right => (self.amount, 0),
            Direction::Up => (0, -self.amount),
            Direction::Down => (0, self.amount)
        }
    }
    fn direction(&self) -> (i32, i32) {
        if (self.is_zero()) {
            return (0, 0);
        }
        match self.direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1)
        }
    }
    fn is_zero(&self) -> bool {
        self.amount == 0
    }
    fn decremented(&self) -> Self {
        Move {amount: self.amount - 1, direction: self.direction }
    }
}

impl FromStr for Move {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(" ").collect::<Vec<_>>();
        let direction = match tokens[0] {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(AocError::new("Unknown direction"))
        }?;
        let distance = tokens[1].parse::<i32>()?;
        Ok(Move{amount: distance, direction: direction})
    }
}

struct Rope {
    segments: Vec<(i32, i32)>,
    trail: HashSet<(i32, i32)>
}

impl Rope {
    fn new(len: i32) -> Self {
        let mut ret = Rope { segments: Vec::new(), trail: HashSet::new() };
        for i in 0..len {
            ret.segments.push((0, 0));
        }
        ret.trail.insert((0, 0));
        ret
    }
    fn move_head_by_one(&mut self, direction: (i32, i32)) {
        let (dx, dy) = direction;
        let (mut x, mut y) = self.segments[0];
        x += dx;
        y += dy;
        self.segments[0] = (x, y);
        self.reposition_tail(1);
    }
    fn reposition_tail(&mut self, idx: usize) {
        if idx >= self.segments.len() {
            return;
        }
        let (x0, y0) = self.segments[idx];
        let (x1, y1) = self.segments[idx-1];
        let dx = x1 - x0;
        let dy = y1 - y0;
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }
        self.segments[idx] = (x0 + dx.signum(), y0 + dy.signum());
        if idx == self.segments.len()-1 {
            self.trail.insert(self.segments[idx]);
        }
        self.reposition_tail(idx+1);
    }
    fn move_head(&mut self, mv: Move) {
        if (mv.is_zero()) {
            return;
        }
        self.move_head_by_one(mv.direction());
        self.move_head(mv.decremented());
    }
}

fn read_day9_input() -> AocResult<Vec<Move>> {
    let lines = read_data_lines("day9.txt")?;
    lines.iter()
        .map(|x| 
            x.parse::<Move>()).collect::<AocResult<Vec<_>>>()
}

pub fn day9() {
    let mut rope = Rope::new(2);
    let moves = read_day9_input().unwrap();
    for mv in &moves {
        rope.move_head(*mv);
    }
    println!("Number of tail positions: {}", rope.trail.len());
    let mut rope2: Rope = Rope::new(10);
    for mv in &moves {
        rope2.move_head(*mv);
    }
    println!("Number of tail positions in long rope: {}", rope2.trail.len());
}