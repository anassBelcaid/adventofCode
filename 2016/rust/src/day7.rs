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

pub fn bab_patterns(part: &str) -> Vec<String> {
    // fundtion to return the set of possible bab
    // pattern in the part
    let part: Vec<char> = part.chars().collect();

    let mut results = Vec::new();
    for wind in part.windows(3) {
        if wind[0] == wind[2] && wind[0] != wind[1] {
            results.push(format!("{}{}{}", wind[1], wind[0], wind[1]));
        }
    }
    results
}

pub fn support_ssl(ip: &str) -> bool {
    // todo!("implement once I have all the parts");
    // I need to divide the parts into supernets and hypernets

    let mut supernets = Vec::new();
    let mut hypernets = Vec::new();

    for part in tokens(&ip) {
        if part.starts_with("[") {
            hypernets.push(part.clone());
        } else {
            supernets.push(part.clone());
        }
    }
    // now we loop on all pattern in the supernet
    for supernet in supernets {
        let patterns = bab_patterns(&supernet);

        // now we check if one of the hypernets contains that pattern
        for hypernet in hypernets.iter() {
            for pat in patterns.iter() {
                if hypernet.contains(pat) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn part2(ips: &String) -> usize {
    ips.lines().filter(|x| support_ssl(&x)).count()
}
pub fn main() {
    // first I need to get the part of the string
    let ips = fs::read_to_string("../input/day7").expect("invalid file");
    let answer1 = part1(&ips);
    println!("partI = {answer1}");

    let answer2 = part2(&ips);
    println!("part II = {answer2}");
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

#[test]
fn test2() {
    let line = "aba[bab]xyz".to_string();
    //supports SSL (`aba` outside square brackets with corresponding `bab` within square brackets).
    assert!(support_ssl(&line));

    let line = "xyx[xyx]xyx".to_string();
    //does *not* support SSL (`xyx`, but no corresponding `yxy`).
    assert!(!support_ssl(&line));

    let line = "aaa[kek]eke".to_string();
    // supports SSL (`eke` in supernet with corresponding `kek` in hypernet; the `aaa` sequence is not related, because the interior character must be different).
    assert!(support_ssl(&line));

    let line = "zazbz[bzb]cdb".to_string();
    //supports SSL (`zaz` has no corresponding `aza`, but `zbz` has a corresponding `bzb`, even though `zaz` and `zbz` overlap).
    assert!(support_ssl(&line));
}
