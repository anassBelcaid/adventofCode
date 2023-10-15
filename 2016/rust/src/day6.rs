use counter::Counter;
use std::fs;

// simple parser to parse a single tag
//
fn most_repated_char(signals: &Vec<String>, i: usize) -> char {
    // get the most repeated character in a given position from
    // the vector of strings
    let counts = signals
        .iter()
        .map(|line| line.chars().nth(i))
        .collect::<Counter<_>>();
    counts.k_most_common_ordered(1)[0].0.unwrap()
}
fn partI(signals: &str) -> String {
    // function to get th
    let signals: Vec<String> = signals.split('\n').map(|x| x.to_string()).collect();
    let n = signals.len();
    let m = signals[0].len();

    // lets create the message
    let mut decoded = String::new();
    for pos in 0..m {
        decoded.push(most_repated_char(&signals, pos));
    }

    decoded
}

fn main() {
    let signals = fs::read_to_string("input/day6").expect("file not found");

    // computing the firet answer
    let answer1 = partI(&signals);
    println!("answer1 = {answer1}");
}
