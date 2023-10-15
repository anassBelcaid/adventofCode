use std::fmt::Display;

use regex::Regex;
const WIDTH: usize = 50;
//
// const WIDTH: usize = 7;
const HEIGHT: usize = 6;
// const HEIGHT: usize = 3;

type Matrix = [[bool; WIDTH]; HEIGHT];
#[derive(Debug)]
pub struct LCDScreen {
    lights: Matrix,
}

impl LCDScreen {
    #[inline]
    pub fn new() -> Self {
        Self {
            lights: [[false; WIDTH]; HEIGHT],
        }
    }

    // function to light up rectangle
    pub fn rect(&mut self, x: usize, y: usize) {
        // function to light up the rectangle
        for j in 0..x {
            for i in 0..y {
                self.lights[i][j] = true;
            }
        }
    }
    pub fn rotate_col(&mut self, A: usize, shift: usize) {
        // shift all the pixels  on the colum with index A with [shift]
        // first let's copy the column
        let col: Vec<bool> = self.lights.iter().map(|line| line[A]).collect();

        // now we change the position of each pixel
        for i in 0..HEIGHT {
            let index = (i as i32 - shift as i32).rem_euclid(HEIGHT as i32);
            self.lights[i][A] = col[index as usize];
        }
    }

    pub fn roatate_row(&mut self, A: usize, shift: usize) {
        // shift all the pixels  on the colum with index A with [shift]
        // first let's copy the column
        let col: Vec<bool> = self.lights[A].iter().map(|x| *x).collect();

        // now we change the position of each pixel
        for j in 0..WIDTH {
            let index = (j as i32 - shift as i32).rem_euclid(WIDTH as i32);
            self.lights[A][j] = col[index as usize];
        }
    }

    pub fn num_lights(&self) -> i32 {
        // compute the number of lights
        self.lights
            .iter()
            .map(|line| line.iter().map(|x| if *x { 1 } else { 0 }).sum::<i32>())
            .sum()
    }
}

impl Display for LCDScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let's try to loop over all the lines
        for line in self.lights.iter() {
            for c in line.iter() {
                let c = if *c { '#' } else { '.' };
                write!(f, "{c}")?
            }
            writeln!(f, "")?
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Instruction {
    Rect(usize, usize),
    RotateColumn(usize, usize),
    RotateRow(usize, usize),
}

pub fn parse_instruction(line: &str) -> Option<Instruction> {
    if let Some(captures) = Regex::new(r"^rect (\d+)x(\d+)$").unwrap().captures(line) {
        let width = captures[1].parse().unwrap();
        let height = captures[2].parse().unwrap();
        return Some(Instruction::Rect(width, height));
    }

    if let Some(captures) = Regex::new(r"^rotate row y=(\d+) by (\d+)$")
        .unwrap()
        .captures(line)
    {
        let row = captures[1].parse().unwrap();
        let amount = captures[2].parse().unwrap();
        return Some(Instruction::RotateRow(row, amount));
    }

    if let Some(captures) = Regex::new(r"^rotate column x=(\d+) by (\d+)$")
        .unwrap()
        .captures(line)
    {
        let column = captures[1].parse().unwrap();
        let amount = captures[2].parse().unwrap();
        return Some(Instruction::RotateColumn(column, amount));
    }

    None
}
