use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Edge(usize, i32); // index and distance
                         //
pub struct Network {
    name_idx: HashMap<String, usize>,
    adj: Vec<LinkedList<Edge>>, // adjacency list
}

impl Network {
    pub fn new() -> Self {
        Self {
            name_idx: HashMap::new(),
            adj: Vec::new(),
        }
    }
    pub fn get_index(&mut self, name: &str) -> usize {
        // function to get the index of a given city name
        // The function will include the city in case it doest
        // exists
        if self.name_idx.contains_key(name) {
            return self.name_idx[name];
        } else {
            let n = self.name_idx.len();
            self.name_idx.insert(name.to_string(), n);

            // we don't forget to add also the adjacency
            self.adj.push(LinkedList::new());
            return n;
        }
    }
    pub fn add_road(&mut self, instruction: &str) {
        // function to add a road given the instruction

        // Getting the tokens
        let tokens: Vec<&str> = instruction.split(' ').collect();
        let i = self.get_index(tokens[0]);
        let j = self.get_index(tokens[2]);
        let distance: i32 = tokens[4].parse().expect("Invalid distance");

        self.adj[i].push_back(Edge(j, distance));
        self.adj[j].push_back(Edge(i, distance));
    }
}

fn shortest_path_from(
    network: &Network,
    index: usize,
    visited: &mut Vec<bool>,
    current_dist: i32,
    min_path: &mut i32,
) {
    // function to compute the shortest path starting from index
    // the function will use the concept of backtracking in order
    // to consider all the roads
    visited[index] = true;

    if visited.iter().all(|x| *x) {
        *min_path = (*min_path).min(current_dist);
        return;
    }

    // we still have some town to visit
    // we mark the current node as visited
    for edge in network.adj[index].iter() {
        let j = edge.0;
        let dist = edge.1;
        if !visited[j] {
            shortest_path_from(network, j, visited, current_dist + dist, min_path);
            visited[j] = false;
        }
    }
}

fn longest_path_from(
    network: &Network,
    index: usize,
    visited: &mut Vec<bool>,
    current_dist: i32,
    max_path: &mut i32,
) {
    // function to compute the shortest path starting from index
    // the function will use the concept of backtracking in order
    // to consider all the roads
    visited[index] = true;

    if visited.iter().all(|x| *x) {
        *max_path = (*max_path).max(current_dist);
        return;
    }

    // we still have some town to visit
    // we mark the current node as visited
    for edge in network.adj[index].iter() {
        let j = edge.0;
        let dist = edge.1;
        if !visited[j] {
            longest_path_from(network, j, visited, current_dist + dist, max_path);
            visited[j] = false;
        }
    }
}
fn shortest_path(network: &Network) -> i32 {
    let mut min_path = i32::MAX;
    let n = network.adj.len();

    let mut visited = vec![false; n];
    for i in 0..n {
        shortest_path_from(network, i, &mut visited, 0, &mut min_path);
        visited[i] = false;
    }
    return min_path;
}

fn longest_path(network: &Network) -> i32 {
    let mut max_path = i32::MIN;
    let n = network.adj.len();

    let mut visited = vec![false; n];
    for i in 0..n {
        longest_path_from(network, i, &mut visited, 0, &mut max_path);
        visited[i] = false;
    }
    return max_path;
}

fn main() {
    // First let's read those edges
    let mut file = File::open("input/day9").expect("Invalid path");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("Invalid content");

    // Creating the network
    let mut network = Network::new();

    for line in content.lines() {
        network.add_road(line);
    }

    let answer1 = shortest_path(&network);
    println!("Part I = {answer1}");

    let answer2 = longest_path(&network);
    println!("Part II = {answer2}");
}
