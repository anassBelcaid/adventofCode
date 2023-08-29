use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::hash::Hash;
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

    fn reverse_transformation(&mut self) {
        // function to reverse all the transformation
        for t in self.operations.iter_mut() {
            *t = Transformation(t.1.to_string(), t.0.to_string());
        }
    }
}

fn get_all_replacements(
    transf: &Transformation,
    initial: &str,
    cache: &mut HashSet<String>,
) -> HashSet<String> {
    // function to get all the transformation
    let left = &transf.0;
    let right = &transf.1;

    let m = left.len();
    let n = initial.len();

    // result hashSet
    let mut result = HashSet::new();

    // Creating the vector of results
    let limit = (n as i32 - m as i32 + 1).max(0);

    for i in 0..limit as usize {
        // we have a match
        if initial[i..i + m] == *left {
            let mut copy = initial.to_string();
            copy.replace_range(i..i + m, &right);

            if !cache.contains(&copy) {
                result.insert(copy.clone());
                cache.insert(copy);
            }
        }
    }
    result
}

fn part1(problem: &Problem) -> usize {
    let mut cache = HashSet::new();

    for trans in problem.operations.iter() {
        let output = get_all_replacements(&trans, &problem.start, &mut cache);
        cache.extend(output);
    }

    cache.len()
}

fn part2(problem: &mut Problem) -> usize {
    // function to compute the result for partII

    // first we need to inverse the transformations
    problem.reverse_transformation();

    // now I will create a binary heap to go for gready approach
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Element(i32, usize, String); // - heuristic and the string

    // first we will put the element in the queue
    let mut queue = BinaryHeap::new();

    let h0 = problem.start.len() as i32;

    queue.push(Element(-h0, 0, problem.start.clone()));

    // compute all the transformation
    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let elem = queue.pop().unwrap();

        // Check if it is goal
        if elem.2 == "e" {
            return elem.1;
        }

        // we loop over all the transfromations of the  current string
        let mut neighbors = HashSet::new();

        for trans in problem.operations.iter() {
            let out = get_all_replacements(&trans, &elem.2, &mut visited);
            neighbors.extend(out);
        }

        for molecule in neighbors {
            visited.insert(molecule.clone());
            let h = molecule.len() as i32;

            queue.push(Element(-h, elem.1 + 1, molecule));
        }
    }

    0
}

fn main() {
    let mut file = File::open("input/day19").expect("Invalid file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Invalid content");
    //
    // // Creating the vector of lines
    let input: Vec<String> = content.lines().map(|x| x.to_string()).collect();

    // creating the problem
    let mut problem = Problem::new(input);

    // first part
    let answer1 = part1(&problem);
    println!("Part I = {answer1}");

    // let's rethink the problem of part II
    let answer2 = part2(&mut problem);
    println!("Part II = {answer2}");
}
