use serde_json::value;
use serde_json::Map;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

fn sum_numeric_values(data: &Value) -> i64 {
    // Recursive function to compute the sum of the numeric values in the data
    match data {
        Value::Number(a) => a.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(sum_numeric_values).sum(),
        Value::Object(map) => {
            let mut sum = 0;
            for (_key, val) in map {
                sum += sum_numeric_values(val);
            }
            sum
        }
        _ => 0,
    }
}

fn sum_numeric_values_exclude_reds(data: &Value) -> i64 {
    // Recursive function to compute the sum of the numeric values in the data

    let color = String::from("red");

    // first exclude the reds
    if let Value::Object(map) = data {
        for (_key, value) in map {
            if value.is_string() && value.to_string() == "\"red\"" {
                return 0;
            }
        }
    }
    match data {
        Value::Number(a) => a.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(sum_numeric_values_exclude_reds).sum(),
        Value::Object(map) => {
            let mut sum = 0i64;
            for (_key, value) in map {
                sum += sum_numeric_values_exclude_reds(value);
            }
            sum
        }
        _ => 0,
    }
}

fn main() {
    let mut file = File::open("input/day12").expect("Input file day 12 is not found");

    // parsing the content
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("invalid content in the file");

    // Let begin part2
    let parsed: Value = serde_json::from_str(&content).expect("Invalid data");

    // Answer 1
    let answer1 = sum_numeric_values(&parsed);
    println!("Part I = {answer1}");

    // Answer 2
    let answer2 = sum_numeric_values_exclude_reds(&parsed);
    println!("Part II = {answer2}");
}
