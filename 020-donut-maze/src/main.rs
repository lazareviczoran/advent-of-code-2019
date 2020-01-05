use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Hash, Eq)]
struct Position {
    x: i64,
    y: i64,
}
impl Position {
    pub fn new(x: i64, y: i64) -> Position {
        Position { x, y }
    }
}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let (map_of_the_tunnels, portals_by_pos, portals_by_name) = load_map(String::from("input.txt"));
    print_map(map_of_the_tunnels.clone());
    println!(
        "Donut Maze part1 Solution: {:?}",
        find_shortest_path(&map_of_the_tunnels, &portals_by_pos, &portals_by_name)
    );

    println!(
        "Donut Maze part2 Solution: {}",
        find_shortest_path_with_levels(&map_of_the_tunnels, &portals_by_pos, &portals_by_name)
    );
}

fn find_shortest_path(
    map: &Vec<Vec<char>>,
    portals_by_pos: &HashMap<Position, String>,
    portals_by_name: &HashMap<String, Vec<Position>>,
) -> i64 {
    let start_pos = portals_by_name.get("AA").unwrap()[0].clone();
    let mut queue: Vec<(Position, HashSet<Position>, i64)> = vec![(start_pos, HashSet::new(), 0)];

    while !queue.is_empty() {
        let (position, visited, steps) = queue.remove(0);
        let mut new_visited = visited.clone();
        if new_visited.get(&position).is_some() {
            continue;
        }
        new_visited.insert(position.clone());
        if let Some(portal) = portals_by_pos.get(&position) {
            if portal == "ZZ" {
                return steps;
            }
        }

        // find available steps
        for position in get_available_next_positions(
            map,
            portals_by_pos,
            portals_by_name,
            new_visited.clone(),
            &position,
        ) {
            queue.push((position, new_visited.clone(), steps + 1));
        }
    }

    panic!("Could not find path");
}

fn find_shortest_path_with_levels(
    map: &Vec<Vec<char>>,
    portals_by_pos: &HashMap<Position, String>,
    portals_by_name: &HashMap<String, Vec<Position>>,
) -> i64 {
    let start_pos = portals_by_name.get("AA").unwrap()[0].clone();
    let mut queue: Vec<(Position, i64, i64)> = vec![(start_pos, 0, 0)];
    let mut seen: HashSet<(Position, i64)> = HashSet::new();

    while !queue.is_empty() {
        let (position, steps, level) = queue.remove(0);

        if seen.get(&(position.clone(), level)).is_some() {
            continue;
        }
        seen.insert((position.clone(), level));

        if let Some(portal) = portals_by_pos.get(&position) {
            if portal == "ZZ" && level == 0 {
                return steps;
            }
        }

        // find available steps
        for (position, next_level) in get_available_next_positions_with_levels(
            map,
            portals_by_pos,
            portals_by_name,
            &seen,
            &position,
            level,
        ) {
            queue.push((position, steps + 1, next_level));
        }
    }

    panic!("Could not find path");
}

fn get_available_next_positions_with_levels(
    map: &Vec<Vec<char>>,
    portals_by_pos: &HashMap<Position, String>,
    portals_by_name: &HashMap<String, Vec<Position>>,
    visited: &HashSet<(Position, i64)>,
    curr_pos: &Position,
    level: i64,
) -> Vec<(Position, i64)> {
    let mut next_positions = Vec::new();
    for i in 0..4 {
        let mut next_pos = curr_pos.clone();
        let mut next_level = level;
        match i {
            0 => {
                next_pos.x -= 1;
            }
            1 => {
                next_pos.x += 1;
            }
            2 => {
                next_pos.y -= 1;
            }
            _ => {
                next_pos.y += 1;
            }
        }
        let next_val = get_value(&map, &next_pos);
        if next_val != '#' {
            if next_val.is_ascii_uppercase() {
                let portal = portals_by_pos.get(&curr_pos).unwrap();
                if portal == "AA" || portal == "ZZ" {
                    next_pos = curr_pos.clone();
                } else {
                    if curr_pos.x == 2
                        || curr_pos.x == map.len() as i64 - 3
                        || curr_pos.y == 2
                        || curr_pos.y == map[0].len() as i64 - 3
                    {
                        if next_level > 0 {
                            next_level -= 1;
                        } else {
                            continue;
                        }
                    } else {
                        next_level += 1;
                    }
                    let positions = portals_by_name.get(portal).unwrap();
                    if positions[0] == *curr_pos {
                        if positions.len() > 1 {
                            next_pos = positions[1].clone();
                        }
                    } else {
                        next_pos = positions[0].clone();
                    }
                }
            }
            if visited.get(&(next_pos.clone(), next_level)).is_none() {
                next_positions.push((next_pos, next_level));
            }
        }
    }

    next_positions
}

