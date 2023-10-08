use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
enum Turn {
    R, // turn right
    L, // turn left
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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

    pub fn walk_with_memory(
        &mut self,
        instruction: &Instruction,
        cache: &mut HashSet<(i32, i32)>,
    ) -> bool {
        // function to walk with memory thereturn bool is for informing the user
        // either we did encounter a seen the position
        // if so we can never continue
        if instruction.turn == Turn::R {
            match self.dir {
                Direction::NORTH => {
                    self.dir = Direction::EAST;
                    // self.x += instruction.steps
                    for _i in 0..instruction.steps {
                        self.x += 1;
                        if cache.contains(&(self.x, self.y)) {
                            return true;
                        }
                        cache.insert((self.x, self.y));
                    }
                }
                Direction::EAST => {
                    self.dir = Direction::SOUTH;
                    for _i in 0..instruction.steps {
                        self.y -= 1;
                        if cache.contains(&(self.x, self.y)) {
                            return true;
                        }
                        cache.insert((self.x, self.y));
                    }
                }
                Direction::SOUTH => {
                    self.dir = Direction::OUEST;
                    // self.x -= instruction.steps

                    for _i in 0..instruction.steps {
                        self.x -= 1;
                        if cache.contains(&(self.x, self.y)) {
                            return true;
                        }
                        cache.insert((self.x, self.y));
                    }
                }
                Direction::OUEST => {
                    self.dir = Direction::NORTH;
                    for _i in 0..instruction.steps {
                        self.y += 1;
                        if cache.contains(&(self.x, self.y)) {
                            return true;
                        }
                        cache.insert((self.x, self.y));
                    }
                }
            }
        } else {
            match self.dir {
                Direction::NORTH => {
                    self.dir = Direction::OUEST;
                    // self.x -= instruction.steps
                    for _i in 0..instruction.steps {
                        self.x -= 1;
                        if cache.contains(&(self.x, self.y)) {
                            return true;
                        }
                        cache.insert((self.x, self.y));
                    }
                }
                Direction::EAST => {
                    self.dir = Direction::NORTH;

                    for _i in 0..instruction.steps {
                        self.y += 1;
                        if cache.contains(&(self.x, self.y)) {
                            return true;
                        }
                        cache.insert((self.x, self.y));
                    }
                }
                Direction::SOUTH => {
                    self.dir = Direction::EAST;
                    for _i in 0..instruction.steps {
                        self.x += 1;
                        if cache.contains(&(self.x, self.y)) {
                            return true;
                        }
                        cache.insert((self.x, self.y));
                    }
                }
                Direction::OUEST => {
                    self.dir = Direction::SOUTH;

                    for _i in 0..instruction.steps {
                        self.y -= 1;
                        if cache.contains(&(self.x, self.y)) {
                            return true;
                        }
                        cache.insert((self.x, self.y));
                    }
                }
            }
        }
        false
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

pub fn part_i(instructions: &str) -> i32 {
    let mut position = Position::new(0, 0);

    for instruction in instructions.split(',').map(Instruction::new) {
        position.execute(&instruction);
    }
    position.manhatan_distance()
}

pub fn part_ii(instructions: &str) -> i32 {
    let mut position = Position::new(0, 0);
    let mut cache = HashSet::new();

    for instruction in instructions.split(',').map(Instruction::new) {
        let seen = position.walk_with_memory(&instruction, &mut cache);
        if seen {
            return position.manhatan_distance();
        }
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

    let answer1 = part_i(&directions);
    println!("Part I = {answer1}");

    let answer2 = part_ii(&directions);
    println!("Part II = {answer2}");
}
