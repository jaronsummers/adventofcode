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