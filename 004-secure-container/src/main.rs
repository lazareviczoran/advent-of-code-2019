use std::collections::HashMap;

fn main() {
    let (passwords1, passwords2) = get_potential_password_count();
    println!("Secure Container part1 Solution: {}", passwords1);
    println!("Secure Container part2 Solution: {}", passwords2);
}

fn get_potential_password_count() -> (i32, i32) {
    let mut count = 0;
    let mut count2 = 0;
    for pass in 137683..=596253 {
        let (is_valid, is_valid_with_extra_rule) = is_valid_password(pass);
        if is_valid {
            count = count + 1;
        }
        if is_valid_with_extra_rule {
            count2 = count2 + 1;
        }
    }
    (count, count2)
}

fn is_valid_password(password: i32) -> (bool, bool) {
    if password < 137683 && password > 596253 {
        return (false, false);
    }

    let mut prev_char: char = ' ';
    let str_pass = password.to_string();
    let mut str_pass_chars = str_pass.chars();
    let mut has_repeated_digits = false;
    let mut repeadet_digits: HashMap<char, i32> = HashMap::new();
    while let Some(digit) = str_pass_chars.next() {
        if prev_char != ' ' {
            if prev_char > digit {
                return (false, false);
            } else if prev_char == digit {
                has_repeated_digits = true;
                if let Some(digit_count) = repeadet_digits.get(&digit) {
                    repeadet_digits.insert(digit, digit_count + 1);
                } else {
                    repeadet_digits.insert(digit, 1);
                }
            }
        }
        prev_char = digit;
    }

    return (
        has_repeated_digits,
        repeadet_digits.values().find(|&&v| v == 1).is_some(),
    );
}

#[cfg(test)]
mod test {
    use super::is_valid_password;

    #[test]
    fn first_sample_input() {
        assert_eq!(is_valid_password(222222), (true, false));
    }

    #[test]
    fn second_sample_input() {
        assert_eq!(is_valid_password(223450), (false, false));
    }

    #[test]
    fn third_sample_input() {
        assert_eq!(is_valid_password(234789), (false, false));
    }

    #[test]
    fn fourth_sample_input() {
        assert_eq!(is_valid_password(223344), (true, true));
    }

    #[test]
    fn fifth_sample_input() {
        assert_eq!(is_valid_password(234555), (true, false));
    }

    #[test]
    fn sixth_sample_input() {
        assert_eq!(is_valid_password(222233), (true, true));
    }
}
