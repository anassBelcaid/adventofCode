use std::fs::File;
use std::io::Read;

// the main question is in the model how to correctly model the iput

pub trait Action {
    fn apply(&self, a: &mut usize, b: &mut usize) -> i32;
}

/*
 * Action half
 */

#[derive(Debug)]
struct Half {
    index: usize,
    register: usize, // which register to modify
}
impl Action for Half {
    fn apply(&self, a: &mut usize, b: &mut usize) -> i32 {
        if self.register == 0 {
            *a /= 2;
        } else {
            *b /= 2;
        }
        return (self.index + 1) as i32;
    }
}
//---------------------------------------------

/*
 * Action triple
 */

#[derive(Debug)]
struct Triple {
    index: usize,
    register: usize, // which register to modify
}
impl Action for Triple {
    fn apply(&self, a: &mut usize, b: &mut usize) -> i32 {
        if self.register == 0 {
            *a *= 3;
        } else {
            *b *= 3;
        }
        return (self.index + 1) as i32;
    }
}
//---------------------------------------------

/*
 * Action Increment
 */

#[derive(Debug)]
struct Increment {
    index: usize,
    register: usize, // which register to modify
}
impl Action for Increment {
    fn apply(&self, a: &mut usize, b: &mut usize) -> i32 {
        if self.register == 0 {
            *a += 1;
        } else {
            *b += 1;
        }
        return (self.index + 1) as i32;
    }
}
//---------------------------------------------

//---------------------------------------------

/*
 * Action jump off set
 */

#[derive(Debug)]
struct Jump {
    index: usize,
    offset: i32,
}
impl Action for Jump {
    fn apply(&self, _a: &mut usize, _b: &mut usize) -> i32 {
        self.index as i32 + self.offset
    }
}
//---------------------------------------------

/*
 * Action jump if Even
 */

#[derive(Debug)]
struct JumpIfEven {
    index: usize,
    register: usize,
    offset: i32,
}
impl Action for JumpIfEven {
    fn apply(&self, a: &mut usize, b: &mut usize) -> i32 {
        if self.register == 0 && *a % 2 == 0 {
            return self.index as i32 + self.offset;
        }

        if self.register == 1 && *b % 2 == 0 {
            return self.index as i32 + self.offset;
        }

        (self.index + 1) as i32
    }
}
//---------------------------------------------

/*
 * Action jump if Odd
 */

#[derive(Debug)]
struct JumpIfOne {
    index: usize,
    register: usize,
    offset: i32,
}
impl Action for JumpIfOne {
    fn apply(&self, a: &mut usize, b: &mut usize) -> i32 {
        if self.register == 0 && *a == 1 {
            return self.index as i32 + self.offset;
        }

        if self.register == 1 && *b == 1 {
            return self.index as i32 + self.offset;
        }

        (self.index + 1) as i32
    }
}
//---------------------------------------------

pub fn main() {
    // let's read the input
    let mut file = File::open("input/day23").expect("File not found");
    // let mut file = File::open("baby").expect("File not found");

    let mut instructions = String::new();

    // reading the content in the instructions
    file.read_to_string(&mut instructions)
        .expect("invalid content");

    // Create the vector of instruction (sadly I will keep &
    // processing them each time ( I should learn to do polynormphsim)
    // I will create a vector of apply action
    let mut a = 0usize;
    let mut b = 0usize;
    let mut actions: Vec<Box<dyn Action>> = Vec::new();

    // the hard part filling the actions
    for (index, instruction) in instructions.lines().enumerate() {
        // Getting the tokens
        let tokens: Vec<String> = instruction.split(',').map(|x| x.to_string()).collect();

        // Getting the type and register
        let action_type: Vec<String> = tokens[0].split(' ').map(|x| x.to_string()).collect();

        // now we match the type of the action
        if action_type[0] == "hlf".to_string() {
            let register = if action_type[1] == "a".to_string() {
                0
            } else {
                1
            };
            actions.push(Box::new(Half { index, register }));
        }

        // second action triple
        if action_type[0] == "tpl".to_string() {
            let register = if action_type[1] == "a".to_string() {
                0
            } else {
                1
            };
            actions.push(Box::new(Triple { index, register }));
        }

        // increase
        if action_type[0] == "inc".to_string() {
            let register = if action_type[1] == "a".to_string() {
                0
            } else {
                1
            };
            actions.push(Box::new(Increment { index, register }));
        }

        // jump
        if action_type[0] == "jmp".to_string() {
            let offset = action_type[1].parse().unwrap();
            actions.push(Box::new(Jump { index, offset }));
        }

        // jump if odd
        if action_type[0] == "jio".to_string() {
            let offset: i32 = tokens[1].trim().parse().unwrap();
            let register = if action_type[1] == "a".to_string() {
                0
            } else {
                1
            };

            actions.push(Box::new(JumpIfOne {
                index,
                register,
                offset,
            }));
        }

        // jump if odd
        if action_type[0] == "jie".to_string() {
            let offset: i32 = tokens[1].trim().parse().unwrap();
            let register = if action_type[1] == "a".to_string() {
                0
            } else {
                1
            };

            actions.push(Box::new(JumpIfEven {
                index,
                register,
                offset,
            }));
        }
    }

    // now simply iterate over all the values
    let mut index = 0;
    let n = actions.len() as i32;

    while index >= 0 && index < n {
        index = actions[index as usize].apply(&mut a, &mut b);
    }
    println!(" part1 is {b}");

    // part 2
    a = 1;
    b = 0;
    index = 0;

    while index >= 0 && index < n {
        index = actions[index as usize].apply(&mut a, &mut b);
    }
    println!("part2 is {b}");
}
