mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod utils;

use crate::day1::day1_part1;
use crate::day1::day1_part2;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;
use crate::day6::day6;

struct Op {
    name: &'static str,
    action: fn() -> ()
}

static OPS: [Op; 7] = [
    Op{name:"day1_part1", action: day1_part1},
    Op{name:"day1_part2", action: day1_part2},
    Op{name:"day2", action: day2},
    Op{name:"day3", action: day3},
    Op{name:"day4", action: day4},
    Op{name:"day5", action: day5},
    Op{name:"day6", action: day6},
];

fn execute(action: &str) {
    for op in &OPS {
        if action == op.name {
            (op.action)();
            return;
        }
    }
    println!("Unknown operation: {}", action);
}

fn main() {
    if std::env::args().count() == 1 {
        execute(&OPS.last().unwrap().name);
    } else {
        for arg in std::env::args().skip(1) {
            execute(&arg);
        }
    }
}