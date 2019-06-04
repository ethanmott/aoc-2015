use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input/day06.txt").expect("Error opening day06 input file.");
    let lines: Vec<String> = BufReader::new(file).lines().into_iter()
        .map(|line| line.expect("Failed to read line."))
        .collect();
    let instructions: Vec<Instruction> = lines.iter()
        .map(|l| parse_instruction(l))
        .collect();

    let mut part1_grid: LightGrid<bool> = LightGrid::<bool>::new();
    let mut part2_grid: LightGrid<u32> = LightGrid::<u32>::new();

    instructions.iter()
        .for_each(|i| {
            part1_grid.handle(i);
            part2_grid.handle(i);
        });

    let part1_lights_on_count = part1_grid.lights.iter()
        .flat_map(|r| r.iter())
        .filter(|&b| *b == true)
        .count();

    let part2_total_brightness: u32 = part2_grid.lights.iter()
        .flat_map(|r| r.iter())
        .sum();

    println!("day06.1: {}", part1_lights_on_count);
    println!("day06.2: {}", part2_total_brightness);
}

const GRID_WIDTH: usize = 1000;

struct LightGrid<T> {
    lights: Vec<Vec<T>>
}

impl LightGrid<bool> {
    fn new() -> Self {
        LightGrid {
            lights: vec![vec![false; GRID_WIDTH]; GRID_WIDTH]
        }
    }
}

impl LightGrid<u32> {
    fn new() -> Self {
        LightGrid {
            lights: vec![vec![0; GRID_WIDTH]; GRID_WIDTH]
        }
    }
}

trait InstructionHandler<T> {
    fn handle(&mut self, i: &Instruction);
}

impl InstructionHandler<LightGrid<bool>> for LightGrid<bool> {
    fn handle(&mut self, i: &Instruction) {
        let rows = self.lights.iter_mut()
            .skip(i.top_left.x as usize)
            .take((i.bottom_right.x - i.top_left.x + 1) as usize);

        for row in rows {
            let values = row.iter_mut()
                .skip(i.top_left.y as usize)
                .take((i.bottom_right.y - i.top_left.y + 1) as usize);

            for is_on in values {
                match i.instruction_type {
                    InstructionType::TurnOn => *is_on = true,
                    InstructionType::TurnOff => *is_on = false,
                    InstructionType::Toggle => *is_on = !*is_on
                }
            }
        }
    }
}

impl InstructionHandler<LightGrid<u32>> for LightGrid<u32> {
    fn handle(&mut self, i: &Instruction) {
        let rows = self.lights.iter_mut()
            .skip(i.top_left.x as usize)
            .take((i.bottom_right.x - i.top_left.x + 1) as usize);

        for row in rows {
            let values = row.iter_mut()
                .skip(i.top_left.y as usize)
                .take((i.bottom_right.y - i.top_left.y + 1) as usize);

            for brightness in values {
                match i.instruction_type {
                    InstructionType::TurnOn => *brightness += 1,
                    InstructionType::TurnOff => {
                        if *brightness > 0 {
                            *brightness -= 1
                        }
                    },
                    InstructionType::Toggle => *brightness += 2
                }
            }
        }
    }
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive(Debug)]
enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    top_left: Coordinate,
    bottom_right: Coordinate
}

fn parse_instruction(line: &str) -> Instruction {
    let typ = line.chars()
        .take_while(|c| !c.is_numeric())
        .collect::<String>();

    let (top_left, bottom_right) = parse_coordinates(&line[typ.len()..]);

    Instruction {
        instruction_type: parse_instruction_type(typ.as_str()),
        top_left,
        bottom_right
    }
}

fn parse_instruction_type(s: &str) -> InstructionType {
    match s {
        "turn on " => InstructionType::TurnOn,
        "turn off " => InstructionType::TurnOff,
        "toggle " => InstructionType::Toggle,
        _ => panic!("Unknown instruction type: {}", s)
    }
}

// ¯\_(ツ)_/¯
fn parse_coordinates(s: &str) -> (Coordinate, Coordinate) {
    let mut split = s.split(" through ");
    let mut c1 = split.next().unwrap().split(",");
    let mut c2 = split.next().unwrap().split(",");

    (
        Coordinate {
            x: c1.next().unwrap().parse::<i32>().unwrap(),
            y: c1.next().unwrap().parse::<i32>().unwrap()
        },
        Coordinate {
            x: c2.next().unwrap().parse::<i32>().unwrap(),
            y: c2.next().unwrap().parse::<i32>().unwrap()
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = vec![
            "turn on 0,0 through 999,999".to_string(),
            "toggle 0,0 through 999,0".to_string(),
            "turn off 499,499 through 500,500".to_string()
        ];

        let instructions: Vec<Instruction> = lines.iter()
            .map(|l| parse_instruction(l))
            .collect();

        let mut grid = LightGrid::<bool>::new();

        instructions.iter()
            .for_each(|i| grid.handle(i));

        let lights_on_count = grid.lights.iter()
            .flat_map(|r| r.iter())
            .filter(|&b| *b == true)
            .count();

        assert_eq!(lights_on_count, 998996);
    }

    #[test]
    fn test_part2() {
        let lines = vec![
            "turn on 0,0 through 0,0".to_string(),
            "toggle 0,0 through 999,999".to_string()
        ];

        let instructions: Vec<Instruction> = lines.iter()
            .map(|l| parse_instruction(l))
            .collect();

        let mut grid = LightGrid::<u32>::new();

        instructions.iter()
            .for_each(|i| grid.handle(i));

        let total_brightness: u32 = grid.lights.iter()
            .flat_map(|r| r.iter())
            .sum();

        assert_eq!(total_brightness, 2000001);
    }
}