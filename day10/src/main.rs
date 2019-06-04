fn main() {
    let mut result_part1 = "1113222113".to_string();

    for _ in 0..40 {
        result_part1 = look_and_say(result_part1);
    }

    println!("day 10.1: {}", result_part1.len());

    let mut result_part2 = "1113222113".to_string();

    for _ in 0..50 {
        result_part2 = look_and_say(result_part2);
    }

    println!("day 10.2: {}", result_part2.len());
}

fn look_and_say(s: String) -> String {
    if s.len() == 0 {
        return "".to_string();
    }

    let mut result = String::new();
    let mut chars = s.chars().peekable();
    let mut consecutive_char_count = 1;

    while let Some(c) = chars.next() {
        while chars.peek() == Some(&c) {
            consecutive_char_count += 1;
            chars.next();
        }

        result.push_str(format!("{}{}", consecutive_char_count, c).as_str());
        consecutive_char_count = 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1".to_string()), "11".to_string());
        assert_eq!(look_and_say("11".to_string()), "21".to_string());
        assert_eq!(look_and_say("21".to_string()), "1211".to_string());
        assert_eq!(look_and_say("1211".to_string()), "111221".to_string());
        assert_eq!(look_and_say("111221".to_string()), "312211".to_string());
    }
}