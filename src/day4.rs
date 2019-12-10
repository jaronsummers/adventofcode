use counter::Counter;

pub fn read_input(input: &str) -> (u32, u32) {
    let v: Vec<u32> = input.split("-").filter_map(|x| x.parse::<u32>().ok()).collect();
    return (v[0], v[1]);
}

pub fn star7(input: (u32, u32)) -> u32{
    let mut count: u32 = 0;
    for x in input.0..input.1 + 1 {
        let mut adjacency = false;
        let mut ascending = true;
        let i = x.to_string().into_bytes();
        for c in 0..6 {
            if c != 5 {
                if i[c] == i[c +1] {
                    adjacency = true;
                }
                if i[c] > i[c + 1] {
                    ascending = false;
                }
            }
        }
        if adjacency && ascending {
            count += 1;
        }
    }
    return count;
}

pub fn star8(input: (u32, u32)) -> u32{
    let mut count: u32 = 0;
    for x in input.0..input.1 + 1 {
        let mut adjacency = false;
        let mut ascending = true;
        let i = x.to_string().into_bytes();
        let char_counts = i.iter().collect::<Counter<_>>();
        for c in 0..6 {
            if c != 5 {
                if i[c] > i[c + 1] {
                    ascending = false;
                }
            }
        }
        for d in char_counts.iter() {
            if *d.1 == 2 as usize {
                adjacency = true;
            }
        }
        if adjacency && ascending {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_star7_one() {
        let inputs = (111111, 111122);
        let result = star7(inputs);
        assert_eq!(result, 10);
    }
    #[test]
    fn test_star8_one() {
        let inputs = (111111, 111111);
        let result = star8(inputs);
        assert_eq!(result, 0);
    }
    #[test]
    fn test_star8_two() {
        let inputs = (111111, 111122);
        let result = star8(inputs);
        assert_eq!(result, 1);
    }
}