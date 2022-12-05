use std::str::FromStr;
use std::vec::Vec;
use crate::utils::{read_data_lines, AocError, AocResult};

struct Column {
    stack: Vec<char>
}

struct Crates {
    stacks: Vec<Column>
}

impl Crates {
    fn make() -> Crates {
        Crates { stacks: vec![
            Column { stack: vec!['W', 'D', 'G', 'B', 'H', 'R', 'V'] },
            Column { stack: vec!['J', 'N', 'G', 'C', 'R', 'F' ] },
            Column { stack: vec!['L', 'S', 'F', 'H', 'D', 'N', 'J'] },
            Column { stack: vec!['J', 'D', 'S', 'V'] },
            Column { stack: vec!['S', 'H', 'D', 'R', 'Q', 'W', 'N', 'V'] },
            Column { stack: vec!['P', 'G', 'H', 'C', 'M'] },
            Column { stack: vec!['F', 'J', 'B', 'G', 'L', 'Z', 'H', 'C'] },
            Column { stack: vec!['S', 'J', 'R'] },
            Column { stack: vec!['L', 'G', 'S', 'R', 'B', 'N', 'V', 'M'] }
        ] }
    }
    fn make_example() -> Crates {
        Crates { stacks: vec![
            Column { stack: vec!['Z', 'N' ] },
            Column { stack: vec!['M', 'C', 'D' ] },
            Column { stack: vec!['P'] },
        ] }
    }
}

impl Crates {
    fn execute(&mut self, m: &Move) {
        self.move_crates(m.amount, m.source, m.destination);
    }
    fn execute_batched_move(&mut self, m: &Move) {
        self.move_crates_batch(m.amount, m.source, m.destination);
    }
    fn move_crates_batch(&mut self, num: i32, src: usize, dst: usize)
    {
        let mut tmp = Vec::new();
        for i in 0..num {
            tmp.push(self.stacks[src].stack.pop().unwrap());
        }
        while tmp.len() != 0 {
            self.stacks[dst].stack.push(tmp.pop().unwrap());
        }
    }
    fn move_crates(&mut self, num: i32, src: usize, dst: usize)
    {
        if num <= 0 {
            return;
        }
        let val = self.stacks[src].stack.pop().unwrap();
        self.stacks[dst].stack.push(val);
        self.move_crates(num-1, src, dst);
    }
    fn print_state(&self) {
        let max = self.stacks.iter().map(|x| x.stack.len()).max().unwrap();
        for i in (0..max).rev() {
            for col in &self.stacks {
                if col.stack.len() <= i {
                    print!("  ");
                } else {
                    print!("{} ", col.stack[i]);
                }
            }
            println!("");
        }
        for i in 0..self.stacks.len() {
            print!("{} ", i+1);
        }
        println!("");
    }
    fn output(&self) {
        for col in &self.stacks {
            if col.stack.len() == 0 {
                print!(" ");
            } else {
                print!("{}", col.stack.last().unwrap());
            }
        }
        println!("");
    }
}

struct Move {
    amount: i32,
    source: usize,
    destination: usize
}

impl FromStr for Move {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // move 2 from 2 to 7
        let tokens = s.split(' ').collect::<Vec<&str>>();
        if tokens.len() != 6 {
            return Err(AocError::new("Bad move command"));
        }
        let amount = tokens[1].parse::<i32>()?;
        let source = tokens[3].parse::<usize>()? - 1;
        let destination = tokens[5].parse::<usize>()? - 1;
        Ok(Move{amount: amount, source:source, destination: destination})
    }
}

fn read_day5_input() -> AocResult<Vec<Move>> {
    let lines = read_data_lines("day5.txt")?;
    lines.iter()
        .filter(|x| x.contains("move"))
        .map(|x| x.parse::<Move>() )
        .collect()
}

pub fn day5() {
    let program = read_day5_input().unwrap();
    let mut crates = Crates::make();
    crates.print_state();
    let mut counter = 0;
    for cmd in program {
        counter = counter + 1;
        println!("");
        println!("{}. move {} from {} to {}", counter, cmd.amount, cmd.source+1, cmd.destination+1);
        println!("");
        crates.execute_batched_move(&cmd);
        crates.print_state();
    }
    crates.output();
}