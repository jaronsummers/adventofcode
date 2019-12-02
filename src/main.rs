use crate::star4::do_the_thing;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

mod star1;
mod star2;
mod star3;
mod star4;
mod computer;


pub fn read_input(filename: &str) -> Result<Vec<u32>, std::io::Error>{
    // TODO: consider using a session cookie to retrieve the configs and write them out if they aren't present
    // probably a better behavior would be to return an object that can be iterated over as needed
    let mut data: Vec<u32> = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().parse::<u32>().unwrap();
        data.push(line)
    }
    return Ok(data);
}

pub fn read_comma_input(filename: &str) -> Result<Vec<u32>, std::io::Error>{
    // TODO: consider using a session cookie to retrieve the configs and write them out if they aren't present
    // probably a better behavior would be to return an object that can be iterated over as needed
    // refactor so that input can be read in a more generalized way
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_line(&mut data).expect("Unable to read line");
    let vec: Vec<u32> = data.split(",").filter_map(|x| x.parse::<u32>().ok()).collect();
    return Ok(vec);
}

fn main() {
    // TODO: turn this into a CLI where I can type "adventofcode.exe 2019 star1" to get the answer to star1
    // let args = env::args();
    let cur = "star3.txt";

    let base = env::current_dir().unwrap();
    let config_path = base.join("inputs").join(&cur);
    let inputs = match read_comma_input(config_path.to_str().unwrap()) {
        Ok(inputs) => inputs,
        Err(_) => std::process::exit(1)
    };
    let (result1, result2) = do_the_thing(inputs);
    println!("{} + {}", result1, result2);
}
