use std::fs::File;
use std::io::Read;

pub fn is_valid(a: i32, b: i32, c: i32) -> bool {
    // function to verify if the triangle is good
    a + b > c && a + c > b && b + c > a
}

pub fn part1(matrix: &Vec<Vec<i32>>) -> usize {
    // function to get the answer to part1

    matrix
        .iter()
        .filter(|line| is_valid(line[0], line[1], line[2]))
        .count()
}

pub fn part2(matrix: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;
    for chunk in matrix.chunks(3) {
        for j in 0..3 {
            if is_valid(chunk[0][j], chunk[1][j], chunk[2][j]) {
                count += 1;
            }
        }
    }
    count
}

pub fn get_triange_matrix() -> Vec<Vec<i32>> {
    let mut file = File::open("input/day3").expect("file day3 not found");
    let mut triangles = String::new();

    file.read_to_string(&mut triangles)
        .expect("invalid content");

    let mut matrix = Vec::new();

    for line in triangles.lines() {
        if line != "" {
            let mut lenghts = Vec::new(); // line
            for token in line.split(' ') {
                if token != "" {
                    lenghts.push(token.parse::<i32>().unwrap());
                }
            }
            matrix.push(lenghts);
        }
    }
    matrix
}
pub fn main() {
    // answer to the first part
    let matrix = get_triange_matrix();

    let answer1 = part1(&matrix);
    println!("Part I = {answer1}");

    // computing part2
    let answer2 = part2(&matrix);
    println!("Part II = {answer2}");
}
