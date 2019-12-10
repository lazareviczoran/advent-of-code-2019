use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let mut i = 0;
    let mut memory_map: HashMap<i128, i128> = HashMap::new();
    for v in contents.split_terminator(',') {
        let val = v.parse::<i128>().unwrap();
        memory_map.insert(i, val);
        i += 1;
    }

    let output = compute_boost_key_code(&mut memory_map.clone());
    println!("Sensor Boost part1 Solution: {:?}", output);

    let distress_signal_coordinates = compute_boost_distress_signal(&mut memory_map.clone());
    println!(
        "Sensor Boost part2 Solution: {:?}",
        distress_signal_coordinates
    );
}

fn compute_boost_key_code(memory: &mut HashMap<i128, i128>) -> i128 {
    let (output, _, _, _) = compute(memory, vec![1], 0, 0, 0);

    output[0]
}

fn compute_boost_distress_signal(memory: &mut HashMap<i128, i128>) -> i128 {
    let (output, _, _, _) = compute(memory, vec![2], 0, 0, 0);

    output[0]
}

fn compute(
    memory: &mut HashMap<i128, i128>,
    input: Vec<i128>,
    op_position: i128,
    rel_position: i128,
    input_position: usize,
) -> (Vec<i128>, i128, i128, usize) {
    let mut output = Vec::new();
    let mut op_pos = op_position;
    let mut rel_base = rel_position;
    let mut input_pos = input_position;
    while op_pos != 99 {
        let (op_code, param_modes) = extract_op_code_and_param_modes(memory, op_pos);

        let move_by;
        match op_code {
            99 => break,
            1 => {
                let write_address =
                    get_write_address(memory, op_code, op_pos, rel_base, param_modes[2]);
                let args = get_argument_values(memory, op_pos, rel_base, param_modes);
                memory.insert(write_address, args[0] + args[1]);
                move_by = 4;
            }
            2 => {
                let write_address =
                    get_write_address(memory, op_code, op_pos, rel_base, param_modes[2]);
                let args = get_argument_values(memory, op_pos, rel_base, param_modes);
                memory.insert(write_address, args[0] * args[1]);
                move_by = 4;
            }
            3 => {
                let write_address =
                    get_write_address(memory, op_code, op_pos, rel_base, param_modes[0]);
                if input_pos == input.len() {
                    return (output, op_pos, rel_base, input_pos);
                }
                memory.insert(write_address, input[input_pos]);
                input_pos = input_pos + 1;
                move_by = 2;
            }
            4 => {
                let args = get_argument_values(memory, op_pos, rel_base, param_modes);
                output.push(args[0]);
                move_by = 2;
            }
            5 => {
                let args = get_argument_values(memory, op_pos, rel_base, param_modes);
                if args[0] > 0 {
                    op_pos = args[1];
                    continue;
                }
                move_by = 3;
            }
            6 => {
                let args = get_argument_values(memory, op_pos, rel_base, param_modes);
                if args[0] == 0 {
                    op_pos = args[1];
                    continue;
                }
                move_by = 3;
            }
            7 => {
                let write_address =
                    get_write_address(memory, op_code, op_pos, rel_base, param_modes[2]);
                let args = get_argument_values(memory, op_pos, rel_base, param_modes);
                if args[0] < args[1] {
                    memory.insert(write_address, 1);
                } else {
                    memory.insert(write_address, 0);
                }
                move_by = 4;
            }
            8 => {
                let write_address =
                    get_write_address(memory, op_code, op_pos, rel_base, param_modes[2]);
                let args = get_argument_values(memory, op_pos, rel_base, param_modes);
                if args[0] == args[1] {
                    memory.insert(write_address, 1);
                } else {
                    memory.insert(write_address, 0);
                }
                move_by = 4;
            }
            9 => {
                let args = get_argument_values(memory, op_pos, rel_base, param_modes);
                rel_base += args[0];
                move_by = 2;
            }
            _ => panic!("Something went wrong: {}", op_code),
        }
        op_pos = op_pos + move_by;
    }
    (output, -1, -1, usize::max_value())
}

fn get_value(memory: &mut HashMap<i128, i128>, key: i128) -> i128 {
    if let Some(value) = memory.get(&key) {
        return *value;
    } else {
        let value = 0;
        memory.insert(key, value);
        return value;
    }
}

fn get_argument_values(
    memory: &mut HashMap<i128, i128>,
    op_position: i128,
    rel_position: i128,
    param_modes: Vec<i128>,
) -> Vec<i128> {
    let mut args = Vec::new();
    for i in 0..param_modes.len() {
        match param_modes[i] {
            0 => {
                let pos = get_value(memory, op_position + (i as i128) + 1);
                args.push(get_value(memory, pos));
            }
            1 => {
                args.push(get_value(memory, op_position + (i as i128) + 1));
            }
            2 => {
                let pos = rel_position + get_value(memory, op_position + (i as i128) + 1);
                args.push(get_value(memory, pos));
            }
            _ => panic!("Unexpected param mode"),
        }
    }
    args
}

fn get_write_address(
    memory: &mut HashMap<i128, i128>,
    op_code: i128,
    op_position: i128,
    rel_position: i128,
    param_mode: i128,
) -> i128 {
    let addr;
    let mut offset = 3;
    if op_code == 3 {
        offset = 1;
    }
    match param_mode {
        0 => addr = get_value(memory, op_position + offset),
        2 => addr = rel_position + get_value(memory, op_position + offset),
        _ => panic!("Unexpected param mode"),
    }
    addr
}

fn extract_op_code_and_param_modes(
    memory: &mut HashMap<i128, i128>,
    pos: i128,
) -> (i128, Vec<i128>) {
    let val = get_value(memory, pos);
    let op_code = val % 100;
    let mut modes = Vec::new();
    let mut modes_digits = val / 100;
    let param_num;
    match op_code {
        1 | 2 | 7 | 8 => param_num = 3,
        5 | 6 => param_num = 2,
        3 | 4 | 9 => param_num = 1,
        99 => param_num = 0,
        _ => panic!("Invalid op code {}", op_code),
    }
    for _ in 0..param_num {
        modes.push(modes_digits % 10);
        modes_digits /= 10;
    }
    (op_code, modes)
}

#[cfg(test)]
mod test {
    use super::compute;
    use super::compute_boost_key_code;
    use std::collections::HashMap;

    #[test]
    fn part1_sample_input1() {
        let mut memory_map: HashMap<i128, i128> = HashMap::new();
        let mut i = 0;
        for v in vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ] {
            memory_map.insert(i, v);
            i += 1;
        }
        let (output, _, _, _) = compute(&mut memory_map, vec![], 0, 0, 0);
        assert_eq!(
            output,
            [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn part1_sample_input2() {
        let mut memory_map: HashMap<i128, i128> = HashMap::new();
        let mut i = 0;
        for v in vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0] {
            memory_map.insert(i, v);
            i += 1;
        }
        let (output, _, _, _) = compute(&mut memory_map, vec![], 0, 0, 0);
        println!("{:?}", output);
        let mut number = output[0];
        let mut length = 0;
        while number > 0 {
            length += 1;
            number = number / 10;
        }
        assert_eq!(length, 16);
    }

    #[test]
    fn part1_sample_input3() {
        let mut memory_map: HashMap<i128, i128> = HashMap::new();
        let mut i = 0;
        for v in vec![104, 1125899906842624, 99] {
            memory_map.insert(i, v);
            i += 1;
        }
        let boost_key_code = compute_boost_key_code(&mut memory_map);
        assert_eq!(boost_key_code, 1125899906842624);
    }
}
