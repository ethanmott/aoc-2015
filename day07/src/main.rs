use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let file = File::open("input/day07.txt").expect("Error opening day07 input file.");
    let nodes: Vec<WiringNode> = BufReader::new(file).lines().into_iter()
        .map(|line| line.expect("Failed to read line."))
        .map(|l| parse_wiring(&l))
        .collect();

    let mut values_part1: HashMap<WireIdentifier, Wiring> = HashMap::new();
    let mut values_part2: HashMap<WireIdentifier, Wiring> = HashMap::new();

    for (wiring, identifier) in nodes {
        values_part1.insert(identifier.clone(), wiring.clone());
        values_part2.insert(identifier.clone(), wiring.clone());
    }

    let part1_result = resolve(&WireOrValue::Wire("a".to_string()), &mut values_part1);

    println!("day 7.1: {}", part1_result);

    values_part2.insert("b".to_string(), Wiring::Direct(WireOrValue::Value(part1_result)));

    let part2_result = resolve(&WireOrValue::Wire("a".to_string()), &mut values_part2);

    println!("day 7.2: {}", part2_result);
}

type WireIdentifier = String;
type WiringNode = (Wiring, WireIdentifier);

#[derive(Clone, Eq, PartialEq)]
#[derive(Debug)]
enum WireOrValue {
    Wire(WireIdentifier),
    Value(u16),
}

#[derive(Clone)]
#[derive(Debug)]
enum Wiring {
    Direct(WireOrValue),
    Not(WireOrValue),
    And(WireOrValue, WireOrValue),
    Or(WireOrValue, WireOrValue),
    LShift(WireOrValue, WireOrValue),
    RShift(WireOrValue, WireOrValue),
}

fn resolve(value: &WireOrValue, values: &mut HashMap<WireIdentifier, Wiring>) -> u16 {
    match value {
        &WireOrValue::Value(v) => v,
        &WireOrValue::Wire(ref wire_id) => {
            let origin = values.get(wire_id).unwrap().clone();
            let val = match origin {
                Wiring::Direct(ref v) => resolve(v, values),
                Wiring::Not(ref v) => !resolve(v, values),
                Wiring::And(ref v1, ref v2) => resolve(v1, values) & resolve(v2, values),
                Wiring::Or(ref v1, ref v2) => resolve(v1, values) | resolve(v2, values),
                Wiring::LShift(ref v1, ref v2) => resolve(v1, values) << resolve(v2, values),
                Wiring::RShift(ref v1, ref v2) => resolve(v1, values) >> resolve(v2, values),
            };

            *values.entry(wire_id.to_string()).or_insert(Wiring::Direct(WireOrValue::Value(0))) = Wiring::Direct(WireOrValue::Value(val));

            val
        }
    }
}

fn parse_wiring(s: &str) -> (Wiring, WireIdentifier) {
    let parts: Vec<&str> = s.split(" ").collect();

    match parts.len() {
        3 => (Wiring::Direct(parse_wire_or_value(parts[0])), parts[2].to_string()),
        4 => (Wiring::Not(parse_wire_or_value(parts[1])), parts[3].to_string()),
        5 => {
            let left = parse_wire_or_value(parts[0]);
            let right = parse_wire_or_value(parts[2]);
            let wiring = match parts[1] {
                "AND" => Wiring::And(left, right),
                "OR" => Wiring::Or(left, right),
                "LSHIFT" => Wiring::LShift(left, right),
                "RSHIFT" => Wiring::RShift(left, right),
                x => panic!("Invalid wiring: {}", x)
            };

            (wiring, parts[4].to_string())
        }
        _ => panic!("Unknown wiring: {}", s)
    }
}

fn parse_wire_or_value(s: &str) -> WireOrValue {
    let result = s.parse::<u16>();

    match result {
        Ok(value) => WireOrValue::Value(value),
        Err(_) => WireOrValue::Wire(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wire_or_value() {
        assert_eq!(parse_wire_or_value("abc"), WireOrValue::Wire("abc".to_string()));
        assert_eq!(parse_wire_or_value("1"), WireOrValue::Value(1));
        assert_eq!(parse_wire_or_value("65535"), WireOrValue::Value(65535));
    }
}