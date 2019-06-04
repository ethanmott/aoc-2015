use std::fs::read_to_string;

fn main() {
    let prefix = read_to_string("input/day04.txt").expect("Error opening day04 input file.");

    println!("day04: {}", find_matching_hash(prefix.as_str(), "00000"));
}

fn find_matching_hash(prefix: &str, needle: &str) -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        let candidate = format!("{}{}", prefix, counter.to_string());
        let hash = md5::compute(&candidate);
        let hash_str = format!("{:?}", hash);

        if hash_str.starts_with(needle) {
            break counter;
        }
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(find_matching_hash("abcdef", "00000"), 609043);
        assert_eq!(find_matching_hash("pqrstuv", "00000"), 1048970);
    }
}