use regex::Regex;

fn partI(door: &str) -> String {
    // function to find the password of the first part
    // of the problem
    let mut post = 0; // value that we add to add
    let mut result = String::new();

    while result.len() < 8 {
        // compute the value of the post
        let string = format!("{door}{post}");

        let hash = format!("{:x}", md5::compute(&string));

        if hash.starts_with("00000") {
            result.push(hash.chars().nth(5).unwrap());
        }

        post += 1;
    }
    result
}
fn partII(door: &str) -> String {
    // function to get the second part of the problem
    let mut chars = ['_'; 8];
    let mut post = 0; // value that we add to add
    let re = Regex::new("^0{5}[0-7]").unwrap();

    // many loop to fill all the characters
    while chars.iter().any(|x| *x == '_') {
        let string = format!("{door}{post}");

        let hash = format!("{:x}", md5::compute(&string));

        if re.is_match(&hash) {
            // println!("found a match {hash}");
            // I could use regex but I prefer this
            let index = hash.chars().nth(5).unwrap() as u8 - '0' as u8;
            let c = hash.chars().nth(6).unwrap();
            if chars[index as usize] == '_' {
                chars[index as usize] = c;
                // println!("{}", chars.iter().collect::<String>());
            }
        }

        post += 1;
    }

    chars.iter().collect::<String>()
}
pub fn main() {
    // let door = "abc".to_string();
    let door = "ugkcyxxp".to_string();

    // Getting the first part
    let answer1 = partI(&door);
    println!("part I = {answer1}");

    // Getting the second part
    let answer2 = partII(&door);
    println!("part II = {answer2}");
}
