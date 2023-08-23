use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Ingredient {
    properties: Vec<i64>,
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

pub fn score(ingredients: &Vec<Ingredient>, partition: &Vec<i64>) -> i64 {
    // compute the score of the partition

    assert_eq!(partition.len(), ingredients.len());

    // we create the values for each property
    let mut scores = vec![0i64; 4];

    for i in 0..4 {
        // four ingredients

        let score_ingredient: i64 = ingredients
            .iter()
            .zip(partition)
            .map(|(ing, p)| ing.properties[i] * p)
            .sum::<i64>()
            .max(0);

        scores[i] = score_ingredient as i64;
    }

    // we return the product of the ingredient scores
    scores.iter().product::<i64>()
}

fn compute_calories(ingredients: &Vec<Ingredient>, partition: &Vec<i64>) -> i64 {
    let mut sum = 0i64;

    for i in 0..ingredients.len() {
        sum += ingredients[i].properties[4] * partition[i];
    }
    sum
}
fn backtrack(
    ingredients: &Vec<Ingredient>,
    partition: &mut Vec<i64>,
    index: usize,
    max_score: &mut i64,
    teaspones: usize,      // remaining teaspone
    calories: Option<i64>, // needed calories
) {
    // base case
    if index == ingredients.len() - 1 {
        // we affect the remaining teaspons to the final ingredient
        partition[index] = teaspones as i64;
        let s = score(ingredients, partition);
        if *max_score < s {
            // now we match with calories if needed a fixed amount
            match calories {
                Some(v) => {
                    if compute_calories(ingredients, partition) == v {
                        *max_score = s;
                    }
                }
                None => *max_score = s,
            }
        }
        return;
    }

    for part in 0..=teaspones {
        partition[index] = part as i64;
        backtrack(
            ingredients,
            partition,
            index + 1,
            max_score,
            teaspones - part,
            calories,
        );
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
    backtrack(&ingredients, &mut partition, 0, &mut max_score, 100, None);
    println!("Part I = {}", max_score);

    // going for part 2
    let mut answer2 = 0;
    backtrack(
        &ingredients,
        &mut partition,
        0,
        &mut answer2,
        100,
        Some(500),
    );
    println!("Part II = {}", answer2);
}
