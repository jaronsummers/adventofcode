#[path = "computer.rs"]
mod computer;

pub fn do_the_thing(input: Vec<u32>) -> u32 {
    return computer::computer(input, 12, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::read_comma_input;
    #[test]
    fn test_problem_answer() {
        let cur = "star3.txt";

        let base = env::current_dir().unwrap();
        let config_path = base.join("inputs").join(&cur);
        let inputs = match read_comma_input(config_path.to_str().unwrap()) {
            Ok(inputs) => inputs,
            Err(_) => std::process::exit(1)
        };
        let result = do_the_thing(inputs);
        assert_eq!(result, 9706670);
    }
}