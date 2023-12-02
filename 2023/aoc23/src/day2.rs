use aoc23::cubeconundrum::parse_game;
use std::fs::read_to_string;

fn partI() -> i32 {
    // get the answer to the first part
    let content = read_to_string("input/day2").expect("invalid file");
    // let content = read_to_string("baby").expect("invalid file");
    let mut sum = 0;

    for line in content.lines() {
        let game = parse_game(line).unwrap().1;
        if game.possible() {
            sum += game.id;
        }
    }
    sum as i32
}

fn part_ii() -> i32 {
    let content = read_to_string(("input/day2")).expect("invalid file");

    let mut sum = 0;

    for line in content.lines() {
        // Getting the game
        let game = parse_game(line).unwrap().1;
        sum += game.power();
    }
    sum as i32
}

fn main() {
    // matching with a tag
    let answer1 = partI();
    println!("answer1 = {answer1}");

    // answering part2
    let answer2 = part_ii();
    println!("part II = {answer2}");
}
