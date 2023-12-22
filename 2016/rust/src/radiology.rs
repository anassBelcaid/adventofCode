use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use regex::Regex;

// Enum fo represent an RTG either a machine or a generator, the index will serve to replace the
// complicated name, another struct will store the
// relation between the names and the indices
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RTG {
    MICROSHIP(usize),
    GENERATOR(usize),
}

#[derive(Debug)]
pub struct Radiology {
    pub floors: Vec<Vec<RTG>>, // element in each floor
    // elevator: usize,                   // index of the elevator
    idx_names: HashMap<usize, String>, // name idx,
    name_idx: HashMap<String, usize>,  // name idx,
    pub config: Vec<Vec<bool>>,        // either each rgt (i) is in floor j
}

impl Radiology {
    #[inline]
    pub fn new() -> Self {
        let floors: Vec<Vec<RTG>> = vec![Vec::new(); 4];
        let config: Vec<Vec<bool>> = vec![Vec::new(); 4];
        Self {
            floors,
            // elevator: 0,
            idx_names: HashMap::new(),
            name_idx: HashMap::new(),
            config,
        }
    }
    pub fn add_rtg_floor(&mut self, name: String, roof: usize) {
        let idx = self.add_rtg(name);
        self.floors[roof].push(RTG::MICROSHIP(idx));
        self.floors[roof].push(RTG::GENERATOR(idx));
    }

    pub fn add_rtg(&mut self, name: String) -> usize {
        // function to add an RTG to the idx_names and return it's index
        if self.name_idx.contains_key(&name) {
            return self.name_idx[&name];
        } else {
            let idx = self.idx_names.len();
            self.idx_names.insert(idx, name.to_string());
            self.name_idx.insert(name.to_string(), idx);
            return idx;
        }
    }

    pub fn process_line(&mut self, line: &str, roof: usize) {
        // function to precess a line
        // println!("line is {line}");
        let micro_pattern =
            Regex::new(r"([a-z]+)-compatible microchip").expect("invalid expression");

        // matching all the groups in the micro_pattern
        for capture in micro_pattern.captures_iter(line) {
            let name = &capture[1];
            let idx = self.add_rtg(name.to_string());
            self.floors[roof].push(RTG::MICROSHIP(idx));
        }

        // nowe we process the generator
        let generator_pattern = Regex::new(r"([a-z]+) generator").expect("invalid expression");

        // matching all the groups in the micro_pattern
        for capture in generator_pattern.captures_iter(line) {
            let name = &capture[1];
            let idx = self.add_rtg(name.to_string());
            self.floors[roof].push(RTG::GENERATOR(idx));
        }
    }

    pub fn initate_config(&mut self) {
        // function to initiate the config from the current position
        let n = self.idx_names.len();

        for i in 0..4 {
            self.config[i] = vec![false; 2 * n];
        }

        for (i, floor) in self.floors.iter().enumerate() {
            for elem in floor {
                match elem {
                    RTG::MICROSHIP(j) => self.config[i][2 * j + 1] = true,
                    RTG::GENERATOR(j) => self.config[i][2 * j] = true,
                }
            }
        }
    }
}

// Define the search state for the BFS
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    pub config: Vec<Vec<bool>>, // this a cool abstraction as we don't need now the names
    pub elevator: usize,
}

impl State {
    #[inline]
    pub fn new(config: &Vec<Vec<bool>>) -> Self {
        let config = config.clone();

        Self {
            config,
            elevator: 0,
        }
    }

    pub fn is_goal(&self) -> bool {
        // function to verify if the current position is goal
        self.config.last().unwrap().iter().all(|x| *x)
    }

    pub fn is_safe(&self) -> bool {
        // function to verify if a floor is safe

        // let idx = self.elevator;

        for roof in &self.config {
            // check if there is any generator in the room the could fry other chips
            let any_generator = roof.chunks(2).map(|chunk| chunk[0]).any(|f| f);
            let micro_without_gen = roof.chunks(2).map(|chunk| chunk[1] && !chunk[0]).any(|f| f);

            if any_generator && micro_without_gen {
                return false;
            }
        }
        true
    }

    // now for the complex function
    // generate the set of neighbors
    pub fn get_transition(&self) -> Vec<Self> {
        // function to get all the possible [safe] configuration
        let mut results = Vec::new();
        let mut indices = Vec::new();
        for (i, v) in self.config[self.elevator].iter().enumerate() {
            if *v {
                indices.push(i);
            }
        }

        // loop over the move of a single item
        for item in indices.iter() {
            let mut other = self.clone();
            let i = self.elevator;

            // check if we could move up
            if i < 3 {
                other.elevator = i + 1;
                other.config[i + 1][*item] = true;
                other.config[i][*item] = false;

                if other.is_safe() {
                    results.push(other);
                }
            }

            // now we check down
            let mut other = self.clone();
            if i > 0 {
                other.elevator = i - 1;
                other.config[i - 1][*item] = true;
                other.config[i][*item] = false;

                if other.is_safe() {
                    results.push(other);
                }
            }
        }

        // now we loop on two items move
        for comb in indices.iter().combinations(2) {
            let mut other = self.clone();
            let i = self.elevator;

            // check if we could move up
            if i < 3 {
                other.elevator = i + 1;
                other.config[i + 1][*comb[0]] = true;
                other.config[i][*comb[0]] = false;

                other.config[i + 1][*comb[1]] = true;
                other.config[i][*comb[1]] = false;

                if other.is_safe() {
                    results.push(other.clone());
                }
            }

            // now we check down
            let mut other = self.clone();
            if i > 0 {
                other.elevator = i - 1;
                other.config[i - 1][*comb[0]] = true;
                other.config[i][*comb[0]] = false;

                other.config[i - 1][*comb[1]] = true;
                other.config[i][*comb[1]] = false;

                if other.is_safe() {
                    results.push(other.clone());
                }
            }
        }

        results
    }
}

pub fn bfs(state: &State) -> usize {
    // function to get the number of moves to get the goal
    let mut queue: VecDeque<(State, usize)> = VecDeque::new();
    let mut visited = HashSet::new();

    // enqueue the first state
    queue.push_back((state.clone(), 0));
    visited.insert(state.clone());

    while !queue.is_empty() {
        let (state, moves) = queue.pop_front().unwrap();
        // println!("checking \n{state}");

        if state.is_goal() {
            return moves;
        }

        // now we enter the transition
        for nei in state.get_transition() {
            if !visited.contains(&nei) {
                queue.push_back((nei.clone(), moves + 1));
                visited.insert(nei.clone());
            }
        }
    }

    0
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..4).rev() {
            let elev = if self.elevator == i { "E" } else { "." };
            write!(f, "F{}  {elev}  ", i + 1)?;
            for j in 0..self.config[0].len() {
                let exists = if self.config[i][j] { "*" } else { "." };
                write!(f, "{exists} ")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
