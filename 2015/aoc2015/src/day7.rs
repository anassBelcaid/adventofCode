use crate::gates::Gate;
use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::Read;
mod gates;

fn main() {
    // Creatin the file
    // let mut file = File::open("input/baby").expect("Invalid baby file");

    let mut file = File::open("input/day7").expect("Invalid baby file");
    // Content
    let mut connexions = String::new();

    file.read_to_string(&mut connexions)
        .expect("invalid content");

    // let mut gates = Vec::new();
    let mut gates = LinkedList::new();
    let mut gate_values = HashMap::new();

    for connexion in connexions.lines() {
        let gate = Gate::from(connexion);

        // known value
        if gate.parents.is_empty() {
            gate_values.insert(gate.name.clone(), gate.value);
        } else {
            gates.push_back(gate);
        }
    }

    // transform all the numerical values to known values
    for gate in gates.iter() {
        for parent in gate.parents.iter() {
            if parent.parse::<i32>().is_ok() {
                gate_values.insert(parent.clone(), parent.parse::<u16>().unwrap());
            }
        }
    }

    while !gates.is_empty() {
        let mut gate = gates.pop_front().unwrap();

        if gate.all_known(&gate_values) {
            gate.reduce(&mut gate_values);
        } else {
            gates.push_back(gate);
        }
    }

    // filtering
    let answer1 = gate_values["a"];
    println!("part1 I = {}", answer1);
}
