use fancy_regex::Regex;
use std::fs::File;
use std::io::Read;

fn part1(content: &str) -> i64 {
    // function to read the Json file and compute the sum of all the numbers
    // we will use (not sure and I will bite my tongue in part 2) regular expressions
    // to parse all the numbers

    let pattern = Regex::new(r"-?\d+").unwrap();
    let mut sum = 0i64;

    for number in pattern.find_iter(&content) {
        let val: i64 = number
            .expect("invalid regular")
            .as_str()
            .parse()
            .expect("Invalid number");

        // println!("val is {val}");
        sum += val;
    }
    sum
}

fn main() {
    let mut file = File::open("input/day12").expect("Input file day 12 is not found");

    // parsing the content
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("invalid content in the file");

    // Solvign the first part
    let answer1 = part1(&content);
    println!("part I = {answer1}");
}
