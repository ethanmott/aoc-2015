use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input/day01.txt").expect("Error opening day01 input file.");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Error reading day01 input file.");

    println!("{}", day01(&buf));
}

fn day01(input: &[u8]) -> i32 {
    let mut floor: i32 = 0;

    for c in input {
        match c {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => panic!("Invalid command character.")
        }
    }

    floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        assert_eq!(day01("".as_bytes()), 0);
        assert_eq!(day01("(".as_bytes()), 1);
        assert_eq!(day01(")".as_bytes()), -1);
        assert_eq!(day01("((".as_bytes()), 2);
        assert_eq!(day01("))".as_bytes()), -2);
        assert_eq!(day01("()(".as_bytes()), 1);
        assert_eq!(day01(")))))(((((".as_bytes()), 0);
    }
}