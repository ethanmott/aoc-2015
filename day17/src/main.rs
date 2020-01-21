use common::files;
use std::cmp::min;

fn main() {
    let containers: Vec<i32> = files::get_file_lines("day17.txt").iter()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    println!("{:?}", solve(&containers, 150));
}

fn solve(containers: &Vec<i32>, target_sum: i32) -> (i32, i32) {
    let current_combo: Vec<i32> = Vec::new();
    let mut combos: Vec<Vec<i32>> = Vec::new();

    permute(containers, target_sum, &current_combo, &mut combos);

    let mut min_size = std::usize::MAX;

    for combo in combos.iter() {
        min_size = min(min_size, combo.len());
    }

    let matching: Vec<&Vec<i32>> = combos.iter()
        .filter(|&c| c.len() == min_size)
        .collect();

    (combos.len() as i32, matching.len() as i32)
}

fn permute(
    containers: &Vec<i32>,
    target_sum: i32,
    current_combo: &Vec<i32>,
    combos: &mut Vec<Vec<i32>>
) {
    let sum: i32 = current_combo.iter().sum();

    if sum == target_sum {
        combos.push(current_combo.clone());
    } else if sum > target_sum {
        return;
    }

    for (i, _) in containers.iter().enumerate() {
        let remaining = &containers[i+1..].to_vec();
        let mut c = current_combo.clone();
        c.push(containers[i]);

        permute(remaining, target_sum, &c, combos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let containers = vec![20, 15, 10, 5, 5];

        assert_eq!(solve(&containers, 25), (4, 3))
    }
}
