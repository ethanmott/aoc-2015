fn main() {
    let input = "hxbxxyzz";

    println!("{:?}", get_next_pw(input));
}

fn get_next_pw(seed: &str) -> String {
    let mut candidate = increment_pw(seed);

    while !has_increasing_straight(candidate.as_str())
        || contains_forbidden_letters(candidate.as_str())
        || num_unique_pairs(candidate.as_str()) < 2 {
        candidate = increment_pw(candidate.as_str());
    }

    candidate
}

fn increment_pw(s: &str) -> String {
    let mut result = String::new();
    let mut done_incrementing = false;

    for c in s.chars().rev() {
        if done_incrementing {
            result.push(c);
        } else {
            let next_char = (((c as u8 - b'a') + 1) % 26 + b'a') as char;
            if next_char != 'a' {
                done_incrementing = true;
            }

            result.push(next_char);
        }
    }

    result.chars().rev().collect::<String>()
}

fn has_increasing_straight(s: &str) -> bool {
    for x in b'a'..=b'x' {
        let mut straight = String::new();
        straight.push(x as char);
        straight.push((x + 1) as char);
        straight.push((x + 2) as char);

        if s.contains(straight.as_str()) {
            return true
        }
    }

    false
}

fn contains_forbidden_letters(s: &str) -> bool {
    s.contains("i") || s.contains("o") || s.contains("l")
}

fn num_unique_pairs(s: &str) -> i32 {
    let mut num_pairs = 0;

    for x in b'a'..=b'z' {
        let mut pair = String::new();
        pair.push(x as char);
        pair.push(x as char);

        if s.contains(pair.as_str()) {
            num_pairs += 1
        }
    }

    num_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_pw() {
        assert_eq!(get_next_pw("hxbxwxba"), "hxbxxyzz")
    }

    #[test]
    fn test_has_increasing_straight() {
        assert_eq!(has_increasing_straight("abc"), true);
        assert_eq!(has_increasing_straight("bcd"), true);
        assert_eq!(has_increasing_straight("xyz"), true);
        assert_eq!(has_increasing_straight("zzzabc"), true);

        assert_eq!(has_increasing_straight(""), false);
        assert_eq!(has_increasing_straight("ab"), false);
        assert_eq!(has_increasing_straight("abxycz"), false);
    }

    #[test]
    fn test_num_unique_pairs() {
        assert_eq!(num_unique_pairs("ab"), 0);
        assert_eq!(num_unique_pairs("abb"), 1);
        assert_eq!(num_unique_pairs("abbb"), 1);
        assert_eq!(num_unique_pairs("aabb"), 2);
        assert_eq!(num_unique_pairs("aaaazbbbb"), 2);
    }
}