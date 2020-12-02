use std::collections::HashSet;
use crate::get_reader_from_filename;
use std::io::BufRead;

#[derive(Clone)]
pub enum Direction {
    R,
    L,
    U,
    D
}

impl std::str::FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::R),
            "L" => Ok(Direction::L),
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            _ => Err(format!("xx{}xx", s))
        }
    }
}

pub fn read_input(filename: &str) -> Result<(Vec<(Direction, usize)>, Vec<(Direction, usize)>), std::io::Error> {
    let result: (Vec<(Direction, usize)>, Vec<(Direction, usize)>);
    let mut data = String::new();
    let mut data2 = String::new();
    let mut reader = get_reader_from_filename(filename);
    reader.read_line(&mut data).expect("Unable to read line");
    reader.read_line(&mut data2).expect("Unable to read line");
    result = (parse_wire_input(data.as_ref()), parse_wire_input(data2.as_ref()));
    return Ok(result);
}

pub fn parse_wire_input(data: &str) -> Vec<(Direction, usize)> {
    let vec: Vec<(Direction, usize)> = data.split(",").filter_map(
        |x|
            {
                let y: Direction = x[0..1].parse().unwrap();
                return Some((y, x[1..].trim().parse::<usize>().unwrap()))
            }
    ).collect();
    return vec.to_vec();
}

pub fn get_entered_coords(mut wire_path: Vec<(Direction, usize)>) -> HashSet<(i32, i32)> {
    let mut coords = HashSet::new();
    let mut pos = (0, 0);
    for x in wire_path.iter_mut() {
        match x.0 {
            Direction::R => {
                while x.1 != 0 {
                    pos.0 += 1;
                    x.1 -= 1;
                    coords.insert(pos.clone());
                }
            },
            Direction::L => {
                while x.1 != 0 {
                    pos.0 -= 1;
                    x.1 -= 1;
                    coords.insert(pos.clone());
                }
            },
            Direction::U =>  {
                while x.1 != 0 {
                    pos.1 += 1;
                    x.1 -= 1;
                    coords.insert(pos.clone());
                }
            },
            Direction::D =>  {
                while x.1 != 0 {
                    pos.1 -= 1;
                    x.1 -= 1;
                    coords.insert(pos.clone());
                }
            },
        }
    };
    return coords.to_owned();
}



pub fn count_steps(mut wire_path: Vec<(Direction, usize)>, target: &(i32, i32)) -> i32 {
    let mut pos = (0, 0);
    let mut steps = 0;
    for x in wire_path.iter_mut() {
        match x.0 {
            Direction::R => {
                while x.1 != 0 {
                    pos.0 += 1;
                    x.1 -= 1;
                    steps += 1;
                    if pos == *target {
                        return steps;
                    }
                }
            },
            Direction::L => {
                while x.1 != 0 {
                    pos.0 -= 1;
                    x.1 -= 1;
                    steps += 1;
                    if pos == *target {
                        return steps;
                    }
                }
            },
            Direction::U =>  {
                while x.1 != 0 {
                    pos.1 += 1;
                    x.1 -= 1;
                    steps += 1;
                    if pos == *target {
                        return steps;
                    }
                }
            },
            Direction::D =>  {
                while x.1 != 0 {
                    pos.1 -= 1;
                    x.1 -= 1;
                    steps += 1;
                    if pos == *target {
                        return steps;
                    }
                }
            },
        }
    };
    panic!("idk");
}

pub fn star5(wire_paths: (Vec<(Direction, usize)>, Vec<(Direction, usize)>)) -> u32 {
    let path_one = get_entered_coords(wire_paths.0);
    let path_two = get_entered_coords(wire_paths.1);
    let intersection: HashSet<_> = path_one.intersection(&path_two).collect();
    let mut nearest_match = std::i32::MAX;
    for pos in intersection.iter() {
        if pos.0.abs() + pos.1.abs() < nearest_match {
            nearest_match = pos.0.abs() + pos.1.abs();
        }
    }
    return nearest_match as u32;
}

pub fn star6(wire_paths: (Vec<(Direction, usize)>, Vec<(Direction, usize)>)) -> u32 {
    let path_one = get_entered_coords(wire_paths.0.to_vec());
    let path_two = get_entered_coords(wire_paths.1.to_vec());
    let intersection: HashSet<_> = path_one.intersection(&path_two).collect();
    let mut nearest_match = std::i32::MAX;
    let mut steps1;
    let mut steps2;
    for pos in intersection.iter() {
        // TODO: this is a crazy amount of vector cloning and retracing the same steps over and over again
        steps1 = count_steps(wire_paths.0.to_vec(), pos);
        steps2 = count_steps(wire_paths.1.to_vec(), pos);
        if steps1 + steps2 < nearest_match {
            nearest_match = steps1 + steps2;
        }
    }
    return nearest_match as u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_star6_one() {
        let input = (
            parse_wire_input("R8,U5,L5,D3"),
            parse_wire_input("U7,R6,D4,L4")
        );
        let result = star6(input);
        assert_eq!(result, 30);
    }
    #[test]
    fn test_star6_two() {
        let input = (
            parse_wire_input("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            parse_wire_input("U62,R66,U55,R34,D71,R55,D58,R83")
        );
        let result = star6(input);
        assert_eq!(result, 610);
    }
    #[test]
    fn test_star6_three() {
        let input = (
            parse_wire_input("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            parse_wire_input("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        );
        let result = star6(input);
        assert_eq!(result, 410);
    }
    #[test]
    fn test_star5_one() {
        let input = (
            parse_wire_input("R8,U5,L5,D3"),
            parse_wire_input("U7,R6,D4,L4")
        );
        let result = star5(input);
        assert_eq!(result, 6);
    }
    #[test]
    fn test_star5_two() {
        let input = (
            parse_wire_input("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            parse_wire_input("U62,R66,U55,R34,D71,R55,D58,R83")
        );
        let result = star5(input);
        assert_eq!(result, 159);
    }
    #[test]
    fn test_star5_three() {
        let input = (
            parse_wire_input("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            parse_wire_input("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        );
        let result = star5(input);
        assert_eq!(result, 135);
    }
}