use std::collections::HashMap;

// naming the type
type Cache = HashMap<String, u16>;

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
    pub name: String,
    pub op: Opertation,
    pub parents: Vec<String>,
    pub value: u16,
}

impl Gate {
    pub fn from(instruction: &str) -> Self {
        // println!("got line {instruction}");
        let mut components: Vec<String> = instruction.split(' ').map(|x| x.to_string()).collect();

        // the name is always in the end
        let name = components.last().unwrap().clone();

        // We create the vector of the parent and we initiate it after
        let mut parents = Vec::new();

        // we initiate the value
        let mut value = 0;

        // and finaly we initiate the operation
        let mut op = Opertation::NONE;

        // now we extracted the name, we remove it and and also the arrow sign ->
        components.pop();
        components.pop();

        match components.len() {
            1 => {
                let parent = components[0].to_string();
                if parent.parse::<i32>().is_ok() {
                    value = parent.parse().unwrap();
                } else {
                    parents.push(parent);
                }
            }
            2 => {
                op = Opertation::NOT;
                parents.push(components[1].clone());
            }
            _ => {
                parents.push(components[0].clone());
                parents.push(components[2].clone());
                if components[1] == "AND".to_string() {
                    op = Opertation::AND;
                }

                if components[1] == "OR".to_string() {
                    op = Opertation::OR;
                }

                if components[1] == "LSHIFT".to_string() {
                    op = Opertation::LSHIFT;
                }

                if components[1] == "RSHIFT".to_string() {
                    op = Opertation::RSHIFT;
                }
            }
        }

        Self {
            name,
            op,
            parents,
            value,
        }
    }
    pub fn reduce(&mut self, cache: &mut Cache) {
        // function to recude anc compute the value of the gate
        // println!("reducing {:?}", self);

        if self.parents.len() == 2 {
            let left = cache[&self.parents[0]];
            let right = cache[&self.parents[1]];
            // println!("values are  left = {}, right = {}", left, right);
            self.parents.clear();

            match self.op {
                Opertation::OR => self.value = left | right,
                Opertation::AND => self.value = left & right,
                Opertation::LSHIFT => self.value = left.wrapping_shl(right as u32),
                Opertation::RSHIFT => self.value = left.wrapping_shr(right as u32),
                _ => panic!("Unknown operation"),
            }
        } else {
            let left = cache[&self.parents[0]];
            self.parents.clear();

            match self.op {
                Opertation::NOT => self.value = !left,
                Opertation::NONE => self.value = left,
                _ => panic!("Unknown operation"),
            }
        }
        // println!("{} => {}", self.name, self.value);

        cache.insert(self.name.clone(), self.value);
    }

    pub fn all_known(&self, cache: &Cache) -> bool {
        //verify if all the parent have values
        for p in self.parents.iter() {
            if !cache.contains_key(p) {
                return false;
            }
        }
        true
    }
}
