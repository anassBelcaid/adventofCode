use fancy_regex::Regex;
use std::{fs::File, io::Read};

pub fn is_good(message: &str) -> bool {
    // let pattern 1
    let pat1 = Regex::new(r".*[aeiou].*[aeiou].*[aeiou].*").expect("Invalid regular expression");
    let pat2 = Regex::new(r"([a-z])\1").expect("Invalid regular expression");
    let pat3 = Regex::new(r"(ab|cd|pq|xy)").expect("Invalid regular expression");

    pat1.is_match(&message).unwrap()
        && pat2.is_match(&message).unwrap()
        && !pat3.is_match(&message).unwrap()
}

pub fn is_good_v2(message: &str) -> bool {
    let pat1 = Regex::new(r"([a-z]{2}).*\1").expect("Invalid exp");
    let pat2 = Regex::new(r"([a-z])[a-z]\1").expect("Invalid exp");

    pat1.is_match(message).unwrap() && pat2.is_match(message).unwrap()
}

fn main() {
    // Creating the file
    let mut lines = String::new();
    let mut file = File::open("input/day5").expect("Invalid input file");

    // Reading the content of the file into the string
    file.read_to_string(&mut lines).expect("Invalid content");

    // Now we loop over the line
    let answer1: i32 = lines.lines().map(|x| is_good(x) as i32).sum();
    println!("Part I = {}", answer1);

    // Now we check the second part
    let answer2: i32 = lines.lines().map(|x| is_good_v2(x) as i32).sum();
    println!("part II = {}", answer2);
}

#[test]
fn test1() {
    // is nice because it has at least three vowels (`u...i...o...`),
    // a double letter (`...dd...`),
    // and none of the disallowed substrings.
    let message = "ugknbfddgicrmopn".to_string();

    assert!(is_good(&message));
}

#[test]
fn test2() {
    // is nice because it has at least three vowels (`u...i...o...`), a double letter (`...dd...`),
    // and none of the disallowed substrings.
    let message = "aaa".to_string();

    assert!(is_good(&message));
}

#[test]
fn test3() {
    // is nice because it has at least three vowels (`u...i...o...`),
    // a double letter (`...dd...`),
    // and none of the disallowed substrings.
    let message = "jchzalrnumimnmhp".to_string();
    assert!(!is_good(&message));
}
#[test]
fn test4() {
    let message = "haegwjzuvuyypxyu".to_string(); //is naughty because it contains the string `xy`.
    assert!(!is_good(&message));
}
#[test]
fn test5() {
    let message = "vszwmarrgswjxmb".to_string(); // is naughty because it contains only one vowel.
    assert!(!is_good(&message));
}
#[test]
fn test6() {
    let message = "qjhvhtzxzqqjkmpb".to_string(); // is naughty because it contains only one vowel.
    assert!(is_good_v2(&message));
}

#[test]
fn test7() {
    let message = "xxyxx".to_string(); // is naughty because it contains only one vowel.
    assert!(is_good_v2(&message));
}

#[test]
fn test8() {
    let message = "uurcxstgmygtbstg".to_string(); // is naughty because it contains only one vowel.
    assert!(!is_good_v2(&message));
}

#[test]
fn test9() {
    let message = "ieodomkazucvgmuy".to_string(); // is naughty because it contains only one vowel.
    assert!(!is_good_v2(&message));
}
