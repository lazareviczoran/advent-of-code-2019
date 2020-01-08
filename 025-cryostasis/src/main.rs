use regex::Regex;
use std::collections::{HashMap, HashSet};
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

    run_program(&mut memory_map.clone());
}

fn run_program(memory: &mut HashMap<i64, i64>) {
    let mut op_pos = 0;
    let mut rel_pos = 0;
    let mut input_pos = 0;
    let mut input = vec![];

    let (items, moves_to_psf) = explore_and_take_items(
        memory,
        &mut input,
        &mut op_pos,
        &mut rel_pos,
        &mut input_pos,
    );

    // move to pressure sensitive floor
    let mut output = String::new();
    for cmd in moves_to_psf {
        output = run_command(
            memory,
            &mut op_pos,
            &mut rel_pos,
            &mut input_pos,
            &mut input,
            cmd.as_str(),
        );
    }

    let items_count = items.len();
    let mut current_set = (1 << items_count) - 1;
    while output.contains("you are ejected back to the checkpoint") {
        current_set -= 1;
        let mut removed_items: HashSet<String> = HashSet::new();
        for i in 0..items_count {
            if !has_bit(current_set, i as u8) {
                let curr_item = items[i].clone();
                removed_items.insert(curr_item.clone());
                let mut drop_cmd = String::from("drop ");
                drop_cmd.push_str(&curr_item);
                run_command(
                    memory,
                    &mut op_pos,
                    &mut rel_pos,
                    &mut input_pos,
                    &mut input,
                    drop_cmd.as_str(),
                );
            }
        }
        output = run_command(
            memory,
            &mut op_pos,
            &mut rel_pos,
            &mut input_pos,
            &mut input,
            "west",
        );
        for item in removed_items {
            let mut take_cmd = String::from("take ");
            take_cmd.push_str(&item);
            run_command(
                memory,
                &mut op_pos,
                &mut rel_pos,
                &mut input_pos,
                &mut input,
                take_cmd.as_str(),
            );
        }
    }
    println!("{}", output);
}

fn has_bit(keys: i32, i: u8) -> bool {
    keys & (1 << i) == (1 << i)
}

fn explore_and_take_items(
    memory: &mut HashMap<i64, i64>,
    input: &mut Vec<i64>,
    op_pos: &mut i64,
    rel_pos: &mut i64,
    input_pos: &mut usize,
) -> (Vec<String>, Vec<String>) {
    let output = compute(memory, input, op_pos, rel_pos, input_pos);
    let message = convert_to_string(output);
    let place_regex = Regex::new(r"==\s(.*?)\s==").unwrap();
    let command_regex = Regex::new(r"-\s(.*)").unwrap();
    let mut made_moves = Vec::new();
    let mut moves_to_psf = Vec::new();
    let mut move_cmd;
    let mut place = place_regex.captures(&message).unwrap()[1].to_string();
    let mut visited_rooms: HashSet<String> = HashSet::new();
    let mut items: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    for next_move in command_regex.captures_iter(&message) {
        stack.push(String::from(next_move.get(1).unwrap().as_str()));
    }
    visited_rooms.insert(place.clone());
    while !stack.is_empty() {
        move_cmd = stack.pop().unwrap();
        made_moves.push(move_cmd.clone());
        let opposite_move = get_opposite_move(move_cmd.clone());

        let curr_message =
            run_command(memory, op_pos, rel_pos, input_pos, input, move_cmd.as_str());
        let caps = place_regex.captures(&curr_message).unwrap();
        place = caps[1].to_string();

        if place == "Pressure-Sensitive Floor" {
            moves_to_psf = made_moves.clone();
            continue;
        }
        if visited_rooms.get(&place).is_some() {
            continue;
        }

        visited_rooms.insert(place.clone());
        let mut next_moves = Vec::new();
        for next_move in command_regex.captures_iter(&curr_message.as_str()) {
            let next_move_str = String::from(next_move.get(1).unwrap().as_str());
            if is_valid_move(next_move_str.clone()) {
                if opposite_move != next_move_str {
                    next_moves.push(next_move_str);
                }
            } else {
                if is_takeable_item(next_move_str.clone()) {
                    items.push(next_move_str.clone());
                    let mut take_cmd = String::from("take ");
                    take_cmd.push_str(&next_move_str);
                    run_command(memory, op_pos, rel_pos, input_pos, input, &take_cmd);
                }
            }
        }
        stack.push(opposite_move.clone());
        if !next_moves.is_empty() {
            stack.append(&mut next_moves);
        }
    }

    (items, moves_to_psf)
}

