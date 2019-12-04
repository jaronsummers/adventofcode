use crate::get_reader_from_filename;
use std::io::BufRead;

pub fn read_input(filename: &str) -> Result<Vec<u32>, std::io::Error>{
    // probably a better behavior would be to return an object that can be iterated over as needed
    let mut data: Vec<u32> = Vec::new();
    let reader = get_reader_from_filename(filename);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().parse::<u32>().unwrap();
        data.push(line)
    }
    return Ok(data);
}

pub fn star1(mut input: Vec<u32>) -> u32 {
    let mut acc: u32 = 0;
    while let Some(x) = input.pop() {
        // integer division rounds down
        acc += (x / 3) - 2;
    }
    return acc;
}
pub fn star2(mut input: Vec<u32>) -> u32 {
    let mut acc: u32 = 0;
    while let Some(mut x) = input.pop() {
        let mut fuel= 0;
        // 8 is the largest mass of fuel that requires no additional fuel
        while x > 8 {
            // integer division rounds down
            x = (x / 3) - 2;
            fuel += x;
        }
        acc += fuel;
    }
    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_star1_one() {
        let result: u32 = star1(vec![100]);
        assert_eq!(result, 31);
    }
    #[test]
    fn test_star1_multiple() {
        let result: u32 = star1(vec![100, 50]);
        assert_eq!(result, 45);
    }
    #[test]
    fn test_star2_one() {
        let result: u32 = star2(vec![100]);
        assert_eq!(result, 39);
    }
    #[test]
    fn test_star3_multiple() {
        let result: u32 = star2(vec![100, 50]);
        assert_eq!(result, 55);
    }
    #[test]
    fn test_star4_small() {
        let result: u32 = star2(vec![8]);
        assert_eq!(result, 0);
    }
}