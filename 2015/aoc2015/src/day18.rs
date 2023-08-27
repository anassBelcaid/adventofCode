use std::fmt::Display;
use std::fs::File;
use std::io::Read;

//-------------- Creating the types ---------------------
//struct

const ON: u8 = b'#';
const OFF: u8 = b'.';

#[derive(Debug)]
struct Position(usize, usize);

struct Grid {
    grid: Vec<Vec<u8>>,
}

//------------------------------------------------------
impl Grid {
    #[inline]
    fn new(s: String) -> Self {
        // Create the grid from it's string representation

        let mut grid = Vec::new();

        for line in s.lines() {
            grid.push(line.bytes().collect::<Vec<u8>>());
        }
        Self { grid }
    }
    fn neighbors(&self, pos: &Position) -> Vec<Position> {
        // function to compute the neighbors of a given position
        //
        let n = self.grid.len();
        let m = self.grid[0].len();

        let dxs = vec![-1, 0, 1, 1, 1, 0, -1, -1];
        let dys = vec![1, 1, 1, 0, -1, -1, -1, 0];

        let mut neigh = Vec::new();

        for (dx, dy) in dxs.iter().zip(dys) {
            let x = pos.0 as i32 + dx;
            let y = pos.1 as i32 + dy;

            if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
                neigh.push(Position(x as usize, y as usize));
            }
        }
        neigh
    }

    fn num_on(&self, pos: &Position) -> usize {
        self.neighbors(&pos)
            .iter()
            .filter(|p| self.grid[p.0][p.1] == ON)
            .count()
    }

    fn num_on_lights(&self) -> usize {
        // return the number of on lights on the grid

        self.grid
            .iter()
            .map(|line| line.iter().filter(|&x| *x == ON).count())
            .sum()
    }

    fn next(&mut self) {
        // function to apply the logic behind and the lights and give the next state

        // we will use a queue of values to change in order to avoid copying the matrix
        let mut states = Vec::new(); // Vector for the position that need to be changed
        let mut values = Vec::new(); // the new value for each changed state

        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                // Getting the position
                let p = Position(i, j);

                // Getting the number of on lights
                let n = self.num_on(&p);

                //  getting the value of the light
                let v = self.grid[i][j];

                // case for a ligth on to off
                if v == ON && n != 2 && n != 3 {
                    states.push(Position(i, j));
                    values.push(OFF);
                }

                // case for light to switch from off to on
                if v == OFF && n == 3 {
                    states.push(Position(i, j));
                    values.push(ON);
                }
            }
        }

        // now we modify the values on the grid
        for (pos, val) in states.iter().zip(values) {
            self.grid[pos.0][pos.1] = val;
        }
    }

    fn is_corner(&self, p: &Position) -> bool {
        // function to check if a position is in the corner
        if p.0 == 0 && p.1 == 0 {
            return true;
        }

        if p.0 == 0 && p.1 == self.grid[0].len() - 1 {
            return true;
        }

        if p.0 == self.grid.len() - 1 && p.1 == 0 {
            return true;
        }

        if p.0 == self.grid.len() - 1 && p.1 == self.grid[0].len() - 1 {
            return true;
        }

        false
    }

    fn next2(&mut self) {
        // function to apply the logic behind and the lights and give the next state
        // second part of the game

        // we will use a queue of values to change in order to avoid copying the matrix
        let mut states = Vec::new(); // Vector for the position that need to be changed
        let mut values = Vec::new(); // the new value for each changed state

        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                // Getting the position
                let p = Position(i, j);

                // Getting the number of on lights
                let n = self.num_on(&p);

                //  getting the value of the light
                let v = self.grid[i][j];

                // case for a ligth on to off
                if v == ON && n != 2 && n != 3 {
                    if !self.is_corner(&p) {
                        states.push(Position(i, j));
                        values.push(OFF);
                    }
                }

                // case for light to switch from off to on
                if v == OFF && n == 3 {
                    states.push(Position(i, j));
                    values.push(ON);
                }
            }
        }

        // now we modify the values on the grid
        for (pos, val) in states.iter().zip(values) {
            self.grid[pos.0][pos.1] = val;
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print!("{esc}c", esc = 27 as char);

        for line in self.grid.iter() {
            for c in line {
                write!(f, "{}", if *c == ON { '#' } else { '.' })?;
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

fn part1(grid: &mut Grid) -> usize {
    // advance the sate for a number of iterations
    for _i in 0..100 {
        grid.next();
    }
    grid.num_on_lights()
}

fn part2(grid: &mut Grid) -> usize {
    // advance the sate for a number of iterations
    let n = grid.grid.len();
    let m = grid.grid[0].len();
    let cx: Vec<usize> = vec![0, n - 1, 0, n - 1];
    let cy: Vec<usize> = vec![0, 0, m - 1, m - 1];
    for (x, y) in cx.iter().zip(cy.iter()) {
        grid.grid[*x][*y] = ON;
    }
    for _i in 0..100 {
        grid.next2();
    }
    // println!("{grid}");
    grid.num_on_lights()
}
fn main() {
    // Should be given to a lib helper function
    let mut file = File::open("input/day18").expect("invalid content");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("invalid content");

    // let's first load the grid from the content
    let mut grid = Grid::new(content);

    // let's test the number of on lights
    let answer1 = part1(&mut grid);
    println!("Part I = {answer1}");

    // let answer part II
    let mut file = File::open("input/day18").expect("invalid content");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("invalid content");

    // let's first load the grid from the content
    let mut grid = Grid::new(content);
    let answer2 = part2(&mut grid);
    println!("Part II = {answer2}");
}
