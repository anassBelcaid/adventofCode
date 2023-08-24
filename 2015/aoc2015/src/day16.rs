// we could represent the Aunt as a structure with each field but an array is simple for
// computation

// Here is the order of the caractesistics
// children: 3
// cats: 7
// samoyeds: 2
// pomeranians: 3
// akitas: 0
// vizslas: 0
// goldfish: 5
// trees: 3
// cars: 2
// perfumes: 1

use fancy_regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type Aunt = Vec<Option<i32>>;

pub fn create_feature_idx() -> HashMap<String, usize> {
    let features = vec![
        "children",
        "cats",
        "samoyeds",
        "pomeranians",
        "akitas",
        "vizslas",
        "goldfish",
        "trees",
        "cars",
        "perfumes",
    ];

    let mut result = HashMap::new();

    for (i, feat) in features.iter().enumerate() {
        result.insert(feat.to_string(), i);
    }
    result
}

fn read_aunt(line: &str, feat_idx: &HashMap<String, usize>) -> Vec<Option<i32>> {
    // function to read the value of  aunt in text and return its
    // features usign regex
    let mut result: Vec<Option<i32>> = vec![None; feat_idx.len()];

    let pattern = Regex::new(r"\w+: (\d+)").unwrap();

    for m in pattern.find_iter(line) {
        let mut features: Vec<String> = m
            .unwrap()
            .as_str()
            .split(' ')
            .map(|x| x.to_owned())
            .collect();

        // remove the :
        features[0].pop();
        let value: i32 = features[1].parse().unwrap();

        let index = feat_idx.get(&features[0]).expect("invalid key");

        result[*index] = Some(value);
    }

    result
}

// should wrap those in type for avoiding repeating the same type
fn match1(left: &Vec<Option<i32>>, right: &Vec<Option<i32>>) -> bool {
    for (a, b) in left.iter().zip(right) {
        if a.is_some() && b.is_some() {
            if a.unwrap() != b.unwrap() {
                return false;
            }
        }
    }
    true
}

// should wrap those in type for avoiding repeating the same type
fn match2(elem: &Vec<Option<i32>>, gt: &Vec<Option<i32>>) -> bool {
    // matching with elem agains tthe ground truth gt

    for (i, (a, b)) in elem.iter().zip(gt).enumerate() {
        if a.is_some() && b.is_some() {
            let a = a.unwrap();
            let b = b.unwrap();
            match i {
                1 | 7 => {
                    if a <= b {
                        return false;
                    }
                } // cats and trees greater than that many
                3 | 6 => {
                    if a >= b {
                        return false;
                    }
                } // pomeranians and goldfish  less that that
                _ => {
                    if a != b {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn part1(aunts: &Vec<Aunt>, signature: &Aunt) -> usize {
    // Get the indice of the aunt that match the signature
    for (i, aunt) in aunts.iter().enumerate() {
        if match1(aunt, signature) {
            return i + 1;
        }
    }
    0
}

fn part2(aunts: &Vec<Aunt>, signature: &Aunt) -> usize {
    // Get the indice of the aunt that match the signature
    for (i, aunt) in aunts.iter().enumerate() {
        if match2(aunt, signature) {
            return i + 1;
        }
    }
    0
}
fn main() {
    // Input feature
    let signature = vec![
        Some(3),
        Some(7),
        Some(2),
        Some(3),
        Some(0),
        Some(0),
        Some(5),
        Some(3),
        Some(2),
        Some(1),
    ];

    // now we read those features
    let mut file = File::open("input/day16").expect("File not found");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Invalid content");

    // Creating the fature index
    let feat_idx = create_feature_idx();

    let mut aunts = Vec::new();
    for line in content.lines() {
        let aunt = read_aunt(&line, &feat_idx);
        aunts.push(aunt);
    }

    // begin partI
    let answer1 = part1(&aunts, &signature);
    println!("Part I = {answer1} ");

    // begin Part II
    let answer2 = part2(&aunts, &signature);
    println!("Part II = {answer2} ");
}
