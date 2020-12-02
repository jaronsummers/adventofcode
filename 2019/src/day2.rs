use crate::get_reader_from_filename;
use std::io::BufRead;
extern crate random_integer;

use crate::computer;

pub fn read_input(filename: &str) -> Result<Vec<i32>, std::io::Error>{
    // probably a better behavior would be to return an object that can be iterated over as needed
    let mut data = String::new();
    let mut reader = get_reader_from_filename(filename);
    reader.read_line(&mut data).expect("Unable to read line");
    let vec: Vec<i32> = data.split(",").filter_map(|x| x.parse::<i32>().ok()).collect();
    return Ok(vec);
}

pub fn star3(mut memory: Vec<i32>) -> i32 {
    memory[1] = 12;
    memory[2] = 2;
    return computer::computer_standard_io(memory)
}

pub fn star4(mut memory: Vec<i32>, desired_output: i32) -> i32 {
    memory[1] = 0;
    memory[2] = 0;
    let mut memory_copy: Vec<i32>;
    let mut test_input1: i32 = 0;
    let mut test_input2: i32 = 0;
    let mut result: i32 = 0;
    while result != desired_output {
        memory_copy = memory.to_vec();
        // This is not actually solving the problem at all, but computers are fast
        test_input1 = random_integer::random_i32(0, 99);
        test_input2 = random_integer::random_i32(0, 99);
        memory_copy[1] = test_input1;
        memory_copy[2] = test_input2;

        result = computer::computer_standard_io(memory_copy);
        println!("{}", result);
    }
    return test_input1 * 100 + test_input2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn test_star3_one() {
        let inputs: Vec<i32> = "1,9,10,3,2,3,11,0,99,30,40,50,0,4,7,10".split(",").filter_map(|x| x.parse::<i32>().ok()).collect();
        let result = star3(inputs);
        assert_eq!(result, 100);
    }
    #[test]
    fn test_star4_problem_answer() {
        let cur = "day2.txt";

        let base = env::current_dir().unwrap();
        let config_path = base.join("inputs").join(&cur);
        let inputs = match read_input(config_path.to_str().unwrap()) {
            Ok(inputs) => inputs,
            Err(_) => std::process::exit(1)
        };
        let result = star4(inputs, 19690720);
        assert_eq!(result, 2552);
    }
}