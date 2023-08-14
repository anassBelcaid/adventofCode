use std::collections::{HashMap, HashSet, LinkedList};

// naming the type
//
// Creating the struct of a gate
#[derive(Debug)]
pub enum Opertation {
    OR,
    NOT,
    AND,
    RSHIFT,
    LSHIFT,
    NONE,
}

#[derive(Debug)]
pub struct Gate {
    pub op: Opertation,
    pub parents: Vec<usize>,
    pub value: Option<u16>,
    pub got_signal: bool, //Field indicating if the gate got a signal
}

impl Gate {
    pub fn new() -> Self {
        Self {
            op: Opertation::NONE,
            parents: Vec::new(),
            value: None,
            got_signal: false,
        }
    }
}

#[derive(Debug)]
pub struct Network {
    // Class to represent a network of gates
    pub gates: Vec<Gate>,
    pub name_idx: HashMap<String, usize>, // index of each name
    pub adj: Vec<LinkedList<usize>>,      // adjacency list
}

impl Network {
    pub fn new() -> Self {
        Self {
            gates: Vec::new(),
            name_idx: HashMap::new(),
            adj: Vec::new(),
        }
    }
    pub fn all_known(&self, i: usize) -> bool {
        // return the boolean indicating if
        // all the parents of [i] are known
        for j in self.gates[i].parents.iter() {
            if self.gates[*j].value.is_none() {
                return false;
            }
        }
        return true;
    }

    pub fn populate_values(&mut self, new_nodes: &mut LinkedList<usize>) {
        // function to populate values from the list of new values
        // new values holds the indices that got a new value
        let mut seen: HashSet<usize> = HashSet::new();

        while !new_nodes.is_empty() {
            let i = new_nodes.pop_front().unwrap();

            if !seen.contains(&i) {
                // check if we could compute the value for the node
                if !self.gates[i].got_signal && self.all_known(i) {
                    self.compute_value(i);
                }

                // if it has its signal it could propagate to its children
                if self.gates[i].got_signal {
                    seen.insert(i);
                    for j in self.adj[i].iter() {
                        new_nodes.push_back(*j);
                    }
                }
            }
        }
    }
    pub fn reset_all(&mut self) {
        // function to reset all the non final nodes
        for gate in self.gates.iter_mut() {
            if gate.parents.len() > 0 {
                gate.got_signal = false;
                gate.value = None;
            }
        }
    }

    pub fn compute_value(&mut self, i: usize) {
        // function to compute the value of the node
        // [i] given its parents

        let value;
        let op = &self.gates[i].op;
        let parents = self.gates[i].parents.clone();

        match parents.len() {
            1 => {
                let other = self.gates[parents[0]].value.unwrap();
                match op {
                    Opertation::NONE => value = Some(other),
                    _ => value = Some(!other),
                }
            }
            _ => {
                let left = self.gates[parents[0]].value.unwrap();
                let right = self.gates[parents[1]].value.unwrap();

                match op {
                    Opertation::AND => value = Some(left & right),
                    Opertation::OR => value = Some(left | right),
                    Opertation::RSHIFT => value = Some(left.wrapping_shr(right as u32)),
                    _ => value = Some(left.wrapping_shl(right as u32)),
                }
            }
        }
        self.gates[i].got_signal = true;
        self.gates[i].value = value;
    }
    pub fn get_idx(&mut self, name: &str) -> usize {
        // function to get the idex of the gate
        // Either the gate exist or it will be created
        // the function will return the index of in the array
        if self.name_idx.contains_key(name) {
            return self.name_idx[name];
        } else {
            let n = self.gates.len();
            self.name_idx.insert(name.to_string(), n);
            let mut gate = Gate::new();
            // check if the name is value
            let val = name.parse::<u16>();
            if val.is_ok() {
                gate.value = Some(val.unwrap());
                gate.got_signal = true;
            }
            self.gates.push(gate);

            // we also add place in the adjacency list
            self.adj.push(LinkedList::new());
            return n;
        }
    }

    pub fn add_gate_from_insruction(&mut self, inst: &str) {
        // function to add the gates given in the instruction
        // split the string
        let mut components: Vec<&str> = inst.split(' ').to_owned().collect();

        let name = components.pop().unwrap();
        let i = self.get_idx(name);

        // removing the -> sign
        components.pop();

        // now we add the rest
        match components.len() {
            1 => {
                let j = self.get_idx(components[0]);
                // now we add the connexions
                self.gates[i].parents.push(j);
                self.adj[j].push_back(i);
            }
            2 => {
                let j = self.get_idx(components[1]);
                self.gates[i].op = Opertation::NOT;
                self.gates[i].parents.push(j);
                self.adj[j].push_back(i);
            }

            _ => {
                let j = self.get_idx(components[0]);
                let k = self.get_idx(components[2]);
                self.gates[i].parents.push(j);
                self.gates[i].parents.push(k);
                self.adj[j].push_back(i);
                self.adj[k].push_back(i);

                // now we match the operation
                if components[1] == "AND" {
                    self.gates[i].op = Opertation::AND;
                }

                if components[1] == "OR" {
                    self.gates[i].op = Opertation::OR;
                }

                if components[1] == "LSHIFT" {
                    self.gates[i].op = Opertation::LSHIFT
                }

                if components[1] == "RSHIFT" {
                    self.gates[i].op = Opertation::RSHIFT;
                }
            }
        }
    }
}
