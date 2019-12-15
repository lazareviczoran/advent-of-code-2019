use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}
impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }
}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let re = Regex::new(r"^<x=(.*?),\s+y=(.*?),\s+z=(.*?)>").unwrap();
    let moon_positions: Vec<Position> = contents
        .split_terminator('\n')
        .map(|v| {
            let caps = re.captures(v).unwrap();
            let x = &caps[1].parse::<i32>().unwrap();
            let y = &caps[2].parse::<i32>().unwrap();
            let z = &caps[3].parse::<i32>().unwrap();
            Position::new(*x, *y, *z)
        })
        .collect();
    let velocities: Vec<Position> = vec![Position::new(0, 0, 0); moon_positions.len()];

    let total_energy =
        calculate_total_energy(&mut moon_positions.clone(), &mut velocities.clone(), 1000);

    println!("The N-Body Problem part1 Solution: {:?}", total_energy);

    let total_steps =
        calculate_steps_to_repeating(&mut moon_positions.clone(), &mut velocities.clone());

    println!("The N-Body Problem part2 Solution: {:?}", total_steps);
}

fn calculate_steps_to_repeating(
    positions: &mut Vec<Position>,
    velocities: &mut Vec<Position>,
) -> i128 {
    let (cycle_x, cycle_y, cycle_z) = find_cycles(positions, velocities);
    let i = lcm(cycle_x as i128, lcm(cycle_y as i128, cycle_z as i128));

    i
}

fn lcm(a: i128, b: i128) -> i128 {
    let mut res = a * b;
    res /= gcd(a, b);
    res
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn find_cycles(positions: &mut Vec<Position>, velocities: &mut Vec<Position>) -> (i32, i32, i32) {
    let initial_positions = positions.clone();
    let initial_velocities = velocities.clone();
    let mut cycle_x = 0;
    let mut cycle_y = 0;
    let mut cycle_z = 0;
    let mut steps = 1;
    move_moons(positions, velocities);
    while cycle_x == 0 || cycle_y == 0 || cycle_z == 0 {
        let mut matching_x =
            positions[0].x == initial_positions[0].x && velocities[0].x == initial_velocities[0].x;
        let mut matching_y =
            positions[0].y == initial_positions[0].y && velocities[0].y == initial_velocities[0].y;
        let mut matching_z =
            positions[0].z == initial_positions[0].z && velocities[0].z == initial_velocities[0].z;
        for i in 1..positions.len() {
            matching_x = matching_x
                && (positions[i].x == initial_positions[i].x
                    && velocities[i].x == initial_velocities[i].x);
            matching_y = matching_y
                && (positions[i].y == initial_positions[i].y
                    && velocities[i].y == initial_velocities[i].y);
            matching_z = matching_z
                && (positions[i].z == initial_positions[i].z
                    && velocities[i].z == initial_velocities[i].z);
        }
        if cycle_x == 0 && matching_x {
            cycle_x = steps;
        }
        if cycle_y == 0 && matching_y {
            cycle_y = steps;
        }
        if cycle_z == 0 && matching_z {
            cycle_z = steps;
        }
        steps += 1;
        move_moons(positions, velocities);
    }
    (cycle_x, cycle_y, cycle_z)
}

fn calculate_total_energy(
    positions: &mut Vec<Position>,
    velocities: &mut Vec<Position>,
    steps: usize,
) -> i32 {
    let mut total_energy = 0;
    for _ in 0..steps {
        move_moons(positions, velocities);
    }

    for i in 0..positions.len() {
        let mut pot = 0;
        pot += positions[i].x.abs();
        pot += positions[i].y.abs();
        pot += positions[i].z.abs();

        let mut kin = 0;
        kin += velocities[i].x.abs();
        kin += velocities[i].y.abs();
        kin += velocities[i].z.abs();
        total_energy += pot * kin;
    }

    total_energy
}

fn move_moons(positions: &mut Vec<Position>, velocities: &mut Vec<Position>) {
    apply_gravity(positions, velocities);
    for i in 0..positions.len() {
        positions[i].x += velocities[i].x;
        positions[i].y += velocities[i].y;
        positions[i].z += velocities[i].z;
    }
}

fn apply_gravity(positions: &mut Vec<Position>, velocities: &mut Vec<Position>) {
    for i in 0..positions.len() {
        for j in 0..positions.len() {
            if i != j {
                if positions[i].x > positions[j].x {
                    velocities[i].x -= 1;
                } else if positions[i].x < positions[j].x {
                    velocities[i].x += 1;
                }
                if positions[i].y > positions[j].y {
                    velocities[i].y -= 1;
                } else if positions[i].y < positions[j].y {
                    velocities[i].y += 1;
                }
                if positions[i].z > positions[j].z {
                    velocities[i].z -= 1;
                } else if positions[i].z < positions[j].z {
                    velocities[i].z += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::calculate_steps_to_repeating;
    use super::calculate_total_energy;
    use super::Position;

    #[test]
    fn part1_sample_input1() {
        let mut positions: Vec<Position> = vec![
            Position::new(-1, 0, 2),
            Position::new(2, -10, -7),
            Position::new(4, -8, 8),
            Position::new(3, 5, -1),
        ];
        let mut velocities: Vec<Position> = vec![Position::new(0, 0, 0); positions.len()];

        let energy = calculate_total_energy(&mut positions, &mut velocities, 10);
        assert_eq!(positions[0], Position::new(2, 1, -3));
        assert_eq!(positions[1], Position::new(1, -8, 0));
        assert_eq!(positions[2], Position::new(3, -6, 1));
        assert_eq!(positions[3], Position::new(2, 0, 4));
        assert_eq!(energy, 179);
    }

    #[test]
    fn part1_sample_input2() {
        let mut positions: Vec<Position> = vec![
            Position::new(-8, -10, 0),
            Position::new(5, 5, 10),
            Position::new(2, -7, 3),
            Position::new(9, -8, -3),
        ];
        let mut velocities: Vec<Position> = vec![Position::new(0, 0, 0); positions.len()];
        let energy = calculate_total_energy(&mut positions, &mut velocities, 100);

        assert_eq!(positions[0], Position::new(8, -12, -9));
        assert_eq!(positions[1], Position::new(13, 16, -3));
        assert_eq!(positions[2], Position::new(-29, -11, -1));
        assert_eq!(positions[3], Position::new(16, -13, 23));
        assert_eq!(energy, 1940);
    }

    #[test]
    fn part2_sample_input1() {
        let mut positions: Vec<Position> = vec![
            Position::new(-1, 0, 2),
            Position::new(2, -10, -7),
            Position::new(4, -8, 8),
            Position::new(3, 5, -1),
        ];
        let mut velocities: Vec<Position> = vec![Position::new(0, 0, 0); positions.len()];

        let steps = calculate_steps_to_repeating(&mut positions, &mut velocities);
        assert_eq!(steps, 2772);
    }

    #[test]
    fn part2_sample_input2() {
        let mut positions: Vec<Position> = vec![
            Position::new(-8, -10, 0),
            Position::new(5, 5, 10),
            Position::new(2, -7, 3),
            Position::new(9, -8, -3),
        ];
        let mut velocities: Vec<Position> = vec![Position::new(0, 0, 0); positions.len()];
        let steps = calculate_steps_to_repeating(&mut positions, &mut velocities);

        assert_eq!(steps, 4686774924);
    }
}
