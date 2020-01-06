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

    println!(
        "Category Six part1 Solution: {}",
        find_y_sent_to_255(&mut memory_map.clone())
    );

    println!(
        "Category Six part2 Solution: {}",
        find_first_duplicate_from_nat(&mut memory_map)
    );
}

#[derive(Clone, Debug)]
struct Computer {
    memory: HashMap<i64, i64>,
    op_pos: i64,
    rel_pos: i64,
    input_pos: usize,
    input: Vec<i64>,
}
impl Computer {
    pub fn new(memory: HashMap<i64, i64>, i: i64) -> Computer {
        Computer {
            memory: memory,
            op_pos: 0,
            rel_pos: 0,
            input_pos: 0,
            input: vec![i],
        }
    }
}

fn find_first_duplicate_from_nat(memory: &mut HashMap<i64, i64>) -> i64 {
    let mut computers = Vec::new();
    let mut network_queue: Vec<Vec<(i64, i64)>> = Vec::new();
    for i in 0..50 {
        let mut comp = Computer::new(memory.clone(), i as i64);
        compute(&mut comp);

        computers.push(comp);
        network_queue.push(Vec::new());
    }
    let mut nat: (i64, i64) = (-10, -10);
    let mut nat_y_values: Vec<i64> = Vec::new();

    loop {
        for i in 0..50 {
            if network_queue[i].len() > 0 {
                while !network_queue[i].is_empty() {
                    let (x, y) = network_queue[i].remove(0);
                    computers[i].input.push(x);
                    computers[i].input.push(y);
                }
            } else {
                computers[i].input.push(-1);
            }
            read_package(&mut computers[i], &mut nat, &mut network_queue);
        }
        let mut is_idle = true;
        for i in 0..50 {
            is_idle = is_idle && network_queue[i].is_empty();
        }
        if is_idle {
            let (x, y) = nat;
            if x == -10 {
                panic!("nat was not set");
            }
            computers[0].input.push(x);
            computers[0].input.push(y);
            if !nat_y_values.is_empty() && nat_y_values[0] == y {
                return y;
            }
            nat_y_values.insert(0, y);
            read_package(&mut computers[0], &mut nat, &mut network_queue);
        }
    }
}

fn read_package(
    computer: &mut Computer,
    nat: &mut (i64, i64),
    network_queue: &mut Vec<Vec<(i64, i64)>>,
) {
    let mut output = compute(computer);
    while !output.is_empty() {
        let send_to = output.remove(0);
        let x = output.remove(0);
        let y = output.remove(0);
        if send_to == 255 {
            *nat = (x, y);
        } else {
            network_queue[send_to as usize].push((x, y));
        }
    }
}

fn find_y_sent_to_255(memory: &mut HashMap<i64, i64>) -> i64 {
    let mut computers = Vec::new();
    let mut network_queue: Vec<Vec<(i64, i64)>> = Vec::new();
    for i in 0..50 {
        let mut comp = Computer::new(memory.clone(), i as i64);
        compute(&mut comp);

        computers.push(comp);
        network_queue.push(Vec::new());
    }

    loop {
        for i in 0..50 {
            if network_queue[i].len() > 0 {
                while !network_queue[i].is_empty() {
                    let (x, y) = network_queue[i].remove(0);
                    computers[i].input.push(x);
                    computers[i].input.push(y);
                }
            } else {
                computers[i].input.push(-1);
            }
            let mut output = compute(&mut computers[i]);
            while !output.is_empty() {
                let send_to = output.remove(0);
                let x = output.remove(0);
                let y = output.remove(0);
                if send_to == 255 {
                    return y;
                }
                network_queue[send_to as usize].push((x, y));
            }
        }
    }
}

fn compute(computer: &mut Computer) -> Vec<i64> {
    let mut output = Vec::new();
    loop {
        let (op_code, param_modes) =
            extract_op_code_and_param_modes(&mut computer.memory, computer.op_pos);

        let move_by;
        match op_code {
            99 => break,
            1 => {
                let write_address = get_write_address(
                    &mut computer.memory,
                    op_code,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes[2],
                );
                let args = get_argument_values(
                    &mut computer.memory,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes,
                );
                computer.memory.insert(write_address, args[0] + args[1]);
                move_by = 4;
            }
            2 => {
                let write_address = get_write_address(
                    &mut computer.memory,
                    op_code,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes[2],
                );
                let args = get_argument_values(
                    &mut computer.memory,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes,
                );
                computer.memory.insert(write_address, args[0] * args[1]);
                move_by = 4;
            }
            3 => {
                let write_address = get_write_address(
                    &mut computer.memory,
                    op_code,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes[0],
                );
                if computer.input_pos == computer.input.len() {
                    return output;
                }
                computer
                    .memory
                    .insert(write_address, computer.input[computer.input_pos]);
                computer.input_pos = computer.input_pos + 1;
                move_by = 2;
            }
            4 => {
                let args = get_argument_values(
                    &mut computer.memory,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes,
                );
                output.push(args[0]);
                move_by = 2;
            }
            5 => {
                let args = get_argument_values(
                    &mut computer.memory,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes,
                );
                if args[0] > 0 {
                    computer.op_pos = args[1];
                    continue;
                }
                move_by = 3;
            }
            6 => {
                let args = get_argument_values(
                    &mut computer.memory,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes,
                );
                if args[0] == 0 {
                    computer.op_pos = args[1];
                    continue;
                }
                move_by = 3;
            }
            7 => {
                let write_address = get_write_address(
                    &mut computer.memory,
                    op_code,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes[2],
                );
                let args = get_argument_values(
                    &mut computer.memory,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes,
                );
                if args[0] < args[1] {
                    computer.memory.insert(write_address, 1);
                } else {
                    computer.memory.insert(write_address, 0);
                }
                move_by = 4;
            }
            8 => {
                let write_address = get_write_address(
                    &mut computer.memory,
                    op_code,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes[2],
                );
                let args = get_argument_values(
                    &mut computer.memory,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes,
                );
                if args[0] == args[1] {
                    computer.memory.insert(write_address, 1);
                } else {
                    computer.memory.insert(write_address, 0);
                }
                move_by = 4;
            }
            9 => {
                let args = get_argument_values(
                    &mut computer.memory,
                    computer.op_pos,
                    computer.rel_pos,
                    param_modes,
                );
                computer.rel_pos += args[0];
                move_by = 2;
            }
            _ => panic!("Something went wrong: {}", op_code),
        }
        computer.op_pos = computer.op_pos + move_by;
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
