use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Triangle {
    pub fn new(lenghts: Vec<i32>) -> Self {
        Self {
            a: lenghts[0],
            b: lenghts[1],
            c: lenghts[2],
        }
    }

    pub fn is_valid(&self) -> bool {
        // function to verify if the triangle is good
        let a = self.a;
        let b = self.b;
        let c = self.c;
        a + b > c && a + c > b && b + c > a
    }
}

pub fn part1(instructions: &str) -> i32 {
    // function to get the answer to part1
    let mut count = 0;
    for line in instructions.lines() {
        if line != "" {
            let mut weights = Vec::new();
            for token in line.trim().split(' ') {
                if token != "" {
                    weights.push(token.trim().parse::<i32>().unwrap());
                }
            }

            let t = Triangle::new(weights);
            if t.is_valid() {
                count += 1;
            }
        }
    }
    count
}
pub fn main() {
    let mut file = File::open("input/day3").expect("file day3 not found");
    let mut triangles = String::new();

    file.read_to_string(&mut triangles)
        .expect("invalid content");

    // answer to the first part
    let answer1 = part1(&triangles);
    println!("answer1 = {answer1}");
}
