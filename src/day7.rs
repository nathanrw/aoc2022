use crate::utils::{read_data_lines, AocResult, AocError};
use core::str::FromStr;

enum Line {
    Ls,
    Cd(String),
    Directory(String),
    File(String, usize),
}

impl FromStr for Line {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(" ").collect::<Vec<_>>();
        // todo: bad num tokens
        match tokens[0] {
            "$" => {
                match tokens[1] {
                    "cd" => Ok(Line::Cd(tokens[2].to_owned())),
                    "ls" => Ok(Line::Ls),
                    _ => Err(AocError::new("Unknown command"))
                }
            },
            "dir" => Ok(Line::Directory(tokens[1].to_owned())),
            _ => {
                let sz = tokens[0].parse::<usize>()?;
                Ok(Line::File(tokens[1].to_owned(), sz))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum FsItem {
    File(String, usize),
    Directory(String, Vec<usize>)
}

#[derive(Debug, Clone)]
struct Fs {
    stack: Vec<usize>,
    items: Vec<FsItem>
}

impl Fs {
    fn cd(&mut self, dst: &str) {
        if dst == ".." {
            self.stack.pop();
        } else if dst == "/" {
            self.stack.clear();
        } else {
            for idx in self.ls() {
                match &self.items[idx] {
                    FsItem::Directory(name, _) => {
                        if name == dst {
                            self.stack.push(idx);
                            return;
                        }
                    },
                    _ => {},
                };
            }
            // todo: error
        }
    }
    fn additem(&mut self, item: FsItem) {
        self.items.push(item);
        let idx = self.items.len()-1;
        let curr = self.current();
        match &self.items[curr] {
            FsItem::File(_, _) => {}, // error
            FsItem::Directory(name, items) => {
                let mut tmpitems = items.clone();
                tmpitems.push(idx);
                self.items[curr] = FsItem::Directory(name.clone(), tmpitems)
            }
        }
    }
    fn mkdir(&mut self, name: &str) {
        if name == "/" {
            return;
        }
        self.additem(FsItem::Directory(name.to_owned(), Vec::new()));
    }
    fn touch(&mut self, name: &str, size: usize) {
        if name == "/" {
            return;
        }
        self.additem(FsItem::File(name.to_owned(), size));
    }
    fn current(&self) -> usize {
        match self.stack.last() {
            Some(x) => *x,
            None => 0
        }
    }
    fn ls(&self) -> Vec<usize> {
        match &self.items[self.current()] {
            FsItem::File(_, _) => Vec::new(),
            FsItem::Directory(_, items) => items.clone()
        }
    }
    fn isdir(&self, idx: usize) -> bool {
        match &self.items[idx] {
            FsItem::Directory(_, _) => true,
            _ => false,
        }
    }
    fn sizeof(&self, idx: usize) -> usize {
        match &self.items[idx] {
            FsItem::File(_, size) => *size,
            FsItem::Directory(_, children) => 
                children.iter().map(|x| self.sizeof(*x)).sum()
        }
    }
    fn new() -> Fs {
        Fs { 
            stack: Vec::new(), 
            items: vec!(FsItem::Directory("/".to_owned(), Vec::new())) 
        }
    }
    fn init(&mut self, lines: &Vec<Line>) {
        for line in lines {
            match line {
                Line::Ls => {},
                Line::File(name, size) => self.touch(name, *size),
                Line::Directory(name) => self.mkdir(name),
                Line::Cd(name) => self.cd(name)
            }
        }
        self.cd("/");
    }
    fn print(&self) {
        self.print_node(0, 0);
    }
    fn print_node(&self, idx: usize, indent: usize) {
        match &self.items[idx] {
            FsItem::File(name, size) => {
                println!("{:0$} {name} {size}", indent);
            }
            FsItem::Directory(name, children) => {
                println!("{:0$} {name}", indent);
                for child in children {
                    self.print_node(*child, indent + 2);
                }
            }
        }
    }
}


fn read_day7_input() -> AocResult<Fs> {
    let lines = read_data_lines("day7.txt")?;
    let program = lines.iter().map(|x| x.parse::<Line>()).collect::<AocResult<Vec<_>>>()?;
    let mut ret = Fs::new();
    ret.init(&program);
    Ok(ret)
}

pub fn day7() {
    let fs = read_day7_input().unwrap();
    let required = 30000000;
    let total_available = 70000000;
    let total_consumed = fs.sizeof(0);
    let size_of_smallish: usize = (0..fs.items.len())
        .filter(|x| fs.isdir(*x))
        .map(|x| fs.sizeof(x))
        .filter(|x| *x <= 100000)
        .sum();
    let mut candidates = (0..fs.items.len())
        .filter(|x| fs.isdir(*x))
        .map(|x| fs.sizeof(x))
        .filter(|x| (total_available - (total_consumed - x)) >= required)
        .collect::<Vec<_>>();
    candidates.sort();
    println!("disk size: {}", total_available);
    println!("required: {}", required);
    println!("total consumed: {}", total_consumed);
    println!("Total size of smallish: {}", size_of_smallish);
    println!("Smallest candidate: {}", candidates[0]);
}