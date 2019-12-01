use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let module_masses: Vec<i32> = contents
        .split_terminator('\n')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let mut total1 = 0;
    let mut total2 = 0;
    for mass in module_masses {
        total1 = total1 + calculate_fuel(mass);
        total2 = total2 + calculate_all_the_fuel(mass);
    }

    println!("Rocket Equasion part1 Solution: {}", total1);

    println!("Rocket Equasion part2 Solution: {}", total2);
}

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_all_the_fuel(mass: i32) -> i32 {
    let value = calculate_fuel(mass);
    if value > 0 {
        return value + calculate_all_the_fuel(value);
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::calculate_all_the_fuel;
    use super::calculate_fuel;

    #[test]
    fn part1_first_sample_input() {
        assert_eq!(calculate_fuel(12), 2);
    }

    #[test]
    fn part1_second_sample_input() {
        assert_eq!(calculate_fuel(14), 2);
    }

    #[test]
    fn part1_third_sample_input() {
        assert_eq!(calculate_fuel(1969), 654);
    }

    #[test]
    fn part1_fourth_sample_input() {
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn part2_first_sample_input() {
        assert_eq!(calculate_all_the_fuel(14), 2);
    }

    #[test]
    fn part2_second_sample_input() {
        assert_eq!(calculate_all_the_fuel(1969), 966);
    }

    #[test]
    fn part2_third_sample_input() {
        assert_eq!(calculate_all_the_fuel(100756), 50346);
    }
}
