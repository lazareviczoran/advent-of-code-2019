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
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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

    let (output, _, _, _, _) = compute(&mut memory_map.clone(), &mut vec![], 0, 0, 0);
    let mut camera_output = get_camera_output(output);
    let sum_of_aligment_params = calibrate_cameras(&mut camera_output);

    println!(
        "Set and Forget part1 Solution: {:?}",
        sum_of_aligment_params
    );

    memory_map.insert(0, 2);
    let collected_scaffolds = get_vacuum_robot_report(&mut memory_map);
    println!("Set and Forget part2 Solution: {}", collected_scaffolds);
}

fn get_vacuum_robot_report(memory: &mut HashMap<i64, i64>) -> i64 {
    let (output, mut op_pos, mut rel_pos, mut input_pos, _) = compute(memory, &mut vec![], 0, 0, 0);
    let mut camera_output = get_camera_output(output);
    print_output(camera_output.clone());

    let start_pos = locate_start_pos(&mut camera_output);
    let moves = get_movements(&mut camera_output);
    let prepared_moves = prepare_moves(
        moves,
        camera_output[start_pos.x as usize][start_pos.y as usize],
    );
    let (repeating_patterns, _) = find_repeting_patterns(prepared_moves.clone(), 3);

    let collected_scaffolds = enter_prompted_values(
        memory,
        prepared_moves,
        repeating_patterns,
        &mut op_pos,
        &mut rel_pos,
        &mut input_pos,
    );

    collected_scaffolds
}

fn enter_prompted_values(
    memory: &mut HashMap<i64, i64>,
    prepared_moves: String,
    patterns: Vec<String>,
    op_pos: &mut i64,
    rel_pos: &mut i64,
    input_pos: &mut usize,
) -> i64 {
    let mut input = Vec::new();
    let mut routines = prepared_moves.clone();
    let routine_names = ["A", "B", "C"];
    for i in 0..patterns.len() {
        routines = routines.replace(&patterns[i], &routine_names[i]);
    }

    println!("Entering: {}", routines);
    input.append(&mut convert_to_int_arr(routines));
    let (output, new_op_pos, new_rel_pos, new_input_pos, _) =
        compute(memory, &mut input, *op_pos, *rel_pos, *input_pos);
    *op_pos = new_op_pos;
    *rel_pos = new_rel_pos;
    *input_pos = new_input_pos;

    println!("{}", convert_to_string(output));

    for pattern in patterns {
        println!("Entering: {}", pattern);
        input.append(&mut convert_to_int_arr(pattern));
        let (output, new_op_pos, new_rel_pos, new_input_pos, _) =
            compute(memory, &mut input, *op_pos, *rel_pos, *input_pos);
        *op_pos = new_op_pos;
        *rel_pos = new_rel_pos;
        *input_pos = new_input_pos;
        println!("{}", convert_to_string(output));
    }

    input.append(&mut convert_to_int_arr("n".to_string()));
    println!("Entering: n");
    let (output, new_op_pos, new_rel_pos, new_input_pos, _) =
        compute(memory, &mut input, *op_pos, *rel_pos, *input_pos);
    *op_pos = new_op_pos;
    *rel_pos = new_rel_pos;
    *input_pos = new_input_pos;

    output[output.len() - 1]
}

fn find_repeting_patterns(moves: String, remaining_routines: i64) -> (Vec<String>, bool) {
    if remaining_routines == 0 {
        if moves.len() == 0 {
            return (Vec::new(), true);
        }
        return (Vec::new(), false);
    }

    let items: Vec<&str> = moves.split_terminator(',').collect();
    for i in (1..15).rev() {
        let mut chunk_items = items.clone();
        chunk_items.truncate(i);
        let mut chunk = chunk_items.join(&",");
        if chunk.len() > 20 {
            continue;
        }

        let (pattern, rest) = moves.split_at(chunk.len());
        if rest.contains(pattern) {
            let mut res = Vec::new();
            res.push(chunk.to_string());
            chunk.push(',');
            let mut new_moves = moves.replace(&chunk, "").replace(",,", ",");
            chunk.pop();
            new_moves = new_moves.replace(&chunk, "").replace(",,", ",");
            let (mut other_patterns, ok) =
                find_repeting_patterns(new_moves, remaining_routines - 1);
            if ok {
                res.append(&mut other_patterns);
                return (res, ok);
            }
        } else {
            chunk.push(',');
        }
    }
    return (Vec::new(), false);
}

fn prepare_moves(moves: Vec<char>, initial_dir: char) -> String {
    let mut i = 1;
    let mut steps = 0;
    let mut new_format_moves = String::new();

    if moves[0] != initial_dir {
        new_format_moves.push(determine_rotation_move(initial_dir, moves[0]));
        new_format_moves.push(',');
    }

    while i < moves.len() {
        steps += 1;
        if moves[i - 1] != moves[i] {
            if steps > 0 {
                new_format_moves.push_str(&steps.to_string());
                new_format_moves.push(',');
            }
            steps = 0;
            let change_dir_move = determine_rotation_move(moves[i - 1], moves[i]);
            new_format_moves.push(change_dir_move);
            if i < moves.len() {
                new_format_moves.push(',');
            }
        } else if i == moves.len() - 1 {
            steps += 1;
            new_format_moves.push_str(&steps.to_string());
        }

        i += 1;
    }

    new_format_moves
}

fn determine_rotation_move(prev_dir: char, new_dir: char) -> char {
    match prev_dir {
        '^' => {
            if new_dir == '<' {
                return 'L';
            } else {
                return 'R';
            }
        }
        'v' => {
            if new_dir == '<' {
                return 'R';
            } else {
                return 'L';
            }
        }
        '<' => {
            if new_dir == '^' {
                return 'R';
            } else {
                return 'L';
            }
        }
        '>' => {
            if new_dir == '^' {
                return 'L';
            } else {
                return 'R';
            }
        }
        _ => panic!("Invalid move {}", prev_dir),
    }
}

