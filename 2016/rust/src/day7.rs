use std::fs;
use std::slice::Windows;
fn tokens(string: &str) -> Vec<String> {
    // create a vector of the tokens.
    let mut results: Vec<String> = Vec::new();
    let mut token = String::new();

    for c in string.chars() {
        if c == '[' {
            results.push(token.clone());
            token.clear();
        }
        if c == ']' {
            token.push(c);
            results.push(token.clone());
            token.clear();
            continue;
        }

        token.push(c);
    }
    if token.len() > 0 {
        results.push(token);
    }
    results
}

fn is_abba(slice: &[char]) -> bool {
    if slice[0] != slice[3] {
        return false;
    }
    if slice[1] != slice[2] {
        return false;
    }
    if slice[1] == slice[0] {
        return false;
    }
    true
}

fn has_abba(slice: &str) -> bool {
    // function to verify if the string has the ABBA pattern
    // we will use a sliding window pattern
    let slice: Vec<char> = slice.chars().collect();
    for window in slice.windows(4) {
        if is_abba(&window) {
            return true;
        }
    }
    false
}

pub fn support_tls(line: &str) -> bool {
    // function to group the logic for a line that support the tls or not
    //
    let mut test_ab = false; // boolean for any part without [] and has abba
    for part in tokens(line) {
        if has_abba(&part) {
            test_ab = true;
            if part.starts_with("[") {
                return false;
            }
        }
    }
    test_ab
}

pub fn part1(ips: &String) -> usize {
    ips.lines().filter(|x| support_tls(&x)).count()
}
pub fn main() {
    // first I need to get the part of the string
    let ips = fs::read_to_string("../input/day7").expect("invalid file");
    let answer1 = part1(&ips);
    println!("partI = {answer1}");
}

#[test]
fn test1() {
    let line = "abba[mnop]qrst".to_string();
    assert!(support_tls(&line));

    let line = "abcd[bddb]xyyx".to_string();
    assert!(!support_tls(&line));

    let line = "aaaa[qwer]tyui".to_string();
    assert!(!support_tls(&line));

    let line = "ioxxoj[asdfgh]zxcvbn".to_string();
    assert!(support_tls(&line));
}
