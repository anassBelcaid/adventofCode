use regex::Regex;
use std::fs;
#[test]
fn test_v1_1() {
    let line = "ADVENT".to_string();
    // /no markers and decompresses to itself with no changes, resulting in a decompressed length of `6`.
    let output = len_decompressed(&line);
    assert_eq!(output, 6);
}
#[test]
fn test_v1_2() {
    let line = "A(1x5)BC".to_string();
    // repeats only the `B` a total of `5` times, becoming `ABBBBBC` for a decompressed length of `7`.
    let output = len_decompressed(&line);
    assert_eq!(output, 7);
}

#[test]
fn test_v1_3() {
    let line = "(3x3)XYZ".to_string();
    let output = len_decompressed(&line);
    assert_eq!(output, 9);
}

#[test]
fn test_v1_4() {
    let line = "A(2x2)BCD(2x2)EFG".to_string();
    let output = len_decompressed(&line);
    assert_eq!(output, 11);
}
#[test]
fn test_v1_5() {
    let line = "(6x1)(1x3)A".to_string();
    let output = len_decompressed(&line);
    assert_eq!(output, 6);
}

#[test]
fn test_v1_6() {
    let line = "X(8x2)(3x3)ABCY".to_string();
    let output = len_decompressed(&line);
    assert_eq!(output, 18);
}

#[test]
fn test_v2_1() {
    let line = "(3x3)XYZ".to_string();
    let output = len_decompressed_v2(&line);
    assert_eq!(output, 9);
}

#[test]
fn test_v2_2() {
    let line = "X(8x2)(3x3)ABCY".to_string();
    let output = len_decompressed_v2(&line);
    assert_eq!(output, 20);
}

#[test]
fn test_v2_3() {
    let line = "(27x12)(20x12)(13x14)(7x10)(1x12)A".to_string();
    let output = len_decompressed_v2(&line);
    assert_eq!(output, 241920);
}

#[test]
fn test_v2_4() {
    let line = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string();
    let output = len_decompressed_v2(&line);
    assert_eq!(output, 445);
}

fn len_decompressed(region: &str) -> usize {
    // function to compute the lenght of the
    // region of interest (could be a slice)
    let pattern = Regex::new(r"\((\d+)x(\d+)\)").expect("invalid regex");

    if let Some(m) = pattern.find(region) {
        // Getting the region of interest in the pattern
        let start = m.start();
        let end = m.end();
        // println!("string is {region}");
        // println!("start is {start}\t end is {end}");

        // now we must get the groups
        let captures = pattern.captures(region).unwrap();

        let num_char: usize = captures[1].parse().unwrap();
        let repeat: usize = captures[2].parse().unwrap();

        return start + num_char * repeat + len_decompressed(&region[end + num_char..]);
    } else {
        return region.len();
    }
}

fn len_decompressed_v2(region: &str) -> usize {
    // function to compute the lenght of the
    // region of interest (could be a slice)
    let pattern = Regex::new(r"\((\d+)x(\d+)\)").expect("invalid regex");

    if let Some(m) = pattern.find(region) {
        // Getting the region of interest in the pattern
        let start = m.start();
        let end = m.end();
        // println!("string is {region}");
        // println!("start is {start}\t end is {end}");

        // now we must get the groups
        let captures = pattern.captures(region).unwrap();

        let num_char: usize = captures[1].parse().unwrap();
        let repeat: usize = captures[2].parse().unwrap();

        return start
            + len_decompressed_v2(&region[end..end + num_char]) * repeat
            + len_decompressed_v2(&region[end + num_char..]);
    } else {
        return region.len();
    }
}

pub fn part_i() -> usize {
    let line = fs::read_to_string("../input/day9").expect("invalid file or content");
    let line = line.trim();

    len_decompressed(&line)
}

pub fn part_ii() -> usize {
    let line = fs::read_to_string("../input/day9").expect("invalid file or content");
    let line = line.trim();

    len_decompressed_v2(&line)
}

pub fn main() {
    // part i
    let answer1 = part_i();
    println!("part I = {answer1}");

    // part ii
    let answer2 = part_ii();
    println!("part II = {answer2}");
}
