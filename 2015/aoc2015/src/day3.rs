use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Position(i32, i32);
// represent the Santa logic
struct Santa {
    x: i32,
    y: i32,
    cache: HashSet<Position>,
}

impl Santa {
    pub fn new() -> Self {
        let mut m = Santa {
            x: 0,
            y: 0,
            cache: HashSet::new(),
        };
        // insert the first position
        m.cache.insert(Position(0, 0));
        m
    }

    // The correct name is move but cannot use since it s a keywoard
    pub fn change(&mut self, c: char) {
        // apply the Santa movement logic
        match c {
            '>' => self.x += 1,
            '<' => self.x -= 1,
            '^' => self.y += 1,
            'v' => self.y -= 1,
            _ => panic!("invalid direction"),
        }

        self.cache.insert(Position(self.x, self.y));
    }
}

fn main() {
    let mut file = File::open("input/day3").expect("Input file 3 not found");

    let mut directions = String::new();
    file.read_to_string(&mut directions)
        .expect("Invalid content");

    // Now we create the santa and we stat moving it.
    let mut santa = Santa::new();

    for direction in directions.chars() {
        santa.change(direction);
    }
    println!("Part1 : {}", santa.cache.len());
}
