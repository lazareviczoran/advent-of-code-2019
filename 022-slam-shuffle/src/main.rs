use std::fs::File;
use std::io::prelude::*;

fn main() {
    let shuffle_list = load_shuffle_list("input.txt");
    let mut deck = (0..10007 as i64).collect();
    shuffle_deck_by_list(&mut deck, &shuffle_list);
    let mut res_pos = 0;
    while deck[res_pos] != 2019 {
        res_pos += 1;
    }

    println!("Slam Shuffle part1 Solution: {}", res_pos);

    let mut deck = (0..119315717514047 as i64).collect();
    let mut i = 0;
    while i < 101741582076661i64 {
        shuffle_deck_by_list(&mut deck, &shuffle_list);
        i += 1;
    }
    let mut res_pos = 0;
    while deck[res_pos] != 2020 {
        res_pos += 1;
    }

    println!("Slam Shuffle part2 Solution: {}", res_pos);
}

#[derive(Clone, Debug)]
struct ShuffleType {
    name: String,
    arg: i64,
}

fn shuffle_deck_by_list(deck: &mut Vec<i64>, shuffle_list: &Vec<ShuffleType>) {
    for shuffle_type in shuffle_list {
        match shuffle_type.name.as_ref() {
            "deal with increment" => deal_with_increment_shuffle(deck, shuffle_type.arg),
            "cut" => cut_shuffle(deck, shuffle_type.arg),
            "deal into new stack" => deal_into_new_stack_shuffle(deck),
            _ => panic!("invalid shuffle type {}", shuffle_type.name),
        }
    }
}

fn deal_into_new_stack_shuffle(cards: &mut Vec<i64>) {
    let mut res = Vec::new();
    for i in 0..cards.len() {
        res.insert(0, cards[i]);
    }
    *cards = res;
}

fn cut_shuffle(cards: &mut Vec<i64>, n: i64) {
    let mut cut_pos = n;
    if cut_pos < 0 {
        cut_pos += cards.len() as i64;
    }
    let mut res = Vec::new();
    let (left, right) = cards.split_at(cut_pos as usize);
    res.append(&mut right.to_vec());
    res.append(&mut left.to_vec());
    *cards = res;
}

fn deal_with_increment_shuffle(cards: &mut Vec<i64>, n: i64) {
    let mut res = vec![-1; cards.len()];
    let cards_count = cards.len() as i64;
    for i in 0..cards.len() {
        let curr = (i as i64 * n + shift) % cards_count;

        res[curr as usize] = cards[i];
    }
    *cards = res;
}

fn load_shuffle_list(filename: &str) -> Vec<ShuffleType> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input file");
    contents
        .split_terminator('\n')
        .map(|v| {
            let text = v.to_string();
            let name;
            let mut arg = 0;
            if text.contains("deal with increment") || text.contains("cut") {
                let (shuffle, arg_str) = text.split_at(text.rfind(' ').unwrap());
                name = shuffle.to_string();
                arg = arg_str.trim().parse::<i64>().unwrap();
            } else {
                name = text;
            }
            ShuffleType {
                name: name,
                arg: arg,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::cut_shuffle;
    use super::deal_into_new_stack_shuffle;
    use super::deal_with_increment_shuffle;
    use super::load_shuffle_list;
    use super::shuffle_deck_by_list;

    #[test]
    fn shuffle_test1() {
        let mut deck = (0..10 as i64).collect();
        deal_into_new_stack_shuffle(&mut deck);
        assert_eq!(deck, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn shuffle_test2() {
        let mut deck = (0..10 as i64).collect();
        cut_shuffle(&mut deck, 3);
        assert_eq!(deck, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn shuffle_test3() {
        let mut deck = (0..10 as i64).collect();
        cut_shuffle(&mut deck, -4);
        assert_eq!(deck, [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn shuffle_test4() {
        let mut deck = (0..10 as i64).collect();
        deal_with_increment_shuffle(&mut deck, 3);
        assert_eq!(deck, [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn part1_sample_input1() {
        let mut deck = (0..10 as i64).collect();
        let shuffle_list = load_shuffle_list("test-input1.txt");
        shuffle_deck_by_list(&mut deck, &shuffle_list);
        assert_eq!(deck, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn part1_sample_input2() {
        let mut deck = (0..10 as i64).collect();
        let shuffle_list = load_shuffle_list("test-input2.txt");
        shuffle_deck_by_list(&mut deck, &shuffle_list);
        assert_eq!(deck, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn part1_sample_input3() {
        let mut deck = (0..10 as i64).collect();
        let shuffle_list = load_shuffle_list("test-input3.txt");
        shuffle_deck_by_list(&mut deck, &shuffle_list);
        assert_eq!(deck, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn part1_sample_input4() {
        let mut deck = (0..10 as i64).collect();
        let shuffle_list = load_shuffle_list("test-input4.txt");
        shuffle_deck_by_list(&mut deck, &shuffle_list);
        assert_eq!(deck, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
