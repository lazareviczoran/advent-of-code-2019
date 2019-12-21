use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let input_data: Vec<i32> = contents
        .chars()
        .map(|v| v.to_string().parse::<i32>().unwrap())
        .collect();

    let base_pattern = vec![0, 1, 0, -1];
    let mut res = input_data.clone();
    for _ in 0..100 {
        res = get_next_phase(res, &base_pattern);
    }

    let (first, _rest) = res.split_at(8);
    println!("Flawed Frequency Transmission part1 Solution: {:?}", first);

    let mut real_signal_input = Vec::new();
    for _ in 0..10000 {
        real_signal_input.append(&mut input_data.clone());
    }
    let mut res = real_signal_input.clone();
    let offset = convert_offset(res.clone());
    for _ in 0..100 {
        res = get_next_phase(res, &base_pattern);
    }
    let (_, rest) = res.split_at(offset as usize);
    let (result, _) = rest.split_at(8);

    println!("Flawed Frequency Transmission part2 Solution: {:?}", result);
}

fn convert_offset(input: Vec<i32>) -> i32 {
    let mut offset = 0;
    for i in 0..7 {
        offset = offset * 10 + input[i];
    }

    offset
}

fn get_next_phase(input: Vec<i32>, base_pattern: &Vec<i32>) -> Vec<i32> {
    let res = calculate_phase(input, base_pattern);
    convert_output(res)
}

fn calculate_phase(input: Vec<i32>, base_pattern: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..input.len() {
        let mut val = 0;
        for j in 0..input.len() {
            let pattern_val = calculate_nth_pattern_value(&base_pattern, i as i32, j as i32);
            if pattern_val != 0 {
                val += input[j] * pattern_val;
            }
        }
        res.push(val);
    }

    res
}

fn convert_output(output: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for item in output {
        res.push(item.abs() % 10);
    }

    res
}

fn calculate_nth_pattern_value(base_pattern: &Vec<i32>, repeat_num: i32, n: i32) -> i32 {
    let length = base_pattern.len() as i32;
    let cycle = length + repeat_num * length;

    let pos = ((n + 1 % cycle) / (1 + repeat_num)) % length;

    base_pattern[(pos) as usize]
}

#[cfg(test)]
mod test {
    use super::calculate_nth_pattern_value;
    use super::convert_offset;
    use super::get_next_phase;

    #[test]
    fn calculate_pattern_test1() {
        assert_eq!(calculate_nth_pattern_value(&vec![0, 1, 0, -1], 0, 5), 0);
        assert_eq!(calculate_nth_pattern_value(&vec![0, 1, 0, -1], 0, 6), -1);
        assert_eq!(calculate_nth_pattern_value(&vec![0, 1, 0, -1], 0, 7), 0);
    }

    #[test]
    fn calculate_pattern_test2() {
        assert_eq!(calculate_nth_pattern_value(&vec![0, 1, 0, -1], 3, 5), 1);
        assert_eq!(calculate_nth_pattern_value(&vec![0, 1, 0, -1], 3, 6), 1);
        assert_eq!(calculate_nth_pattern_value(&vec![0, 1, 0, -1], 3, 7), 0);
    }

    #[test]
    fn calculate_pattern_test3() {
        assert_eq!(calculate_nth_pattern_value(&vec![0, 1, 0, -1], 6, 5), 0);
        assert_eq!(calculate_nth_pattern_value(&vec![0, 1, 0, -1], 6, 6), 1);
        assert_eq!(calculate_nth_pattern_value(&vec![0, 1, 0, -1], 6, 7), 1);
    }

    #[test]
    fn part1_sample_input1() {
        assert_eq!(
            get_next_phase(vec![1, 2, 3, 4, 5, 6, 7, 8], &vec![0, 1, 0, -1]),
            [4, 8, 2, 2, 6, 1, 5, 8]
        );
    }

    #[test]
    fn part1_sample_input2() {
        assert_eq!(
            get_next_phase(vec![4, 8, 2, 2, 6, 1, 5, 8], &vec![0, 1, 0, -1]),
            [3, 4, 0, 4, 0, 4, 3, 8]
        );
    }

    #[test]
    fn part1_sample_input3() {
        assert_eq!(
            get_next_phase(vec![3, 4, 0, 4, 0, 4, 3, 8], &vec![0, 1, 0, -1]),
            [0, 3, 4, 1, 5, 5, 1, 8]
        );
    }

