use common::files;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: i32,
    stamina: i32,
    rest_duration: i32,
    distance_traveled: i32,
    points: i32,
}

impl Reindeer {
    fn new(name: String, speed: i32, stamina: i32, rest_duration: i32) -> Reindeer {
        Reindeer {
            name,
            speed,
            stamina,
            rest_duration,
            distance_traveled: 0,
            points: 0,
        }
    }
}

fn main() {
    let reindeer: Vec<Reindeer> = files::get_file_lines("day14.txt").iter()
        .map(|l| parse_reindeer(l))
        .collect();

    let race_duration = 2503;

    let (distance_winner, points_winner) = race(reindeer, race_duration);
    println!("part1: {}", distance_winner.distance_traveled);
    println!("part2: {}", points_winner.points);
}

fn race(mut reindeer: Vec<Reindeer>, duration: i32) -> (Reindeer, Reindeer) {
    for i in 1..duration+1 {
        let mut leader_distance = 0;

        for r in reindeer.iter_mut() {
            let is_moving =
                i % (r.stamina + r.rest_duration) <= r.stamina
                    && i % (r.stamina + r.rest_duration) != 0;

            if is_moving {
                r.distance_traveled += r.speed;
            }

            leader_distance = max(leader_distance, r.distance_traveled);
        }

        for r in reindeer.iter_mut() {
            if r.distance_traveled == leader_distance {
                r.points += 1;
            }
        }
    }

    let distance_winner = reindeer.iter().max_by_key(|&r| r.distance_traveled).unwrap().clone();
    let points_winner = reindeer.iter().max_by_key(|&r| r.points).unwrap().clone();

    (distance_winner, points_winner)
}

fn parse_reindeer(line: &String) -> Reindeer {
    let parts: Vec<&str> = line.split(' ').collect();

    Reindeer::new(
        parts[0].to_string(),
        parts[3].parse::<i32>().unwrap(),
        parts[6].parse::<i32>().unwrap(),
        parts[13].parse::<i32>().unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race() {
        let comet = Reindeer::new("Comet".to_string(), 14, 10, 127);
        let dancer = Reindeer::new("Dancer".to_string(), 16, 11, 162);

        let (distance_winner, points_winner) = race(vec![comet, dancer], 1000);
        assert_eq!(distance_winner.distance_traveled, 1120);
        assert_eq!(points_winner.points, 689);
    }
}
