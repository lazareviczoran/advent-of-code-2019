use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Hash, Eq)]
struct Node {
    name: char,
    x: i64,
    y: i64,
}
impl Node {
    pub fn new(name: char, x: i64, y: i64) -> Node {
        Node { name, x, y }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.x == other.x && self.y == other.y
    }
}

#[derive(Clone, Debug)]
struct Distance {
    value: i64,
    doors: HashSet<Node>,
}
impl Distance {
    pub fn new(distance: i64) -> Distance {
        Distance {
            value: distance,
            doors: HashSet::new(),
        }
    }
}

fn main() {
    let mut map_of_the_tunnels = load_map(String::from("input.txt"));
    print_map(&mut map_of_the_tunnels);

    let (_shortest_path, steps_count) = get_collect_keys_shortest_path(&mut map_of_the_tunnels);
    println!(
        "Many-Worlds Interpretation part1 Solution: {:?}",
        steps_count
    );

    // println!("Many-Worlds Interpretation part2 Solution: {}", collected_scaffolds);
}

fn get_collect_keys_shortest_path(map: &mut Vec<Vec<char>>) -> (Vec<char>, i64) {
    let shortest_path: Vec<char> = Vec::new();
    let (start_pos, all_keys, _) = locate_start_pos_and_all_keys_and_doors(map);

    let distances = init_distance_map(map, &all_keys, &start_pos);
    println!("distances\n");
    print_distances(&distances);

    let visited_all = (1 << all_keys.len()) - 1;
    let mask = 0;
    let mut dp: HashMap<(i64, Node), i64> = HashMap::new();
    let shortest_path_steps = tsp(
        mask,
        start_pos.clone(),
        all_keys.clone(),
        all_keys,
        distances,
        &mut dp,
        visited_all,
    );

    (shortest_path, shortest_path_steps)
}

fn tsp(
    mask: i64,
    curr_node: Node,
    all_keys: HashSet<Node>,
    remaining_keys: HashSet<Node>,
    distances: HashMap<(char, char), Distance>,
    dp: &mut HashMap<(i64, Node), i64>,
    visited_all: i64,
) -> i64 {
    if 64 - mask.count_zeros() == (all_keys.len() - 1) as u32 {
        let mut new_remaining_keys = remaining_keys.clone();
        new_remaining_keys.remove(&curr_node);
        let mut iter = new_remaining_keys.iter();

        return distances
            .get(&(curr_node.name, iter.next().unwrap().name))
            .unwrap()
            .value;
    }
    if let Some(value) = dp.get(&(mask, curr_node.clone())) {
        return *value;
    }
    let curr_node_name = curr_node.clone().name;

    let mut ans = i64::max_value();
    let mut distances_clone = distances.clone();

    for (_, distance) in distances_clone.iter_mut() {
        (*distance)
            .doors
            .retain(|k| k.name != curr_node_name.to_ascii_uppercase())
    }

    // for each unvisited and unlocked
    let mut new_remaining_keys = remaining_keys.clone();
    new_remaining_keys.remove(&curr_node);

    let mut neighbours = new_remaining_keys.clone();
    neighbours.retain(|k| {
        distances_clone
            .get(&(curr_node_name, k.name))
            .unwrap()
            .doors
            .len()
            == 0
    });
    for v in neighbours.iter() {
        let city = v.name as u8 - 'a' as u8;
        if mask & (1 << city) == 0 {
            let new_ans = distances_clone
                .get(&(curr_node_name, v.name))
                .unwrap()
                .value
                + tsp(
                    mask | (1 << city),
                    v.clone(),
                    all_keys.clone(),
                    new_remaining_keys.clone(),
                    distances_clone.clone(),
                    dp,
                    visited_all,
                );

            if ans > new_ans {
                ans = new_ans;
            }
        }
    }
    dp.insert((mask, curr_node), ans);
    ans
}

