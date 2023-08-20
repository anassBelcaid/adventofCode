use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Reinder {
    name: String,
    speed: u32,     // speed of flying
    fly_time: u32,  // time to rest
    rest_time: u32, // rest time
    position: u32,
    // state of the reinder time for either the running staete (positive number ,
    // or negative number representing the time to complete the rest)
    state: i32,
}

impl Reinder {
    #[inline]
    pub fn new(name: String, speed: u32, fly_time: u32, rest_time: u32) -> Self {
        Self {
            name,
            speed,
            fly_time,
            rest_time,
            state: fly_time as i32,
            position: 0,
        }
    }

    fn from(line: &str) -> Self {
        // function to get the instruction from a description

        let tokens: Vec<String> = line.split(' ').map(|x| x.to_string()).collect();
        let name = tokens[0].to_string();
        let speed = tokens[3].parse::<u32>().unwrap();
        let fly_time = tokens[6].parse::<u32>().unwrap();
        let rest_time = tokens[13].parse::<u32>().unwrap();
        Self::new(name, speed, fly_time, rest_time)
    }

    pub fn step(&mut self) {
        // function to advance the reinder in the next second
        if self.state > 0 {
            // Reinder was running
            self.position += self.speed;
            self.state -= 1;
            if self.state == 0 {
                // we need to rest
                self.state = -(self.rest_time as i32);
            }
        } else {
            // Reinder was resting
            self.state += 1;
            if self.state == 0 {
                self.state = self.fly_time as i32;
            }
        }
    }
}
fn main() {
    let mut file = File::open("input/day14").expect("invalid file");

    // REading the content of the file
    let mut lines = String::new();
    file.read_to_string(&mut lines).expect("Invalid content");

    let mut reinders = Vec::new();
    for line in lines.lines() {
        reinders.push(Reinder::from(line));
    }

    for _ in 0..2503 {
        for reinder in reinders.iter_mut() {
            reinder.step();
        }
    }

    let answer1 = reinders.iter().map(|x| x.position).max().unwrap();
    println!("part I = {answer1}");
}
