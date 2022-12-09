use crate::utils::{read_data_lines, AocResult, AocError};

use std::io;
use std::io::BufRead;
use std::fs::File;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::Read;
use core::num::ParseIntError;

struct Cell {
    height: i32,
    visible: bool
}

struct Heightmap {
    width: i32,
    height: i32,
    values: Vec<Vec<Cell>>
}

impl Heightmap {
    fn cell(&mut self, x: i32, y: i32) -> &mut Cell {
        &mut self.values[y as usize][x as usize]
    }
    fn cell2(&self, x: i32, y: i32) -> &Cell { // todo: how to do this?
        &self.values[y as usize][x as usize]
    }
    fn look(&mut self, height: i32, mut x: i32, mut y: i32, dx: i32, dy: i32) {
        let mut max_height = height;
        while self.in_bounds(x, y) {
            let cell = self.cell(x, y);
            if cell.height > max_height {
                cell.visible = true;
                max_height = cell.height;
            }
            x += dx;
            y += dy;
        }
    }
    fn view_distance(&self, height: i32, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
        if !self.in_bounds(x, y) {
            0
        } else if self.cell2(x, y).height < height {
            1 + self.view_distance(height, x+dx, y+dy, dx, dy)
        } else {
            1
        }
    }
    fn view_score(&self, x: i32, y: i32) -> i32{
        let height = self.cell2(x, y).height;
        self.view_distance(height, x, y-1, 0, -1) *  // up
            self.view_distance(height, x+1, y, 1, 0) *  // right
            self.view_distance(height, x, y+1, 0, 1) * // down
            self.view_distance(height, x-1, y, -1, 0) // left
    }
    fn best_view_score(&self) -> i32 {
        (0..(self.width*self.height))
            .map(|i| self.view_score(i as i32 % self.width, i as i32 / self.width))
            .max().unwrap()
    }
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    fn examine(&mut self) {
        // down
        for x in 0..self.width {
            self.look(-1, x, 0, 0, 1);
        }
        // left
        for y in 0..self.height {
            self.look(-1, self.width-1, y, -1, 0);
        }
        // up
        for x in 0..self.width {
            self.look(-1, x, self.height-1, 0, -1);
        }
        // right
        for y in 0..self.height {
            self.look(-1, 0, y, 1, 0);
        }
    }
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.cell2(x, y).visible {self.cell2(x, y).height.to_string()} else { " ".to_string() });
            }
            println!();
        }
    }
    fn print2(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.cell2(x, y).height);
            }
            println!();
        }
    }
}

fn read_day8_input() -> AocResult<Heightmap>{
    let lines = read_data_lines("day8.txt")?;
    let map = lines.iter()
        .map(|x| x.chars()
            .map(|y| y.to_string().parse::<i32>().map(|z| Cell {height: z, visible: false }))
            .collect::<Result<Vec<Cell>, ParseIntError>>())
        .collect::<Result<Vec<Vec<Cell>>, ParseIntError>>()?;
    let height = map.len();
    let width = map[0].len();
    for row in &map {
        if width != row.len() {
            return Err(AocError::new("Inconsistent number of rows"));
        }
    }
    Ok(Heightmap {width: width as i32, height: height as i32, values: map })
}

pub fn day8() {
    let mut hm = read_day8_input().unwrap();
    hm.examine();
    hm.print2();
    println!();
    hm.print();
    let num_visible = hm.values.iter()
        .map(|x| x.iter()
            .filter(|y| y.visible )
            .count())
        .sum::<usize>();
    println!("Num visible trees: {}", num_visible);
    let best = hm.best_view_score();
    println!("Best tree score: {}", best);
}