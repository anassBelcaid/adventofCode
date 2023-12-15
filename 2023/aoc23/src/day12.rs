use std::fs::read_to_string;

use aoc23::hotspring::Spring;
pub fn part_1() -> usize {
    // function to get the answer to the first part
    let content = read_to_string("baby").expect("invalid file");

    for line in content.lines() {
        let spring = Spring::new(&line);
        println!("{spring:?}");
    }
    0
}
pub fn main() {
    println!("let's compute those combinaison");
    let answer1 = part_1();
    println!("part I = {answer1}");
}
