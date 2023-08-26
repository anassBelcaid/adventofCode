use std::fs::File;
use std::io::Read;

type Matrix = Vec<Vec<i32>>;

fn print_matrix(dp: &Matrix) {
    for i in 0..dp.len() {
        for j in 0..dp[0].len() {
            print!("{} ", dp[i][j]);
        }
        println!("");
    }
}

fn compute_value(capacity: usize, j: usize, capacities: &Vec<usize>, dp: &mut Matrix) -> i32 {
    // base case
    if dp[capacity][j] != -1 {
        return dp[capacity][j];
    }

    let mut res = compute_value(capacity, j - 1, capacities, dp);

    if capacity >= capacities[j - 1] {
        res += compute_value(capacity - capacities[j - 1], j - 1, capacities, dp)
    }

    dp[capacity][j] = res;
    res
}
fn num_combinations(target: usize, capacities: &Vec<usize>) -> i32 {
    // compute the number of ways we could fill the target with capacities.
    let m = capacities.len();
    let mut dp = vec![vec![-1; m + 1]; target + 1];

    // known values
    dp[0][0] = 1;
    for i in 1..=target {
        dp[i][0] = 0;
    }
    for j in 1..=m {
        dp[0][j] = 1;
    }

    let val = compute_value(target, m, &capacities, &mut dp);
    return val;
}

fn main() {
    let mut file = File::open("input/day17").expect("Invalid file");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("invalid content");
    let mut capacities: Vec<usize> = content.lines().map(|x| x.parse().unwrap()).collect();
    capacities.sort();

    let target = 150;

    let answer1 = num_combinations(target, &capacities);
    println!("Part I = {answer1}");
}
