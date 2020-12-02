#[cfg(test)] #[macro_use] extern crate lazy_static;

use std::io::BufReader;
use std::fs::File;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod computer;
// TODO: consider using a session cookie to retrieve the configs and write them out if they aren't present

pub fn get_reader_from_filename(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("file failed to open");
    let reader = BufReader::new(file);
    return reader;
}

fn help() {
    println!("Usage: adventofcode <year> <starnumber>, for example adventofcode 2019 3");
}

fn main() {
    // TODO: turn this into a CLI where I can type "adventofcode.exe 2019 star1" to get the answer to star1
    let args: Vec<String> = env::args().collect();
    let input_file_name: &str;
    let base = env::current_dir().unwrap();
    let year: i32;
    let star: i32;
    match args.len() {
        3 => {
            year = *match &args[1].parse::<i32>() {
                Ok(year) => year,
                Err(_) => std::process::exit(1)
            };
            star = *match &args[2].parse::<i32>() {
                Ok(year) => year,
                Err(_) => std::process::exit(1)
            };
        }
        _ => {
            help();
            return;
        }
    }
    // todo: parse input more carefully and raise errors
    match year {
        2019 => match star {
            1 => {
                use day1::{read_input, star1};
                input_file_name = "day1.txt";
                let config_path = base.join("inputs").join(&input_file_name);
                let inputs = read_input(config_path.to_str().unwrap()).unwrap();
                let result = star1(inputs);
                println!("{}", result);
            },
            2 => {
                use day1::{read_input, star2};
                input_file_name = "day1.txt";
                let config_path = base.join("inputs").join(&input_file_name);
                let memory = read_input(config_path.to_str().unwrap()).unwrap();
                let result = star2(memory);
                println!("{}", result);
            },
            3 => {
                    use day2::{read_input, star3};
                    input_file_name = "day2.txt";
                    let config_path = base.join("inputs").join(&input_file_name);
                    let inputs = read_input(config_path.to_str().unwrap()).unwrap();
                    let result = star3(inputs);
                    println!("{}", result);
            },
            4 => {
                use day2::{read_input, star4};
                input_file_name = "day2.txt";
                let desired_output: i32 = 19690720;
                let config_path = base.join("inputs").join(&input_file_name);
                let memory = read_input(config_path.to_str().unwrap()).unwrap();
                let result = star4(memory, desired_output);
                println!("{}", result);
            },
            5 => {
                use day3::{read_input, star5};
                input_file_name = "day3.txt";
                let config_path = base.join("inputs").join(&input_file_name);
                let inputs = read_input(config_path.to_str().unwrap()).unwrap();
                let result = star5(inputs);
                println!("{}", result);
            },
            6 => {
                use day3::{read_input, star6};
                input_file_name = "day3.txt";
                let config_path = base.join("inputs").join(&input_file_name);
                let inputs = read_input(config_path.to_str().unwrap()).unwrap();
                let result = star6(inputs);
                println!("{}", result);
            },
            7 => {
                use day4::{read_input, star7};
                let inputs = read_input("271973-785961");
                let result = star7(inputs);
                println!("{}", result);
            },
            8 => {
                use day4::{read_input, star8};
                let inputs = read_input("271973-785961");
                let result = star8(inputs);
                println!("{}", result);
            },
            9 => {
                use day2::read_input;
                use day5::star9;
                input_file_name = "day5.txt";
                let config_path = base.join("inputs").join(&input_file_name);
                let memory = read_input(config_path.to_str().unwrap()).unwrap();
                star9(memory);
            },
            10 => {
                use day2::read_input;
                use day5::star10;
                input_file_name = "day5.txt";
                let config_path = base.join("inputs").join(&input_file_name);
                let memory = read_input(config_path.to_str().unwrap()).unwrap();
                star10(memory);
            }
            _ => {
                help();
                return;
            }
        }
        _ => {
            help();
            return;
        }
    }
}
