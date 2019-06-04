use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("day08.txt").expect("Error opening day08 input file.");
    let lines: Vec<String> = BufReader::new(file).lines().into_iter()
        .map(|line| line.expect("Failed to read line."))
        .collect();

    let total_chars: i32 = lines.iter()
        .map(|l| l.len() as i32)
        .sum();

    let total_unescaped_chars: i32 = lines.iter()
        .map(|l| {
            unescaped_length(l.as_str()).unwrap()
        })
        .sum();

    println!("day08.1: ({} - {}) = {}", total_chars, total_unescaped_chars, total_chars - total_unescaped_chars);

    let total_escaped_chars: i32 = lines.iter()
        .map(|l| escaped_length(l.as_str()))
        .sum();

    println!("day08.2: ({} - {}) = {}", total_escaped_chars, total_chars, total_escaped_chars - total_chars);
}

fn unescaped_length(s: &str) -> Result<i32, String> {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().enumerate();

    while let Some((_, c)) = chars.next() {
        if c == '\\' {
            match chars.next() {
                None => {
                    return Err(format!("Expected a char after \\ but found none."));
                },
                Some((_, c2)) => {
                    result.push(match c2 {
                        '\\' => '\\',
                        '\"' =>  '\"',
                        'x' => {
                            let _ = chars.next().unwrap();
                            let _ = chars.next().unwrap();

                            '?'
                        },
                        x=>  x
                    });
                }
            }
        } else {
            result.push(c);
        }
    }

    Ok((result.len() as i32) - 2)
}

fn escaped_length(s: &str) -> i32 {
    let mut result = String::with_capacity(s.len() * 4);

    result.push('\"');

    for c in s.chars() {
        if c == '\\' || c == '\"' {
            result.push('\\');
        }

        result.push(c);
    }

    result.push('\"');

    result.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescaped_length() {
        assert_eq!(unescaped_length("\"\"").unwrap(), 0);
        assert_eq!(unescaped_length("\"abc\"").unwrap(), 3);
        assert_eq!(unescaped_length("\"aaa\\\"aaa\"").unwrap(), 7);
        assert_eq!(unescaped_length("\"\\x27\"").unwrap(), 1);
    }

    #[test]
    fn test_escaped_length() {
        assert_eq!(escaped_length("\"\""), 6);
        assert_eq!(escaped_length("\"abc\""), 9);
        assert_eq!(escaped_length("\"aaa\\\"aaa\""), 16);
        assert_eq!(escaped_length("\"\\x27\""), 11);
    }
}