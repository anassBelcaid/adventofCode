use aoc2016::lcdscreen::{parse_instruction, Instruction, LCDScreen};
use std::fs;

fn part_i(lines: &String) -> i32 {
    let mut screen = LCDScreen::new();

    for line in lines.lines() {
        if let Some(instruction) = parse_instruction(&line) {
            match instruction {
                Instruction::Rect(x, y) => screen.rect(x, y),
                Instruction::RotateRow(A, shift) => screen.roatate_row(A, shift),
                Instruction::RotateColumn(A, shift) => screen.rotate_col(A, shift),
            }
        }
    }
    screen.num_lights()
}
pub fn main() {
    println!("lets counts those screen");
    // create the screen
    let lines = fs::read_to_string("../input/day8").expect("invalid file");

    let answer1 = part_i(&lines);
    println!("Part I = {answer1}");
}
