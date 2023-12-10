use std::fs::read_to_string;

use aoc23::fertilizer::Garden;

pub fn part_1() -> i64 {
    // function to get the answer to first part
    // let content = read_to_string("baby").expect("invalid content");
    let content = read_to_string("input/day5").expect("invalid content");

    // Creating the garden
    let mut garden = Garden::new(&content);
    garden.sort_sections();

    // lets compute the value for the first seed
    let mut min_location = i64::MAX;

    for seed in garden.seeds.iter() {
        min_location = min_location.min(garden.seed_soil_number(*seed));
    }

    min_location
}

pub fn part_2() -> i64 {
    // function to get the answer to first part
    // let content = read_to_string("baby").expect("invalid content");
    let content = read_to_string("input/day5").expect("invalid content");

    // Creating the garden
    let garden = Garden::new(&content);

    let mut location = 0;

    loop {
        if let Some(v) = garden.location_seed(location) {
            return location;
        }
        location += 1;
    }
}
pub fn main() {
    // computing the first part
    let answer1 = part_1();
    println!("part I = {answer1}");

    // computing the second part
    let answer2 = part_2();
    println!("part II = {answer2}");
}
