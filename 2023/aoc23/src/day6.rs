use std::fs::read_to_string;

use aoc23::boats::Game;

pub fn part_1() -> usize {
    // Reading teh file
    // let content = read_to_string("baby").expect("invalid file");
    let content = read_to_string("input/day6").expect("invalid file");

    // Creating the game
    let game = Game::new(&content);

    // Computing the game margin
    game.margin()
}

pub fn part_2() -> usize {
    // Convert the format the game
    let content = read_to_string("input/day6").expect("invalid file");
    // let content = read_to_string("baby").expect("invalid file");

    // Creating the game
    let game = Game::new(&content);

    // Creating the combined game
    let combined = game.as_single_game();

    // we simply get the only number of ways
    combined.num_ways()
}
pub fn main() {
    // responding to the first part
    let answer1 = part_1();
    println!("part I = {answer1}");

    // computing the second part
    let answer2 = part_2();
    println!("part II = {answer2}");
}