fn is_valid_move(move_cmd: String) -> bool {
    move_cmd == "north" || move_cmd == "south" || move_cmd == "west" || move_cmd == "east"
}

fn is_takeable_item(item: String) -> bool {
    item != "infinite loop"
        && item != "molten lava"
        && item != "escape pod"
        && item != "photons"
        && item != "giant electromagnet"
}

fn get_opposite_move(move_cmd: String) -> String {
    return match move_cmd.as_ref() {
        "north" => String::from("south"),
        "south" => String::from("north"),
        "west" => String::from("east"),
        "east" => String::from("west"),
        _ => panic!("unexpected move {}", move_cmd),
    };
}

fn run_command(
    memory: &mut HashMap<i64, i64>,
    op_pos: &mut i64,
    rel_pos: &mut i64,
    input_pos: &mut usize,
    input: &mut Vec<i64>,
    cmd: &str,
) -> String {
    input.append(&mut convert_to_int_arr(cmd.to_string()));
    let output = compute(memory, input, op_pos, rel_pos, input_pos);

    convert_to_string(output)
}

fn convert_to_string(array: Vec<i64>) -> String {
    let mut res = String::new();
    for el in array {
        res.push((el as u8) as char);
    }

    res
}

fn convert_to_int_arr(string: String) -> Vec<i64> {
    let mut chars = string.chars();
    let mut res = Vec::new();
    while let Some(ch) = chars.next() {
        res.push(ch as i64);
    }
    res.push('\n' as i64);

    res
}

fn compute(
    memory: &mut HashMap<i64, i64>,
    input: &mut Vec<i64>,
    op_pos: &mut i64,
    rel_pos: &mut i64,
    input_pos: &mut usize,
) -> Vec<i64> {
    let mut output = Vec::new();
    loop {
        let (op_code, param_modes) = extract_op_code_and_param_modes(memory, *op_pos);

        let move_by;
        match op_code {
            99 => break,
            1 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[2]);
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                memory.insert(write_address, args[0] + args[1]);
                move_by = 4;
            }
            2 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[2]);
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                memory.insert(write_address, args[0] * args[1]);
                move_by = 4;
            }
            3 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[0]);
                if *input_pos == input.len() {
                    return output;
                }
                memory.insert(write_address, input[*input_pos]);
                *input_pos = *input_pos + 1;
                move_by = 2;
            }
            4 => {
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                output.push(args[0]);
                move_by = 2;
            }
            5 => {
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                if args[0] > 0 {
                    *op_pos = args[1];
                    continue;
                }
                move_by = 3;
            }
            6 => {
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                if args[0] == 0 {
                    *op_pos = args[1];
                    continue;
                }
                move_by = 3;
            }
            7 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[2]);
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                if args[0] < args[1] {
                    memory.insert(write_address, 1);
                } else {
                    memory.insert(write_address, 0);
                }
                move_by = 4;
            }
            8 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[2]);
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                if args[0] == args[1] {
                    memory.insert(write_address, 1);
                } else {
                    memory.insert(write_address, 0);
                }
                move_by = 4;
            }
            9 => {
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                *rel_pos += args[0];
                move_by = 2;
            }
            _ => panic!("Something went wrong: {}", op_code),
        }
        *op_pos = *op_pos + move_by;
    }
    output
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
