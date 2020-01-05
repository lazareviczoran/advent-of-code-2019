use std::cmp::Ordering;
use std::collections::BinaryHeap;
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
    doors: i64,
}
impl Distance {
    pub fn new(distance: i64) -> Distance {
        Distance {
            value: distance,
            doors: 0,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    robots: Vec<Node>,
    remaining_keys: i64,
    cost: i64,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
        // // .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut map_of_the_tunnels = load_map(String::from("input.txt"));
    print_map(&mut map_of_the_tunnels);
    println!(
        "Many-Worlds Interpretation part1 Solution: {:?}",
        get_collect_keys_shortest_path(&mut map_of_the_tunnels)
    );

    map_of_the_tunnels = load_map(String::from("input-pt2.txt"));
    print_map(&mut map_of_the_tunnels);
    println!(
        "Many-Worlds Interpretation part2 Solution: {}",
        get_collect_keys_shortest_path2(&mut map_of_the_tunnels)
    );
}

fn get_collect_keys_shortest_path2(map: &mut Vec<Vec<char>>) -> i64 {
    let (robots, all_keys, _) = locate_start_pos_and_all_keys_and_doors(map);
    let distances = init_distance_map(map, &all_keys, &robots);
    println!("distances\n");
    print_distances(&distances, robots.clone());

    let remaining_keys = (1 << all_keys.len()) - 1;

    let mut seen: HashSet<(String, i64)> = HashSet::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        robots,
        remaining_keys,
        cost: 0,
    });

    while let Some(State {
        robots,
        remaining_keys,
        cost,
    }) = heap.pop()
    {
        let mut new_remaining_keys = remaining_keys;

        let mut curr_pos = String::new();
        for i in 0..robots.len() {
            let curr_node = robots[i].clone();
            let curr_node_name = curr_node.name;
            curr_pos.push(curr_node_name);
            if curr_node_name.is_ascii_lowercase()
                && !has_bit(new_remaining_keys, curr_node_name as u8 - 'a' as u8)
            {
                new_remaining_keys =
                    unset_bit(new_remaining_keys, curr_node_name as u8 - 'a' as u8);
            }
        }
        if seen.get(&(curr_pos.clone(), new_remaining_keys)).is_some() {
            continue;
        }
        seen.insert((curr_pos, new_remaining_keys));

        if new_remaining_keys == 0 {
            return cost;
        }

        for i in 0..robots.len() {
            let curr_node = robots[i].clone();
            let curr_node_name = curr_node.name;

            // for each unvisited and unlocked
            let mut neighbours = all_keys.clone();
            neighbours.retain(|k| {
                let distance_between = distances.get(&(curr_node_name, k.name));
                !has_bit(new_remaining_keys, k.name as u8 - 'a' as u8)
                    && distance_between.is_some()
                    && distance_between.unwrap().doors | !new_remaining_keys == !new_remaining_keys
            });
            for v in neighbours.iter() {
                let mut new_robots = robots.clone();
                new_robots[i] = v.clone();
                heap.push(State {
                    robots: new_robots,
                    remaining_keys: new_remaining_keys,
                    cost: cost + distances.get(&(curr_node_name, v.name)).unwrap().value,
                });
            }
        }
    }

    i64::max_value()
}

fn unset_bit(keys: i64, i: u8) -> i64 {
    keys & !(1 << i)
}

fn set_bit(keys: i64, i: u8) -> i64 {
    keys | (1 << i)
}

fn has_bit(keys: i64, i: u8) -> bool {
    keys & (1 << i) == 0
}

fn get_collect_keys_shortest_path(map: &mut Vec<Vec<char>>) -> i64 {
    let (robots, all_keys, _) = locate_start_pos_and_all_keys_and_doors(map);
    let start_pos = robots[0].clone();

    let distances = init_distance_map(map, &all_keys, &robots);
    println!("distances\n");
    print_distances(&distances, robots.clone());

    let mask = 0;
    let mut dp: HashMap<(i64, Node), i64> = HashMap::new();
    let shortest_path_steps = tsp(mask, start_pos.clone(), all_keys, distances, &mut dp);

    shortest_path_steps
}

