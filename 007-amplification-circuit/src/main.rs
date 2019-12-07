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

    let max_thruster_signal = compute_max_thruster_signal(&mut memory.clone());
    println!(
        "Amplification Circuit part1 Solution: {:?}",
        max_thruster_signal
    );

    let feedback_loop_max_thruster_signal =
        compute_max_thruster_signal_with_feedback_loop(&mut memory.clone());
    println!(
        "Amplification Circuit part2 Solution: {:?}",
        feedback_loop_max_thruster_signal
    );
}

fn compute_max_thruster_signal(memory: &mut Vec<i32>) -> i32 {
    let mut max_thruster_signal = 0;
    // generating permutations using heaps algorithm
    let mut permutations: Vec<Vec<i32>> = Vec::new();
    let mut sequence_items = vec![0, 1, 2, 3, 4];
    calculate_permutations(&mut permutations, &mut sequence_items, 5);

    for perm in permutations {
        let (amplifier_a_output, _, _) = compute(&mut memory.clone(), vec![perm[0], 0], 0, 0);
        let (amplifier_b_output, _, _) = compute(
            &mut memory.clone(),
            vec![perm[1], amplifier_a_output[0]],
            0,
            0,
        );
        let (amplifier_c_output, _, _) = compute(
            &mut memory.clone(),
            vec![perm[2], amplifier_b_output[0]],
            0,
            0,
        );
        let (amplifier_d_output, _, _) = compute(
            &mut memory.clone(),
            vec![perm[3], amplifier_c_output[0]],
            0,
            0,
        );
        let (amplifier_e_output, _, _) = compute(
            &mut memory.clone(),
            vec![perm[4], amplifier_d_output[0]],
            0,
            0,
        );
        if max_thruster_signal < amplifier_e_output[0] {
            max_thruster_signal = amplifier_e_output[0];
        }
    }

    max_thruster_signal
}

fn compute_max_thruster_signal_with_feedback_loop(memory: &mut Vec<i32>) -> i32 {
    let mut max_thruster_signal = 0;
    let mut permutations: Vec<Vec<i32>> = Vec::new();
    let mut sequence_items = vec![5, 6, 7, 8, 9];
    calculate_permutations(&mut permutations, &mut sequence_items, 5);
    let mut amplifier_op_positions: Vec<usize>;
    let mut amplifier_input_positions: Vec<usize>;
    let mut amplifier_outputs;
    let mut amplifier_inputs;
    let mut amplifier_programs;
    let mut curr_amplifier: usize = 0;
    let mut prev_amplifier: usize;

    for perm in permutations {
        amplifier_input_positions = vec![0; 5];
        amplifier_op_positions = vec![0; 5];
        amplifier_outputs = vec![vec![]; 5];
        amplifier_inputs = vec![vec![]; 5];
        amplifier_programs = vec![memory.clone(); 5];
        for i in 0..5 {
            amplifier_inputs[i].push(perm[i]);
        }
        amplifier_inputs[0].push(0);
        while amplifier_input_positions[4] != usize::max_value() {
            prev_amplifier = (curr_amplifier + 4) % 5;
            amplifier_inputs[curr_amplifier].append(&mut amplifier_outputs[prev_amplifier]);
            let (new_output, new_op_position, new_input_position) = compute(
                &mut amplifier_programs[curr_amplifier],
                amplifier_inputs[curr_amplifier].clone(),
                amplifier_op_positions[curr_amplifier],
                amplifier_input_positions[curr_amplifier],
            );
            amplifier_outputs[curr_amplifier] = new_output;
            amplifier_input_positions[curr_amplifier] = new_input_position;
            amplifier_op_positions[curr_amplifier] = new_op_position;
            curr_amplifier = (curr_amplifier + 1) % 5;
        }

        if max_thruster_signal < amplifier_outputs[4][0] {
            max_thruster_signal = amplifier_outputs[4][0];
        }
    }

    max_thruster_signal
}

