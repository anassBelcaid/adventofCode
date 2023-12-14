use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Maze {
    adjacency: Vec<Vec<usize>>, // the position (i,j ) will be saved as i *m + j
    m: usize,
    n: usize,
    start: usize, // starting the position
}

impl Maze {
    #[inline]
    pub fn new(description: &str) -> Self {
        let mut start = 0;

        let n = description.lines().count();
        let m = description.lines().nth(0).unwrap().len();

        // now we can create the adjacency
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n * m];

        // reading the number of
        for (i, line) in description.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let node = i * m + j;

                // case analysis
                if c == '|' {
                    if i > 0 {
                        adjacency[node].push((i - 1) * m + j);
                    }
                    if i < n - 1 {
                        adjacency[node].push((i + 1) * m + j);
                    }
                }

                if c == '-' {
                    if j > 0 {
                        adjacency[node].push(i * m + j - 1);
                    }
                    if j < m - 1 {
                        adjacency[node].push(i * m + j + 1);
                    }
                }

                if c == 'L' {
                    if i > 0 {
                        adjacency[node].push((i - 1) * m + j);
                    }
                    if j < m - 1 {
                        adjacency[node].push(i * m + j + 1);
                    }
                }

                if c == 'J' {
                    if i > 0 {
                        adjacency[node].push((i - 1) * m + j);
                    }
                    if j > 0 {
                        adjacency[node].push(i * m + j - 1);
                    }
                }

                if c == '7' {
                    if i < n - 1 {
                        adjacency[node].push((i + 1) * m + j);
                    }
                    if j > 0 {
                        adjacency[node].push(i * m + j - 1);
                    }
                }

                if c == 'F' {
                    if i < n - 1 {
                        adjacency[node].push((i + 1) * m + j);
                    }
                    if j < m - 1 {
                        adjacency[node].push(i * m + j + 1);
                    }
                }
                if c == 'S' {
                    start = node;
                }
            }
        }
        // fix the connectivity of the start
        let mut start_nei = Vec::new();
        for (node, nei) in adjacency.iter().enumerate() {
            if nei.len() > 0 && nei[0] == start {
                start_nei.push(node);
            }
            if nei.len() > 1 && nei[1] == start {
                start_nei.push(node);
            }
        }

        for v in start_nei {
            adjacency[start].push(v);
        }

        Self {
            adjacency,
            m,
            n,
            start,
        }
    }

    pub fn tail(&self) -> usize {
        let mut answer = 0;

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited = HashSet::new();

        // pushing the starting point to the stack
        queue.push_back((self.start, 0));
        visited.insert(self.start);

        while !queue.is_empty() {
            // popping the next element
            let (current, moves) = queue.pop_front().unwrap();

            let cx = current / self.m;
            let cy = current % self.m;

            // computing the max distance
            answer = answer.max(moves);

            // go the neighbors
            for nei in self.adjacency[current].iter() {
                if !visited.contains(&nei) {
                    queue.push_back((*nei, moves + 1));
                    visited.insert(*nei);
                }
            }
        }

        answer
    }

    pub fn num_nests(&self) -> u32 {
        // compute the number of usize points
        let mut answer = 0u32;

        // Now I should create the cycle
        let mut cycle: Vec<usize> = Vec::new();

        let mut stack: Vec<usize> = Vec::new();
        stack.push(self.start);

        // Creating the set of vistited nodes
        let mut visited = HashSet::new();
        visited.insert(self.start);

        while !stack.is_empty() {
            // get the next element
            let current = stack.pop().unwrap();
            cycle.push(current);

            // pushing the neighbors
            for nei in self.adjacency[current].iter() {
                if !visited.contains(&nei) {
                    stack.push(*nei);
                    visited.insert(*nei);
                }
            }
        }
        let points: Vec<Point> = cycle
            .iter()
            .map(|u| Point {
                x: *u as u32 / self.m as u32,
                y: *u as u32 % self.m as u32,
            })
            .collect();

        // computing the area
        let A = polygon_area(&points[..]);
        // computing the number of inside points
        // using Pick theorem
        // https://en.wikipedia.org/wiki/Pick%27s_theorem
        let b: u32 = points.len() as u32 / 2;
        A as u32 - b + 1
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}
fn polygon_area(points: &[Point]) -> f64 {
    let mut area = 0.0;
    let n = points.len();

    for i in 0..n {
        let j = (i + 1) % n;
        area += (points[i].x as f64) * (points[j].y as f64);
        area -= (points[j].x as f64) * (points[i].y as f64);
    }

    (area / 2.0).abs()
}
