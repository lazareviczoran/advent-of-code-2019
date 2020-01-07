use num::bigint::BigInt;
use num::bigint::ToBigInt;
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

    println!(
        "Slam Shuffle part2 Solution: {}",
        find_nth_start_pos(119315717514047, 2020, &shuffle_list, 101741582076661)
    );
}

fn find_nth_start_pos(
    deck_size: i64,
    target_pos: i64,
    shuffle_list: &Vec<ShuffleType>,
    iteration: i64,
) -> BigInt {
    let list = shuffle_list.clone();
    let first = find_position(deck_size, 0, &list, 1);
    let second = find_position(deck_size, 1, &list, 1);

    let mut a = (second - first) % deck_size;
    if a < 0 {
        a = (a + deck_size) % deck_size;
    }
    let a_k = modular_exponentiation(
        &BigInt::from(a),
        &BigInt::from(iteration),
        &BigInt::from(deck_size),
    );
    let inv = inverse_mod(BigInt::from(a - 1), BigInt::from(deck_size));
    let mut b = ((a_k.clone() - 1) * inv) % deck_size;
    b *= first;

    let inv2 = inverse_mod(BigInt::from(a_k), BigInt::from(deck_size));

    let mut res: BigInt = (target_pos - b) * inv2;
    res %= deck_size;
    if res < BigInt::from(0) {
        res = (res + deck_size) % deck_size
    }
    res
}

fn find_position(
    deck_size: i64,
    init_pos: i64,
    shuffle_list: &Vec<ShuffleType>,
    iteration: i64,
) -> i64 {
    let mut pos = init_pos;
    for _ in 0..iteration {
        for shuffle_type in shuffle_list {
            pos = match shuffle_type.name.as_ref() {
                "deal with increment" => deal_with_increment(deck_size, pos, shuffle_type.arg),
                "cut" => cut(deck_size, pos, shuffle_type.arg),
                "deal into new stack" => deal_into_new_stack(deck_size, pos),
                _ => panic!("invalid shuffle type {}", shuffle_type.name),
            }
        }
    }

    pos
}

fn cut(deck_size: i64, pos: i64, n: i64) -> i64 {
    (pos - n + deck_size) % deck_size
}

fn deal_with_increment(deck_size: i64, pos: i64, n: i64) -> i64 {
    pos * n % deck_size
}

fn deal_into_new_stack(deck_size: i64, pos: i64) -> i64 {
    (-pos - 1 + deck_size) % deck_size
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
        let curr = (i as i64 * n) % cards_count;

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

fn inverse_mod(a: BigInt, b: BigInt) -> BigInt {
    let res = egcd(a, b.clone());

    res.1 % b
}

fn egcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if a == BigInt::from(0) {
        return (b, BigInt::from(0), BigInt::from(1));
    }
    let (gcd, x, y) = egcd(b.clone() % a.clone(), a.clone());
    (gcd, y - (b / a) * x.clone(), x)
}

// The modular_exponentiation() function takes three identical types
// (which get cast to BigInt), and returns a BigInt:
fn modular_exponentiation<T: ToBigInt>(n: &T, e: &T, m: &T) -> BigInt {
    // Convert n, e, and m to BigInt:
    let n = n.to_bigint().unwrap();
    let e = e.to_bigint().unwrap();
    let m = m.to_bigint().unwrap();

    // Sanity check:  Verify that the exponent is not negative:
    assert!(e >= Zero::zero());

    use num::traits::{One, Zero};

    // As most modular exponentiations do, return 1 if the exponent is 0:
    if e == Zero::zero() {
        return One::one();
    }

    // Now do the modular exponentiation algorithm:
    let mut result: BigInt = One::one();
    let mut base = n % &m;
    let mut exp = e;

    // Loop until we can return out result:
    loop {
        if &exp % 2 == One::one() {
            result *= &base;
            result %= &m;
        }

        if exp == One::one() {
            return result;
        }

        exp /= 2;
        base *= base.clone();
        base %= &m;
    }
}

#[cfg(test)]
mod test {
    use super::cut;
    use super::cut_shuffle;
    use super::deal_into_new_stack;
    use super::deal_into_new_stack_shuffle;
    use super::deal_with_increment;
    use super::deal_with_increment_shuffle;
    use super::find_nth_start_pos;
    use super::find_position;
    use super::load_shuffle_list;
    use super::shuffle_deck_by_list;
    use num::bigint::BigInt;

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
    fn rshuffle_test1() {
        assert_eq!(deal_into_new_stack(10, 2), 7);
        assert_eq!(deal_into_new_stack(10, 6), 3);
    }

    #[test]
    fn rshuffle_test2() {
        assert_eq!(cut(10, 7, 3), 4);
        assert_eq!(cut(10, 0, 3), 7);
    }

    #[test]
    fn rshuffle_test3() {
        assert_eq!(cut(10, 8, -4), 2);
        assert_eq!(cut(10, 3, -4), 7);
    }

    #[test]
    fn rshuffle_test4() {
        assert_eq!(deal_with_increment(10, 8, 3), 4);
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

    #[test]
    fn part2_sample_input1() {
        let shuffle_list = load_shuffle_list("test-input1.txt");
        assert_eq!(find_position(10, 3, &shuffle_list, 1), 1);
    }

    #[test]
    fn part2_sample_input2() {
        let shuffle_list = load_shuffle_list("test-input2.txt");
        assert_eq!(find_position(10, 3, &shuffle_list, 1), 0);
    }

    #[test]
    fn part2_sample_input3() {
        let shuffle_list = load_shuffle_list("test-input3.txt");
        assert_eq!(find_position(10, 6, &shuffle_list, 1), 0);
    }

    #[test]
    fn part2_sample_input4() {
        let shuffle_list = load_shuffle_list("test-input4.txt");
        assert_eq!(find_position(10, 2, &shuffle_list, 1), 1);
    }

    #[test]
    fn part2_test() {
        let shuffle_list = load_shuffle_list("input.txt");
        assert_eq!(
            find_nth_start_pos(10007, 4703, &shuffle_list, 1),
            BigInt::from(2019)
        );
    }
}
