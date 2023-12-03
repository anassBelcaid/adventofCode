use aoc23::gearratio::EngineScheme;
use std::fs::read_to_string;

pub fn part_i() -> i32 {
    // function to get the answer of the first part
    let input = read_to_string("input/day3").expect("invalid file");

    // Creating the engine
    let engine = EngineScheme::new(input);

    // Creating the valid numbers
    let numbers = engine.part_numbers();

    numbers.iter().sum::<_>()
}
pub fn part_ii() -> i64 {
    // function to get the answer of the first part
    let input = read_to_string("input/day3").expect("invalid file");
    // let input = read_to_string("baby").expect("invalid file");

    // Creating the engine
    let mut engine = EngineScheme::new(input);

    // create the gears
    engine.fill_gears_numbers();
    let mut answer = 0i64;

    for (_, nei) in engine.gears {
        if nei.len() == 2 {
            answer += nei.iter().product::<i32>() as i64;
        }
    }
    answer
}
pub fn main() {
    // Getting the answer to the first part
    let answer1 = part_i();
    println!("answer I = {answer1}");

    // computing the second part
    let answer2 = part_ii();
    println!("answer II = {answer2} ");
}

#[test]
fn test_part1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();

    // creating the screen
    println!("input \n {input}");
    let engine = EngineScheme::new(input);

    let numbers = engine.part_numbers();

    // computing the sum of the numbers
    let answer1: i32 = numbers.iter().sum();
    assert_eq!(answer1, 4361);
}