    #[test]
    fn part1_sample_input4() {
        assert_eq!(
            get_next_phase(vec![0, 3, 4, 1, 5, 5, 1, 8], &vec![0, 1, 0, -1]),
            [0, 1, 0, 2, 9, 4, 9, 8]
        );
    }

    #[test]
    fn part1_sample_input5() {
        let mut res = vec![
            8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4, 5,
            5, 9, 5,
        ];
        for _ in 0..100 {
            res = get_next_phase(res, &vec![0, 1, 0, -1]);
        }
        assert_eq!(res.starts_with(&[2, 4, 1, 7, 6, 1, 7, 6]), true);
    }

    #[test]
    fn part1_sample_input6() {
        let mut res = vec![
            1, 9, 6, 1, 7, 8, 0, 4, 2, 0, 7, 2, 0, 2, 2, 0, 9, 1, 4, 4, 9, 1, 6, 0, 4, 4, 1, 8, 9,
            9, 1, 7,
        ];
        for _ in 0..100 {
            res = get_next_phase(res, &vec![0, 1, 0, -1]);
        }
        assert_eq!(res.starts_with(&[7, 3, 7, 4, 5, 4, 1, 8]), true);
    }

    #[test]
    fn part1_sample_input7() {
        let mut res = vec![
            6, 9, 3, 1, 7, 1, 6, 3, 4, 9, 2, 9, 4, 8, 6, 0, 6, 3, 3, 5, 9, 9, 5, 9, 2, 4, 3, 1, 9,
            8, 7, 3,
        ];
        for _ in 0..100 {
            res = get_next_phase(res, &vec![0, 1, 0, -1]);
        }
        assert_eq!(res.starts_with(&[5, 2, 4, 3, 2, 1, 3, 3]), true);
    }

    #[test]
    fn part2_sample_input1() {
        let mut real_signal_input = Vec::new();
        for _ in 0..10000 {
            real_signal_input.append(&mut vec![
                0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5, 4, 7,
                4, 6, 6, 4,
            ]);
        }
        let mut res = real_signal_input.clone();
        let offset = convert_offset(res.clone());
        for _ in 0..100 {
            res = get_next_phase(res, &vec![0, 1, 0, -1]);
        }
        let (_, rest) = res.split_at(offset as usize);
        let (result, _) = rest.split_at(8);
        assert_eq!(result, [8, 4, 4, 6, 2, 0, 2, 6]);
    }

    // #[test]
    // fn part2_sample_input2() {
    //     let mut real_signal_input = Vec::new();
    //     for _ in 0..10000 {
    //         real_signal_input.append(&mut vec![
    //             0, 2, 9, 3, 5, 1, 0, 9, 6, 9, 9, 9, 4, 0, 8, 0, 7, 4, 0, 7, 5, 8, 5, 4, 4, 7, 0, 3,
    //             4, 3, 2, 3,
    //         ]);
    //     }
    //     let mut res = real_signal_input.clone();
    //     let offset = convert_offset(res.clone());
    //     for _ in 0..100 {
    //         res = get_next_phase(res, &vec![0, 1, 0, -1]);
    //     }
    //     let (_, rest) = res.split_at(offset as usize);
    //     let (result, _) = rest.split_at(8);
    //     assert_eq!(result, [7, 8, 7, 2, 5, 2, 7, 0]);
    // }

    // #[test]
    // fn part2_sample_input3() {
    //     let mut real_signal_input = Vec::new();
    //     for _ in 0..10000 {
    //         real_signal_input.append(&mut vec![
    //             0, 3, 0, 8, 1, 7, 7, 0, 8, 8, 4, 9, 2, 1, 9, 5, 9, 7, 3, 1, 1, 6, 5, 4, 4, 6, 8, 5,
    //             0, 5, 1, 7,
    //         ]);
    //     }
    //     let mut res = real_signal_input.clone();
    //     let offset = convert_offset(res.clone());
    //     for _ in 0..100 {
    //         res = get_next_phase(res, &vec![0, 1, 0, -1]);
    //     }
    //     let (_, rest) = res.split_at(offset as usize);
    //     let (result, _) = rest.split_at(8);
    //     assert_eq!(result, [5, 3, 5, 5, 3, 7, 3, 1]);
    // }
}