fn get_movements(camera_output: &mut Vec<Vec<char>>) -> Vec<char> {
    let mut moves = Vec::new();
    let mut curr_pos = locate_start_pos(camera_output);
    let mut visited: Vec<Vec<bool>> =
        vec![vec![false; camera_output[0].len()]; camera_output.len()];
    let mut intersections = Vec::new();
    let mut prev_pos = curr_pos.clone();
    let mut dir = camera_output[curr_pos.x as usize][curr_pos.y as usize];
    loop {
        if let Some(next_pos) = move_to_next(
            camera_output,
            &mut visited,
            &mut intersections,
            &mut dir,
            &prev_pos,
            &curr_pos,
        ) {
            prev_pos = curr_pos.clone();
            curr_pos = next_pos;
            moves.push(dir);
        } else {
            return moves;
        }
    }
}

fn get_camera_output(output: Vec<i64>) -> Vec<Vec<char>> {
    let mut output_string = String::new();
    let mut prev_value = output[0];
    for value in output {
        if (prev_value as u8 as char) == '\n' && (value as u8 as char) == '\n'
            || !['.', '#', '\n', '^', 'v', '<', '>'].contains(&(value as u8 as char))
        {
            continue;
        }
        output_string.push(value as u8 as char);
        prev_value = value;
    }
    let width = output_string.find('\n').unwrap();
    let rows: Vec<&str> = output_string.split_terminator('\n').collect();
    let height = rows.len();
    let mut camera_output = vec![vec!['0'; height]; width];
    let mut j = 0;
    let mut i;
    for row in rows {
        i = 0;
        let mut items = row.chars();
        while let Some(ch) = items.next() {
            if j >= height || i >= width {
                return camera_output;
            }
            camera_output[i][j] = ch;
            i += 1;
        }
        j += 1;
    }

    camera_output
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

fn calibrate_cameras(camera_output: &mut Vec<Vec<char>>) -> i64 {
    let mut curr_pos = locate_start_pos(camera_output);
    let mut visited: Vec<Vec<bool>> =
        vec![vec![false; camera_output[0].len()]; camera_output.len()];
    let mut intersections = Vec::new();
    let mut prev_pos = curr_pos.clone();
    let mut initial_dir = camera_output[curr_pos.x as usize][curr_pos.y as usize];
    loop {
        if let Some(next_pos) = move_to_next(
            camera_output,
            &mut visited,
            &mut intersections,
            &mut initial_dir,
            &prev_pos,
            &curr_pos,
        ) {
            prev_pos = curr_pos.clone();
            curr_pos = next_pos;
        } else {
            // calculate intersections
            let mut total = 0;
            for intersection in intersections {
                total += intersection.x * intersection.y;
            }
            return total;
        }
    }
}

fn move_to_next(
    camera_output: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    intersections: &mut Vec<Point>,
    dir: &mut char,
    prev_pos: &Point,
    curr_pos: &Point,
) -> Option<Point> {
    let h = camera_output[0].len() as i64;
    let w = camera_output.len() as i64;
    let (step_x, step_y) = determine_step(*dir);
    let new_pos = Point::new(curr_pos.x + step_x, curr_pos.y + step_y);
    if is_valid_pos(&new_pos, w, h)
        && new_pos != *prev_pos
        && camera_output[new_pos.x as usize][new_pos.y as usize] == '#'
    {
        let pos = new_pos.clone();
        if visited[pos.x as usize][pos.y as usize] {
            intersections.push(pos.clone());
        } else {
            visited[pos.x as usize][pos.y as usize] = true;
        }
        return Some(pos);
    } else {
        // find new direction
        let mut next_position = None;
        let potential_dirs;
        if *dir == '>' || *dir == '<' {
            potential_dirs = vec!['^', 'v'];
        } else {
            potential_dirs = vec!['>', '<'];
        }
        for new_dir in potential_dirs {
            let (new_step_x, new_step_y) = determine_step(new_dir);
            let mut pos = curr_pos.clone();
            pos.x += new_step_x;
            pos.y += new_step_y;
            if new_dir != *dir
                && is_valid_pos(&pos, w, h)
                && camera_output[pos.x as usize][pos.y as usize] == '#'
            {
                *dir = new_dir;
                visited[pos.x as usize][pos.y as usize] = true;
                next_position = Some(pos);
            }
        }
        return next_position;
    }
}

fn is_valid_pos(new_pos: &Point, w: i64, h: i64) -> bool {
    if new_pos.x < 0 || new_pos.x >= w {
        return false;
    }
    if new_pos.y < 0 || new_pos.y >= h {
        return false;
    }

    true
}

fn determine_step(dir: char) -> (i64, i64) {
    let mut step_x = 0;
    let mut step_y = 0;
    match dir {
        '>' => step_x = 1,
        '<' => step_x = -1,
        '^' => step_y = -1,
        'v' => step_y = 1,
        _ => panic!("Invalid direction {}", dir),
    }

    (step_x, step_y)
}

fn locate_start_pos(camera_output: &mut Vec<Vec<char>>) -> Point {
    for i in 0..camera_output.len() {
        for j in 0..camera_output[0].len() {
            let curr_char = camera_output[i][j];
            if curr_char == '>' || curr_char == '<' || curr_char == '^' || curr_char == 'v' {
                return Point::new(i as i64, j as i64);
            }
        }
    }
    panic!("Could not find robot");
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
