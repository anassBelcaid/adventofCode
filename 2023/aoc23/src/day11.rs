use std::fs::read_to_string;

use aoc23::cosmicexpansion::{Galaxies, EMPTY_DISTANCE};

pub fn part_1() -> usize {
    // function to answer the first part of the puzzle
    let content = read_to_string("input/day11").expect("invalid content");

    let mut universe = Galaxies::new(&content);
    universe.galaxies_distances()
}

pub fn part_2() -> usize {
    // function to answer the first part of the puzzle
    let content = read_to_string("input/day11").expect("invalid content");

    // change the time travel for each space
    unsafe {
        EMPTY_DISTANCE = 1000000;
    }
    let mut universe = Galaxies::new(&content);
    universe.galaxies_distances()
}

pub fn main() {
    let answer1 = part_1();
    println!("Part I = {answer1}");

    // let' test teh second part
    let answer2 = part_2();
    println!("part II = {answer2}");
}
