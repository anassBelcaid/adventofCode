use std::{fs::File, io::Read};
fn floor_position(directions: &str) -> usize {
    // function to return the position of the floor
    let mut elevation = 0;
    for (index, c) in directions.trim_end().chars().into_iter().enumerate() {
        if c == '(' {
            elevation += 1;
        } else {
            elevation -= 1;
        }
        if elevation < 0 {
            return index + 1;
        }
    }
    0usize
}
fn main() {
    // Creating the file
    let mut file = File::open("day1_1").expect("Invalide file");

    // Reading the message
    let mut directions: String = String::new();
    file.read_to_string(&mut directions)
        .expect("Unvalide string");

    // using iterator
    println!("{}", directions);
    let result: i32 = directions
        .trim_end()
        .chars()
        .into_iter()
        .map(|x| if x == '(' { 1 } else { -1 })
        .sum();
    println!("part 1 : {}", result);

    // now computing part 2
    let position = floor_position(&directions);
    println!("part 2 : {}", position);
}
