use std::fs::read_to_string;

use aoc23::wasteland::Network;

pub fn part_1() -> usize {
    // function to get the first part of the puzzle
    // let network_description = read_to_string("baby").expect("invalid file");
    let network_description = read_to_string("input/day8").expect("invalid file");

    // parsing the content
    let network = Network::new(&network_description);

    // computing the number of moves to get out
    let moves = network.num_moves_get_out();

    moves
}

pub fn part_2() -> usize {
    // function to get now all the ghosts to their desitination
    let network_description = read_to_string("input/day8").expect("invalid file");

    // parsing the content
    let network = Network::new(&network_description);

    // compute the number of moves for all the ghost to reach the end
    network.num_moves_ghosts()
}
pub fn main() {
    println!("let's compute the path");

    // let answer1 = part_1();
    // println!("part I = {answer1}");

    // computing the second part
    let answer2 = part_2();
    println!("part II = {answer2}");
}
