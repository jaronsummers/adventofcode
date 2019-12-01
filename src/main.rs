use crate::star2::do_the_thing;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

//mod star1;
mod star2;

pub fn read_input(filename: &str) -> Result<Vec<u32>, std::io::Error>{
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


fn main() {
    // TODO: turn this into a CLI where I can type "adventofcode.exe star1" to get the answer to star1
    // let args = env::args();
    let cur = "star1.txt";

    let base = env::current_dir().unwrap();
    let config_path = base.join("inputs").join(&cur);
    let inputs = match read_input(config_path.to_str().unwrap()) {
        Ok(inputs) => inputs,
        Err(_) => std::process::exit(1)
    };
    let result: u32 = do_the_thing(inputs);
    println!("{}", result);
}
