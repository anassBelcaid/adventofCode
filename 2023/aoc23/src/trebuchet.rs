use std::collections::HashMap;

use regex::Regex;
pub fn decrypt(line: &str) -> i32 {
    let pattern = regex::Regex::new(r"[0-9]").expect("invalid regex");
    let groupes: Vec<_> = pattern.find_iter(line).map(|x| x.as_str()).collect();

    return groupes.first().unwrap().parse::<i32>().unwrap() * 10
        + groupes.last().unwrap().parse::<i32>().unwrap();
}
pub fn decrypt2(line: &str) -> i32 {
    let chars: Vec<_> = line.chars().collect();
    let mut values = Vec::new();
    for i in 0..line.len() {
        if chars[i].is_digit(10) {
            values.push(chars[i].to_digit(10).unwrap() as i32);
        }

        if line[i..].starts_with("one") {
            values.push(1);
        }

        if line[i..].starts_with("two") {
            values.push(2);
        }

        if line[i..].starts_with("three") {
            values.push(3);
        }

        if line[i..].starts_with("four") {
            values.push(4);
        }

        if line[i..].starts_with("five") {
            values.push(5);
        }

        if line[i..].starts_with("six") {
            values.push(6);
        }

        if line[i..].starts_with("seven") {
            values.push(7);
        }

        if line[i..].starts_with("eight") {
            values.push(8);
        }

        if line[i..].starts_with("nine") {
            values.push(9);
        }
    }
    values.first().unwrap() * 10 + values.last().unwrap()
}
