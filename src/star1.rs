
// for future problems this should probably be generalized


pub fn do_the_thing(mut input: Vec<u32>) -> u32 {
    let mut acc: u32 = 0;
    while let Some(x) = input.pop() {
        // integer division rounds down
        acc += (x / 3) - 2;
    }
    return acc;
}


