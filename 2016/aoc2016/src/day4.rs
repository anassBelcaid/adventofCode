use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Room {
    most: BinaryHeap<(usize, Reverse<char>)>, // binary heap for the most 5 common letter
    id: usize,
    hash: String,
}

impl Room {
    pub fn new(line: &str) -> Self {
        // wrapper to get the room speceficis
        let line: Vec<char> = line.chars().collect();
        let n = line.len();

        let mut in_hash = false;
        let mut counts = HashMap::new();
        let mut hash = String::new();
        let mut id = String::new();

        for c in line.iter() {
            if c.is_lowercase() {
                if in_hash {
                    hash.push(*c)
                } else {
                    *counts.entry(c).or_default() += 1;
                }
            }

            if *c == '[' {
                in_hash = true;
            }
            if c.is_digit(10) {
                id.push(*c);
            }
        }
        //creating the heap
        let mut most = BinaryHeap::new();
        for (key, val) in counts {
            most.push((val, Reverse(*key)));
        }
        Self {
            most,
            id: id.parse::<usize>().unwrap(),
            hash,
        }
    }

    fn is_good(&mut self) -> bool {
        // function to verify if the room is good
        // getting the hash
        let mut computed_hash = String::new();
        for _i in 0..5 {
            if let Some(v) = self.most.pop() {
                computed_hash.push(v.1 .0);
            }
        }
        // println!("{computed_hash}\t {}", self.hash);
        computed_hash == self.hash
    }
}

fn get_rooms() -> Vec<String> {
    // function to get the room
    let mut file = File::open("input/day4").expect("file day4 not found");
    let mut lines = String::new();

    file.read_to_string(&mut lines).expect("invalid content");

    let mut results = Vec::new();

    for line in lines.lines() {
        if line != "" {
            results.push(line.to_string());
        }
    }
    results
}
pub fn part1(rooms: &Vec<String>) -> usize {
    let mut sum = 0;
    for room in rooms.iter() {
        let mut room = Room::new(room);
        if room.is_good() {
            sum += room.id;
        }
    }
    return sum;
}

pub fn get_message(line: &str) -> String {
    let mut result = String::new();

    for c in line.chars() {
        if c.is_digit(10) {
            return result;
        }

        result.push(c);
    }
    result
}
pub fn decrypt(message: String, cypher: usize) -> String {
    // function to decrypt the message
    let cypher = cypher.rem_euclid(26);

    let mut result = String::new();

    for c in message.chars() {
        if c == '-' {
            result.push(' ');
        } else {
            let diff = (c as u8 - 'a' as u8 + cypher as u8) % 26;
            let new_c = ('a' as u8 + diff) as char;
            result.push(new_c);
        }
    }

    result
}
pub fn part2() -> usize {
    // the model mutate so I need to read the data again
    let mut file = File::open("input/day4").expect("file day4 not found");
    let mut lines = String::new();
    let target = "northpole object storage ".to_string();

    file.read_to_string(&mut lines).expect("invalid content");

    // let mut results = Vec::new();

    for line in lines.lines() {
        let message = get_message(&line);
        let mut room = Room::new(line);
        if room.is_good() {
            let decrypted = decrypt(message, room.id);
            if decrypted == target {
                return room.id;
            }
        }
    }
    0
    // results
}

pub fn main() {
    let rooms = get_rooms();
    let answer1 = part1(&rooms);
    println!("answer1 is {answer1}");

    let answer2 = part2();
    println!("answer1 is {answer2}");
}