fn init_distance_map(
    map: &mut Vec<Vec<char>>,
    keys: &HashSet<Node>,
    start_pos: &Node,
) -> HashMap<(char, char), Distance> {
    let mut distances = HashMap::new();
    let mut keys_with_start = keys.clone();
    keys_with_start.insert(start_pos.clone());
    let keys_clone = keys_with_start.clone();

    for key in keys_with_start {
        find_distances_from(map, &mut distances, &keys_clone, &key);
    }

    distances
}

fn find_distances_from(
    map: &mut Vec<Vec<char>>,
    distances: &mut HashMap<(char, char), Distance>,
    keys: &HashSet<Node>,
    from: &Node,
) {
    let mut queue = vec![from.clone()];
    let mut remaining_keys = keys.clone();
    let mut visited = HashSet::new();
    let mut parents: HashMap<Node, Node> = HashMap::new();
    visited.insert((from.x, from.y));

    while !queue.is_empty() && !remaining_keys.is_empty() {
        let curr_pos = queue.remove(0);
        if remaining_keys.contains(&curr_pos) {
            remaining_keys.remove(&curr_pos);
            if distances.get(&(from.name, curr_pos.name)).is_none() {
                let mut curr_el = curr_pos.clone();
                let mut distance = Distance::new(0);
                while let Some(parent) = parents.get(&curr_el) {
                    distance.value += 1;
                    if curr_el.name.is_ascii_uppercase() {
                        distance.doors.insert(curr_el);
                    }
                    curr_el = parent.clone();
                }
                distances.insert((from.name, curr_pos.name), distance);
            }
        }
        for pos in get_available_next_positions(map, &curr_pos) {
            if visited.get(&(pos.x, pos.y)).is_none() {
                let new_pos = pos.clone();
                visited.insert((pos.x, pos.y));
                parents.insert(pos, curr_pos.clone());

                queue.push(new_pos);
            }
        }
    }
}

fn get_available_next_positions(map: &mut Vec<Vec<char>>, curr_pos: &Node) -> Vec<Node> {
    let mut next_positions = Vec::new();
    let mut next_pos = curr_pos.clone();
    next_pos.x -= 1;
    if next_pos.x >= 0 && get_value(map, &next_pos) != '#' {
        next_pos.name = get_value(map, &next_pos);
        next_positions.push(next_pos);
    }
    next_pos = curr_pos.clone();
    next_pos.x += 1;
    if next_pos.x < map.len() as i64 && get_value(map, &next_pos) != '#' {
        next_pos.name = get_value(map, &next_pos);
        next_positions.push(next_pos);
    }
    next_pos = curr_pos.clone();
    next_pos.y -= 1;
    if next_pos.y >= 0 && get_value(map, &next_pos) != '#' {
        next_pos.name = get_value(map, &next_pos);
        next_positions.push(next_pos);
    }
    next_pos = curr_pos.clone();
    next_pos.y += 1;
    if next_pos.y < map[0].len() as i64 && get_value(map, &next_pos) != '#' {
        next_pos.name = get_value(map, &next_pos);
        next_positions.push(next_pos);
    }

    next_positions
}

fn get_value(map: &mut Vec<Vec<char>>, curr_pos: &Node) -> char {
    map[curr_pos.x as usize][curr_pos.y as usize]
}

fn locate_start_pos_and_all_keys_and_doors(
    map: &mut Vec<Vec<char>>,
) -> (Node, HashSet<Node>, HashSet<Node>) {
    let mut start_pos = Node::new('@', 0, 0);
    let mut keys = HashSet::new();
    let mut doors = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let curr_char = map[i][j];

            if curr_char == '@' {
                start_pos = Node::new('@', i as i64, j as i64);
            } else if curr_char.is_ascii_lowercase() {
                keys.insert(Node::new(curr_char, i as i64, j as i64));
            } else if curr_char.is_ascii_uppercase() {
                doors.insert(Node::new(curr_char, i as i64, j as i64));
            }
        }
    }

    (start_pos, keys, doors)
}

fn load_map(filename: String) -> Vec<Vec<char>> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let asteroid_map: Vec<Vec<char>> = contents
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
    asteroid_map
}

