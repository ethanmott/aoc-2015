use common::files;
use std::cmp::max;

fn main() {
    let ingredients: Vec<Ingredient> = files::get_file_lines("day15.txt").iter()
        .map(|l| parse_ingredient(l))
        .collect();

    println!("{:?}", solve(ingredients, 100));
}

fn solve(ingredients: Vec<Ingredient>, num_teaspoons: i32) -> (i32, i32) {
    let mut max_score = 0;
    let mut max_score_500_cals = 0;

    for i in 0..=num_teaspoons {
        for j in 0..=num_teaspoons - i {
            for k in 0..=num_teaspoons - i - j {
                for l in 0..=num_teaspoons - i - j - k {
                    if i + j + k + l == num_teaspoons {
                        let sugar = ingredients.get(0).unwrap();
                        let sprinkles = ingredients.get(1).unwrap();
                        let candy = ingredients.get(2).unwrap();
                        let chocolate = ingredients.get(3).unwrap();

                        let capacity = i * sugar.capacity + j * sprinkles.capacity + k * candy.capacity + l * chocolate.capacity;
                        let durability = i * sugar.durability + j * sprinkles.durability + k * candy.durability + l * chocolate.durability;
                        let flavor = i * sugar.flavor + j * sprinkles.flavor + k * candy.flavor + l * chocolate.flavor;
                        let texture = i * sugar.texture + j * sprinkles.texture + k * candy.texture + l * chocolate.texture;
                        let calories = i * sugar.calories + j * sprinkles.calories + k * candy.calories + l * chocolate.calories;

                        let total_score = max(0, capacity) * max(0, durability) * max(0, flavor) * max(0, texture);

                        max_score = max(max_score, total_score);

                        if calories == 500 {
                            max_score_500_cals = max(max_score_500_cals, total_score);
                        }
                    }
                }
            }
        }
    }

    (max_score, max_score_500_cals)
}

#[derive(Clone, Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_ingredient(line: &String) -> Ingredient {
    let parts: Vec<&str> = line.split(' ').collect();

    let mut capacity = parts[2].to_string();
    capacity.pop();

    let mut durability = parts[4].to_string();
    durability.pop();

    let mut flavor = parts[6].to_string();
    flavor.pop();

    let mut texture = parts[8].to_string();
    texture.pop();

    Ingredient {
        name: parts[0].to_string(),
        capacity: capacity.parse::<i32>().unwrap(),
        durability: durability.parse::<i32>().unwrap(),
        flavor: flavor.parse::<i32>().unwrap(),
        texture: texture.parse::<i32>().unwrap(),
        calories: parts[10].parse::<i32>().unwrap(),
    }
}