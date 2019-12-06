use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let relations: Vec<&str> = contents.split_terminator('\n').collect();
    let nodes_map = init_tree(relations);
    let root = nodes_map.values().find(|n| n.parent.is_none()).unwrap();
    let you = nodes_map.values().find(|n| n.name == "YOU").unwrap();

    println!(
        "Universal Orbit Map part1 Solution: {}",
        calculate_orbits_number(root.clone(), &nodes_map, 0)
    );
    println!(
        "Universal Orbit Map part2 Solution: {}",
        calculate_orbital_transfers_to_santa(
            &you.parent.as_ref().unwrap(),
            &nodes_map,
            0,
            HashSet::new()
        )
    );
}

#[derive(Clone, Debug)]
struct Node {
    name: String,
    parent: Option<String>,
    children: Vec<String>,
    visited: bool,
}
impl Node {
    pub fn new(name: String, parent: Option<String>) -> Node {
        Node {
            name,
            parent,
            children: Vec::new(),
            visited: false,
        }
    }
}

fn init_tree(relations: Vec<&str>) -> HashMap<String, Node> {
    let mut nodes_map: HashMap<String, Node> = HashMap::new();
    for relation in relations {
        let elements: Vec<&str> = relation.split_terminator(')').collect();
        if let Some(node1) = nodes_map.get_mut(elements[0]) {
            node1.children.push(elements[1].to_string());
        } else {
            let mut node = Node::new(elements[0].to_string(), None);
            node.children.push(elements[1].to_string());
            nodes_map.insert(elements[0].to_string(), node);
        }
        if let Some(node2) = nodes_map.get_mut(elements[1]) {
            node2.parent = Some(elements[0].to_string());
        } else {
            nodes_map.insert(
                elements[1].to_string(),
                Node::new(elements[1].to_string(), Some(elements[0].to_string())),
            );
        }
    }
    nodes_map
}

fn calculate_orbits_number(
    curr_node: Node,
    nodes_map: &HashMap<String, Node>,
    curr_sum: i32,
) -> i32 {
    if curr_node.children.is_empty() {
        return curr_sum;
    }
    let mut sum = curr_sum;
    for child in curr_node.children {
        let child_node = (*nodes_map).get(&child).unwrap().clone();
        sum = sum + calculate_orbits_number(child_node, nodes_map, curr_sum + 1);
    }
    sum
}

fn calculate_orbital_transfers_to_santa(
    curr_node_id: &String,
    nodes_map: &HashMap<String, Node>,
    curr_distance: i32,
    visited_nodes: HashSet<String>,
) -> i32 {
    if is_in_same_orbit_with_santa(curr_node_id.clone(), nodes_map) {
        return curr_distance - 1;
    }
    let nodes_to_visit_id_list =
        get_available_nodes(curr_node_id, nodes_map, visited_nodes.clone());
    if nodes_to_visit_id_list.is_empty() {
        return i32::max_value();
    }
    let mut min_dist = i32::max_value();
    for node_id in nodes_to_visit_id_list {
        let mut new_visited_nodes: HashSet<String> = visited_nodes.clone();
        new_visited_nodes.insert(node_id.clone());
        let new_dist = calculate_orbital_transfers_to_santa(
            &node_id,
            nodes_map,
            curr_distance + 1,
            new_visited_nodes,
        );
        if min_dist > new_dist {
            min_dist = new_dist;
        }
    }
    return min_dist;
}

fn is_in_same_orbit_with_santa(node_id: String, nodes_map: &HashMap<String, Node>) -> bool {
    let curr_node = nodes_map.get(&node_id).unwrap();
    let parent = &curr_node.parent;
    let santa = String::from("SAN");
    if parent.is_some() {
        let mut curr_node_parent = nodes_map.get(parent.as_ref().unwrap()).unwrap();
        if curr_node_parent.children.contains(&santa) {
            return true;
        }
        while curr_node_parent.parent.is_some() {
            if curr_node_parent.name == santa {
                return true;
            }
            curr_node_parent = nodes_map
                .get(curr_node_parent.parent.as_ref().unwrap())
                .unwrap()
        }
    }
    return false;
}

fn get_available_nodes(
    curr_node_id: &String,
    nodes_map: &HashMap<String, Node>,
    visited_nodes: HashSet<String>,
) -> Vec<String> {
    let curr_node = nodes_map.get(curr_node_id).unwrap();
    let mut available_nodes = Vec::new();
    if let Some(parent) = &curr_node.parent {
        if !visited_nodes.contains(parent) {
            available_nodes.push(parent.to_string())
        }
    }
    if !curr_node.children.is_empty() {
        for child in &curr_node.children {
            if !visited_nodes.contains(child) {
                available_nodes.push(child.to_string())
            }
        }
    }
    available_nodes
}

#[cfg(test)]
mod test {
    use super::calculate_orbital_transfers_to_santa;
    use super::calculate_orbits_number;
    use super::init_tree;
    use std::collections::HashSet;

    #[test]
    fn p1_sample_input() {
        let relations = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];

        let nodes_map = init_tree(relations);
        let root = nodes_map.values().find(|n| n.parent.is_none()).unwrap();

        assert_eq!(calculate_orbits_number(root.clone(), &nodes_map, 0), 42);
    }

    #[test]
    fn p2_sample_input() {
        let relations = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];

        let nodes_map = init_tree(relations);
        let root = nodes_map.values().find(|n| n.name == "YOU").unwrap();

        assert_eq!(
            calculate_orbital_transfers_to_santa(
                &root.parent.as_ref().unwrap(),
                &nodes_map,
                0,
                HashSet::new()
            ),
            4
        );
    }
}
