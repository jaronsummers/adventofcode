use std::io;
use std::io::{BufRead, Write};

pub fn computer_standard_io(memory: Vec<i32>) -> i32 {
    let mut stdout = io::stdout();
    return computer(memory, &mut stdout);
}
// TODO: refactor this into a library
fn computer(mut memory: Vec<i32>, mut output: impl Write) -> i32 {
    let mut pos: i32 = 0;
    let mut settings = parse_mode(memory[pos as usize]);
    let mut buff1: i32;
    let mut buff2: i32;
    let mut buff3: i32;
    let mut buff4: i32;
    while settings.2 != 99 as i32 {

        match settings.2 {
            1 => {
                // ADD
                buff1 = match settings.0 {
                    0 => memory[memory[pos as usize + 1] as usize],
                    1 => memory[pos as usize + 1],
                    _ => std::process::exit(2)
                };
                buff2 = match settings.1 {
                    0 => memory[memory[pos as usize + 2] as usize],
                    1 => memory[pos as usize + 2],
                    _ => std::process::exit(3)
                };
                buff3 = buff1 + buff2;
                buff4 = memory[pos as usize +3];
                memory[buff4 as usize] = buff3;
                pos += 4;
            },
            // MUL
            2 => {
                buff1 = match settings.0 {
                    0 => memory[memory[pos as usize + 1] as usize],
                    1 => memory[pos as usize + 1],
                    _ => std::process::exit(2)
                };
                buff2 = match settings.1 {
                    0 => memory[memory[pos as usize + 2] as usize],
                    1 => memory[pos as usize + 2],
                    _ => std::process::exit(3)
                };
                buff3 = buff1 * buff2;
                buff4 = memory[pos as usize +3];
                memory[buff4 as usize] = buff3;
                pos += 4;
            },
            // INPUT
            3 => {
                #[cfg(not(test))]
                    let input = std::io::stdin();
                #[cfg(not(test))]
                    let mut input = input.lock();
                #[cfg(test)]
                    let mut input = tests::STDIN.lock().unwrap().take().unwrap();

                let mut line = String::new();
                input.read_line(&mut line).unwrap();
                let parsed_input = line.trim().parse::<i32>().ok().unwrap();
                buff4 = memory[pos as usize + 1];
                memory[buff4 as usize] = parsed_input;
                pos += 2;
            },
            // OUTPUT
            4 => {
                buff1 = match settings.0 {
                    0 => memory[memory[pos as usize + 1] as usize],
                    1 => memory[pos as usize + 1],
                    _ => std::process::exit(2)
                };
                // TODO: figure out how to use println still
                output.write_all(buff1.to_string().as_bytes());
                pos += 2;

            },
            5 => {
                // Jump if True
                buff1 = match settings.0 {
                    0 => memory[memory[pos as usize + 1] as usize],
                    1 => memory[pos as usize + 1],
                    _ => std::process::exit(3)
                };
                buff2 = match settings.1 {
                    0 => memory[memory[pos as usize + 2] as usize],
                    1 => memory[pos as usize + 2],
                    _ => std::process::exit(3)
                };
                pos = match buff1 {
                    0 => pos + 3,
                    _ => buff2

                };
            },
            6 => {
                // Jump if False
                buff1 = match settings.0 {
                    0 => memory[memory[pos as usize + 1] as usize],
                    1 => memory[pos as usize + 1],
                    _ => std::process::exit(2)
                };
                buff2 = match settings.1 {
                    0 => memory[memory[pos as usize + 2] as usize],
                    1 => memory[pos as usize + 2],
                    _ => std::process::exit(3)
                };
                pos = match buff1 {
                    0 => buff2,
                    _ => pos + 3

                };
            },
            7 => {
                // LT
                buff1 = match settings.0 {
                    0 => memory[memory[pos as usize + 1] as usize],
                    1 => memory[pos as usize + 1],
                    _ => std::process::exit(2)
                };
                buff2 = match settings.1 {
                    0 => memory[memory[pos as usize + 2] as usize],
                    1 => memory[pos as usize + 2],
                    _ => std::process::exit(3)
                };
                buff4 = memory[pos as usize +3];
                match buff1 < buff2 {
                    true => memory[buff4 as usize] = 1,
                    false => memory[buff4 as usize] = 0
                };
                pos += 4;
            },
            8 => {
                // EQ
                buff1 = match settings.0 {
                    0 => memory[memory[pos as usize + 1] as usize],
                    1 => memory[pos as usize + 1],
                    _ => std::process::exit(2)
                };
                buff2 = match settings.1 {
                    0 => memory[memory[pos as usize + 2] as usize],
                    1 => memory[pos as usize + 2],
                    _ => std::process::exit(3)
                };
                buff4 = memory[pos as usize +3];
                match buff1 == buff2 {
                    true => memory[buff4 as usize] = 1,
                    false => memory[buff4 as usize] = 0
                };
                pos += 4;
            }

            _ => panic!(format!("Tried to call opcode {}", settings.2))
        };
        settings = parse_mode(memory[pos as usize]);
    }
    return memory[0];
}

