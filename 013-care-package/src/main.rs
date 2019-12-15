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

    let mut visited_map: HashMap<(i128, i128), i128> = HashMap::new();

    let painted_panels_number = find_block_tiles_count(&mut memory_map.clone(), &mut visited_map);

    println!("Care Package part1 Solution: {:?}", painted_panels_number);

    visited_map = HashMap::new();
    let mut free_game_memory_map = memory_map.clone();
    free_game_memory_map.insert(0, 2);

    println!("Care Package part2 Solution:");
    run_game(&mut free_game_memory_map, &mut visited_map);
}

fn find_block_tiles_count(
    memory: &mut HashMap<i128, i128>,
    visited: &mut HashMap<(i128, i128), i128>,
) -> i128 {
    run_game(memory, visited);
    let mut count = 0;
    for value in visited.values() {
        if *value == 2 {
            count += 1;
        }
    }

    count
}

fn run_game(memory: &mut HashMap<i128, i128>, visited: &mut HashMap<(i128, i128), i128>) {
    let mut inputs = Vec::new();
    let (mut output, mut op_pos, mut rel_pos, mut input_pos, mut op_code) =
        compute(memory, &inputs, 0, 0, 0);
    let mut curr_ball_pos_x = 0;
    let mut curr_padle_pos_x = 0;
    while op_code != 99 {
        let mut i = 0;
        while i < output.len() {
            let new_x = output[i];
            let new_y = output[i + 1];
            let new_id = output[i + 2];
            if !(new_x == -1 && new_y == 0) {
                visited.insert((new_x, new_y), new_id);
                if new_id == 4 {
                    curr_ball_pos_x = new_x;
                } else if new_id == 3 {
                    curr_padle_pos_x = new_x;
                }
            }
            i += 3;
        }
        if curr_ball_pos_x < curr_padle_pos_x {
            inputs.push(-1);
        } else if curr_ball_pos_x > curr_padle_pos_x {
            inputs.push(1);
        } else {
            inputs.push(0);
        }
        let (new_output, new_op_pos, new_rel_pos, new_input_pos, new_op_code) =
            compute(memory, &inputs, op_pos, rel_pos, input_pos);
        output = new_output;
        op_pos = new_op_pos;
        rel_pos = new_rel_pos;
        input_pos = new_input_pos;
        op_code = new_op_code;
    }

    // find the latest output score
    let mut i = 0;
    while i < output.len() {
        let new_x = output[i];
        let new_y = output[i + 1];
        let new_id = output[i + 2];
        if new_x == -1 && new_y == 0 {
            println!("The final score is {}", new_id);
        }
        i += 3;
    }
}

fn compute(
    memory: &mut HashMap<i128, i128>,
    input: &Vec<i128>,
    op_position: i128,
    rel_position: i128,
    input_position: usize,
) -> (Vec<i128>, i128, i128, usize, i128) {
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
