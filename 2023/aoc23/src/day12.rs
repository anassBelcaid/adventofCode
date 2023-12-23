use std::fs::read_to_string;

use aoc23::hotspring::Spring;

pub fn part_1() -> i64 {
    // function to get the answer to the first part
    let content = read_to_string("input/day12").expect("invalid file");

    content
        .lines()
        .map(|line| Spring::new(&line))
        .map(|spring| spring.num_ways())
        .sum()
}

pub fn part_2() -> i64 {
    // function to compute the second part
    let content = read_to_string("input/day12").expect("invalid file");

    let mut answer = 0;
    for line in content.lines() {
        // Create the spring
        let mut spring = Spring::new(&line);

        // fold the spring
        spring.fold();

        answer += spring.num_ways();
    }
    answer
}
pub fn main() {
    // answer the first part
    let answer1 = part_1();
    println!("part I = {answer1}");

    // answer the second part
    let answer2 = part_2();
    println!("part II = {answer2}");
}
