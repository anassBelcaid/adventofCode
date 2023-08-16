use std::fs::File;
use std::io::Read;

fn look_and_say(number: &str) -> String {
    // function to apply the concept of look and say
    // which will compress the string by saying each repetition of each digit.

    let mut result = String::new();
    let chars: Vec<char> = number.chars().collect();
    let n = chars.len();

    let mut i = 0;

    while i < n {
        // Getting the ref character
        let c = chars[i];
        let mut count = 1;
        i += 1;

        while i < n && chars[i] == c {
            i += 1;
            count += 1;
        }

        // Saving the result
        let rep = count.to_string();
        result.push_str(&rep);
        result.push(c);
    }
    result
}
fn main() {
    let mut file = File::open("input/day10").expect("Invalid file");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("invalid string");

    let mut value = content.trim().to_string();
    for _ in 0..40 {
        value = look_and_say(&value);
    }
    let answer1 = value.len();
    println!("Part I = {answer1}");
}
