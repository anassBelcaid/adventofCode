use crate::gates::Network;
use std::collections::LinkedList;
use std::fs::File;
use std::io::Read;
mod gates;

fn main() {
    // Creatin the file
    // let mut file = File::open("input/baby").expect("Invalid baby file");

    let mut file = File::open("input/day7").expect("Invalid baby file");
    // Content
    let mut instructions = String::new();

    file.read_to_string(&mut instructions)
        .expect("invalid content");

    // Creating the network
    let mut network = Network::new();

    for instruction in instructions.lines() {
        network.add_gate_from_insruction(instruction);
    }

    // now we group all the elements that could send their value
    let mut new_nodes = LinkedList::new();
    for (i, gate) in network.gates.iter().enumerate() {
        if gate.value.is_some() {
            new_nodes.push_back(i);
        }
    }

    // let the new nodes populate their values
    network.populate_values(&mut new_nodes);

    // populate the adjacency
    let idx_a = network.name_idx["a"];
    let answer1 = network.gates[idx_a].value.unwrap();
    println!("part I = {}", answer1);

    // now we compute the value of second part
    let idx_b = network.name_idx["b"];
    network.reset_all();
    network.gates[idx_b].value = Some(answer1);
    network.gates[idx_b].got_signal = true;

    //
    // this should be put into a function ???? since it repeated
    new_nodes.clear();
    for (i, gate) in network.gates.iter().enumerate() {
        if gate.value.is_some() {
            new_nodes.push_back(i);
        }
    }
    network.populate_values(&mut new_nodes);
    //
    let answer2 = network.gates[idx_a].value.unwrap();
    println!("part I = {}", answer2);
}
