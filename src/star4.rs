extern crate random_integer;

#[path = "computer.rs"]
mod computer;

pub fn do_the_thing(memory: Vec<u32>) -> (u32, u32) {
    let desired_output: u32 = 19690720;
    let mut memory_copy: Vec<u32>;
    let mut test_input1: u32 = 0;
    let mut test_input2: u32 = 0;
    let mut result: u32 = 0;
    while result != desired_output {
        memory_copy = memory.to_vec();
        // This is not actually solving the problem at all, but computers are fast
        test_input1 = random_integer::random_u32(0, 99);
        test_input2 = random_integer::random_u32(0, 99);
        result = computer::computer(memory_copy, test_input1, test_input2);
    }
    return (test_input1, test_input2)
}