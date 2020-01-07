use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let map = load_map("input.txt");
    println!(
        "Planet of Discord part1 Solution: {}",
        calculate_biodiversity_rating(&mut map.clone())
    );

    println!(
        "Planet of Discord part2 Solution: {}",
        find_bugs_in_recursive_area(&mut map.clone(), 200)
    );
}

fn find_bugs_in_recursive_area(map: &mut Vec<Vec<char>>, iterations: usize) -> i64 {
    let mut maps: HashMap<i64, Vec<Vec<char>>> = HashMap::new();
    let width = map.len();
    let height = map[0].len();
    let empty_map = vec![vec!['.'; height]; width];

    maps.insert(0, map.clone());
    for i in 0..iterations {
        let mut count_map: HashMap<(i64, i64, i64), (char, i64)> = HashMap::new();
        let next_inner = i as i64 + 1;
        let next_outer = -next_inner;
        if maps.get_mut(&next_outer).is_none() {
            maps.insert(next_outer, vec![vec!['.'; height]; width]);
        }
        if maps.get_mut(&next_inner).is_none() {
            maps.insert(next_inner, vec![vec!['.'; height]; width]);
        }
        let maps_clone = maps.clone();
        for level in maps_clone.keys() {
            for j in 0..height {
                for i in 0..width {
                    detect_adjs_recursively(
                        &mut maps,
                        (i as i64, j as i64, *level),
                        &mut count_map,
                    );
                }
            }
        }

        for (level, level_map) in maps.iter_mut() {
            for j in 0..height {
                for i in 0..width {
                    if i == 2 && j == 2 {
                        continue;
                    }
                    if let Some((val, bugs)) = count_map.get(&(i as i64, j as i64, *level)) {
                        if *val == '#' && *bugs != 1 {
                            level_map[i][j] = '.';
                        } else if *val == '.' && (*bugs == 1 || *bugs == 2) {
                            level_map[i][j] = '#';
                        }
                    }
                }
            }
        }
    }

    let mut bugs_count = 0;
    let mut sorted_keys = Vec::new();
    for key in maps.keys() {
        sorted_keys.push(key);
    }
    sorted_keys.sort();
    for level in sorted_keys {
        let level_map = maps.get(level).unwrap();
        if *level_map != empty_map {
            for j in 0..height {
                for i in 0..width {
                    if level_map[i][j] == '#' {
                        bugs_count += 1;
                    }
                }
            }
        }
    }
    bugs_count
}

fn detect_adjs_recursively(
    maps: &mut HashMap<i64, Vec<Vec<char>>>,
    (x, y, level): (i64, i64, i64),
    count_map: &mut HashMap<(i64, i64, i64), (char, i64)>,
) {
    let outer_level = level - 1;
    let inner_level = level + 1;
    let curr_level_map = maps.get_mut(&level).unwrap().clone();
    let width = curr_level_map.len();
    let height = curr_level_map[0].len();
    let mut count_bugs = 0;
    // count left
    let next_x = x - 1;
    if next_x < 0 {
        if let Some(outer_map) = maps.get_mut(&outer_level) {
            if outer_map[1][2] == '#' {
                count_bugs += 1;
            }
        } else {
            maps.insert(outer_level, vec![vec!['.'; height]; width]);
        }
    } else if next_x == 2 && y == 2 {
        if let Some(inner_map) = maps.get_mut(&inner_level) {
            let new_x;
            if x == 1 {
                new_x = 0;
            } else {
                new_x = inner_map.len() - 1;
            }
            for new_y in 0..inner_map[0].len() {
                if inner_map[new_x][new_y] == '#' {
                    count_bugs += 1;
                }
            }
        } else {
            maps.insert(inner_level, vec![vec!['.'; height]; width]);
        }
    } else if curr_level_map[next_x as usize][y as usize] == '#' {
        count_bugs += 1;
    }
    // count right
    let next_x = x + 1;
    if next_x == width as i64 {
        if let Some(outer_map) = maps.get_mut(&outer_level) {
            if outer_map[3][2] == '#' {
                count_bugs += 1;
            }
        } else {
            maps.insert(outer_level, vec![vec!['.'; height]; width]);
        }
    } else if next_x == 2 && y == 2 {
        if let Some(inner_map) = maps.get_mut(&inner_level) {
            let new_x;
            if x == 1 {
                new_x = 0;
            } else {
                new_x = inner_map.len() - 1;
            }
            for new_y in 0..inner_map[0].len() {
                if inner_map[new_x][new_y] == '#' {
                    count_bugs += 1;
                }
            }
        } else {
            maps.insert(inner_level, vec![vec!['.'; height]; width]);
        }
    } else if curr_level_map[next_x as usize][y as usize] == '#' {
        count_bugs += 1;
    }
    // count up
    let next_y = y - 1;
    if next_y < 0 {
        if let Some(outer_map) = maps.get_mut(&outer_level) {
            if outer_map[2][1] == '#' {
                count_bugs += 1;
            }
        } else {
            maps.insert(outer_level, vec![vec!['.'; height]; width]);
        }
    } else if next_y == 2 && x == 2 {
        if let Some(inner_map) = maps.get_mut(&inner_level) {
            let new_y;
            if y == 1 {
                new_y = 0;
            } else {
                new_y = inner_map[0].len() - 1;
            }
            for new_x in 0..inner_map[0].len() {
                if inner_map[new_x][new_y] == '#' {
                    count_bugs += 1;
                }
            }
        } else {
            maps.insert(inner_level, vec![vec!['.'; height]; width]);
        }
    } else if curr_level_map[x as usize][next_y as usize] == '#' {
        count_bugs += 1;
    }
    // count down
    let next_y = y + 1;
    if next_y == height as i64 {
        if let Some(outer_map) = maps.get_mut(&outer_level) {
            if outer_map[2][3] == '#' {
                count_bugs += 1;
            }
        } else {
            maps.insert(outer_level, vec![vec!['.'; height]; width]);
        }
    } else if next_y == 2 && x == 2 {
        if let Some(inner_map) = maps.get_mut(&inner_level) {
            let new_y;
            if y == 1 {
                new_y = 0;
            } else {
                new_y = inner_map[0].len() - 1;
            }
            for new_x in 0..inner_map[0].len() {
                if inner_map[new_x][new_y] == '#' {
                    count_bugs += 1;
                }
            }
        } else {
            maps.insert(inner_level, vec![vec!['.'; height]; width]);
        }
    } else if curr_level_map[x as usize][next_y as usize] == '#' {
        count_bugs += 1;
    }
    count_map.insert(
        (x, y, level),
        (curr_level_map[x as usize][y as usize], count_bugs),
    );
}

