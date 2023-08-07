use md5;
use std::{fs::File, io::Read};
fn is_good_md5(secret: &str, num_zeros: usize) -> bool {
    // Function to verify if the md5 hash of the secret is a
    // good hash (start with five zeros)

    let translation = format!("{:x}", md5::compute(secret));
    let zero = "0";
    translation.starts_with(&zero.repeat(num_zeros))
}

fn good_number(input: &str, num_zeros: usize) -> i32 {
    for number in 0..i32::MAX {
        // Creating the secret
        let secret = format!("{}{}", input, number);

        // checking the value
        if is_good_md5(&secret, num_zeros) {
            return number as i32;
        }
    }
    0i32
}
fn main() {
    // Creating the file
    let mut file = File::open("input/day4").expect("Invalid input 4");

    // the mutable string to store the input
    let mut input = String::new();

    // Reading the content of the file into the string
    file.read_to_string(&mut input).expect("Invalid content");
    let input = input.trim_end();

    let answer1 = good_number(&input, 5usize);
    println!("Part I : {}", answer1);

    let answer2 = good_number(&input, 6usize);
    println!("Part II : {}", answer2)
}
