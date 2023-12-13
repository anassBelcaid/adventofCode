use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::{
    character::complete::{alpha1, newline},
    sequence::terminated,
    IResult,
};
use num_integer::lcm;
#[derive(Debug)]
enum Direction {
    L,
    R,
}
impl Direction {
    #[inline]
    pub fn from(c: char) -> Self {
        match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("unrocognized char"),
        }
    }
}

// now network of the island
#[derive(Debug)]
pub struct Network {
    topology: Vec<(usize, usize)>,    // connectivity each node
    name_idx: HashMap<String, usize>, // hash map name -> idx
    idx_name: HashMap<usize, String>,
    directions: Vec<Direction>, // loop of directions
}

impl Network {
    #[inline]
    pub fn new(description: &str) -> Self {
        Self::parse(&description).unwrap().1
    }

    pub fn get_ghots_indices(&self) -> Vec<usize> {
        let mut ghosts = Vec::new();
        for (i, name) in self.idx_name.iter() {
            if name.ends_with("A") {
                ghosts.push(*i);
            }
        }

        ghosts
    }

    pub fn ghost_cycle(&self, ghost: usize) -> u32 {
        // function to get the length of the ghost cycle
        let mut g = ghost;
        let mut moves = 0u32;

        while !self.idx_name.get(&g).unwrap().ends_with("Z") {
            let direction = &self.directions[moves as usize % self.directions.len()];

            match direction {
                Direction::L => g = self.topology[g].0,
                Direction::R => g = self.topology[g].1,
            }
            moves += 1;
        }

        moves
    }

    pub fn num_moves_ghosts(&self) -> usize {
        // compute the number of moves for all the ghosts to
        // reach the end

        let ghosts = self.get_ghots_indices();

        // for each ghost we keep the the cycle number
        let cycles: Vec<_> = ghosts.iter().map(|x| self.ghost_cycle(*x)).collect();
        // println!("{cycles:?}");
        // computing the lcm of the cycles

        cycles.iter().fold(1, |acc, &num| lcm(acc, num as i64)) as usize
    }

    pub fn num_moves_get_out(&self) -> usize {
        // compute the num of moves to get out

        // Getting the index of the "AAA" site
        let mut current = *self.name_idx.get("AAA").unwrap();
        let mut moves = 0;
        let target = *self.name_idx.get("ZZZ").unwrap();

        loop {
            if current == target {
                return moves;
            }
            // Getting the current move
            let index = moves % self.directions.len();

            match self.directions[index] {
                Direction::L => current = self.topology[current].0,
                Direction::R => current = self.topology[current].1,
            }
            moves += 1;
        }
    }
    // getting the index of node
    fn get_index(&mut self, name: &str) -> usize {
        // function to get the index of a given name
        // if the name don't exist in the hash, we will add it

        let name = name.to_string();
        if self.name_idx.contains_key(&name) {
            return self.name_idx[&name];
        }

        // we should add the name
        let n = self.name_idx.len();

        self.name_idx.insert(name.to_string(), n);
        self.idx_name.insert(n, name.to_string());
        self.topology.push((0, 0));
        return n;
    }

    // function to parse the content
    fn parse(content: &str) -> IResult<&str, Self> {
        let topology: Vec<(usize, usize)> = Vec::new();
        let name_idx: HashMap<String, usize> = HashMap::new();
        let idx_name: HashMap<usize, String> = HashMap::new();

        // first we need to parse the directions
        let (content, directions) = terminated(alpha1, newline)(content)?;

        // Getting the directrions
        let directions: Vec<Direction> = directions.chars().map(|c| Direction::from(c)).collect();

        let mut network = Self {
            topology,
            name_idx,
            idx_name,
            directions,
        };
        let (content, _) = newline(content)?;
        for line in content.lines() {
            // parsing the first item
            let (rest, node1) = terminated(alphanumeric1, tag(" = ("))(line)?;
            let (rest, node2) = terminated(alphanumeric1, tag(", "))(rest)?;
            let (rest, node3) = terminated(alphanumeric1, tag(")"))(rest)?;

            // Getting the idxs
            let idx1 = network.get_index(node1);
            let idx2 = network.get_index(node2);
            let idx3 = network.get_index(node3);

            network.topology[idx1] = (idx2, idx3);
        }
        // now we need to process the lines
        Ok(("", network))
    }
}
