use std::fs::read_to_string;

use aoc23::hotspring::Spring;

pub fn part_1() -> usize {
    // function to get the answer to the first part
    let content = read_to_string("input/day12").expect("invalid file");

    content
        .lines()
        .map(|line| Spring::new(&line))
        .map(|spring| spring.num_ways())
        .sum()
    // for line in content.lines().nth(1) {
    //     let spring = Spring::new(line);
    //     let num_ways = spring.num_ways();
    //     println!("num ways is {num_ways}");
    // }
    // 0
}
pub fn main() {
    let answer1 = part_1();
    println!("part I = {answer1}");
}
