use std::fs::File;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
enum Turn {
    R, // turn right
    L, // turn left
}

enum Direction {
    NORTH,
    EAST,
    SOUTH,
    OUEST,
}

#[derive(Debug)]
struct Instruction {
    turn: Turn,
    steps: i32,
}
impl Instruction {
    pub fn new(rep: &str) -> Self {
        let parts = rep.trim();

        let first = parts.chars().nth(0).unwrap();
        let turn = if first == 'R' { Turn::R } else { Turn::L };
        let steps: i32 = parts[1..].parse().unwrap();
        Self { turn, steps }
    }
}

struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            dir: Direction::NORTH,
        }
    }

    pub fn manhatan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    pub fn execute(&mut self, instruction: &Instruction) {
        // function to move the current position
        if instruction.turn == Turn::R {
            match self.dir {
                Direction::NORTH => {
                    self.dir = Direction::EAST;
                    self.x += instruction.steps
                }
                Direction::EAST => {
                    self.dir = Direction::SOUTH;
                    self.y -= instruction.steps
                }
                Direction::SOUTH => {
                    self.dir = Direction::OUEST;
                    self.x -= instruction.steps
                }
                Direction::OUEST => {
                    self.dir = Direction::NORTH;
                    self.y += instruction.steps
                }
            }
        } else {
            match self.dir {
                Direction::NORTH => {
                    self.dir = Direction::OUEST;
                    self.x -= instruction.steps
                }
                Direction::EAST => {
                    self.dir = Direction::NORTH;
                    self.y += instruction.steps
                }
                Direction::SOUTH => {
                    self.dir = Direction::EAST;
                    self.x += instruction.steps
                }
                Direction::OUEST => {
                    self.dir = Direction::SOUTH;
                    self.y -= instruction.steps
                }
            }
        }
    }
}

pub fn partI(instructions: &str) -> i32 {
    let mut position = Position::new(0, 0);

    for instruction in instructions.split(',').map(Instruction::new) {
        position.execute(&instruction);
    }
    position.manhatan_distance()
}
pub fn main() {
    let mut file = File::open("input/day1").expect("file day1 not found");
    // let mut file = File::open("baby").expect("file day1 not found");
    let mut directions = String::new();

    file.read_to_string(&mut directions)
        .expect("invalid contents");

    // The directions are split by , so let's read the input

    let answer1 = partI(&directions);
    println!("Part I = {answer1}");
}
