use itertools::Itertools;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;

fn get_weights() -> Vec<i32> {
    // function to get the weights from the input
    let mut file = File::open("input/day24").expect("file not found");
    // let mut file = File::open("input/baby").expect("file not found");
    let mut lines = String::new();
    file.read_to_string(&mut lines).expect("invalid content");

    let mut result = Vec::new();
    for line in lines.lines() {
        result.push(line.parse().unwrap());
    }
    result
}

fn part_i() -> i64 {
    // function to comute the first part

    // Getting the weights
    let weights = get_weights();
    let n = weights.len();

    // Getting the target sum
    let target: i32 = weights.iter().sum::<i32>() / 3;

    // now print the combination
    // Let's try to solve the problem with a fixed combination size
    for comb_size in 1..=n {
        let mut pqueue = BinaryHeap::new();
        for comb in weights.iter().combinations(comb_size) {
            // check the sum
            // compute the sum
            let s: i32 = comb.iter().map(|x| *x).sum();
            if s == target {
                let p: i64 = comb.iter().map(|x| **x as i64).product();
                pqueue.push(-p);
            }
        }

        if let Some(v) = pqueue.pop() {
            return -v;
        }
    }
    0
}
pub fn main() {
    let answer1 = part_i();
    println!("Part I = {answer1}");
}
