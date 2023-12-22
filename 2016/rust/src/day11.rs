use aoc2016::radiology::{bfs, Radiology, State, RTG};
use std::fs::read_to_string;

fn part_i() -> usize {
    // let instructions = read_to_string("baby").expect("Invalid content");
    let instructions = read_to_string("../input/day11").expect("Invalid content");
    let instructions = instructions.trim();

    let mut rd = Radiology::new();

    for (i, line) in instructions.lines().enumerate() {
        rd.process_line(line, i);
    }
    rd.initate_config();

    let initial = State::new(&rd.config);
    println!("{initial}");

    // bfs
    bfs(&initial)
}

fn part_ii() -> usize {
    // let instructions = read_to_string("baby").expect("Invalid content");
    let instructions = read_to_string("../input/day11").expect("Invalid content");
    let instructions = instructions.trim();

    let mut rd = Radiology::new();

    for (i, line) in instructions.lines().enumerate() {
        rd.process_line(line, i);
    }

    // add the elerium  in floor 0
    rd.add_rtg_floor("elerium".to_string(), 0);

    // add the dilithium in floor 0
    rd.add_rtg_floor("dilithium".to_string(), 0);

    rd.initate_config();

    let initial = State::new(&rd.config);
    println!("{initial}");

    // bfs
    bfs(&initial)
}

pub fn main() {
    // let answer1 = part_i();
    // println!("answer I = {answer1}");
    //
    let answer2 = part_ii();
    println!("answer II = {answer2}");
}
