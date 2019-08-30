use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;
use common::files;

fn main() {
    let lines = files::get_file_lines("day05.txt");

    let part1_result = lines.iter()
        .filter(|line| part1_is_nice(line))
        .count();

    println!("day05.1: {}", part1_result);

    let part2_result = lines.iter()
        .filter(|line| part2_is_nice(line))
        .count();

    println!("day05.2: {}", part2_result);
}

fn part1_is_nice(s: &str) -> bool {
    let blacklist = vec!["ab", "cd", "pq", "xy"];

    has_3_vowels(s) && has_repeating(s) && !violates_blacklist(s, &blacklist)
}

fn has_3_vowels(s: &str) -> bool {
    Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap().is_match(s)
}

fn has_repeating(s: &str) -> bool {
    for i in 1..s.len() {
        let c1 = &s[i - 1..i];
        let c2 = &s[i..i + 1];

        if c1 == c2 {
            return true;
        }
    }

    false
}

fn violates_blacklist(s: &str, blacklist: &[&str]) -> bool {
    blacklist.iter()
        .any(|&blacklisted_str| s.contains(blacklisted_str))
}

type NGrams<'a> = HashMap<&'a str, Vec<Range<usize>>>;

fn part2_is_nice(s: &str) -> bool {
    let two_grams = build_ngrams(s, 2);
    let three_grams = build_ngrams(s, 3);

    let r1 = two_grams.iter()
        .any(|(_, ranges)|
            ranges.iter()
                .any(|r1|
                    ranges.iter()
                        .any(|r2|
                            (r1.start < r2.start && r1.end <= r2.start) || (r1.start > r2.end && r1.end >= r1.end)
                        )
                )
        );

    let r2 = three_grams.iter()
        .any(|(&s, _)| s.ends_with(s.chars().next().unwrap()));

    r1 && r2
}

fn build_ngrams(s: &str, n: usize) -> NGrams {
    let mut n_grams = NGrams::new();

    if n > s.len() {
        return n_grams;
    }

    for i in n - 1..s.len() {
        let n_gram = &s[i - (n - 1)..i + 1];

        if !n_grams.contains_key(n_gram) {
            n_grams.insert(n_gram, Vec::new());
        }

        n_grams.get_mut(n_gram).unwrap().push(i - (n - 1)..i + 1);
    }

    n_grams
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_is_nice() {
        assert_eq!(part1_is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(part1_is_nice("aaa"), true);
        assert_eq!(part1_is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(part1_is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(part1_is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_part2_is_nice() {
        assert_eq!(part2_is_nice("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(part2_is_nice("xxyxx"), true);
        assert_eq!(part2_is_nice("uurcxstgmygtbstg"), false);
        assert_eq!(part1_is_nice("ieodomkazucvgmuy"), false);
    }

    #[test]
    fn test_build_ngrams() {
        assert_eq!(build_ngrams("", 1).len(), 0);
        assert_eq!(build_ngrams("testtt", 10).len(), 0);

        let result = build_ngrams("xxxa", 2);

        assert_eq!(result.len(), 2);
        assert_eq!(result.get("xx").unwrap().len(), 2);
        assert_eq!(result.get("xa").unwrap().len(), 1);
    }
}