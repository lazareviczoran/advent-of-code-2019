use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let memory: Vec<i32> = contents
        .split_terminator(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    let result_list = compute(&mut memory.clone(), 1);
    println!(
        "Sunny with a Chance of Asteroids part1 Solution: {:?}",
        result_list
    );

    let result_list = compute(&mut memory.clone(), 5);
    println!(
        "Sunny with a Chance of Asteroids part2 Solution: {:?}",
        result_list
    );
}

fn compute(memory: &mut Vec<i32>, input: i32) -> Vec<i32> {
    let mut output = Vec::new();
    let mut op_position = 0;
    while op_position < memory.len() {
        let (op_code, param_modes) = extract_op_code_and_param_modes(memory.to_vec(), op_position);

        let move_by;
        match op_code {
            99 => break,
            1 => {
                let store_index = memory[op_position + 3] as usize;
                let args = get_argument_values(memory.to_vec(), op_position, param_modes);
                memory[store_index] = args[0] + args[1];
                move_by = 4;
            }
            2 => {
                let store_index = memory[op_position + 3] as usize;
                let args = get_argument_values(memory.to_vec(), op_position, param_modes);
                memory[store_index] = args[0] * args[1];
                move_by = 4;
            }
            3 => {
                let store_index = memory[op_position + 1] as usize;
                memory[store_index] = input;
                move_by = 2;
            }
            4 => {
                let args = get_output_value(memory.to_vec(), op_position, param_modes);
                output.push(args[0]);
                move_by = 2;
            }
            5 => {
                let args = get_argument_values(memory.to_vec(), op_position, param_modes);
                if args[0] > 0 {
                    op_position = args[1] as usize;
                    continue;
                }
                move_by = 3;
            }
            6 => {
                let args = get_argument_values(memory.to_vec(), op_position, param_modes);
                if args[0] == 0 {
                    op_position = args[1] as usize;
                    continue;
                }
                move_by = 3;
            }
            7 => {
                let args = get_argument_values(memory.to_vec(), op_position, param_modes);
                let store_index = memory[op_position + 3] as usize;
                if args[0] < args[1] {
                    memory[store_index] = 1;
                } else {
                    memory[store_index] = 0;
                }
                move_by = 4;
            }
            8 => {
                let args = get_argument_values(memory.to_vec(), op_position, param_modes);
                let store_index = memory[op_position + 3] as usize;
                if args[0] == args[1] {
                    memory[store_index] = 1;
                } else {
                    memory[store_index] = 0;
                }
                move_by = 4;
            }
            _ => panic!("Something went wrong"),
        }
        op_position = op_position + move_by;
    }
    output
}

fn get_argument_values(memory: Vec<i32>, op_position: usize, param_modes: Vec<i32>) -> Vec<i32> {
    let mut args = Vec::new();
    for i in 0..param_modes.len() {
        if param_modes[i] > 0 {
            args.push(memory[op_position + i + 1]);
        } else {
            args.push(memory[memory[op_position + i + 1] as usize]);
        }
    }
    args
}

fn get_output_value(memory: Vec<i32>, op_position: usize, param_modes: Vec<i32>) -> Vec<i32> {
    let mut args = Vec::new();
    if param_modes[0] > 0 {
        args.push(memory[op_position + 1]);
    } else {
        args.push(memory[memory[op_position + 1] as usize]);
    }
    args
}

fn extract_op_code_and_param_modes(memory: Vec<i32>, pos: usize) -> (i32, Vec<i32>) {
    let val = memory[pos];
    let op_code = val % 100;
    let mut modes = vec![0; 2];
    let mut modes_digits = val / 100;
    let mut i = 0;
    while modes_digits > 0 {
        modes[i] = modes_digits % 10;
        modes_digits = modes_digits / 10;
        i = i + 1;
    }
    (op_code, modes)
}

#[cfg(test)]
mod test {
    use super::compute;

    #[test]
    fn part1_sample_input1() {
        assert_eq!(compute(&mut vec![3, 0, 4, 0, 99], 1), [1]);
    }

    #[test]
    fn part2_sample_input1() {
        let program = &mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        // input is less than 8
        assert_eq!(compute(&mut program.clone(), 7), [0]);
        // input is equal to 8
        assert_eq!(compute(&mut program.clone(), 8), [1]);
        // input other than 8
        assert_eq!(compute(&mut program.clone(), 9), [0]);
    }

    #[test]
    fn part2_sample_input2() {
        let program = &mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        // input is less than 8
        assert_eq!(compute(&mut program.clone(), 7), [1]);
        // input is equal to 8
        assert_eq!(compute(&mut program.clone(), 8), [0]);
        // input is greater than 8
        assert_eq!(compute(&mut program.clone(), 9), [0]);
    }

    #[test]
    fn part2_sample_input3() {
        let program = &mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        // input is less than 8
        assert_eq!(compute(&mut program.clone(), 7), [0]);
        // input is equal to 8
        assert_eq!(compute(&mut program.clone(), 8), [1]);
        // input is greater than 8
        assert_eq!(compute(&mut program.clone(), 9), [0]);
    }

    #[test]
    fn part2_sample_input4() {
        let program = &mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        // input is less than 8
        assert_eq!(compute(&mut program.clone(), 7), [1]);
        // input is equal to 8
        assert_eq!(compute(&mut program.clone(), 8), [0]);
        // input is greater than 8
        assert_eq!(compute(&mut program.clone(), 9), [0]);
    }

    #[test]
    fn part2_sample_input5() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        // input is zero
        assert_eq!(compute(&mut program.clone(), 0), [0]);
        // input is not zero
        assert_eq!(compute(&mut program.clone(), -3), [1]);
        assert_eq!(compute(&mut program.clone(), 10), [1]);
        assert_eq!(compute(&mut program.clone(), 1), [1]);
    }

    #[test]
    fn part2_sample_input6() {
        let program = &mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        // input is zero
        assert_eq!(compute(&mut program.clone(), 0), [0]);
        // input is not zero
        assert_eq!(compute(&mut program.clone(), 10), [1]);
    }

    #[test]
    fn part2_sample_input7() {
        let program = &mut vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        // input is less than 8
        assert_eq!(compute(&mut program.clone(), 1), [999]);
        // input is equal to 8
        assert_eq!(compute(&mut program.clone(), 8), [1000]);
        // input is greater than 8
        assert_eq!(compute(&mut program.clone(), 10), [1001]);
    }
}
