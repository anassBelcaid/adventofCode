pub mod robots;
use std::fs::read_to_string;

fn part_I(input: &str) -> (usize, i32) {
    // function to respond to the firest part
    // This should be an exercice on how to model a situation and adapt to variance
    let mut factory = robots::Factory::new(input);

    // simulate the work on the factory with the robots.
    factory.simulate();

    // getting the target
    let part1 = factory.comparisons[&(17, 61)];
    let part2 = factory.outputs[..=2].iter().product();

    return (part1, part2);
}

pub fn main() {
    // let input = read_to_string("baby").expect("invalid file");
    let input = read_to_string("../input/day10").expect("invalid file");
    let input = input.trim();

    // getting the first part of the puzzle
    let (answer1, answer2) = part_I(&input);
    println!("answer part I = {answer1}");
    println!("answer part II = {answer2}");
}