fn print_map(map: &mut Vec<Vec<char>>) {
    // print!("{}[2J", 27 as char);
    let w = map.len();
    let h = map[0].len();
    let mut sb = String::new();
    for i in 0..w {
        for j in 0..h {
            sb.push(map[i][j]);
        }
        sb.push('\n');
    }
    println!("{}", sb)
}

fn print_distances(map: &HashMap<(char, char), Distance>) {
    let mut string = String::new();
    string.push_str(format!("{:4}", ' ').as_str());
    let mut keys: Vec<char> = (0..26).map(|x| ('a' as u8 + x) as char).collect();
    keys.insert(0, '@');
    for i in keys.clone() {
        string.push_str(format!("{:4}", i).as_str());
    }
    string.push('\n');
    for i in 0..keys.len() {
        string.push_str(format!("{:4}", keys[i]).as_str());
        for j in 0..keys.len() {
            if let Some(dist) = map.get(&(keys[i], keys[j])) {
                string.push_str(format!("{:4}", dist.value).as_str());
            } else {
                string.push_str(format!("{:4}", "-").as_str())
            }
        }
        string.push('\n');
    }
    println!("{}", string);
}

#[cfg(test)]
mod test {
    use super::get_collect_keys_shortest_path;
    use super::load_map;

    #[test]
    fn part1_sample_input1() {
        let mut map_of_the_tunnels = load_map(String::from("test-input1.txt"));
        let (_shortest_path, steps_count) = get_collect_keys_shortest_path(&mut map_of_the_tunnels);
        assert_eq!(steps_count, 8);
    }

    #[test]
    fn part1_sample_input2() {
        let mut map_of_the_tunnels = load_map(String::from("test-input2.txt"));
        let (_shortest_path, steps_count) = get_collect_keys_shortest_path(&mut map_of_the_tunnels);
        assert_eq!(steps_count, 86);
    }

    #[test]
    fn part1_sample_input3() {
        let mut map_of_the_tunnels = load_map(String::from("test-input3.txt"));
        let (_shortest_path, steps_count) = get_collect_keys_shortest_path(&mut map_of_the_tunnels);
        assert_eq!(steps_count, 132);
    }

    #[test]
    fn part1_sample_input4() {
        let mut map_of_the_tunnels = load_map(String::from("test-input4.txt"));
        let (_shortest_path, steps_count) = get_collect_keys_shortest_path(&mut map_of_the_tunnels);
        assert_eq!(steps_count, 136);
    }

    #[test]
    fn part1_sample_input5() {
        let mut map_of_the_tunnels = load_map(String::from("test-input5.txt"));
        let (_shortest_path, steps_count) = get_collect_keys_shortest_path(&mut map_of_the_tunnels);
        assert_eq!(steps_count, 81);
    }

    // #[test]
    // fn part2_sample_input1() {
    //     let mut real_signal_input = Vec::new();
    //     for _ in 0..10000 {
    //         real_signal_input.append(&mut vec![
    //             0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5, 4, 7,
    //             4, 6, 6, 4,
    //         ]);
    //     }
    //     let mut res = real_signal_input.clone();
    //     let offset = convert_offset(res.clone());
    //     for _ in 0..100 {
    //         calculate_phase2(&mut res, &vec![0, 1, 0, -1]);
    //     }
    //     let (_, rest) = res.split_at(offset as usize);
    //     let (result, _) = rest.split_at(8);
    //     assert_eq!(result, [8, 4, 4, 6, 2, 0, 2, 6]);
    // }

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
    //         calculate_phase2(&mut res, &vec![0, 1, 0, -1]);
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
    //         calculate_phase2(&mut res, &vec![0, 1, 0, -1]);
    //     }
    //     let (_, rest) = res.split_at(offset as usize);
    //     let (result, _) = rest.split_at(8);
    //     assert_eq!(result, [5, 3, 5, 5, 3, 7, 3, 1]);
    // }
}