fn calculate_biodiversity_rating(map: &mut Vec<Vec<char>>) -> i64 {
    let mut prev_states = HashSet::new();
    loop {
        prev_states.insert(map.clone());
        let mut count_map: HashMap<(i64, i64), (char, i64)> = HashMap::new();
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                detect_adjs(map, (i, j), &mut count_map);
            }
        }

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if let Some((val, bugs)) = count_map.get(&(i as i64, j as i64)) {
                    if *val == '#' && *bugs != 1 {
                        map[i][j] = '.';
                    } else if *val == '.' && (*bugs == 1 || *bugs == 2) {
                        map[i][j] = '#';
                    }
                }
            }
        }

        if let Some(_state) = prev_states.get(map) {
            let mut res = 0;
            for i in 0..map.len() {
                for j in 0..map[0].len() {
                    if map[i][j] == '#' {
                        res += 2i64.pow((j * map[0].len() + i) as u32);
                    }
                }
            }
            return res;
        }
    }
}

fn detect_adjs(
    map: &mut Vec<Vec<char>>,
    (x, y): (usize, usize),
    count_map: &mut HashMap<(i64, i64), (char, i64)>,
) {
    let mut count_bugs = 0;
    if x > 0 && map[x - 1][y] == '#' {
        count_bugs += 1;
    }
    if x < map.len() - 1 && map[x + 1][y] == '#' {
        count_bugs += 1;
    }
    if y > 0 && map[x][y - 1] == '#' {
        count_bugs += 1;
    }
    if y < map[0].len() - 1 && map[x][y + 1] == '#' {
        count_bugs += 1;
    }
    count_map.insert((x as i64, y as i64), (map[x][y], count_bugs));
}

fn print_map(map: Vec<Vec<char>>) {
    // print!("{}[2J", 27 as char);
    let w = map.len();
    let h = map[0].len();
    let mut sb = String::new();
    for j in 0..h {
        for i in 0..w {
            if i == 2 && j == 2 {
                sb.push('?');
            } else {
                sb.push(map[i][j]);
            }
        }
        sb.push('\n');
    }
    println!("{}", sb)
}

fn load_map(filename: &str) -> Vec<Vec<char>> {
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

    let width = map[0].len();
    let height = map.len();
    let mut res = vec![vec![' '; height]; width];
    for i in 0..width {
        for j in 0..height {
            res[i][j] = map[j][i];
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::calculate_biodiversity_rating;
    use super::find_bugs_in_recursive_area;
    use super::load_map;

    #[test]
    fn part1_sample_input1() {
        let mut map = load_map("test-input.txt");
        assert_eq!(calculate_biodiversity_rating(&mut map), 2129920);
    }

    #[test]
    fn part2_sample_input1() {
        let mut map = load_map("test-input.txt");
        assert_eq!(find_bugs_in_recursive_area(&mut map, 10), 99);
    }
}
