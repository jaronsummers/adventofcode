
// for future problems this should probably be generalized


pub fn do_the_thing(mut input: Vec<u32>) -> u32 {
    let mut acc: u32 = 0;
    while let Some(x) = input.pop() {
        // integer division rounds down
        acc += (x / 3) - 2;
    }
    return acc;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let result: u32 = do_the_thing(vec![100]);
        assert_eq!(result, 31);
    }
    #[test]
    fn test_multiple() {
        let result: u32 = do_the_thing(vec![100, 50]);
        assert_eq!(result, 45);
    }
}