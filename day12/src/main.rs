use std::fs::File;
use std::io::{BufReader, Read};
use serde_json::Value;

fn main() {
    let contents = read_file("input/day12.json");

    let v: Value = serde_json::from_str(contents.as_str()) .unwrap();

    println!("day13.1: {}", visit_part1(v.clone()));
    println!("day13.2: {}", visit_part2(v.clone()));
}

fn visit_part1(value: Value) -> i64 {
    if value.is_number() {
       return value.as_i64().unwrap();
    } else if value.is_object() {
        let mut sum = 0;

        for (_k, v) in value.as_object().unwrap() {
            sum += visit_part1(v.clone());
        }

        return sum;
    } else if value.is_array() {
        let mut sum = 0;

        for v in value.as_array().unwrap() {
            sum += visit_part1(v.clone());
        }

        return sum;
    }

    return 0;
}

fn visit_part2(value: Value) -> i64 {
    if value.is_number() {
        return value.as_i64().unwrap();
    } else if value.is_object() {
        let obj = value.as_object().unwrap();

        for (_k, v) in obj {
            if v.is_string() && v.as_str().unwrap() == "red" {
                return 0;
            }
        }

        let mut sum = 0;

        for (_k, v) in obj {
            sum += visit_part2(v.clone());
        }

        return sum;
    } else if value.is_array() {
        let mut sum = 0;

        for v in value.as_array().unwrap() {
            sum += visit_part2(v.clone());
        }

        return sum;
    }

    return 0;
}

fn read_file(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };

    contents
}
