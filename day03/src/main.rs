use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Add;


fn main() {
    let mut file = File::open("day03.txt").expect("Error opening day03 input file.");
    let mut moves = Vec::new();
    file.read_to_end(&mut moves).expect("Error reading day03 input file.");

    let houses_visits = house_visits(&moves);

    println!("part1: {}", multivisits_count(houses_visits));
    println!("part2: {}", part2(&moves));
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinates(isize, isize);

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, other: Self) -> Self::Output {
        Coordinates(self.0 + other.0, self.1 + other.1)
    }
}

type HouseVisits = HashMap<Coordinates, usize>;

fn house_visits(moves: &[u8]) -> HouseVisits {
    moves.iter()
        .map(|c| parse_move(c))
        .fold(
            {
                let mut map = HashMap::new();
                let starting_position = Coordinates(0, 0);
                // Start at (0, 0)
                map.insert(starting_position, 1);

                (starting_position, map)
            },
            |(santa, mut houses), next_coordinate_relative| {
                let next_coordinate = santa + next_coordinate_relative;
                houses.entry(next_coordinate).and_modify(|e| *e += 1).or_insert(1);

                (next_coordinate, houses)
            },
        )
        .1
}

fn part2(moves: &[u8]) -> usize {
    let moves_santa: Vec<u8> = moves.iter().step_by(2).cloned().collect();
    let moves_robo_santa: Vec<u8> = moves.iter().skip(1).step_by(2).cloned().collect();

    let mut house_visits_combined = house_visits(moves_santa.as_ref());
    let house_visits_robo_santa = house_visits(moves_robo_santa.as_ref());

    // Combine the two
    house_visits_combined.extend(house_visits_robo_santa);

    multivisits_count(house_visits_combined)
}

fn multivisits_count(house_visits: HouseVisits) -> usize {
    house_visits.iter()
        .filter(|(_, &visit_count)| visit_count >= 1)
        .count()
}

fn parse_move(c: &u8) -> Coordinates {
    match c {
        b'^' => Coordinates(0, 1),
        b'>' => Coordinates(1, 0),
        b'v' => Coordinates(0, -1),
        b'<' => Coordinates(-1, 0),
        _ => Coordinates(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(multivisits_count(house_visits(">".as_bytes())), 2);
        assert_eq!(multivisits_count(house_visits("^>v<".as_bytes())), 4);
        assert_eq!(multivisits_count(house_visits("^v^v^v^v^v".as_bytes())), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("^v".as_bytes()), 3);
        assert_eq!(part2("^>v<".as_bytes()), 3);
        assert_eq!(part2("^v^v^v^v^v".as_bytes()), 11);
    }

    #[test]
    fn test_parse_move() {
        assert_eq!(parse_move(&b'^'), Coordinates(0, 1));
        assert_eq!( parse_move(&b'>'), Coordinates(1, 0));
        assert_eq!( parse_move(&b'v'), Coordinates(0, -1));
        assert_eq!(parse_move(&b'<'), Coordinates(-1, 0));
        assert_eq!(parse_move(&b'x'), Coordinates(0, 0));
    }
}