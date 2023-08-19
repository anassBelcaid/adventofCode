use permutohedron::LexicalPermutation;
use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::Read;

struct Edge(usize, i32); // structure of an edge (index, value)

// now let's think about the graph representation
struct Network {
    pub idx_name: HashMap<String, usize>, // hash map to get the index of each element (string)
    pub adj: Vec<LinkedList<Edge>>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            idx_name: HashMap::new(),
            adj: Vec::new(),
        }
    }

    pub fn get_index(&mut self, name: &str) -> usize {
        // function to get the index of a name
        // if the name doesn't exist we add it
        if self.idx_name.contains_key(name) {
            return self.idx_name[name];
        } else {
            let n = self.idx_name.len();

            self.idx_name.insert(name.to_string(), n);

            // adding the adjacency
            self.adj.push(LinkedList::new());

            return n;
        }
    }

    pub fn add_instruction(&mut self, instruction: &str) {
        // function to add the edges in the instruction
        let mut tokens: Vec<String> = instruction.split(' ').map(|x| x.to_string()).collect();

        tokens[10].pop();

        let i = self.get_index(&tokens[0]);
        let j = self.get_index(&tokens[10]);
        let sign = &tokens[2];

        let mut value: i32 = tokens[3].parse().expect("Invalid");

        // changing the sign for lose
        if sign == "lose" {
            value *= -1;
        }
        self.adj[i].push_back(Edge(j, value));
    }

    pub fn get_adj_matrix(&self) -> Vec<Vec<i32>> {
        // function to get the adjacecny matrix since the graph is complete
        let n = self.idx_name.len();

        // Creating the adjacency
        let mut result = vec![vec![0; n]; n];

        // we must fill the nodes
        for i in 0..n {
            for elem in self.adj[i].iter() {
                result[i][elem.0] = elem.1;
            }
        }

        result
    }

    #[inline]
    fn sitting_score(places: &Vec<usize>, adjacency: &Vec<Vec<i32>>) -> i32 {
        // function to compute the score of a sitting index by places
        let mut sum = 0;

        let n = places.len();

        for i in 0..n {
            let right = if i < n - 1 { i + 1 } else { 0 };
            let left = if i > 0 { i - 1 } else { n - 1 };

            let index = places[i];
            let right = places[right];
            let left = places[left];
            sum += adjacency[index][right] + adjacency[index][left];
        }
        sum
    }
}

fn best_sitting(adjacency: &Vec<Vec<i32>>) -> i32 {
    // compute the best sitting for the network
    let n = adjacency.len();

    let mut sitting: Vec<usize> = (0..n).collect();

    let mut answer = Network::sitting_score(&sitting, &adjacency);

    while sitting.next_permutation() {
        // update the best sitting
        let val = Network::sitting_score(&sitting, &adjacency);
        answer = answer.max(val);
    }
    answer
}

fn main() {
    // Reading the content of the graph
    let mut file = File::open("input/day13").expect("Invalid input file");

    // reading the content
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Invalid content inside the file");

    // Creating the network
    let mut network = Network::new();

    for line in content.lines() {
        network.add_instruction(line);
    }

    // compute the adjacency
    let mut adjacency = network.get_adj_matrix();

    let answer1 = best_sitting(&adjacency);
    println!("Part I = {answer1}");

    // now we remove the adjacency by adding ourselves
    let n = adjacency.len();

    for line in adjacency.iter_mut() {
        line.push(0);
    }
    adjacency.push(vec![0; n + 1]);

    let answer2 = best_sitting(&adjacency);
    println!("Part II = {answer2}");
}
