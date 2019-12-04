use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let wires: Vec<&str> = contents.split_terminator('\n').collect();
    let first_wire_moves = wires[0].split_terminator(',').collect();
    let second_wire_moves = wires[1].split_terminator(',').collect();

    let (first_wire_positions, second_wire_positions) =
        calculate_positions(first_wire_moves, second_wire_moves);

    let (part1_solution, part2_solution) =
        get_closest_intersection_distance(first_wire_positions, second_wire_positions);

    println!("Crossed Wires part1 Solution: {}", part1_solution);
    println!("Crossed Wires part2 Solution: {}", part2_solution);
}

#[derive(Clone)]
struct Position {
    point: Point,
    steps: i32,
}
impl Position {
    pub fn new(point: Point, steps: i32) -> Position {
        Position { point, steps }
    }
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn calculate_positions(w1_moves: Vec<&str>, w2_moves: Vec<&str>) -> (Vec<Position>, Vec<Position>) {
    let mut pos1 = vec![Position::new(Point::new(0, 0), 0)];
    let mut pos2 = vec![Position::new(Point::new(0, 0), 0)];
    let len1 = w1_moves.len();
    let len2 = w2_moves.len();
    let len;
    if len1 < len2 {
        len = len2;
    } else {
        len = len1;
    }
    for i in 0..len {
        if i < len1 {
            pos1.push(calculate_next_position(w1_moves[i], pos1[i].point.clone()));
        }
        if i < len2 {
            pos2.push(calculate_next_position(w2_moves[i], pos2[i].point.clone()));
        }
    }

    (pos1, pos2)
}

fn calculate_next_position(next_move_str: &str, previous_pos: Point) -> Position {
    let mut next_move_chars = next_move_str.chars();
    let op = next_move_chars.next().unwrap();
    let steps = next_move_chars.as_str().parse::<i32>().unwrap();

    match op {
        'R' => Position::new(Point::new(previous_pos.x + steps, previous_pos.y), steps),
        'L' => Position::new(Point::new(previous_pos.x - steps, previous_pos.y), steps),
        'U' => Position::new(Point::new(previous_pos.x, previous_pos.y + steps), steps),
        'D' => Position::new(Point::new(previous_pos.x, previous_pos.y - steps), steps),
        _ => panic!("Unexpected move direction"),
    }
}

fn get_closest_intersection_distance(
    w1_positions: Vec<Position>,
    w2_positions: Vec<Position>,
) -> (i32, i32) {
    let mut distance = i32::max_value();
    let mut first_intersection_steps_sum = i32::max_value();
    let w1_len = w1_positions.len();
    let w2_len = w2_positions.len();
    let mut curr_line1;
    let mut curr_line2;
    let mut w1_steps_acc = 0;
    let mut w2_steps_acc;
    for i in 0..w1_len {
        w2_steps_acc = 0;
        curr_line1 = vec![
            w1_positions[i].clone(),
            w1_positions[(i + 1) % w1_len].clone(),
        ];
        for j in 0..w2_len {
            curr_line2 = vec![
                w2_positions[j].clone(),
                w2_positions[(j + 1) % w2_len].clone(),
            ];
            if let Some(intersection_point) =
                get_intersection_point(curr_line1.clone(), curr_line2.clone())
            {
                let new_distance = intersection_point.x.abs() + intersection_point.y.abs();
                if distance > new_distance {
                    distance = new_distance;
                }

                let new_steps_sum = w1_steps_acc
                    + get_distance_between_points(
                        curr_line1[0].point.clone(),
                        intersection_point.clone(),
                    )
                    + w2_steps_acc
                    + get_distance_between_points(
                        curr_line2[0].point.clone(),
                        intersection_point.clone(),
                    );
                if first_intersection_steps_sum > new_steps_sum {
                    first_intersection_steps_sum = new_steps_sum;
                }
            }
            w2_steps_acc = w2_steps_acc + curr_line2[1].steps;
        }
        w1_steps_acc = w1_steps_acc + curr_line1[1].steps;
    }
    (distance, first_intersection_steps_sum)
}

fn get_intersection_point(line1: Vec<Position>, line2: Vec<Position>) -> Option<Point> {
    let horizontal_line;
    let vertical_line;
    let p11;
    let p12;
    let p21;
    let p22;
    if line1[0].point.x != line1[1].point.x {
        horizontal_line = line1;
        vertical_line = line2;
    } else {
        horizontal_line = line2;
        vertical_line = line1;
    }
    if horizontal_line[0].point.x > horizontal_line[1].point.x {
        p11 = horizontal_line[1].point.clone();
        p12 = horizontal_line[0].point.clone();
    } else {
        p11 = horizontal_line[0].point.clone();
        p12 = horizontal_line[1].point.clone();
    }
    if vertical_line[0].point.y > vertical_line[1].point.y {
        p21 = vertical_line[1].point.clone();
        p22 = vertical_line[0].point.clone();
    } else {
        p21 = vertical_line[0].point.clone();
        p22 = vertical_line[1].point.clone();
    }
    if p11.x < p21.x
        && p21.x <= p12.x
        && p21.y < p11.y
        && p11.y <= p22.y
        && p21.x != 0
        && p11.y != 0
    {
        return Some(Point::new(p21.x, p11.y));
    }
    None
}

fn get_distance_between_points(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

#[cfg(test)]
mod test {
    use super::calculate_positions;
    use super::get_closest_intersection_distance;

    #[test]
    fn first_sample_input() {
        let (first_wire_positions, second_wire_positions) =
            calculate_positions(vec!["R8", "U5", "L5", "D3"], vec!["U7", "R6", "D4", "L4"]);
        assert_eq!(
            get_closest_intersection_distance(first_wire_positions, second_wire_positions),
            (6, 30)
        );
    }

    #[test]
    fn second_sample_input() {
        let (first_wire_positions, second_wire_positions) = calculate_positions(
            vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
            vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
        );
        assert_eq!(
            get_closest_intersection_distance(first_wire_positions, second_wire_positions),
            (159, 610)
        );
    }

    #[test]
    fn third_sample_input() {
        let (first_wire_positions, second_wire_positions) = calculate_positions(
            vec![
                "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
            ],
            vec![
                "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
            ],
        );
        assert_eq!(
            get_closest_intersection_distance(first_wire_positions, second_wire_positions),
            (135, 410)
        );
    }
}
