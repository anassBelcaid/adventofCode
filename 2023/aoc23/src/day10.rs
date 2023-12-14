use std::fs::read_to_string;

use aoc23::maze::Maze;

pub fn part_1() -> usize {
    // function to compute the answer to the first part
    let content = read_to_string("input/day10").expect("invalid file");

    // Creating maze
    let maze = Maze::new(&content);

    // computing the tail
    maze.tail()
}

pub fn part_2() -> u32 {
    // function to compute the second part which involves computing
    // the number of nodes inside the cycle
    // let content = read_to_string("baby3").expect("invalid file");
    let content = read_to_string("input/day10").expect("invalid file");

    // Creating the maze
    let maze = Maze::new(&content);

    // computing the number of nodes inside
    maze.num_nests()
}

pub fn main() {
    // computing the first part
    let answer1 = part_1();
    println!("part I = {answer1}");

    // computing the second part
    let answer2 = part_2();
    println!("part II = {answer2}");
}