pub fn parse_mode(mode: i32) -> (i32, i32, i32){
    let r1_mode = mode / 100 % 10;
    let r2_mode = mode / 1000 % 10;
    let opcode = mode % 100;
    return (r1_mode, r2_mode, opcode);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::io::Cursor;

    lazy_static! {
        static ref GUARD: Mutex<()> = Mutex::new(());
        pub static ref STDIN: Mutex<Option<Cursor<Vec<u8>>>> = Mutex::new(None);
        pub static ref STDOUT: Mutex<Option<Cursor<Vec<u8>>>> = Mutex::new(None);
    }

    #[test]
    fn test_valid_computer_input() {
        let input = vec![1002,4,3,4,33];
        let mut output_buffer = Vec::new();
        let result = computer(input, &mut output_buffer);
        assert_eq!(result, 1002);
    }

    #[test]
    fn test_io() {
        let _guard = GUARD.lock().unwrap();
        let memory = vec![3,5,4,5,99,0];
        let input = Cursor::new(b"5\n".to_vec());
        *STDIN.lock().unwrap() = Some(input);
        let mut output_buffer = Vec::new();
        computer(memory, &mut output_buffer);
        let b = &output_buffer[0..1];
        let s = std::str::from_utf8(b).unwrap();
        assert_eq!(s.parse::<i32>().unwrap(), 5 as i32);
    }
    #[test]
    fn test_eq_pos_false() {
        let _guard = GUARD.lock().unwrap();
        let memory = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let input = Cursor::new(b"5\n".to_vec());
        *STDIN.lock().unwrap() = Some(input);
        let mut output_buffer = Vec::new();
        computer(memory, &mut output_buffer);
        let b = &output_buffer[0..1];
        let s = std::str::from_utf8(b).unwrap();
        assert_eq!(s.parse::<i32>().unwrap(), 0 as i32);
    }

    #[test]
    fn test_eq_pos_true() {
        let _guard = GUARD.lock().unwrap();
        let memory = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let input = Cursor::new(b"8\n".to_vec());
        *STDIN.lock().unwrap() = Some(input);
        let mut output_buffer = Vec::new();
        computer(memory, &mut output_buffer);
        let b = &output_buffer[0..1];
        let s = std::str::from_utf8(b).unwrap();
        assert_eq!(s.parse::<i32>().unwrap(), 1 as i32);
    }

    #[test]
    fn test_lt() {
        let _guard = GUARD.lock().unwrap();
        let memory = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let input = Cursor::new(b"5\n".to_vec());
        *STDIN.lock().unwrap() = Some(input);
        let mut output_buffer = Vec::new();
        computer(memory, &mut output_buffer);
        let b = &output_buffer[0..1];
        let s = std::str::from_utf8(b).unwrap();
        assert_eq!(s.parse::<i32>().unwrap(), 1 as i32);

    }
    #[test]
    fn test_eq_imm() {
        let _guard = GUARD.lock().unwrap();
        let memory = vec![3,3,1108,-1,8,3,4,3,99];
        let input = Cursor::new(b"5\n".to_vec());
        *STDIN.lock().unwrap() = Some(input);
        let mut output_buffer = Vec::new();
        computer(memory, &mut output_buffer);
        let b = &output_buffer[0..1];
        let s = std::str::from_utf8(b).unwrap();
        assert_eq!(s.parse::<i32>().unwrap(), 0 as i32);

    }
    #[test]
    fn test_lt_imm() {
        let _guard = GUARD.lock().unwrap();
        let memory = vec![3,3,1107,-1,8,3,4,3,99];
        let input = Cursor::new(b"5\n".to_vec());
        *STDIN.lock().unwrap() = Some(input);
        let mut output_buffer = Vec::new();
        computer(memory, &mut output_buffer);
        let b = &output_buffer[0..1];
        let s = std::str::from_utf8(b).unwrap();
        assert_eq!(s.parse::<i32>().unwrap(), 1 as i32);
    }
    #[test]
    fn test_jump_pos_go() {
        let _guard = GUARD.lock().unwrap();
        let memory = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let input = Cursor::new(b"5\n".to_vec());
        *STDIN.lock().unwrap() = Some(input);
        let mut output_buffer = Vec::new();
        computer(memory, &mut output_buffer);
        let b = &output_buffer[0..1];
        let s = std::str::from_utf8(b).unwrap();
        assert_eq!(s.parse::<i32>().unwrap(), 1 as i32);
    }
    #[test]
    fn test_jump_pos_stay() {
        let _guard = GUARD.lock().unwrap();
        let memory = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let input = Cursor::new(b"0\n".to_vec());
        *STDIN.lock().unwrap() = Some(input);
        let mut output_buffer = Vec::new();
        computer(memory, &mut output_buffer);
        let b = &output_buffer[0..1];
        let s = std::str::from_utf8(b).unwrap();
        assert_eq!(s.parse::<i32>().unwrap(), 0 as i32);
    }
    #[test]
    fn test_jump_imm() {
        let _guard = GUARD.lock().unwrap();
        let memory = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let input = Cursor::new(b"5\n".to_vec());
        *STDIN.lock().unwrap() = Some(input);
        let mut output_buffer = Vec::new();
        computer(memory, &mut output_buffer);
        let b = &output_buffer[0..1];
        let s = std::str::from_utf8(b).unwrap();
        assert_eq!(s.parse::<i32>().unwrap(), 1 as i32);
    }
}