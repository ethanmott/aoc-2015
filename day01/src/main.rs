use common::files;

fn main() {
    let bytes = files::get_file_bytes("day01.txt");

    println!("{}", day01(&bytes));
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