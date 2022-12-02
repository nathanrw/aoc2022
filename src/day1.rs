use std::fs::File;
use std::vec::Vec;
use std::io::{self, BufRead};

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

pub fn day1_part1() {
    let calories_array = read_day1_input();
    let max = calories_array.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
    println!("Most calorific elf: {}", max);
    println!("Most calories on an elf: {}", calories_array[max]);
}

pub fn day1_part2() {
    let mut calories_array = read_day1_input();
    calories_array.sort();
    calories_array.reverse();
    println!("Calories of top 3 elves: {}", calories_array[0..3].iter().sum::<i32>())
}