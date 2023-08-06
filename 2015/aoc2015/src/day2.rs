use std::{fs::File, io::Read};
#[derive(Debug)]
struct Box {
    width: i32, // width
    length: i32,
    height: i32,
}

impl Box {
    pub fn new(dim: Vec<i32>) -> Self {
        // function to create the box from the vector of dimensions
        Self {
            width: dim[0],
            length: dim[1],
            height: dim[2],
        }
    }
    pub fn needed_paper(&self) -> i32 {
        // function to return the needed paper for the box
        let mut surfaces = vec![
            self.width * self.length,
            self.width * self.height,
            self.length * self.height,
        ];

        // sorting
        surfaces.sort();

        // now we reteurn the value
        2 * surfaces.iter().sum::<i32>() + surfaces.iter().min().unwrap()
    }
    pub fn needed_rubon(&self) -> i32 {
        // needed rubon for the box
        let mut dims = vec![self.width, self.length, self.height];

        // sorting
        dims.sort();

        2 * (dims[0] + dims[1]) + dims[0] * dims[1] * dims[2]
    }
}
fn main() {
    // Reading the input file
    let mut file = File::open("input/day2").expect("Invalid input day 2 ");

    // reading the content in the string
    let mut entries = String::new();

    // answer
    let mut answer = 0;
    let mut answer2 = 0;

    file.read_to_string(&mut entries)
        .expect("Invalid content in the input");

    for line in entries.lines() {
        // now we vectorize the lines
        let dimensions: Vec<i32> = line.split('x').map(|x| x.parse::<i32>().unwrap()).collect();

        // Now creating the box
        let b = Box::new(dimensions);

        // now we compute the paper size associated with b
        answer += b.needed_paper();

        // computing for part 2
        answer2 += b.needed_rubon();
    }
    println!("Part 1 : {answer}");
    println!("Part 2 : {answer2}");
}
