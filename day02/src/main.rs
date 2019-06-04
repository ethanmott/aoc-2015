use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total = 0;

    let file = File::open("day02.txt").expect("Error opening day02 input file.");

    for (_index, line) in BufReader::new(file).lines().enumerate() {
        total += line.map(|l| sq_ft_needed(l.as_str()))
            .expect("Error reading day02 input file");
    }

    println!("{}", total);
}

fn sq_ft_needed(line: &str) -> i32 {
    let vec: Vec<i32> = line.split('x')
        .map(|x| x.parse::<i32>().expect("Failed to parse line."))
        .collect::<Vec<i32>>();

    let l = vec[0];
    let w = vec[1];
    let h = vec[2];

    (2 * l * w) + (2 * w * h)  + (2 * h * l) + slack_needed(l, w, h)
}

fn slack_needed(l: i32, w: i32, h: i32) -> i32 {
    if l >= w && l >= h {
        return w * h
    } else if w >= l && w >= h {
        return l * h
    } else {
        return l * w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sq_ft_needed() {
        assert_eq!(sq_ft_needed("2x3x4"), 58);
        assert_eq!(sq_ft_needed("1x1x10"), 43);
    }

    #[test]
    fn test_slack_needed() {
        assert_eq!(slack_needed(2, 3, 4), 6);
        assert_eq!(slack_needed(1, 1, 10), 1);
        assert_eq!(slack_needed(2, 2, 2), 4);
    }
}