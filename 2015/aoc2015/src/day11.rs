use std::collections::HashSet;
use std::hash::Hash;
use std::slice::Windows;

#[derive(Debug)]
pub struct Password {
    pub pass: Vec<u8>,
}

impl Password {
    pub fn new(code: &str) -> Self {
        Self {
            pass: code.bytes().collect(),
        }
    }
    pub fn next(&mut self, index: usize) -> bool {
        // function to generate the next element I could use an iterator but
        // for now I will only use the simple function.
        if index == 0 && self.pass[index] == b'z' {
            return false;
        }

        if self.pass[index] == b'z' {
            self.pass[index] = b'a';
            return self.next(index - 1);
        } else {
            self.pass[index] += 1;
            return true;
        }
    }

    pub fn string(&self) -> String {
        // return the string representation of the string
        String::from_utf8(self.pass.clone()).unwrap()
    }
}

pub fn condition1(password: &Password) -> bool {
    // function to verify the first condition which is
    // a straing increasing sequence of at least three characters

    for wind in password.pass.windows(3) {
        if wind[0] == wind[1] - 1 && wind[1] == wind[2] - 1 {
            return true;
        }
    }
    false
}

pub fn condition2(password: &Password) -> bool {
    // function to check the second condition
    //also straighforward
    for b in password.pass.iter() {
        if *b == b'i' || *b == b'o' || *b == b'l' {
            return false;
        }
    }
    true
}

pub fn condition3(password: &Password) -> bool {
    // Now for the third condition
    let mut cache = HashSet::new();

    for wind in password.pass.windows(2) {
        if wind[0] == wind[1] {
            cache.insert(wind[0]);
        }
    }
    cache.len() >= 2
}

fn next_password(password: &mut Password) -> String {
    // function to compute the next password  that verifies all the conditions

    while password.next(7) {
        if condition1(password) && condition2(password) && condition3(password) {
            return password.string();
        }
    }

    return String::new();
}
fn main() {
    // defining my input
    let my_input = "hepxcrrq";

    // Defining the new structure on the input
    let mut pass = Password::new(my_input);

    // computing the first answer
    let answer1 = next_password(&mut pass);
    println!("Part I {answer1}");
}

#[test]
fn test1() {
    let mut password = Password::new("abcdefgh");
    let expected = String::from("abcdffaa");
    let result = next_password(&mut password);
    assert_eq!(expected, result);
}

#[test]
fn test2() {
    let mut password = Password::new("ghijklmn");
    let expected = String::from("ghjaabcc");
    let result = next_password(&mut password);
    assert_eq!(expected, result);
}
