use std::collections::HashMap;
use std::f64;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    distance: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y, distance: 0 }
    }
}

fn main() {
    let mut asteroid_map = load_map(String::from("input.txt"));

    let (best_location, detection_count) = find_best_detection_location(&mut asteroid_map);

    println!("Monitoring Station part1 Solution: {:?}", detection_count);

    let vaporized_asteroids = destroy_asteroids(&mut asteroid_map, best_location);
    println!(
        "Monitoring Station part2 Solution: {}",
        vaporized_asteroids[199].x * 100 + vaporized_asteroids[199].y
    );
}

fn destroy_asteroids(asteroid_map: &mut Vec<Vec<char>>, laser_location: Point) -> Vec<Point> {
    let mut detected_asteroids =
        find_all_detections(asteroid_map, laser_location.x, laser_location.y);
    let mut vaporized_asteroids = Vec::new();
    let mut curr_rotation;
    let mut angles: Vec<String> = detected_asteroids.keys().cloned().collect();
    angles.sort_by(|a, b| {
        return a
            .parse::<f64>()
            .unwrap()
            .partial_cmp(&b.parse::<f64>().unwrap())
            .unwrap();
    });
    let mut i = angles.len() - 1;

    while !detected_asteroids.is_empty() {
        curr_rotation = angles[i].clone();
        if let Some(array) = detected_asteroids.get_mut(&curr_rotation) {
            if array.len() > 0 {
                let item = array.remove(0);
                vaporized_asteroids.push(item);
            }
            if array.len() == 0 {
                detected_asteroids.remove(&curr_rotation);
            }
        }
        i = (i + angles.len() - 1) % angles.len();
    }

    vaporized_asteroids
}

fn find_best_detection_location(asteroid_map: &mut Vec<Vec<char>>) -> (Point, usize) {
    let row_count = asteroid_map.len();
    let column_count = asteroid_map[0].len();
    let mut most_detection_counts = 0;
    let mut best_position = None;
    for y in 0..column_count {
        for x in 0..row_count {
            if asteroid_map[x][y] == '#' {
                let detection_count = find_all_detections(asteroid_map, y as i32, x as i32)
                    .keys()
                    .len();
                if detection_count > most_detection_counts {
                    most_detection_counts = detection_count;
                    // creating (y,x) point
                    best_position = Some(Point::new(y as i32, x as i32));
                }
            }
        }
    }
    (best_position.unwrap(), most_detection_counts)
}

fn find_all_detections(
    asteroid_map: &mut Vec<Vec<char>>,
    y: i32,
    x: i32,
) -> HashMap<String, Vec<Point>> {
    let mut detected_asteroids: HashMap<String, Vec<Point>> = HashMap::new();
    let row_count = asteroid_map.len();
    let column_count = asteroid_map[0].len();
    for j in 0..column_count as i32 {
        for i in 0..row_count as i32 {
            if x == i && y == j {
                continue;
            } else if asteroid_map[i as usize][j as usize] == '#' {
                // calculate angle, put into map and order by distance
                let angle = ((j - y) as f64).atan2((i - x) as f64).to_string();
                let mut curr_point = Point::new(j, i);
                let curr_distance =
                    get_distance_between_points(Point::new(y, x), curr_point.clone());
                curr_point.distance = curr_distance;
                if let Some(array) = detected_asteroids.get_mut(&angle) {
                    let mut index = 0;
                    while index < array.len() && array[index].distance < curr_distance {
                        index += 1;
                    }
                    array.insert(index, curr_point);
                } else {
                    detected_asteroids.insert(angle, vec![curr_point]);
                }
            }
        }
    }

    detected_asteroids
}

fn get_distance_between_points(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
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

#[cfg(test)]
mod test {
    use super::destroy_asteroids;
    use super::find_best_detection_location;
    use super::load_map;

    #[test]
    fn part1_sample_input1() {
        let mut map = load_map(String::from("test-input.txt"));
        let (location, _) = find_best_detection_location(&mut map);
        assert_eq!(vec![location.x, location.y], [3, 4]);
    }

    #[test]
    fn part1_sample_input2() {
        let mut map = load_map(String::from("test-input2.txt"));
        let (location, _) = find_best_detection_location(&mut map);
        assert_eq!(vec![location.x, location.y], [5, 8]);
    }

    #[test]
    fn part1_sample_input3() {
        let mut map = load_map(String::from("test-input3.txt"));
        let (location, _) = find_best_detection_location(&mut map);
        assert_eq!(vec![location.x, location.y], [1, 2]);
    }

    #[test]
    fn part1_sample_input4() {
        let mut map = load_map(String::from("test-input4.txt"));
        let (location, _) = find_best_detection_location(&mut map);
        assert_eq!(vec![location.x, location.y], [6, 3]);
    }

    #[test]
    fn part1_sample_input5() {
        let mut map = load_map(String::from("test-input5.txt"));
        let (location, _) = find_best_detection_location(&mut map);
        assert_eq!(vec![location.x, location.y], [11, 13]);
    }

    #[test]
    fn part2_sample_input1() {
        let mut map = load_map(String::from("test-input5.txt"));
        let (location, _) = find_best_detection_location(&mut map);
        let vaporized_asteroids = destroy_asteroids(&mut map, location);
        assert_eq!(
            vec![vaporized_asteroids[199].x, vaporized_asteroids[199].y],
            [8, 2]
        );
    }
}