fn tsp(
    mask: i64,
    curr_node: Node,
    all_keys: HashSet<Node>,
    distances: HashMap<(char, char), Distance>,
    dp: &mut HashMap<(i64, Node), i64>,
) -> i64 {
    if let Some(value) = dp.get(&(mask, curr_node.clone())) {
        return *value;
    }
    let curr_node_name = curr_node.clone().name;

    let mut ans = i64::max_value();

    // for each unvisited and unlocked
    let mut neighbours = all_keys.clone();
    neighbours.retain(|k| {
        let distance_between = distances.get(&(curr_node_name, k.name));
        has_bit(mask, k.name as u8 - 'a' as u8)
            && distance_between.is_some()
            && distance_between.unwrap().doors | mask == mask
    });
    if neighbours.len() == 0 {
        return 0;
    }
    for v in neighbours.iter() {
        let city = v.name as u8 - 'a' as u8;
        if mask & (1 << city) == 0 {
            let best_dist = tsp(
                mask | (1 << city),
                v.clone(),
                all_keys.clone(),
                distances.clone(),
                dp,
            );
            let new_ans = distances.get(&(curr_node_name, v.name)).unwrap().value + best_dist;
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
    robots: &Vec<Node>,
) -> HashMap<(char, char), Distance> {
    let mut distances = HashMap::new();
    let mut keys_with_start = keys.clone();
    for robot in robots {
        keys_with_start.insert(robot.clone());
    }
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
                        distance.doors = set_bit(distance.doors, curr_el.name as u8 - 'A' as u8);
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
) -> (Vec<Node>, HashSet<Node>, HashSet<Node>) {
    let mut robot_count = 1;
    let mut robots = Vec::new();
    let mut keys = HashSet::new();
    let mut doors = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let curr_char = map[i][j];

            if curr_char == '@' {
                let start_pos = Node::new(
                    std::char::from_digit(robot_count, 10).unwrap(),
                    i as i64,
                    j as i64,
                );
                robots.push(start_pos);
                robot_count += 1;
            } else if curr_char.is_ascii_lowercase() {
                keys.insert(Node::new(curr_char, i as i64, j as i64));
            } else if curr_char.is_ascii_uppercase() {
                doors.insert(Node::new(curr_char, i as i64, j as i64));
            }
        }
    }

    (robots, keys, doors)
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

fn print_distances(map: &HashMap<(char, char), Distance>, robots: Vec<Node>) {
    let mut string = String::new();
    string.push_str(format!("{:4}", ' ').as_str());
    let mut keys: Vec<char> = (0..26).map(|x| ('a' as u8 + x) as char).collect();
    for i in 0..robots.len() {
        keys.insert(i, robots[i].name);
    }
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
    use super::get_collect_keys_shortest_path2;
    use super::load_map;

    #[test]
    fn part1_sample_input1() {
        let mut map_of_the_tunnels = load_map(String::from("test-input1.txt"));
        assert_eq!(get_collect_keys_shortest_path2(&mut map_of_the_tunnels), 8);
    }

    #[test]
    fn part1_sample_input2() {
        let mut map_of_the_tunnels = load_map(String::from("test-input2.txt"));
        assert_eq!(get_collect_keys_shortest_path2(&mut map_of_the_tunnels), 86);
    }

    #[test]
    fn part1_sample_input3() {
        let mut map_of_the_tunnels = load_map(String::from("test-input3.txt"));
        assert_eq!(
            get_collect_keys_shortest_path2(&mut map_of_the_tunnels),
            132
        );
    }

    #[test]
    fn part1_sample_input4() {
        let mut map_of_the_tunnels = load_map(String::from("test-input4.txt"));
        assert_eq!(
            get_collect_keys_shortest_path2(&mut map_of_the_tunnels),
            136
        );
    }

    #[test]
    fn part1_sample_input5() {
        let mut map_of_the_tunnels = load_map(String::from("test-input5.txt"));
        assert_eq!(get_collect_keys_shortest_path2(&mut map_of_the_tunnels), 81);
    }

    #[test]
    fn part2_sample_input1() {
        let mut map_of_the_tunnels = load_map(String::from("test-input6.txt"));
        let steps_count = get_collect_keys_shortest_path2(&mut map_of_the_tunnels);
        assert_eq!(steps_count, 8);
    }

    #[test]
    fn part2_sample_input2() {
        let mut map_of_the_tunnels = load_map(String::from("test-input7.txt"));
        let steps_count = get_collect_keys_shortest_path2(&mut map_of_the_tunnels);
        assert_eq!(steps_count, 24);
    }

    #[test]
    fn part2_sample_input3() {
        let mut map_of_the_tunnels = load_map(String::from("test-input8.txt"));
        let steps_count = get_collect_keys_shortest_path2(&mut map_of_the_tunnels);
        assert_eq!(steps_count, 32);
    }

    #[test]
    fn part2_sample_input4() {
        let mut map_of_the_tunnels = load_map(String::from("test-input9.txt"));
        let steps_count = get_collect_keys_shortest_path2(&mut map_of_the_tunnels);
        assert_eq!(steps_count, 72);
    }
}