fn calculate_permutations(result: &mut Vec<Vec<i32>>, sequence: &mut Vec<i32>, n: i32) {
    // generating permutations using heaps algorithm
    if n == 1 {
        // (got a new permutation)
        result.push(sequence.to_vec());
        return;
    }
    for i in 0..n - 1 {
        calculate_permutations(result, sequence, n - 1);
        // always swap the first when odd,
        // swap the i-th when even
        if n % 2 == 0 {
            swap(sequence, n - 1, i);
        } else {
            swap(sequence, n - 1, 0);
        }
    }
    calculate_permutations(result, sequence, n - 1);
}

fn swap(sequence: &mut Vec<i32>, from: i32, to: i32) {
    let temp = sequence[from as usize];
    sequence[from as usize] = sequence[to as usize];
    sequence[to as usize] = temp;
}

fn compute(
    memory: &mut Vec<i32>,
    input: Vec<i32>,
    op_position: usize,
    input_position: usize,
) -> (Vec<i32>, usize, usize) {
    let mut output = Vec::new();
    let mut op_pos = op_position;
    let mut input_pos = input_position;
    while op_pos < memory.len() {
        let (op_code, param_modes) = extract_op_code_and_param_modes(memory.to_vec(), op_pos);

        let move_by;
        match op_code {
            99 => break,
            1 => {
                let store_index = memory[op_pos + 3] as usize;
                let args = get_argument_values(memory.to_vec(), op_pos, param_modes);
                memory[store_index] = args[0] + args[1];
                move_by = 4;
            }
            2 => {
                let store_index = memory[op_pos + 3] as usize;
                let args = get_argument_values(memory.to_vec(), op_pos, param_modes);
                memory[store_index] = args[0] * args[1];
                move_by = 4;
            }
            3 => {
                let store_index = memory[op_pos + 1] as usize;
                if input_pos == input.len() {
                    return (output, op_pos, input_pos);
                }
                memory[store_index] = input[input_pos];
                input_pos = input_pos + 1;
                move_by = 2;
            }
            4 => {
                let args = get_output_value(memory.to_vec(), op_pos, param_modes);
                output.push(args[0]);
                move_by = 2;
            }
            5 => {
                let args = get_argument_values(memory.to_vec(), op_pos, param_modes);
                if args[0] > 0 {
                    op_pos = args[1] as usize;
                    continue;
                }
                move_by = 3;
            }
            6 => {
                let args = get_argument_values(memory.to_vec(), op_pos, param_modes);
                if args[0] == 0 {
                    op_pos = args[1] as usize;
                    continue;
                }
                move_by = 3;
            }
            7 => {
                let args = get_argument_values(memory.to_vec(), op_pos, param_modes);
                let store_index = memory[op_pos + 3] as usize;
                if args[0] < args[1] {
                    memory[store_index] = 1;
                } else {
                    memory[store_index] = 0;
                }
                move_by = 4;
            }
            8 => {
                let args = get_argument_values(memory.to_vec(), op_pos, param_modes);
                let store_index = memory[op_pos + 3] as usize;
                if args[0] == args[1] {
                    memory[store_index] = 1;
                } else {
                    memory[store_index] = 0;
                }
                move_by = 4;
            }
            _ => panic!("Something went wrong"),
        }
        op_pos = op_pos + move_by;
    }
    (output, usize::max_value(), usize::max_value())
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
    use super::compute_max_thruster_signal;
    use super::compute_max_thruster_signal_with_feedback_loop;

    #[test]
    fn part1_sample_input1() {
        assert_eq!(
            compute_max_thruster_signal(&mut vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            43210
        );
    }

    #[test]
    fn part1_sample_input2() {
        assert_eq!(
            compute_max_thruster_signal(&mut vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            54321
        );
    }

    #[test]
    fn part1_sample_input3() {
        assert_eq!(
            compute_max_thruster_signal(&mut vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]),
            65210
        );
    }

    #[test]
    fn part2_sample_input1() {
        assert_eq!(
            compute_max_thruster_signal_with_feedback_loop(&mut vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ]),
            139629729
        );
    }

    #[test]
    fn part2_sample_input2() {
        assert_eq!(
            compute_max_thruster_signal_with_feedback_loop(&mut vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ]),
            18216
        );
    }
}
