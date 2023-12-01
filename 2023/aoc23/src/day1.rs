use aoc23::trebuchet::{decrypt, decrypt2};
use std::fs::read_to_string;

fn part1() -> i32 {
    // Getting the input
    let input = read_to_string("input/day1").expect("invalid file");

    // summing up all the values
    input.lines().map(|x| decrypt(x)).sum()
}

fn part2() -> i32 {
    // Getting the input
    let input = read_to_string("input/day1").expect("invalid file");

    // summing up all the values
    input.lines().map(|x| decrypt2(x)).sum()
}

fn main() {
    // Answering the first part
    let answer1 = part1();
    println!("part I = {answer1}");

    // Answering the second part
    let answer2 = part2();
    println!("part II = {answer2}");
}

#[test]
fn test_descpription() {
    assert_eq!(decrypt("1abc2"), 12);
    assert_eq!(decrypt("pqr3stu8vwx"), 38);
    assert_eq!(decrypt("a1b2c3d4e5f"), 15);
    assert_eq!(decrypt("treb7uchet"), 77);
}
#[test]
fn test2() {
    assert_eq!(decrypt2("twonine"), 29);
    assert_eq!(decrypt2("eightwothree"), 83);
    assert_eq!(decrypt2("abcone2threexyz"), 13);
    assert_eq!(decrypt2("xtwone3four"), 24);
    assert_eq!(decrypt2("4nineeightseven2"), 42);
    assert_eq!(decrypt2("zoneight234"), 14);
    assert_eq!(decrypt2("7pqrstsixteen"), 76);
}
