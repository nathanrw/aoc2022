
use std::io;
use std::io::BufRead;
use std::fs::File;
//use std::str::FromStr;

pub fn read_data_lines(filename: &str) -> io::Result<Vec<String>> {
    let path = "./inputs/".to_owned() + filename;
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect::<Result<_, _>>()
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