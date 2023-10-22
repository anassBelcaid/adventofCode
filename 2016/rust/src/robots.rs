use std::collections::{HashMap, VecDeque};

use regex::Regex;
// Destination of the number to a given destination
#[derive(Debug, Clone)]
pub enum Destination {
    BOT(usize),
    OUTPUT(usize),
    EMPTY,
}

// the bot class to model the dynamics of the flow
#[derive(Debug, Clone)]
pub struct Bot {
    small: Destination,
    large: Destination,
    values: Vec<i32>,
}

#[derive(Debug)]
pub enum Instruction {
    VALUE(i32, usize), // value to go to which robot
    BOT(usize, Destination, Destination),
}

impl Bot {
    #[inline]
    pub fn new() -> Self {
        Self {
            small: Destination::EMPTY,
            large: Destination::EMPTY,
            values: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Factory {
    pub bots: Vec<Bot>,
    pub outputs: Vec<i32>,
    pub comparisons: HashMap<(i32, i32), usize>,
}

impl Factory {
    #[inline]
    pub fn new(line: &str) -> Self {
        let instructions = Self::get_instructions(line);

        // Getting the max instruction
        let mut size = 0;

        for ins in instructions.iter() {
            match ins {
                Instruction::VALUE(_v, u) => size = size.max(*u + 1),
                Instruction::BOT(u, _l, _r) => size = size.max(*u + 1),
            }
        }
        // now we could process with the instruction
        let mut bots = vec![Bot::new(); size];
        let outputs = vec![0; size];

        for ins in instructions {
            match ins {
                Instruction::VALUE(val, index) => bots[index].values.push(val),
                Instruction::BOT(index, small, large) => {
                    bots[index].small = small;
                    bots[index].large = large;
                }
            }
        }
        Self {
            bots,
            outputs,
            comparisons: HashMap::new(),
        }
    }
    pub fn get_instructions(lines: &str) -> Vec<Instruction> {
        // function to get the set of instruction from a given lines
        let mut instructions = Vec::new();
        for line in lines.lines() {
            let ins = parse_line(line);
            instructions.push(ins);
        }
        return instructions;
    }

    pub fn simulate(&mut self) {
        // function to simulate the factory process of values
        let mut queue = VecDeque::new();

        for (i, bot) in self.bots.iter().enumerate() {
            if bot.values.len() == 2 {
                queue.push_back(i);
            }
        }

        // process
        while !queue.is_empty() {
            // Getting a  node with 2 values
            let index = queue.pop_front().unwrap();

            // Getting the current bot
            let first = self.bots[index].values[0];
            let second = self.bots[index].values[1];
            let s = first.min(second);
            let l = first.max(second);

            // sorting the values

            // now we know that his bot compare those values
            self.comparisons.insert((s, l), index);

            // sending the values
            match self.bots[index].small {
                Destination::BOT(u) => {
                    self.bots[u].values.push(s);
                    if self.bots[u].values.len() == 2 {
                        queue.push_back(u);
                    }
                }
                Destination::OUTPUT(u) => {
                    self.outputs[u] = s;
                }
                Destination::EMPTY => panic!("un rocognized option"),
            }
            match self.bots[index].large {
                Destination::BOT(u) => {
                    self.bots[u].values.push(l);
                    if self.bots[u].values.len() == 2 {
                        queue.push_back(u);
                    }
                }
                Destination::OUTPUT(u) => {
                    self.outputs[u] = l;
                }
                Destination::EMPTY => panic!("un rocognized option"),
            }
        }
    }
}

// function to process a given line into a proper instruction, this where I should use nom
// But I'm too lazy

pub fn parse_line(line: &str) -> Instruction {
    // function to parse a single line into an instruction
    let regex1 = Regex::new(r"value (\d+) goes to bot (\d+)").expect("invalid regex");
    let regex2 = Regex::new(r"bot (\d+) gives low to ([a-z]+) (\d+) and high to ([a-z]+) (\d+)")
        .expect("invalid message");
    if let Some(groups) = regex1.captures(line) {
        let val: i32 = groups[1].parse().unwrap();
        let index: usize = groups[2].parse().unwrap();

        return Instruction::VALUE(val, index);
    } else if let Some(groups) = regex2.captures(line) {
        let bot_index: usize = groups[1].parse().unwrap();
        let low_index: usize = groups[3].parse().unwrap();
        let high_index: usize = groups[5].parse().unwrap();
        let des_low = if groups[2] == "bot".to_string() {
            Destination::BOT(low_index)
        } else {
            Destination::OUTPUT(low_index)
        };

        let des_high = if groups[4] == "bot".to_string() {
            Destination::BOT(high_index)
        } else {
            Destination::OUTPUT(high_index)
        };
        return Instruction::BOT(bot_index, des_low, des_high);
    }

    panic!("invalid instruction");
}
