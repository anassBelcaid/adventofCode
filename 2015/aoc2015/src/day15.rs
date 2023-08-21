use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Ingredient {
    properties: Vec<i32>,
}

impl Ingredient {
    #[inline]
    pub fn new(line: &str) -> Self {
        // Reading the ingredient values from a line

        let mut tokens: Vec<String> = line.split(' ').map(|x| x.to_string()).collect();

        // remove the last , from the wanted values
        for i in [2, 4, 6, 8] {
            tokens[i].pop();
        }
        let mut properties = Vec::new();

        for i in [2, 4, 6, 8, 10] {
            properties.push(tokens[i].parse().unwrap());
        }

        Self { properties }
    }
}

pub fn score(ingredients: &Vec<Ingredient>, partition: &Vec<i32>) -> i64 {
    // compute the score of the partition

    assert_eq!(partition.len(), ingredients.len());

    // we create the values for each property
    let mut scores = vec![0; 4];

    for i in 0..4 {
        // four ingredients

        let score_ingredient: i32 = ingredients
            .iter()
            .zip(partition)
            .map(|(ing, p)| ing.properties[i] * p)
            .sum::<i32>()
            .max(0);

        scores[i] = score_ingredient;
    }

    // we return the product of the ingredient scores
    scores.iter().product::<i32>() as i64
}

fn backtrack(
    ingredients: &Vec<Ingredient>,
    partition: &mut Vec<i32>,
    index: usize,
    max_score: &mut i64,
) {
    // base case
    if index == ingredients.len() {
        let s = score(ingredients, partition);
        if *max_score < s {
            *max_score = s;
        }
        return;
    }

    // getting the remaining part
    let mut rem = 100;

    for i in 0..index {
        rem -= partition[i];
    }

    for part in 0..=rem {
        partition[index] = part;
        backtrack(ingredients, partition, index + 1, max_score);
    }
}

fn main() {
    // Creating the file
    let mut file = File::open("input/day15").expect("File not found");

    // let creat the file that whill hold the content
    let mut content = String::new();

    file.read_to_string(&mut content).expect("invalid content");

    let mut ingredients = Vec::new();

    for line in content.lines() {
        ingredients.push(Ingredient::new(line));
    }

    let n = ingredients.len();
    let mut partition = vec![0; n];
    let mut max_score = 0;
    backtrack(&ingredients, &mut partition, 0, &mut max_score);

    println!("Part I = {}", max_score);
}
