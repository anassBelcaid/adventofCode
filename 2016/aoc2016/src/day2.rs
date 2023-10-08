use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
struct Keypad {
    x: usize, // x position of the keypad
    y: usize, // y position of the keypad
}

impl Keypad {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn change(&mut self, c: char, allowed: &mut HashMap<(usize, usize), char>) {
        // function to change given an instruction
        // in the form [U,D,L,R]
        match c {
            'U' => {
                if self.x > 0 && allowed.contains_key(&(self.x - 1, self.y)) {
                    self.x -= 1
                }
            }

            'D' => {
                if allowed.contains_key(&(self.x + 1, self.y)) {
                    self.x += 1
                }
            }

            'R' => {
                if allowed.contains_key(&(self.x, self.y + 1)) {
                    self.y += 1
                }
            }
            'L' => {
                if self.y > 0 && allowed.contains_key(&(self.x, self.y - 1)) {
                    self.y -= 1
                }
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
    let mut pos = Keypad::new(1, 1);
    let mut result = String::new();

    for line in instructions.lines() {
        if line != "" {
            for c in line.chars() {
                pos.change(c, &mut digits);
            }
            result.push(digits[&(pos.x, pos.y)]);
        }
    }
    result
}

fn part2(instructions: &str) -> String {
    // ```
    //     1
    //   2 3 4
    // 5 6 7 8 9
    //   A B C
    //     D
    //
    // ```

    let mut digits = HashMap::new();
    digits.insert((0, 2), '1');
    digits.insert((1, 1), '2');
    digits.insert((1, 2), '3');
    digits.insert((1, 3), '4');
    digits.insert((2, 0), '5');
    digits.insert((2, 1), '6');
    digits.insert((2, 2), '7');
    digits.insert((2, 3), '8');
    digits.insert((2, 4), '9');
    digits.insert((3, 1), 'A');
    digits.insert((3, 2), 'B');
    digits.insert((3, 3), 'C');
    digits.insert((4, 2), 'D');
    let mut result = String::new();
    let mut position = Keypad::new(2, 0);

    for line in instructions.lines() {
        if line != "" {
            for c in line.chars() {
                position.change(c, &mut digits);
            }
            result.push(digits[&(position.x, position.y)]);
        }
    }

    result
}

pub fn main() {
    let mut file = File::open("input/day2").expect("File not found");
    // let mut file = File::open("baby").expect("File not found");
    let mut instructions = String::new();
    file.read_to_string(&mut instructions)
        .expect("invalid content");

    // part 1
    let answer1 = part1(&instructions);
    println!("part I is {answer1}");

    // part2
    let answer2 = part2(&instructions);
    println!("part II is {answer2}");
}
