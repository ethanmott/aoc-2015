use std::collections::HashMap;
use common::files;

fn main() {
    let sues: Vec<Sue> = files::get_file_lines("day16.txt").iter()
        .map(|l| parse_sue(l))
        .collect();

    let mut known_criteria = Criteria::new();
    known_criteria.insert("children".to_string(), 3);
    known_criteria.insert("cats".to_string(), 7);
    known_criteria.insert("samoyeds".to_string(), 2);
    known_criteria.insert("pomeranians".to_string(), 3);
    known_criteria.insert("akitas".to_string(), 0);
    known_criteria.insert("vizslas".to_string(), 0);
    known_criteria.insert("goldfish".to_string(), 5);
    known_criteria.insert("trees".to_string(), 3);
    known_criteria.insert("cars".to_string(), 2);
    known_criteria.insert("perfumes".to_string(), 1);

    println!("part1: {:?}", solve(&sues, &known_criteria, false));
    println!("part2: {:?}", solve(&sues, &known_criteria, true));
}

fn solve(sues: &Vec<Sue>, known_criteria: &Criteria, part2: bool) -> Option<Sue> {
    for sue in sues {
        let mut qualified = true;

        for entry in sue.criteria.keys() {
            let sue_value = sue.criteria.get(entry).unwrap();

            if known_criteria.get(entry).is_none() {
                println!("Unknown entry: {}", entry);
                break;
            }

            let known_value = known_criteria.get(entry).unwrap();

            let greater_than = vec!["cats".to_string(), "trees".to_string()];
            let less_than = vec!["pomeranians".to_string(), "goldfish".to_string()];

            if part2 && greater_than.contains(entry) {
                if sue_value <= known_value {
                    qualified = false;
                }
            } else if part2 && less_than.contains(entry) {
                if sue_value >= known_value {
                    qualified = false
                }
            } else if sue_value != known_value {
                qualified = false;
            }
        }

        if qualified {
            return Some(sue.clone());
        }
    }

    None
}

type Criteria = HashMap<String, i32>;

#[derive(Clone, Debug)]
struct Sue {
    number: i32,
    criteria: Criteria
}

fn parse_sue(line: &String) -> Sue {
    let mut criteria = Criteria::new();

    let parts: Vec<&str> = line.split(' ').collect();

    let mut sue_number = parts[1].to_string();
    sue_number.pop();

    for index in [2 as usize, 4, 6].iter() {
        let mut item = parts[*index].to_string();
        item.pop();

        let mut item_count = parts[*index + 1].to_string();
        if *index != 6 {
            item_count.pop();
        }

        criteria.insert(item, item_count.parse::<i32>().unwrap());
    }

    Sue {
        number: sue_number.parse::<i32>().unwrap(),
        criteria
    }
}