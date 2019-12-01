pub fn do_the_thing(mut input: Vec<u32>) -> u32 {
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
    fn test_one() {
        let result: u32 = do_the_thing(vec![100]);
        assert_eq!(result, 39);
    }
    #[test]
    fn test_multiple() {
        let result: u32 = do_the_thing(vec![100, 50]);
        assert_eq!(result, 55);
    }
    #[test]
    fn test_small() {
        let result: u32 = do_the_thing(vec![8]);
        assert_eq!(result, 0);
    }
}