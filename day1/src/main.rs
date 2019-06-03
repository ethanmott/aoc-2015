use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("day1.txt").expect("Error opening day1 input file.");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Error reading day1 input file.");

    println!("{}", day1(&buf));
}

fn day1(input: &[u8]) -> i32 {
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
    fn test_day1() {
        assert_eq!(day1("".as_bytes()), 0);
        assert_eq!(day1("(".as_bytes()), 1);
        assert_eq!(day1(")".as_bytes()), -1);
        assert_eq!(day1("((".as_bytes()), 2);
        assert_eq!(day1("))".as_bytes()), -2);
        assert_eq!(day1("()(".as_bytes()), 1);
        assert_eq!(day1(")))))(((((".as_bytes()), 0);
    }
}