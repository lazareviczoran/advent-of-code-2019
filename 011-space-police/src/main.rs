use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Point {
    x: i128,
    y: i128,
    distance: i128,
}
impl Point {
    pub fn new(x: i128, y: i128) -> Point {
        Point { x, y, distance: 0 }
    }
}

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

    let mut visited_map: HashMap<(i128, i128), char> = HashMap::new();
    visited_map.insert((0, 0), '.');

    let painted_panels_number =
        calculate_painted_panels(&mut memory_map.clone(), &mut visited_map, 0);
    println!("Space Police part1 Solution: {:?}", painted_panels_number);

    visited_map = HashMap::new();
    visited_map.insert((0, 0), '#');
    calculate_painted_panels(&mut memory_map.clone(), &mut visited_map, 1);

    println!("Space Police part2 Solution:");
    print_registration(&mut visited_map);
}

fn print_registration(map: &mut HashMap<(i128, i128), char>) {
    let mut min_x = i128::max_value();
    let mut max_x = i128::min_value();
    let mut min_y = i128::max_value();
    let mut max_y = i128::min_value();
    for key in map.keys() {
        let (x, y) = key;
        if min_x > *x {
            min_x = *x;
        }
        if max_x < *x {
            max_x = *x;
        }
        if min_y > *y {
            min_y = *y;
        }
        if max_y < *y {
            max_y = *y;
        }
    }
    let mut sb = String::new();
    for j in min_y..=max_y {
        for i in min_x..=max_x {
            if let Some(field_color) = map.get(&(i, j)) {
                sb.push(*field_color);
            } else {
                sb.push('.');
            }
        }
        sb.push('\n');
    }
    println!("{}", sb)
}

fn calculate_painted_panels(
    memory: &mut HashMap<i128, i128>,
    visited: &mut HashMap<(i128, i128), char>,
    initial_input: i128,
) -> i128 {
    let mut step_x = 0;
    let mut step_y = -1;
    let mut current_pos = Point::new(0, 0);
    let mut input_val = initial_input;
    let mut inputs = Vec::new();

    inputs.push(input_val);
    let (mut output, mut op_pos, mut rel_pos, mut input_pos, mut op_code) =
        compute(memory, &inputs, 0, 0, 0);
    while op_code != 99 {
        let mut new_color = '.';
        if output[0] == 1 {
            new_color = '#';
        }
        visited.insert((current_pos.x, current_pos.y), new_color);
        let new_direction = output[1];
        if new_direction == 1 {
            if step_x != 0 {
                if step_x > 0 {
                    step_y = 1;
                } else {
                    step_y = -1;
                }
                step_x = 0;
            } else {
                if step_y > 0 {
                    step_x = -1;
                } else {
                    step_x = 1;
                }
                step_y = 0;
            }
        } else {
            if step_x != 0 {
                if step_x > 0 {
                    step_y = -1;
                } else {
                    step_y = 1;
                }
                step_x = 0;
            } else {
                if step_y > 0 {
                    step_x = 1;
                } else {
                    step_x = -1;
                }
                step_y = 0;
            }
        }

        current_pos.x += step_x;
        current_pos.y += step_y;
        input_val = 0;
        if let Some(new_field_color) = visited.get(&(current_pos.x, current_pos.y)) {
            if new_field_color == &'#' {
                input_val = 1;
            }
        } else {
            visited.insert((current_pos.x, current_pos.y), '.');
        }
        inputs.push(input_val);
        let (new_output, new_op_pos, new_rel_pos, new_input_pos, new_op_code) =
            compute(memory, &inputs, op_pos, rel_pos, input_pos);
        output = new_output;
        op_pos = new_op_pos;
        rel_pos = new_rel_pos;
        input_pos = new_input_pos;
        op_code = new_op_code;
    }

    visited.len() as i128
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
