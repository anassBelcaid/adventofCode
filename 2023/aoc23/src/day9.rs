use std::fs::read_to_string;

use aoc23::mirage::History;
pub fn part_1() -> i64 {
    // function to compute the answr to the first part

    let content = read_to_string("input/day9").expect("invalid file");

    content
        .lines()
        .map(|line| History::new(line))
        .map(|history| history.extraploate_value())
        .sum()
}

pub fn part_2() -> i64 {
    // function to compute the answr to the first part

    let content = read_to_string("input/day9").expect("invalid file");

    content
        .lines()
        .map(|line| History::new(line))
        .map(|history| history.extraploate_left())
        .sum()
}
pub fn main() {
    // computing the answer for the first part
    let answer1 = part_1();
    println!("part I = {answer1}");

    // computing the answer for the second part
    let answer2 = part_2();
    println!("part II = {answer2}");
}
