use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Transformation {
    inputs: Vec<Material>,
    output: Material,
}
impl Transformation {
    pub fn new(inputs: Vec<Material>, output: Material) -> Transformation {
        Transformation { inputs, output }
    }
}

#[derive(Clone, Debug)]
struct Material {
    name: String,
    quantity: i128,
    appearance: i128,
}
impl Material {
    pub fn new(name: String, quantity: i128) -> Material {
        Material {
            name,
            quantity,
            appearance: 0,
        }
    }
}

fn main() {
    let transformations = load_transformations(String::from("input.txt"));

    let min_required_ore =
        find_min_required_ore(&mut transformations.clone(), String::from("FUEL"), 1);

    println!("Space Stoichiometry part1 Solution: {:?}", min_required_ore);

    let max_fuel_amount = calculate_max_fuel(&mut transformations.clone());
    println!("Space Stoichiometry part2 Solution: {}", max_fuel_amount);
}

fn calculate_max_fuel(transformations: &mut HashMap<String, Transformation>) -> i128 {
    let mut ore = 0;
    let mut prev;
    let mut fuel = 1000000;
    let mut increment = 1000000;
    let target_ore = 1000000000000;
    loop {
        prev = ore;
        ore = find_min_required_ore(&mut transformations.clone(), String::from("FUEL"), fuel);

        if prev >= target_ore && ore <= target_ore && increment == 1 {
            break;
        }

        if ore < target_ore {
            if ore - prev > prev {
                increment *= 2;
            }
            fuel += increment;
        } else {
            increment = (increment as f64 / 2f64).ceil() as i128;
            fuel -= increment;
        }
    }

    fuel
}

fn find_min_required_ore(
    transformations: &mut HashMap<String, Transformation>,
    node: String,
    n: i128,
) -> i128 {
    let mut total = 0;
    let mut requirements: HashMap<String, i128> = HashMap::new();
    calculate_requirements(transformations, &mut requirements, node, n);
    for req in requirements.keys() {
        let t = transformations.get(req).unwrap();
        let mut required_multiplicator =
            requirements.get(&t.output.name).unwrap() / t.output.quantity;
        let remainder = requirements.get(&t.output.name).unwrap() % t.output.quantity;
        if remainder > 0 {
            required_multiplicator += 1;
        }
        total += t.inputs[0].quantity * required_multiplicator;
    }
    total
}

fn calculate_requirements(
    transformations: &mut HashMap<String, Transformation>,
    requirements: &mut HashMap<String, i128>,
    node: String,
    n: i128,
) {
    let transf = transformations.get(&node).unwrap().clone();
    if transf.inputs.len() > 1 || transf.inputs[0].name != "ORE" {
        let mut material = transformations.get_mut(&node).unwrap();
        if material.output.appearance > 1 {
            material.output.appearance -= 1;
            return;
        }
        for i in 0..transf.inputs.len() {
            let quantity = requirements.get(&node).unwrap_or(&n);
            let mut multiplier = quantity / transf.output.quantity;
            let extra = quantity % transf.output.quantity;
            if extra > 0 {
                multiplier += 1;
            }
            let mut value = multiplier * transf.inputs[i].quantity;

            if let Some(val) = requirements.get_mut(&transf.inputs[i].name) {
                value += *val;
            }
            requirements.insert(transf.inputs[i].name.clone(), value);
        }
        requirements.remove(&node);
        for i in 0..transf.inputs.len() {
            calculate_requirements(
                transformations,
                requirements,
                transf.inputs[i].name.clone(),
                n,
            );
        }
    }
}

fn load_transformations(filename: String) -> HashMap<String, Transformation> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    let re = Regex::new(r"(\d+) ([A-Za-z]+)").unwrap();
    let mut appearance_count: HashMap<String, i128> = HashMap::new();
    let transformations: Vec<Transformation> = contents
        .split_terminator('\n')
        .map(|s| {
            let mut inputs = Vec::new();
            let mut output = Material::new("".to_string(), 0);
            let mut iter = re.captures_iter(s).peekable();
            while let Some(caps) = iter.next() {
                if iter.peek().is_none() {
                    output.name = caps[2].to_string();
                    output.quantity = caps[1].parse::<i128>().unwrap();
                } else {
                    let material =
                        Material::new(caps[2].to_string(), caps[1].parse::<i128>().unwrap());
                    let mut value = 1;
                    if let Some(count) = appearance_count.get(&material.name) {
                        value += count;
                    }
                    appearance_count.insert(material.name.clone(), value);
                    inputs.push(material);
                }
            }
            Transformation::new(inputs, output)
        })
        .collect();
    let mut map = HashMap::new();
    for transf in transformations {
        let mut t = transf.clone();
        t.output.appearance = *appearance_count.get(&t.output.name).unwrap_or(&0);
        map.insert(transf.output.name, t);
    }
    map
}

#[cfg(test)]
mod test {
    use super::calculate_max_fuel;
    use super::find_min_required_ore;
    use super::load_transformations;

    #[test]
    fn part1_sample_input1() {
        let mut map = load_transformations(String::from("test-input.txt"));
        let required_ores = find_min_required_ore(&mut map, String::from("FUEL"), 1);
        assert_eq!(required_ores, 31);
    }

    #[test]
    fn part1_sample_input2() {
        let mut map = load_transformations(String::from("test-input2.txt"));
        let required_ores = find_min_required_ore(&mut map, String::from("FUEL"), 1);
        assert_eq!(required_ores, 165);
    }

    #[test]
    fn part1_sample_input3() {
        let mut map = load_transformations(String::from("test-input3.txt"));
        let required_ores = find_min_required_ore(&mut map, String::from("FUEL"), 1);
        assert_eq!(required_ores, 13312);
    }

    #[test]
    fn part1_sample_input4() {
        let mut map = load_transformations(String::from("test-input4.txt"));
        let required_ores = find_min_required_ore(&mut map, String::from("FUEL"), 1);
        assert_eq!(required_ores, 180697);
    }

    #[test]
    fn part1_sample_input5() {
        let mut map = load_transformations(String::from("test-input5.txt"));
        let required_ores = find_min_required_ore(&mut map, String::from("FUEL"), 1);
        assert_eq!(required_ores, 2210736);
    }

    #[test]
    fn part2_sample_input3() {
        let mut map = load_transformations(String::from("test-input3.txt"));
        let max_fueld_amount = calculate_max_fuel(&mut map);
        assert_eq!(max_fueld_amount, 82892753);
    }

    #[test]
    fn part2_sample_input4() {
        let mut map = load_transformations(String::from("test-input4.txt"));
        let max_fueld_amount = calculate_max_fuel(&mut map);
        assert_eq!(max_fueld_amount, 5586022);
    }

    #[test]
    fn part2_sample_input5() {
        let mut map = load_transformations(String::from("test-input5.txt"));
        let max_fueld_amount = calculate_max_fuel(&mut map);
        assert_eq!(max_fueld_amount, 460664);
    }
}
