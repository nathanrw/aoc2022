
use std::io;
use std::io::BufRead;
use std::fs::File;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::Read;
//use std::str::FromStr;

pub fn read_data_lines(filename: &str) -> io::Result<Vec<String>> {
    let path = "./inputs/".to_owned() + filename;
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect::<Result<_, _>>()
}

pub fn read_data_text(filename: &str) -> io::Result<String> {
    let path = "./inputs/".to_owned() + filename;
    let file = File::open(path)?;
    let mut buf: String = "".to_string();
    io::BufReader::new(file).read_to_string(&mut buf)?;
    Ok(buf)
}

//pub fn read_data_records<T: FromStr, E = io::Error>(filename: &str) -> Result<Vec<T>, E>
//where E: From<T::Err> + From<io::Error> + Into<io::Error>
//{
//    let lines = read_data_lines(filename)?;
//    lines.iter()
//        .map(|x| x.trim().to_owned())
//        .filter(|x| !x.is_empty())
//        .map(|x| x.parse::<T>() )
//        .collect::<Result<Vec<T>>, E>()
//}

#[derive(Debug)]
pub struct AocError {
    details: String
}

pub type AocResult<T> = Result<T, AocError>;

impl From<std::io::Error> for AocError {
    fn from(e: std::io::Error) -> Self {
        AocError { details: e.to_string() }
    }
}

impl From<std::num::ParseIntError> for AocError {
    fn from(e: std::num::ParseIntError) -> Self {
        AocError { details: e.to_string() }
    }
}

impl Display for AocError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AocError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl AocError {
    pub fn new(msg: &str) -> Self {
        Self{details: msg.to_string()}
    }
}