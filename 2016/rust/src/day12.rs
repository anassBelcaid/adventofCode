use std::{fs::File, io::Read};
pub fn main() {
    println!("We'll use nom to process the data");
    let mut file = File::open("../input/day12").expect("invalid file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("invalid content");

    println!("content is {content}");
}
