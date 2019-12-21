use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
    distance: i64,
}
impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y, distance: 0 }
    }
}

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

    let mut visited_map: HashMap<(i64, i64), i64> = HashMap::new();

    let (fewest_movement_number, repaired_oxygen_system_location, moves_to_closest_repair_location) =
        find_shortest_path_to_oxygen(&mut memory_map.clone(), &mut visited_map);

    println!(
        "Oxygen System part1 Solution: {:?}, repaired oxygen system location: {:?}",
        fewest_movement_number, repaired_oxygen_system_location
    );

    visited_map = HashMap::new();
    println!(
        "Oxygen System part2 Solution: {}",
        find_oxygen_filling_time(
            &mut memory_map.clone(),
            &mut visited_map,
            moves_to_closest_repair_location,
            &repaired_oxygen_system_location
        )
    );
}

fn find_oxygen_filling_time(
    memory: &mut HashMap<i64, i64>,
    visited: &mut HashMap<(i64, i64), i64>,
    init_pos_commands: Vec<i64>,
    start_position: &Point,
) -> i64 {
    let mut max_distance: i64 = 0;
    let mut commands = HashMap::new();

    for cmd in init_pos_commands {
        compute(memory, &mut vec![cmd], 0, 0, 0);
    }

    let mut to_visit = vec![start_position.clone()];
    visited.insert((start_position.x, start_position.y), 2);
    commands.insert((start_position.x, start_position.y), Vec::new());

    while !to_visit.is_empty() {
        let prev_pos = to_visit.remove(0);
        for move_cmd in vec![1, 2, 3, 4] {
            let mut memory_clone = memory.clone();
            let all_commands = commands.get(&(prev_pos.x, prev_pos.y)).unwrap();
            let (mut curr_pos, _) = determine_next_pos_and_opposite_move(&prev_pos, move_cmd);

            if visited.get(&(curr_pos.x, curr_pos.y)).is_none() {
                let mut output_val = 0;
                let mut new_commands = all_commands.clone();
                new_commands.append(&mut vec![move_cmd]);
                for i in 0..new_commands.len() {
                    let (output, _, _, _, _) =
                        compute(&mut memory_clone, &mut vec![new_commands[i]], 0, 0, 0);
                    if i == new_commands.len() - 1 {
                        output_val = output[0];
                    }
                }

                if output_val != 0 {
                    commands.insert((curr_pos.x, curr_pos.y), new_commands);
                    curr_pos.distance = prev_pos.distance + 1;
                    if max_distance < curr_pos.distance {
                        max_distance = curr_pos.distance;
                    }
                    visited.insert((curr_pos.x, curr_pos.y), output_val);
                    to_visit.push(curr_pos);
                }
            }
        }
    }

    max_distance
}

fn find_shortest_path_to_oxygen(
    memory: &mut HashMap<i64, i64>,
    visited: &mut HashMap<(i64, i64), i64>,
) -> (i64, Point, Vec<i64>) {
    let mut min_distance: i64 = i64::max_value();
    let curr_point = Point::new(0, 0);
    let mut oxygen_location = Point::new(0, 0);
    let mut commands = Vec::new();
    let mut commands_acc = Vec::new();
    for move_cmd in vec![1, 2, 3, 4] {
        search_oxygen(
            &mut memory.clone(),
            &mut visited.clone(),
            &mut commands,
            &mut commands_acc,
            &curr_point,
            &mut oxygen_location,
            move_cmd,
            &mut min_distance,
            0,
        );
    }

    (min_distance, oxygen_location, commands)
}

fn search_oxygen(
    memory: &mut HashMap<i64, i64>,
    visited: &mut HashMap<(i64, i64), i64>,
    commands: &mut Vec<i64>,
    commands_acc: &mut Vec<i64>,
    prev_pos: &Point,
    oxygen_location: &mut Point,
    move_cmd: i64,
    min_dist: &mut i64,
    curr_dist: i64,
) {
    let (output, _, _, _, _) = compute(memory, &mut vec![move_cmd], 0, 0, 0);
    let (curr_pos, opposite_move) = determine_next_pos_and_opposite_move(&prev_pos, move_cmd);

    visited.insert((curr_pos.x, curr_pos.y), output[0]);

    if output[0] == 2 {
        if *min_dist > curr_dist + 1 {
            *min_dist = curr_dist + 1;
            *oxygen_location = curr_pos.clone();
            commands_acc.push(move_cmd);
            *commands = commands_acc.clone();
        }
    } else if output[0] == 0 {
        return;
    } else if output[0] == 1 {
        commands_acc.push(move_cmd);
        for next_move_cmd in vec![1, 2, 3, 4] {
            let (next_pos, _opposite_move) =
                determine_next_pos_and_opposite_move(&curr_pos, next_move_cmd);
            if visited.get(&(next_pos.x, next_pos.y)).is_some() {
                continue;
            }
            search_oxygen(
                memory,
                visited,
                commands,
                commands_acc,
                &curr_pos,
                oxygen_location,
                next_move_cmd,
                min_dist,
                curr_dist + 1,
            );
        }
    }

    visited.remove(&(curr_pos.x, curr_pos.y));
    commands_acc.remove(commands_acc.len() - 1);
    compute(memory, &mut vec![opposite_move], 0, 0, 0);
}

fn determine_next_pos_and_opposite_move(curr_pos: &Point, move_cmd: i64) -> (Point, i64) {
    let mut new_pos = curr_pos.clone();
    let opposite_move;
    match move_cmd {
        1 => {
            new_pos.y -= 1;
            opposite_move = 2;
        }
        2 => {
            new_pos.y += 1;
            opposite_move = 1;
        }
        3 => {
            new_pos.x -= 1;
            opposite_move = 4;
        }
        4 => {
            new_pos.x += 1;
            opposite_move = 3;
        }
        _ => panic!("Invalid move cmd {}", move_cmd),
    }
    (new_pos, opposite_move)
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
