use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let mut i = 0;
    let mut memory_map: HashMap<i64, i64> = HashMap::new();
    for v in contents.split_terminator(',') {
        let val = v.parse::<i64>().unwrap();
        memory_map.insert(i, val);
        i += 1;
    }

    let (output, affected_points_count) = calculate_tractor_beam_output(memory_map.clone());
    print_output(output);

    println!("Tractor Beam part1 Solution: {:?}", affected_points_count);

    println!(
        "Tractor Beam part2 Solution: {}",
        find_first_100x100_fit(memory_map)
    );
}

fn find_first_100x100_fit(memory: HashMap<i64, i64>) -> i64 {
    let mut i = 35;
    let mut j = 50;

    loop {
        let (finished, next_i, next_j) = can_fit_100x100(memory.clone(), i, j);
        if finished {
            break;
        }
        i = next_i;
        j = next_j;
    }

    i * 10000 + j
}

fn can_fit_100x100(memory: HashMap<i64, i64>, x: i64, y: i64) -> (bool, i64, i64) {
    let (output, _, _, _, _) = compute(&mut memory.clone(), &mut vec![x + 99, y], 0, 0, 0);
    if output[0] == 0 {
        let (output, _, _, _, _) = compute(&mut memory.clone(), &mut vec![x, y + 1], 0, 0, 0);
        if output[0] == 1 {
            return (false, x, y + 1);
        }
        return (false, x + 1, y + 1);
    }

    let (output, _, _, _, _) = compute(&mut memory.clone(), &mut vec![x, y + 99], 0, 0, 0);
    if output[0] == 0 {
        let (output, _, _, _, _) = compute(&mut memory.clone(), &mut vec![x + 1, y], 0, 0, 0);
        if output[0] == 1 {
            return (false, x + 1, y);
        }
        return (false, x + 1, y + 1);
    }
    return (true, x, y);
}

fn calculate_tractor_beam_output(memory: HashMap<i64, i64>) -> (Vec<Vec<char>>, i64) {
    let mut res = vec![vec!['.'; 50]; 50];
    let mut affected_points_count = 0;
    for j in 0..50 {
        for i in 0..50 {
            let (output, _, _, _, _) = compute(&mut memory.clone(), &mut vec![i, j], 0, 0, 0);
            let output_char = match output[0] {
                0 => '.',
                1 => {
                    affected_points_count += 1;
                    '#'
                }
                _ => panic!("unexpected value {}", output[0]),
            };
            res[i as usize][j as usize] = output_char;
        }
    }

    (res, affected_points_count)
}

fn print_output(output: Vec<Vec<char>>) {
    let mut sb = String::new();
    let h = output[0].len();
    let w = output.len();

    for j in 0..h {
        for i in 0..w {
            sb.push(output[i][j]);
        }
        sb.push('\n');
    }
    println!("{}", sb)
}

fn compute(
    memory: &mut HashMap<i64, i64>,
    input: &Vec<i64>,
    op_position: i64,
    rel_position: i64,
    input_position: usize,
) -> (Vec<i64>, i64, i64, usize, i64) {
    let mut output = Vec::new();
    let mut op_pos = op_position;
    let mut rel_base = rel_position;
    let mut input_pos = input_position;
    let mut operation_code;
    loop {
        let (op_code, param_modes) = extract_op_code_and_param_modes(memory, op_pos);
        operation_code = op_code;

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
                    return (output, op_pos, rel_base, input_pos, operation_code);
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
    (output, -1, -1, usize::max_value(), operation_code)
}

fn get_value(memory: &mut HashMap<i64, i64>, key: i64) -> i64 {
    if let Some(value) = memory.get(&key) {
        return *value;
    } else {
        let value = 0;
        memory.insert(key, value);
        return value;
    }
}

fn get_argument_values(
    memory: &mut HashMap<i64, i64>,
    op_position: i64,
    rel_position: i64,
    param_modes: Vec<i64>,
) -> Vec<i64> {
    let mut args = Vec::new();
    for i in 0..param_modes.len() {
        match param_modes[i] {
            0 => {
                let pos = get_value(memory, op_position + (i as i64) + 1);
                args.push(get_value(memory, pos));
            }
            1 => {
                args.push(get_value(memory, op_position + (i as i64) + 1));
            }
            2 => {
                let pos = rel_position + get_value(memory, op_position + (i as i64) + 1);
                args.push(get_value(memory, pos));
            }
            _ => panic!("Unexpected param mode"),
        }
    }
    args
}

fn get_write_address(
    memory: &mut HashMap<i64, i64>,
    op_code: i64,
    op_position: i64,
    rel_position: i64,
    param_mode: i64,
) -> i64 {
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

fn extract_op_code_and_param_modes(memory: &mut HashMap<i64, i64>, pos: i64) -> (i64, Vec<i64>) {
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
