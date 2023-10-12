fn partI(door: &str) -> String {
    // function to find the password of the first part
    // of the problem
    let mut post = 0; // value that we add to add
    let mut result = String::new();

    while result.len() < 8 {
        // compute the value of the post
        let string = format!("{door}{post}");

        let hash = format!("{:x}", md5::compute(&string));

        if hash.starts_with("00000") {
            result.push(hash.chars().nth(5).unwrap());
        }

        post += 1;
    }
    result
}
pub fn main() {
    // let door = "abc".to_string();
    let door = "ugkcyxxp".to_string();

    let answer1 = partI(&door);
    println!("part I = {answer1}");
}
