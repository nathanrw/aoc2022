use std::str::FromStr;
use std::io;
use std::vec::Vec;

use crate::utils::read_data_lines;

struct Compartment {
    contents_str: String,
    contents: Vec<i32>
}

struct Backpack {
    original: String,
    contents: Vec<i32>,
    left: Compartment,
    right: Compartment
}

fn intersection(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    // note: SLOW
    let mut ret: Vec<i32> = Vec::new();
    for l in &a {
        for r in &b {
            if *l == *r {
                ret.push(*l);
            }
        }
    }
    ret.dedup();
    ret
}

impl Backpack {
    fn common(&self) -> Vec<i32> {
        intersection(self.left.contents.to_vec(), self.right.contents.to_vec())
    }
    fn debug_print(&self) {
        println!("Backpack:");
        println!("  From: {}", self.original);
        println!("  Left:");
        println!("    From: {}", self.left.contents_str);
        println!("    Ordinals:");
        print!("      ");
        for p in &self.left.contents {
            print!("{} ", p);
        }
        println!("");
        println!("  Right:");
        println!("    From: {}", self.right.contents_str);
        println!("    Ordinals:");
        print!("      ");
        for p in &self.right.contents {
            print!("{} ", p);
        }
        println!("");
        println!("  Common:");
        print!("    ");
        for p in self.common() {
            print!("{} ", p);
        }
        println!("");
    }
}

impl FromStr for Compartment {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let contents: Vec<i32> = s.chars().map(get_ascii_ordinal).collect::<io::Result<Vec<i32>>>()?;
        Ok(Compartment { contents_str: s.to_owned(), contents: contents })
    }
}

impl FromStr for Backpack {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_at(s.len()/2);
        let left_compartment = left.parse::<Compartment>()?;
        let right_compartment = right.parse::<Compartment>()?;
        let contents: Vec<i32> = s.chars().map(get_ascii_ordinal).collect::<io::Result<Vec<i32>>>()?;
        Ok(Backpack {original: s.to_owned(), contents: contents, left: left_compartment, right: right_compartment})
    }
}

fn get_ascii_ordinal(c: char) -> io::Result<i32> {
    let ordinals = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    match ordinals.chars().position(|x| x == c) {
        Some(x) => Ok(x as i32 + 1),
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Bad character")),
    }
}

//fn read_day3_input() -> io::Result<Vec<Backpack>> {
//    read_data_records::<Backpack>("day3.txt")
//}

fn read_day3_input() -> io::Result<Vec<Backpack>> {
    let lines = read_data_lines("day3.txt")?;
    lines.iter()
        .map(|x| x.trim().to_owned())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<Backpack>() )
        .collect()
}

pub fn day3() {
    let packs = read_day3_input().unwrap();
    packs[0].debug_print();
    let priority_sum: i32 = packs.iter().map(|x| x.common().iter().sum::<i32>()).sum();
    println!("Backpack common priority sum: {}", priority_sum);
    let group_sum: i32 = packs
        .chunks(3)
        .map(|chunk| chunk
            .iter()
            .map(|pack| pack.contents.to_vec())
            .reduce(|acc, item| intersection(acc.to_vec(), item.to_vec())).unwrap()
            .iter().sum::<i32>())
        .sum();
    println!("Backpack group priority sum: {}", group_sum);
}