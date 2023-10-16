use std::fs;
#[test]
fn test1() {
    let line = "ADVENT".to_string();
    // /no markers and decompresses to itself with no changes, resulting in a decompressed length of `6`.
    let output = decoded_length(&line);
    assert_eq!(output, 6);

    let line = "A(1x5)BC".to_string();
    // repeats only the `B` a total of `5` times, becoming `ABBBBBC` for a decompressed length of `7`.
    let output = decoded_length(&line);
    assert_eq!(output, 7);

    let line = "(3x3)XYZ".to_string();
    let output = decoded_length(&line);
    assert_eq!(output, 9);

    let line = "A(2x2)BCD(2x2)EFG".to_string();
    let output = decoded_length(&line);
    assert_eq!(output, 11);

    let line = "(6x1)(1x3)A".to_string();
    let output = decoded_length(&line);
    assert_eq!(output, 6);

    let line = "X(8x2)(3x3)ABCY".to_string();
    let output = decoded_length(&line);
    assert_eq!(output, 18);
}

fn next_integer(line: &Vec<char>, index: &mut usize) -> i32 {
    // function to return  to read the integer starting from the position [index]
    // and return the next position
    let mut value = 0;
    assert!(line[*index].is_digit(10));

    while line[*index].is_digit(10) {
        value *= 10;
        value += (line[*index] as u8 - '0' as u8) as i32;
        *index += 1;
    }
    value
}

fn decoded_length(line: &str) -> usize {
    // compute the lenght of the decompressed line
    let mut index = 0;
    let mut count = 0usize;
    let n = line.len();
    let line: Vec<char> = line.chars().collect();

    while index < n {
        // normal character
        // println!("in char {}", line[index]);
        if line[index].is_whitespace() {
            index += 1;
        } else if line[index].is_alphabetic() {
            count += 1;
            index += 1;
        } else {
            // assuming a good line this will be a opening parenthesis
            // get the
            // skip the opening parenthesis
            index += 1;
            let num_char = next_integer(&line, &mut index);

            // escape the x
            index += 1;

            // reading how many time to repeat
            let repeat = next_integer(&line, &mut index);
            // println!("repeat is {repeat}\t num_char = {num_char}");

            // escaping the closing character
            index += 1;

            // now we escape the repeated character since we will add them
            count += repeat as usize * num_char as usize;

            index += num_char as usize;
        }
    }
    count
}
pub fn part_i() -> usize {
    let line = fs::read_to_string("../input/day9").expect("invalid file or content");

    decoded_length(&line)
}
pub fn main() {
    let answer1 = part_i();
    println!("part I = {answer1}");
}
