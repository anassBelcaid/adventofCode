use std::collections::{HashMap, HashSet};
// struct to represent the Engine Scheme
type Position = usize;
type GearMap = HashMap<Position, HashSet<i32>>;
#[derive(Debug)]
pub struct EngineScheme {
    pub representation: Vec<Vec<char>>, // screen of the representation
    pub width: usize,                   // width of the screen
    pub height: usize,                  // height of the screen
    pub gears: GearMap,                 // Set of gears and their neighbors,
}
impl EngineScheme {
    #[inline]
    pub fn new(input: String) -> Self {
        let mut representation = Vec::new();

        for line in input.lines() {
            representation.push(line.chars().collect::<Vec<_>>());
        }
        let width = representation[0].len();
        let height = representation.len();
        let gears: GearMap = HashMap::new();

        Self {
            representation,
            width,
            height,
            gears,
        }
    }
    pub fn is_symbol(&self, i: usize, j: usize) -> bool {
        let c = self.representation[i][j];

        // Symbol is not a digit and it not the empty symbol .
        !c.is_digit(10) && c != '.'
    }

    pub fn has_near_symbol(&self, i: usize, j: usize) -> bool {
        // Creating the 8 neighbors diffs
        let dx = vec![1, 1, 0, -1, -1, -1, 0, 1];
        let dy = vec![0, 1, 1, 1, 0, -1, -1, -1, 0];

        for (u, v) in dx.iter().zip(dy.iter()) {
            let x = i as i32 + *u;
            let y = j as i32 + *v;

            if x >= 0 && x < self.height as i32 && y >= 0 && y < self.width as i32 {
                if self.is_symbol(x as usize, y as usize) {
                    return true;
                }
            }
        }
        false
    }

    pub fn near_gears(&self, i: usize, j: usize) -> Vec<Position> {
        // return the list of positins
        // Creating the 8 neighbors diffs
        let dx = vec![1, 1, 0, -1, -1, -1, 0, 1];
        let dy = vec![0, 1, 1, 1, 0, -1, -1, -1, 0];
        let mut results = Vec::new();

        for (u, v) in dx.iter().zip(dy.iter()) {
            let x = i as i32 + *u;
            let y = j as i32 + *v;

            if x >= 0 && x < self.height as i32 && y >= 0 && y < self.width as i32 {
                if self.representation[x as usize][y as usize] == '*' {
                    results.push(x as usize * self.width + y as usize);
                }
            }
        }
        results
    }

    pub fn fill_gears_numbers(&mut self) {
        let mut value = 0i32;
        let mut nei = Vec::new();
        // loop on each line
        for i in 0..self.height {
            for j in 0..self.width {
                if self.representation[i][j].is_digit(10) {
                    value *= 10;
                    value += self.representation[i][j].to_digit(10).unwrap() as i32;
                    for n in self.near_gears(i, j) {
                        nei.push(n);
                    }
                } else {
                    if value != 0 {
                        for n in nei.iter() {
                            self.gears.entry(*n).or_default().insert(value);
                        }
                    }
                    value = 0;
                    nei.clear();
                }
            }
            // when we finish a line en push the number
            if value != 0 {
                for n in nei.iter() {
                    self.gears.entry(*n).or_default().insert(value);
                }
            }

            value = 0;
            nei.clear();
        }
    }

    pub fn part_numbers(&self) -> Vec<i32> {
        // Function to find all the part numbers
        //
        let mut results = Vec::new();

        let mut value = 0i32;
        let mut has_s = false; // mutable bool to check the current number has symbol

        // loop on each line
        for i in 0..self.height {
            for j in 0..self.width {
                if self.representation[i][j].is_digit(10) {
                    value *= 10;
                    value += self.representation[i][j].to_digit(10).unwrap() as i32;
                    if self.has_near_symbol(i, j) {
                        has_s = true;
                    }
                } else {
                    if value != 0 && has_s {
                        results.push(value);
                    }

                    value = 0;
                    has_s = false;
                }
            }
            // when we finish a line en push the number
            if value != 0 && has_s {
                results.push(value);
            }

            value = 0;
            has_s = false;
        }

        results
    }
}
