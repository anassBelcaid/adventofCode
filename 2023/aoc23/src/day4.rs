use std::fs::read_to_string;

use aoc23::scratchcard::ScratchCard;

pub fn part_1() -> i32 {
    // function to get the answer to the first part
    let cards = read_to_string("input/day4").expect("invalid content");
    // let cards = read_to_string("baby").expect("invalid content");

    cards
        .lines()
        .map(|card| ScratchCard::new(card))
        .map(|card| card.points())
        .sum()
}

pub fn part_2() -> i32 {
    // function to get the answer to the first part
    let cards = read_to_string("input/day4").expect("invalid content");
    // let cards = read_to_string("baby").expect("invalid content");

    let cards: Vec<_> = cards.lines().map(|line| ScratchCard::new(line)).collect();
    let n = cards.len();

    // dp vector for the number of card each card could win
    let mut dp = vec![1; n];

    for i in (0..n - 1).rev() {
        // compute the number of cards card[i] will win
        let limit = (i + cards[i].num_winning()).min(n - 1);
        for j in i + 1..=limit {
            dp[i] += dp[j];
        }
    }

    // number of one card but you don't win card 1
    dp.iter().sum()
}
pub fn main() {
    let answer1 = part_1();
    println!("Part I  = {answer1}");
    //
    let answer2 = part_2();
    println!("answer2 = {answer2}");
}
