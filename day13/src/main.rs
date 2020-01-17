use common::files;
use itertools::Itertools;

type Person = String;

#[derive(Clone, Debug)]
struct Table {
    seats: Vec<Seat>
}

impl Table {
    pub fn new(size: usize) -> Self {
        let mut seats: Vec<Seat> = Vec::with_capacity(size);

        for _ in 0..size {
            seats.push(Seat::new())
        }

        Table {
            seats
        }
    }

    fn seat_person_in_first_available_seat(&mut self, person: Person) {
        let num_seats = self.seats.len();

        for i in 0..num_seats {
            let mut seat = &mut self.seats[i];

            if seat.person.is_none() {
                seat.person = Some(person.clone());

                // Update seats to the left and right with new person
                self.seats[((i as i32 - 1) + num_seats as i32) as usize % num_seats].right = Some(person.clone());
                self.seats[(i + 1) % num_seats].left = Some(person.clone());

                return;
            }
        }

        panic!("No seats left for person: {}", person);
    }
}

#[derive(Clone, Debug)]
struct Seat {
    left: Option<Person>,
    person: Option<Person>,
    right: Option<Person>
}

impl Seat {
    pub fn new() -> Self {
        Seat {
            left: None,
            person: None,
            right: None
        }
    }
}

#[derive(Clone, Debug)]
struct SeatingRule {
    person: Person,
    next_to_person: Person,
    happiness_modifier: i32
}

fn main() {
    let rules: Vec<SeatingRule> = files::get_file_lines("day13.txt").iter()
        .map(|l| parse_seating_rule(l))
        .collect();

    let people: Vec<Person> = rules.iter()
        .map(|r| r.person.clone())
        .unique()
        .collect();

//    println!("part1 max: {:?}", find_max_total_happiness(people.clone(), rules.clone()));

    let mut people2 = people.clone();
    people2.push(Person::from("Ethan"));

    println!("part2 max: {:?}", find_max_total_happiness(people2.clone(), rules.clone()));
}

fn find_max_total_happiness(people: Vec<Person>, rules: Vec<SeatingRule>) -> i32 {
    let mut max = i32::min_value();

    for permutation in  people.iter().permutations(people.len()) {
        let mut table = Table::new(permutation.clone().len());

        for person in permutation.clone() {
            table.seat_person_in_first_available_seat(person.clone());
        }

        let happiness = get_table_happiness(table.clone(), rules.clone());

        if happiness > max {
            println!("Found new best so far! {} > {}", happiness, max);
            println!("{:?}", permutation.clone());
            max = happiness;
        }
    }

    max
}

fn get_table_happiness(table: Table, rules: Vec<SeatingRule>) -> i32 {
    let mut happiness: i32 = 0;

    for seat in table.seats.iter() {
        happiness += seat.person.clone().map_or(0, |seated_person| -> i32 {
            let mut local_happiness: i32 = 0;

            for rule in rules.iter() {
                if seated_person == rule.person {
                    local_happiness += seat.left.clone()
                        .filter(|left_person| left_person == &rule.next_to_person)
                        .map_or(0, |_| rule.happiness_modifier);

                    local_happiness += seat.right.clone()
                        .filter(|right_person| right_person == &rule.next_to_person)
                        .map_or(0, |_| rule.happiness_modifier);
                }
            }

//            println!("Seat: {:?}, local happiness: {:?}", seat, local_happiness);

            local_happiness
        });
    }

    happiness
}

fn parse_seating_rule(line: &String) -> SeatingRule {
    let parts: Vec<&str> = line.split(' ').collect();

    let person = parts[0].to_string();
    let mut next_to_person = parts[10].to_string();
    // remove the period
    next_to_person.pop();

    let happiness_direction = parts[2];
    let multiplier = match happiness_direction {
        "gain" => 1,
        "lose" => -1,
        _ => 1
    };
    let happiness_modifier_units = parts[3].parse::<i32>().unwrap();

    println!("{:?}", parts);

    SeatingRule {
        person,
        next_to_person,
        happiness_modifier: multiplier * happiness_modifier_units
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {

    }
}
