use std::fs::File;
use std::io::Read;

fn prime_factors(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut num = n;

    // Factor out all 2s.
    while num % 2 == 0 {
        factors.push(2);
        num /= 2;
    }

    // Factor odd numbers.
    let mut i = 3;
    while i <= (num as f64).sqrt() as i64 {
        while num % i == 0 {
            factors.push(i);
            num /= i;
        }
        i += 2;
    }

    // If n is a prime number greater than 2.
    if num > 2 {
        factors.push(num);
    }

    factors
}

fn divisors(n: i64) -> Vec<i64> {
    let factors = prime_factors(n);
    let mut result = std::collections::HashSet::new();

    result.insert(1);
    result.insert(n);

    for i in 1..(1 << factors.len()) {
        let mut divisor = 1;
        for j in 0..factors.len() {
            if i & (1 << j) != 0 {
                divisor *= factors[j];
            }
        }
        result.insert(divisor);
    }

    let mut sorted_result: Vec<_> = result.into_iter().collect();
    sorted_result.sort();

    sorted_result
}

fn part1(target: usize) -> usize {
    // Function to return the smallest number to get [target] gifts

    // naive approach do a linear search until we get the vector
    let target = target / 10;

    let mut candidate = 2usize;

    loop {
        // println!("candidate = {candidate}");
        let sum_div = divisors(candidate as i64).iter().sum::<i64>();
        if sum_div >= target as i64 {
            return candidate;
        }
        candidate += 1;
    }
}
pub fn main() {
    // Creating the structure to read the file
    let mut file = File::open("input/day20").expect("invalid file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("invalid content");

    let target: usize = content.trim().parse().unwrap();
    // let target = 150;

    // counting the first answer
    let answer1 = part1(target);
    println!("Part I = {answer1}");
}
