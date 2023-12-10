use std::fs::read_to_string;

use aoc23::camelcard::{Card, Hand};

pub fn part_1() -> i64 {
    // function to get the answer to the first part
    // loading the content
    let content = read_to_string("baby").expect("invalid file");

    // Creating the hand
    let mut hands: Vec<_> = content.lines().map(|line| Hand::new(line)).collect();

    // sorting the hands
    hands.sort_unstable();

    let mut answer = 064;
    for (r, hand) in (1..=5).zip(hands.iter()) {
        answer += r as i64 * hand.bidding as i64;
    }

    answer
}
pub fn main() {
    let answer1 = part_1();
    println!("part I = {answer1}");
}