fn get_available_next_positions(
    map: &Vec<Vec<char>>,
    portals_by_pos: &HashMap<Position, String>,
    portals_by_name: &HashMap<String, Vec<Position>>,
    visited: HashSet<Position>,
    curr_pos: &Position,
) -> Vec<Position> {
    let mut next_positions = Vec::new();
    for i in 0..4 {
        let mut next_pos = curr_pos.clone();
        match i {
            0 => {
                next_pos.x -= 1;
            }
            1 => {
                next_pos.x += 1;
            }
            2 => {
                next_pos.y -= 1;
            }
            _ => {
                next_pos.y += 1;
            }
        }
        let next_val = get_value(&map, &next_pos);
        if next_val != '#' {
            if next_val.is_ascii_uppercase() {
                let portal = portals_by_pos.get(&curr_pos).unwrap();
                if portal == "AA" || portal == "ZZ" {
                    next_pos = curr_pos.clone();
                } else {
                    let positions = portals_by_name.get(portal).unwrap();
                    if positions[0] == *curr_pos {
                        if positions.len() > 1 {
                            next_pos = positions[1].clone();
                        }
                    } else {
                        next_pos = positions[0].clone();
                    }
                }
            }
            if visited.get(&next_pos).is_none() {
                next_positions.push(next_pos);
            }
        }
    }

    next_positions
}

fn get_value(map: &Vec<Vec<char>>, curr_pos: &Position) -> char {
    map[curr_pos.x as usize][curr_pos.y as usize]
}

fn load_map(
    filename: String,
) -> (
    Vec<Vec<char>>,
    HashMap<Position, String>,
    HashMap<String, Vec<Position>>,
) {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let map: Vec<Vec<char>> = contents
        .split_terminator('\n')
        .map(|r| {
            let mut chars = r.chars();
            let mut row = Vec::new();
            while let Some(ch) = chars.next() {
                row.push(ch);
            }
            row
        })
        .collect();

    let mut portals_by_pos: HashMap<Position, String> = HashMap::new();
    let mut portals_by_name: HashMap<String, Vec<Position>> = HashMap::new();

    let width = map[0].len();
    let height = map.len();
    let mut res = vec![vec![' '; height]; width];
    for i in 0..width {
        for j in 0..height {
            let curr = map[j][i];
            res[i][j] = curr;
            if i != 0 && i != width - 1 && j != 0 && j != height - 1 && curr.is_ascii_uppercase() {
                let mut portal = String::new();
                let mut position = Position::new(0, 0);
                if map[j - 1][i] == '.' {
                    portal.push(map[j][i]);
                    portal.push(map[j + 1][i]);
                    position = Position::new(i as i64, j as i64 - 1);
                } else if map[j + 1][i] == '.' {
                    portal.push(map[j - 1][i]);
                    portal.push(map[j][i]);
                    position = Position::new(i as i64, j as i64 + 1);
                } else if map[j][i - 1] == '.' {
                    portal.push(map[j][i]);
                    portal.push(map[j][i + 1]);
                    position = Position::new(i as i64 - 1, j as i64);
                } else if map[j][i + 1] == '.' {
                    portal.push(map[j][i - 1]);
                    portal.push(map[j][i]);
                    position = Position::new(i as i64 + 1, j as i64);
                }
                if position != Position::new(0, 0) {
                    portals_by_pos.insert(position.clone(), portal.clone());
                    if let Some(positions) = portals_by_name.get_mut(&portal) {
                        positions.push(position.clone());
                    } else {
                        portals_by_name.insert(portal, vec![position]);
                    }
                }
            }
        }
    }

    (res, portals_by_pos, portals_by_name)
}

fn print_map(map: Vec<Vec<char>>) {
    // print!("{}[2J", 27 as char);
    let w = map.len();
    let h = map[0].len();
    let mut sb = String::new();
    for j in 0..h {
        for i in 0..w {
            sb.push(map[i][j]);
        }
        sb.push('\n');
    }
    println!("{}", sb)
}

fn print_map_status(map: &Vec<Vec<char>>, position: &Position) {
    print!("{}[2J", 27 as char);
    let w = map.len();
    let h = map[0].len();
    let mut sb = String::new();
    for j in 0..h {
        for i in 0..w {
            if position.x == i as i64 && position.y == j as i64 {
                sb.push(' ');
            } else {
                sb.push(map[i][j]);
            }
        }
        sb.push('\n');
    }
    println!("{}", sb)
}

#[cfg(test)]
mod test {
    use super::find_shortest_path;
    use super::find_shortest_path_with_levels;
    use super::load_map;

    #[test]
    fn part1_sample_input1() {
        let (map_of_the_tunnels, portals1, portals2) = load_map(String::from("test-input.txt"));
        assert_eq!(
            find_shortest_path(&map_of_the_tunnels, &portals1, &portals2),
            23
        );
    }

    #[test]
    fn part1_sample_input2() {
        let (map_of_the_tunnels, portals1, portals2) = load_map(String::from("test-input2.txt"));
        assert_eq!(
            find_shortest_path(&map_of_the_tunnels, &portals1, &portals2),
            58
        );
    }

    #[test]
    fn part2_sample_input1() {
        let (map_of_the_tunnels, portals1, portals2) = load_map(String::from("test-input3.txt"));
        let steps_count = find_shortest_path_with_levels(&map_of_the_tunnels, &portals1, &portals2);
        assert_eq!(steps_count, 396);
    }
}
