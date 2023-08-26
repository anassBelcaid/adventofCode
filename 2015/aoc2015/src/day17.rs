use std::collections::HashMap;
use std::collections::VecDeque;
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

fn num_combinations_with_min_containers(target: usize, capacities: &Vec<usize>) -> i32 {
    // function to compute the number of ways to get the sum while using
    // the minimum number of containers

    // first we don't have any path
    let mut min_move = None;
    let mut result = 0;

    let n = capacities.len();

    // we initiate the dfs to seach for the combination and also the number of moves
    struct Elem(usize, usize, usize); // target, index of the search and number of moves
    let mut queue = VecDeque::new();

    // First we are searching the target element using all indices with 0 moves
    queue.push_front(Elem(target, n, 0));

    while !queue.is_empty() {
        // getting the element
        let e = queue.pop_front().unwrap();

        // check if we have a goal
        if e.0 == 0 {
            match min_move {
                None => {
                    min_move = Some(e.2);
                    result += 1;
                }
                Some(v) => {
                    if e.2 == v {
                        result += 1;
                    }
                }
            }
        } else {
            for i in (0..e.1).rev() {
                if e.0 >= capacities[i] {
                    // computing the new capacitiy
                    let v = e.0 - capacities[i];
                    queue.push_back(Elem(v, i, e.2 + 1));
                }
            }
        }
    }

    result
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

    // let start part II
    let answer2 = num_combinations_with_min_containers(target, &capacities);
    println!("Part II = {answer2}");
}
