use std::fs::File;
use std::io::Read;

pub fn num_chars(line: &str) -> i32 {
    // count the real number of characters
    let mut count = 0;
    let mut index = 0usize;
    let n = line.len();

    let chars: Vec<char> = line.chars().collect();

    while index < n {
        if chars[index] != '\\' {
            // normal character
            count += 1;
            index += 1;
        } else {
            // encountered the escape character
            //check if decimal
            index += 1; // go the next
                        //
            if chars[index] == 'x' {
                index += 3;
                count += 1;
            } else {
                index += 1;
                count += 1;
            }
        }
    }

    count
}

fn encoded_len(c: char) -> i32 {
    // function to compute the length to encode a giving character
    if c == '\\' || c == '"' {
        return 2;
    } else {
        return 1;
    }
}

fn main() {
    // Reading the file
    let mut file = File::open("input/day8").expect("Invalid file");

    // strring with the contents
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Invalid content");
    //
    let mut total_chars = 0;
    let mut total_memory = 0; // number in memory

    for line in content.lines() {
        // println!("{line}");
        let n = line.len();

        // Augmenting the number of characters
        total_chars += n as i32;

        let slice = &line[1..n - 1];

        // Increasing the number of real char in memory
        total_memory += num_chars(&slice);
    }

    let answer1 = total_chars - total_memory;
    println!("part I = {answer1}");

    // let compute the second part
    let mut sum_encoded: i32 = 0;
    for line in content.lines() {
        // println!("{line}");
        let m: i32 = line.chars().map(encoded_len).sum();
        // println!("m = {} ", m + 2);
        sum_encoded += m + 2; // the two for the additional ""
    }
    let answer2 = sum_encoded - total_chars;
    println!("Part II = {answer2}");
}
