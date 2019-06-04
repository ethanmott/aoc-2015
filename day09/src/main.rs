use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let file = File::open("day09.txt").expect("Error opening day09 input file.");
    let lines: Vec<String> = BufReader::new(file).lines().into_iter()
        .map(|line| line.expect("Failed to read line."))
        .collect();

    println!("{:?}", run(&lines));
}

fn run(lines: &Vec<String>) -> (u32, u32) {
    let mut cities: HashSet<&str> = HashSet::new();
    let mut distances: HashMap<(&str, &str), u32> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();

        let src = parts[0];
        let target = parts[2];
        let distance = parts[4].parse::<u32>().unwrap();

        cities.insert(src);
        cities.insert(target);
        distances.insert((src, target), distance);
        distances.insert((target, src), distance);

        // Starting at "~"
        distances.insert(("~", src), 0);
        distances.insert(("~", target), 0);
    }

    find_path_distances("~", &cities, HashSet::new(), &distances)
}

fn find_path_distances(
    current_city: &str,
    cities: &HashSet<&str>,
    visited: HashSet<&str>,
    distances: &HashMap<(&str, &str), u32>) -> (u32, u32) {
    if visited.len() == cities.len() {
        return (0, 0);
    }

    let mut min = std::u32::MAX;
    let mut max = 0;

    for city in cities {
        if visited.contains(city) {
            continue;
        }

        let mut new_visited = visited.clone();
        new_visited.insert(city);

        let proposed = find_path_distances(city, &cities, new_visited, distances);

        let proposed_min = proposed.0 + distances.get(&(current_city, *city)).unwrap();
        if proposed_min < min {
            min = proposed_min;
        }

        let proposed_max = proposed.1 + distances.get(&(current_city, *city)).unwrap();
        if proposed_max > max {
            max = proposed_max;
        }
    }

    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(&vec![
            "London to Dublin = 464".to_string(),
            "London to Belfast = 518".to_string(),
            "Dublin to Belfast = 141".to_string()
        ]), (605, 982));
    }
}