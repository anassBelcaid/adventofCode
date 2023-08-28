// TODO : Use scanner for reading
use fancy_regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Transformation(String, String);

#[derive(Debug)]
struct Problem {
    pub operations: Vec<Transformation>, // set of operations
    pub start: String,                   // starting position
}

impl Problem {
    #[inline]
    fn new(input: Vec<String>) -> Self {
        let n = input.len();
        let start = input.last().unwrap().to_string();
        let mut operations = Vec::new();

        for i in 0..n - 2 {
            let operation: Vec<String> = input[i].split(' ').map(|x| x.to_string()).collect();

            operations.push(Transformation(
                operation[0].to_owned(),
                operation[2].to_owned(),
            ));
        }

        Self { start, operations }
    }
}

fn get_all_replacements(transf: &Transformation, initial: &str, cache: &mut HashSet<String>) {
    // function to get all the transformation
    let left = &transf.0;
    let right = &transf.1;

    let m = left.len();
    let n = initial.len();

    // Creating the vector of results

    for i in 0..(n - m + 1) {
        // we have a match
        if initial[i..i + m] == *left {
            let mut copy = initial.to_string();
            copy.replace_range(i..i + m, &right);

            cache.insert(copy);
        }
    }
}

fn part1(problem: &Problem) -> usize {
    let mut cache = HashSet::new();

    for trans in problem.operations.iter() {
        get_all_replacements(&trans, &problem.start, &mut cache);
    }

    cache.len()
}

fn main() {
    let mut file = File::open("input/day19").expect("Invalid file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Invalid content");

    // Creating the vector of lines
    let input: Vec<String> = content.lines().map(|x| x.to_string()).collect();

    // creating the problem
    let problem = Problem::new(input);

    // first part
    let answer1 = part1(&problem);
    println!("Part I = {answer1}");
}
