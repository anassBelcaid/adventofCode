use std::{fs::File, io::Read};
#[derive(Debug)]
struct Square {
    il: usize, // i for the left part
    jl: usize, // j for the left part
    ir: usize, // i for the right partj
    jr: usize, // j for the right part
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<i32>>,
}
impl Grid {
    pub fn new(size: usize) -> Self {
        Self {
            grid: vec![vec![0; size]; size],
        }
    }
    fn turn_on(&mut self, square: Square) {
        // turn on all the lights into the square
        for i in square.il..=square.ir {
            for j in square.jl..=square.jr {
                self.grid[i][j] = 1
            }
        }
    }
    fn turn_off(&mut self, square: Square) {
        // turn on all the lights into the square
        for i in square.il..=square.ir {
            for j in square.jl..=square.jr {
                self.grid[i][j] = 0;
            }
        }
    }

    fn toggle(&mut self, square: Square) {
        // turn on all the lights into the square
        for i in square.il..=square.ir {
            for j in square.jl..=square.jr {
                self.grid[i][j] = if self.grid[i][j] == 1 { 0 } else { 1 }
            }
        }
    }

    fn change(&mut self, square: Square, amount: i32) {
        // turn on all the lights into the square
        for i in square.il..=square.ir {
            for j in square.jl..=square.jr {
                self.grid[i][j] += amount;
                self.grid[i][j] = self.grid[i][j].max(0);
            }
        }
    }
}

// Now we move to describe an operation as as struct
// an operation could be either [turn, toggle]
// The value turn has a additional field [on off]
// Each operation of course has a square of interest
#[derive(Debug)]
enum OP {
    TURN(bool),
    TOGGLE,
}

#[derive(Debug)]
struct Operation {
    op: OP, // the operation
    sq: Square,
}

impl Operation {
    pub fn new(instruction: &str) -> Self {
        // function to create the instruction from the line
        let words: Vec<&str> = instruction.split(' ').to_owned().collect();
        // case of toggle
        if words.len() == 4 {
            // computing the arrays positions
            let b1: Vec<usize> = words[1].split(',').map(|x| x.parse().unwrap()).collect();
            let b2: Vec<usize> = words[3].split(',').map(|x| x.parse().unwrap()).collect();
            return Operation {
                op: OP::TOGGLE,
                sq: Square {
                    il: b1[0],
                    jl: b1[1],
                    ir: b2[0],
                    jr: b2[1],
                },
            };
        } else {
            let state = if words[1] == "on" { true } else { false };
            let op = OP::TURN(state);
            let b1: Vec<usize> = words[2].split(',').map(|x| x.parse().unwrap()).collect();
            let b2: Vec<usize> = words[4].split(',').map(|x| x.parse().unwrap()).collect();
            return Operation {
                op: op,
                sq: Square {
                    il: b1[0],
                    jl: b1[1],
                    ir: b2[0],
                    jr: b2[1],
                },
            };
        }
    }
}
fn main() {
    // First we will read the input from the file
    let mut file = File::open("input/day6").expect("Invalid input file");
    let mut instructions = String::new();
    file.read_to_string(&mut instructions)
        .expect("Invalid content");

    // let create the grid
    let mut grid = Grid::new(1000);

    for instruction in instructions.lines() {
        // Creating the operation for the line
        let oper = Operation::new(instruction);
        match oper.op {
            OP::TOGGLE => grid.toggle(oper.sq),
            OP::TURN(on) => match on {
                true => grid.turn_on(oper.sq),
                false => grid.turn_off(oper.sq),
            },
        }
    }

    //counting the answer
    let answer1: i32 = grid
        .grid
        .iter()
        .map(|v| v.iter().map(|x| *x as i32).sum::<i32>())
        .sum();
    println!("part I : {}", answer1);

    // Now we create part 2
    //
    let mut grid = Grid::new(1000);

    for instruction in instructions.lines() {
        // Creating the operation for the line
        let oper = Operation::new(instruction);
        match oper.op {
            OP::TOGGLE => grid.change(oper.sq, 2),
            OP::TURN(on) => match on {
                true => grid.change(oper.sq, 1),
                false => grid.change(oper.sq, -1),
            },
        }
    }

    //counting the answer
    let answer2: i32 = grid
        .grid
        .iter()
        .map(|v| v.iter().map(|x| *x as i32).sum::<i32>())
        .sum();
    println!("part II : {}", answer2);
}
