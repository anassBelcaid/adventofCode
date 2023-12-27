use std::fs::read_to_string;

use aoc23::mirrorvalley::Valley;

pub fn part_1() -> usize {
    // function to get the answer to the first part

    let valleys = read_to_string("input/day13").expect("invalid file");

    valleys
        .split("\n\n")
        .map(|line| Valley::new(line))
        .map(|valley| valley.reflexion_value())
        .sum()
}

pub fn part_2() -> usize {
    // function to get the second part of the problem

    let valleys = read_to_string("input/day13").expect("invalid file");

    valleys
        .split("\n\n")
        .map(|line| Valley::new(line))
        .map(|valley| valley.smugged_reflexion_value())
        .sum()
}
pub fn main() {
    // compute the first part
    let answer1 = part_1();
    println!("part I = {answer1}");

    // compute the second part
    let answer2 = part_2();
    println!("part II = {answer2}");
}
