use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
struct Keypad {
    x: usize, // x position of the keypad
    y: usize, // y position of the keypad
}

impl Keypad {
    pub fn new() -> Self {
        Self { x: 1, y: 1 }
    }
    pub fn change(&mut self, c: char) {
        // function to change given an instruction
        // in the form [U,D,L,R]
        match c {
            'U' => {
                if self.x > 0 {
                    self.x -= 1
                }
            }

            'D' => {
                self.x = (self.x + 1).min(2);
            }
            'R' => {
                self.y = (self.y + 1).min(2);
            }
            'L' => {
                if self.y > 0 {
                    self.y -= 1
                };
            }
            _ => panic!("Invalid character"),
        }
    }
}

fn part1(instructions: &str) -> String {
    // Creating the static hashmap
    let mut digits = HashMap::new();
    digits.insert((0, 0), '1');
    digits.insert((0, 1), '2');
    digits.insert((0, 2), '3');
    digits.insert((1, 0), '4');
    digits.insert((1, 1), '5');
    digits.insert((1, 2), '6');
    digits.insert((2, 0), '7');
    digits.insert((2, 1), '8');
    digits.insert((2, 2), '9');

    // We start always from (2,2)
    let mut pos = Keypad::new();
    let mut result = String::new();

    for line in instructions.lines() {
        if line != "" {
            for c in line.chars() {
                pos.change(c);
            }
            result.push(digits[&(pos.x, pos.y)]);
        }
    }
    result
}

pub fn main() {
    let mut file = File::open("input/day2").expect("File not found");
    let mut instructions = String::new();
    file.read_to_string(&mut instructions)
        .expect("invalid content");

    let answer1 = part1(&instructions);
    println!("part is {answer1}");
}
