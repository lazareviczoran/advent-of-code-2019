use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let int_list: Vec<i32> = contents
        .split_terminator(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    let mut first_list = int_list.clone();
    first_list[1] = 12;
    first_list[2] = 2;

    let result_list = compute(&mut first_list);
    println!("1202 Program Alarm part1 Solution: {}", result_list[0]);

    let mut second_list = int_list.clone();
    let (noun, verb) = find_noun_and_verb(&mut second_list, 19690720);
    println!(
        "1202 Program Alarm part2 Solution: noun => {}, verb = {}",
        noun, verb
    );
}

fn compute(int_list: &mut Vec<i32>) -> Vec<i32> {
    let mut pos = 0;
    while pos < int_list.len() {
        let opcode = int_list[pos];
        match opcode {
            99 => break,
            1 => {
                let store_index = int_list[pos + 3] as usize;
                int_list[store_index] =
                    int_list[int_list[pos + 1] as usize] + int_list[int_list[pos + 2] as usize];
            }
            2 => {
                let store_index = int_list[pos + 3] as usize;
                int_list[store_index] =
                    int_list[int_list[pos + 1] as usize] * int_list[int_list[pos + 2] as usize];
            }
            _ => panic!("Something went wrong"),
        }
        pos = pos + 4;
    }
    int_list.to_vec()
}

fn find_noun_and_verb(int_list: &mut Vec<i32>, target_val: i32) -> (i32, i32) {
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut list = int_list.clone();
            list[1] = noun;
            list[2] = verb;
            let res_list = compute(&mut list);
            if res_list[0] == target_val {
                return (noun, verb);
            }
        }
    }
    panic!("Could not find initial values for target {}", target_val);
}

#[cfg(test)]
mod test {
    use super::compute;

    #[test]
    fn part1_first_sample_input() {
        assert_eq!(compute(&mut vec![1, 0, 0, 0, 99]), [2, 0, 0, 0, 99]);
    }

    #[test]
    fn part1_second_sample_input() {
        assert_eq!(compute(&mut vec![2, 3, 0, 3, 99]), [2, 3, 0, 6, 99]);
    }

    #[test]
    fn part1_third_sample_input() {
        assert_eq!(
            compute(&mut vec![2, 4, 4, 5, 99, 0]),
            [2, 4, 4, 5, 99, 9801]
        );
    }

    #[test]
    fn part1_fourth_sample_input() {
        assert_eq!(
            compute(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            [30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
