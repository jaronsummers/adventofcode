// TODO: refactor this into a library
pub fn computer(mut memory: Vec<u32>, input1: u32, input2: u32) -> u32 {
    //Setting the computer to the desired state
    memory[1] = input1;
    memory[2] = input2;
    let mut pos: u32 = 0;
    let mut opcode: u32 =  memory[pos as usize];
    let mut buff1: u32;
    let mut buff2: u32;
    let mut buff3: u32;
    while opcode != 99 as u32 {
        buff1 = memory[pos as usize + 1];
        buff2 = memory[pos as usize + 2];
        buff3 = memory[pos as usize + 3];
        memory[buff3 as usize] = match opcode {
            1 => memory[buff1 as usize] + memory[buff2 as usize],
            2 => memory[buff1 as usize] * memory[buff2 as usize],
            _ => std::process::exit(1)
        };
        pos = pos + 4;
        opcode = memory[pos as usize];
    }
    return memory[0];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_computer_input() {
        let input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let result = computer(input, 9, 10);
        assert_eq!(result, 3500);
    }
}