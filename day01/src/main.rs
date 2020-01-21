use common::files;

fn main() {
    let bytes = files::get_file_bytes("day01.txt");

    println!("{:?}", day01(&bytes));
}

fn day01(input: &[u8]) -> (i32, Option<i32>) {
    let mut floor: i32 = 0;
    let mut first_basement_position: Option<i32> = None;

    for (i, c) in input.iter().enumerate() {
        match c {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => panic!("Invalid command character.")
        }

        if floor == -1 && first_basement_position.is_none() {
            first_basement_position = Some((i + 1) as i32);
        }
    }

    (floor, first_basement_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        assert_eq!(day01("".as_bytes()), (0, None));
        assert_eq!(day01("(".as_bytes()), (1, None));
        assert_eq!(day01(")".as_bytes()), (-1, Some(1)));
        assert_eq!(day01("((".as_bytes()), (2, None));
        assert_eq!(day01("))".as_bytes()), (-2, Some(1)));
        assert_eq!(day01("()(".as_bytes()), (1, None));
        assert_eq!(day01(")))))(((((".as_bytes()), (0, Some(1)));
        assert_eq!(day01("()())".as_bytes()), (-1, Some(5)));
    }
}